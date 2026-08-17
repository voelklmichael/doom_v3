static mut rcsid: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut myargc: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut myargv: *mut *mut std::ffi::c_char = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn M_CheckParm(check: *mut std::ffi::c_char) -> std::ffi::c_int {
    todo!("body not yet translated")
}
