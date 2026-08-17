use crate::d_event::*;
use crate::d_ticcmd::*;
use crate::doomtype::*;
use crate::i_system::*;

pub const FRACBITS: std::ffi::c_int = 16;

pub const FRACUNIT: std::ffi::c_int = (1 << FRACBITS);

pub type fixed_t = std::ffi::c_int;

static mut rcsid: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn FixedMul(a: fixed_t, b: fixed_t) -> fixed_t {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn FixedDiv(a: fixed_t, b: fixed_t) -> fixed_t {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn FixedDiv2(a: fixed_t, b: fixed_t) -> fixed_t {
    todo!("body not yet translated")
}
