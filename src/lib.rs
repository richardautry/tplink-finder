use std::os::raw::{c_char, c_uint};
use std::ffi::{CString, CStr};
use tplinker::{
    discovery::discover,
    devices::Device,
    capabilities::Switch,
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
pub extern "C" fn tplinker_discovery(len: *mut c_uint) -> *mut c_char {
    let mut device_descriptions = Vec::new();
    for (addr, data) in discover().unwrap() {
        let device = Device::from_data(addr, &data);
        let sysinfo = data.sysinfo();
        // let device_description = CString::new("{addr}\t{sysinfo.alias}\t{sysinfo.hw_type}").unwrap();
        let device_description = CString::new(format!("{}\t{}\t{}", addr, sysinfo.alias, sysinfo.hw_type)).unwrap();
        device_descriptions.push(device_description);
    }
    let slice = device_descriptions.into_boxed_slice();
    unsafe {
        *len = slice.len() as c_uint;
    }
    // unsafe {
    //     *len = slice.len() as c_uint;
    // }

    Box::into_raw(slice) as *mut c_char
}

#[no_mangle]
pub unsafe extern "C" fn tplinker_vec_destroy(tplinker_vec: *mut i8) {
    let _ = Box::from_raw(tplinker_vec);
}