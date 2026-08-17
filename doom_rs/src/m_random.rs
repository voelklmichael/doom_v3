use crate::doomtype::*;

static mut rcsid: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut rndtable: [std::ffi::c_uchar; (256) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut rndindex: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut prndindex: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn P_Random() -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_Random() -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_ClearRandom() {
    todo!("body not yet translated")
}
