use crate::m_fixed::*;

unsafe extern "C" {
    pub static mut finecosine: *mut fixed_t;
}

pub type angle_t = std::ffi::c_uint;

static mut rcsid: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn SlopeDiv(num: std::ffi::c_uint, den: std::ffi::c_uint) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub static mut finetangent: [std::ffi::c_int; (4096) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut finesine: [std::ffi::c_int; (10240) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut tantoangle: [angle_t; (2049) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated
