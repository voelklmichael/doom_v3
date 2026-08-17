use crate::d_event::*;
use crate::d_items::*;
use crate::d_net::*;
use crate::d_player::*;
use crate::d_think::*;
use crate::d_ticcmd::*;
use crate::doomdata::*;
use crate::doomdef::*;
use crate::doomstat::*;
use crate::doomtype::*;
use crate::i_system::*;
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

pub static mut save_p: *mut byte = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn P_ArchivePlayers() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_UnArchivePlayers() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_ArchiveWorld() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_UnArchiveWorld() {
    todo!("body not yet translated")
}

pub const tc_end: std::ffi::c_int = 0;
pub const tc_mobj: std::ffi::c_int = tc_end + 1;

pub type thinkerclass_t = std::ffi::c_int;

pub unsafe extern "C" fn P_ArchiveThinkers() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_UnArchiveThinkers() {
    todo!("body not yet translated")
}

pub const tc_ceiling: std::ffi::c_int = 0;
pub const tc_door: std::ffi::c_int = tc_ceiling + 1;
pub const tc_floor: std::ffi::c_int = tc_door + 1;
pub const tc_plat: std::ffi::c_int = tc_floor + 1;
pub const tc_flash: std::ffi::c_int = tc_plat + 1;
pub const tc_strobe: std::ffi::c_int = tc_flash + 1;
pub const tc_glow: std::ffi::c_int = tc_strobe + 1;
pub const tc_endspecials: std::ffi::c_int = tc_glow + 1;

pub type specials_e = std::ffi::c_int;

pub unsafe extern "C" fn P_ArchiveSpecials() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_UnArchiveSpecials() {
    todo!("body not yet translated")
}
