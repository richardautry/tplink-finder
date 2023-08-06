use std::net::SocketAddr;
use std::os::raw::{c_char, c_uint, c_void};
use std::ffi::{CString, CStr};
use tplinker::{
    discovery::discover,
    devices::Device,
    capabilities::{DeviceActions, Switch, },
    datatypes::DeviceData,
    datatypes::SysInfo,
    error::{Error, Result},
};
use serde_json::json;
use std::{thread};
use std::time::{Duration, SystemTime};
use tokio::time;
use async_ffi::{FfiFuture, FutureExt};

pub struct FullDevice {
    device: Device,
    device_data: DeviceData,
    addr: SocketAddr
}

pub struct Timer {
    pub seconds: u64,
}

#[no_mangle]
pub extern fn rust_greeting(to: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(to) };
    let recipient = match c_str.to_str() {
        Err(_) => "there",
        Ok(string) => string,
    };

    CString::new("Hello ".to_owned() + recipient).unwrap().into_raw()
}

#[no_mangle]
pub extern fn rust_greeting_free(s: *mut c_char) {
    unsafe {
        if s.is_null() { return }
        CString::from_raw(s)
    };
}

// fn main() {
//     for (addr, data) in discover().unwrap() {
//         let device = Device::from_data(addr, &data);
//         let sysinfo = data.sysinfo();
//         println!("{}\t{}\t{}", addr, sysinfo.alias, sysinfo.hw_type);
//     }
// }

#[no_mangle]
pub extern "C" fn tplinker_discovery(len: *mut c_uint) -> *mut c_void {
    //let mut device_descriptions = Vec::new();
    let mut device_data_objects: Vec<*mut _> = Vec::new();
    for (addr, data) in discover().unwrap() {
        let device = Device::from_data(addr, &data);
        let sysinfo = data.sysinfo();
        // let device_description = CString::new(format!("{}\t{}\t{}", addr, sysinfo.alias, sysinfo.hw_type)).unwrap();
        //let device_description: (CString, CString) = (CString::new(format!("{}", addr)).unwrap(), CString::new(format!("{}", sysinfo.alias)).unwrap());
        // let device_description: [CString; 2] = [CString::new(format!("{}", addr)).unwrap(), CString::new(format!("{}", sysinfo.alias)).unwrap()];
        //device_descriptions.push(device_description);
        // TODO: Need a way to expose device data and allow calling functions on those devices
        // Will need to expose device name and pointer to allow function calls on those devices
        device_data_objects.push(Box::into_raw(Box::new(data)))
    }
    let slice = device_data_objects.into_boxed_slice();
    unsafe {
        *len = slice.len() as c_uint;
    }
    // let boxed = Box::new(device_data_objects);
    
    // TODO: Is this return wrong or incompatible?
    Box::into_raw(slice) as *mut c_void
    // Box::into_raw(boxed) as *mut c_void
}

#[no_mangle]
pub extern "C" fn tplinker_device_discovery(len: *mut c_uint) -> *mut c_void {
    let mut full_devices: Vec<*mut _> = Vec::new();
    for (addr, data) in discover().unwrap() {
        let device: Device = Device::from_data(addr, &data);
        let full_device: FullDevice = FullDevice {
            device: device,
            device_data: data,
            addr: addr
        };
        full_devices.push(Box::into_raw(Box::new(full_device)))
    }
    let slice = full_devices.into_boxed_slice();
    unsafe {
        *len = slice.len() as c_uint;
    }
    Box::into_raw(slice) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn tplinker_vec_destroy(tplinker_vec: *mut i8) {
    let _ = Box::from_raw(tplinker_vec);
}

#[no_mangle]
pub unsafe extern "C" fn device_data_get_alias(device_data: *const DeviceData) -> *mut c_char {
    let device_data = &*device_data;
    println!("RETURNING ALIAS");
    // let device_data: DeviceData = device.send("{}").unwrap();
    let sys_info = device_data.sysinfo();
    // let alias = sys_info.alias.clone();
    // let alias: String = device_data.sysinfo().alias.clone();
    // println!("RETURNING {:?} FOR DEVICE", alias);
    CString::new(sys_info.alias.clone()).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn device_data_get_mac_address(device_data: *const DeviceData) -> *mut c_char {
    let device_data: &DeviceData = &*device_data;
    let sys_info: &SysInfo = device_data.sysinfo();
    CString::new(sys_info.mac.clone()).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn full_device_get_alias(full_device: *const FullDevice) -> *mut c_char {
    let full_device: &FullDevice = &*full_device;
    let sys_info: &SysInfo = full_device.device_data.sysinfo();
    CString::new(sys_info.alias.clone()).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn full_device_get_addr(full_device: *const FullDevice) -> *mut c_char {
    let full_device: &FullDevice = &*full_device;
    CString::new(format!("{}", full_device.addr)).unwrap().into_raw()
}

// TODO: Add device turn on / off
#[no_mangle]
pub unsafe extern "C" fn full_device_is_on(full_device: *const FullDevice) -> bool {
    let full_device: &FullDevice = &*full_device;

    match &full_device.device {
        Device::HS100(device) => { device.is_on().unwrap() },
        Device::Unknown(device) => { 
            let sys_info = match full_device.device.sysinfo() {
                Ok(sys_info) => sys_info,
                Err(e) => return false
            };

            sys_info
            .relay_state
            .map_or(Err(Error::from("No relay state")), |relay_state: u8| {
                Ok(relay_state > 0)
            })
         }.unwrap(),
        _ => false
    }
}

fn check_command_error(value: &serde_json::Value, pointer: &str) -> Result<()> {
    if let Some(err_code) = value.pointer(pointer) {
        if err_code == 0 {
            Ok(())
        } else {
            Err(Error::from(format!("Invalid error code {}", err_code)))
        }
    } else {
        Err(Error::from(format!("Invalid response format: {}", value)))
    }
}

#[no_mangle]
pub unsafe extern "C" fn full_device_switch_off(full_device: *const FullDevice) -> bool {
    let full_device: &FullDevice = &*full_device;
    let device = &full_device.device;
    match device {
        Device::Unknown(device) => {
            let command = json!({
                "system": {"set_relay_state": {"state": 0}}
            }).to_string();
            let result = check_command_error(
                &device.send(&command).unwrap(),
                "/system/set_relay_state/err_code",
            );
            match result {
                Ok(_) => true,
                Err(_) => false,
            }
        },
        _ => false
    }
}

#[no_mangle]
pub unsafe extern "C" fn full_device_switch_on(full_device: *const FullDevice) -> bool {
    let full_device: &FullDevice = &*full_device;
    let device = &full_device.device;
    match device {
        Device::Unknown(device) => {
            let command = json!({
                "system": {"set_relay_state": {"state": 1}}
            }).to_string();
            let result = check_command_error(
                &device.send(&command).unwrap(),
                "/system/set_relay_state/err_code",
            );
            match result {
                Ok(_) => true,
                Err(_) => false,
            }
        },
        _ => false
    }
}

#[no_mangle]
pub unsafe extern "C" fn turn_off_after(length_ms: u64, full_device: *const FullDevice) -> bool {
    let duration: Duration = time::Duration::from_millis(length_ms);
    // TODO: Write a futures version of this that allows us to poll current time elapsed in timer
    thread::sleep(duration);
    full_device_switch_off(full_device)
}

#[no_mangle]
pub unsafe extern "C" fn get_timer(length_ms: u64) -> *mut Timer {
    let timer = Timer {
        seconds: {
            length_ms / 1000
        }
    };
    Box::into_raw(Box::new(timer))
}

#[no_mangle]
pub unsafe extern "C" fn poll_timer(timer: *const Timer) -> u64 {
    let timer = &*timer;
    timer.seconds
}

#[no_mangle]
pub unsafe extern "C" fn start_timer(length_ms: u64, timer: *mut Timer) -> bool {
    let timer_obj = &mut * timer;
    start_timer_async(length_ms, timer_obj);
    true
}

pub fn start_timer_async(length_ms: u64, timer: &mut Timer) {
    let duration: Duration = time::Duration::from_millis(length_ms);
    let mut now = SystemTime::now();
    let end_time = now + duration;
    let mut interval = time::interval(time::Duration::from_secs(1));

    while now.elapsed().expect("").as_secs() < duration.as_secs() {
        // interval.tick().await;
        thread::sleep(Duration::from_secs(1));
        timer.seconds = now.elapsed().expect("Woops wrong time").as_secs();
        println!("{}", timer.seconds);
    }
}

#[no_mangle]
pub unsafe extern "C" fn start_timer_test(length_ms: u64, elapsed: *mut u64) -> FfiFuture<u64> {
    let duration: Duration = time::Duration::from_millis(length_ms);
    let now = SystemTime::now();
    let mut interval = time::interval(time::Duration::from_secs(1));
    let mut now_elapsed = 0;
    async move {
        while now.elapsed().expect("").as_secs() < duration.as_secs() {
            // thread::sleep(Duration::from_secs(1));
            
            interval.tick().await;
            
            now_elapsed = now.elapsed().expect("").as_secs();
            *elapsed = now_elapsed;
            // unsafe {
            //     *elapsed = (now_elapsed * 1000) as c_uint;
            // }
            // println!("{}, {}", now_elapsed, *elapsed);
        }
        now_elapsed
    }
    .into_ffi()
}