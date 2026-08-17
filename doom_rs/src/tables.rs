use crate::m_fixed::*;

pub const FINEANGLES: std::ffi::c_int = 8192;

pub const FINEMASK: std::ffi::c_int = (FINEANGLES - 1);

pub const ANGLETOFINESHIFT: std::ffi::c_int = 19;

unsafe extern "C" {
    pub static mut finecosine: *mut fixed_t;
}

pub const ANG45: std::ffi::c_int = 0x20000000;

pub const ANG90: std::ffi::c_int = 0x40000000;

pub const ANG180: std::ffi::c_int = 0x80000000;

pub const ANG270: std::ffi::c_int = 0xc0000000;

pub const SLOPERANGE: std::ffi::c_int = 2048;

pub const SLOPEBITS: std::ffi::c_int = 11;

pub const DBITS: std::ffi::c_int = (FRACBITS - SLOPEBITS);

pub type angle_t = std::ffi::c_uint;

static mut rcsid: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn SlopeDiv(num: std::ffi::c_uint, den: std::ffi::c_uint) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub static mut finetangent: [std::ffi::c_int; (4096) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut finesine: [std::ffi::c_int; (10240) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut tantoangle: [angle_t; (2049) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated
