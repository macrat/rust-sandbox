use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn greeting(name: *const i8) {
    let name = unsafe { CStr::from_ptr(name) };
    println!("hello {}!", name.to_str().unwrap());
}
