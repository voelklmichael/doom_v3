use crate::d_items::*;
use crate::d_net::*;
use crate::d_player::*;
use crate::d_think::*;
use crate::d_ticcmd::*;
use crate::doomdata::*;
use crate::doomdef::*;
use crate::doomstat::*;
use crate::doomtype::*;
use crate::info::*;
use crate::m_fixed::*;
use crate::p_local::*;
use crate::p_mobj::*;
use crate::p_pspr::*;
use crate::p_spec::*;
use crate::r_bsp::*;
use crate::r_data::*;
use crate::r_defs::*;
use crate::r_draw::*;
use crate::r_local::*;
use crate::r_main::*;
use crate::r_plane::*;
use crate::r_segs::*;
use crate::r_state::*;
use crate::r_things::*;
use crate::tables::*;
use crate::z_zone::*;

static mut rcsid: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut leveltime: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut thinkercap: thinker_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn P_InitThinkers() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_AddThinker(thinker: *mut thinker_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_RemoveThinker(thinker: *mut thinker_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_AllocateThinker(thinker: *mut thinker_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_RunThinkers() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_Ticker() {
    todo!("body not yet translated")
}
