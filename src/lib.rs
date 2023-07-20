use std::os::raw::{c_char, c_uint, c_void};
use std::ffi::{CString, CStr};
use tplinker::{
    discovery::discover,
    devices::Device,
    capabilities::DeviceActions,
    datatypes::DeviceData
};

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
    CString::new(sys_info.mac.clone()).unwrap().into_raw()
}
