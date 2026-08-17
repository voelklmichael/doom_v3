use crate::doomtype::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ticcmd_t {
    pub forwardmove: std::ffi::c_char,
    pub sidemove: std::ffi::c_char,
    pub angleturn: std::ffi::c_short,
    pub consistancy: std::ffi::c_short,
    pub chatchar: byte,
    pub buttons: byte,
}
