use crate::m_fixed::*;

pub const BOXTOP: std::ffi::c_int = 0;
pub const BOXBOTTOM: std::ffi::c_int = BOXTOP + 1;
pub const BOXLEFT: std::ffi::c_int = BOXBOTTOM + 1;
pub const BOXRIGHT: std::ffi::c_int = BOXLEFT + 1;

static mut rcsid: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn M_ClearBox(box_: *mut fixed_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_AddToBox(box_: *mut fixed_t, x: fixed_t, y: fixed_t) {
    todo!("body not yet translated")
}
