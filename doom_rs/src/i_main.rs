use crate::d_event::*;
use crate::d_main::*;
use crate::doomdef::*;
use crate::doomtype::*;
use crate::m_argv::*;

static mut rcsid: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn main(
    argc: std::ffi::c_int,
    argv: *mut *mut std::ffi::c_char,
) -> std::ffi::c_int {
    todo!("body not yet translated")
}
