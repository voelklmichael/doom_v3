use crate::d_englsh::*;
use crate::d_items::*;
use crate::d_net::*;
use crate::d_player::*;
use crate::d_think::*;
use crate::d_ticcmd::*;
use crate::doomdata::*;
use crate::doomdef::*;
use crate::doomstat::*;
use crate::doomtype::*;
use crate::dstrings::*;
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
use crate::s_sound::*;
use crate::sounds::*;
use crate::tables::*;
use crate::z_zone::*;

static mut rcsid: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn T_VerticalDoor(door: *mut vldoor_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn EV_DoLockedDoor(
    line: *mut line_t,
    type_: vldoor_e,
    thing: *mut mobj_t,
) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn EV_DoDoor(line: *mut line_t, type_: vldoor_e) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn EV_VerticalDoor(line: *mut line_t, thing: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_SpawnDoorCloseIn30(sec: *mut sector_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_SpawnDoorRaiseIn5Mins(sec: *mut sector_t, secnum: std::ffi::c_int) {
    todo!("body not yet translated")
}
