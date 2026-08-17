use crate::d_items::*;
use crate::d_player::*;
use crate::d_think::*;
use crate::d_ticcmd::*;
use crate::doomdata::*;
use crate::doomdef::*;
use crate::doomtype::*;
use crate::info::*;
use crate::m_fixed::*;
use crate::p_mobj::*;
use crate::p_pspr::*;
use crate::r_data::*;
use crate::r_defs::*;
use crate::r_state::*;
use crate::tables::*;

static mut rcsid: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut skyflatnum: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut skytexture: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut skytexturemid: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn R_InitSkyMap() {
    todo!("body not yet translated")
}
