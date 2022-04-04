use std::ffi::CString;

fn main() {
    let lib = unsafe { libloading::Library::new("./library.so") };
    match lib {
        Ok(lib) => {
            let func = unsafe { lib.get::<libloading::Symbol<unsafe extern fn(*const i8)>>(b"greeting") };
            match func {
                Ok(func) => {
                    let s = CString::new("world").unwrap();
                    unsafe {
                        func(s.as_ptr());
                    }
                }
                Err(err) => eprintln!("failed to get function: {}", err),
            }
        }
        Err(err) => eprintln!("failed to load library: {}", err),
    }
}
