use crate::d_items::*;
use crate::d_player::*;
use crate::d_think::*;
use crate::d_ticcmd::*;
use crate::doomdata::*;
use crate::doomdef::*;
use crate::doomtype::*;
use crate::info::*;
use crate::m_fixed::*;
use crate::m_random::*;
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

pub unsafe extern "C" fn T_FireFlicker(flick: *mut fireflicker_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_SpawnFireFlicker(sector: *mut sector_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn T_LightFlash(flash: *mut lightflash_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_SpawnLightFlash(sector: *mut sector_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn T_StrobeFlash(flash: *mut strobe_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_SpawnStrobeFlash(
    sector: *mut sector_t,
    fastOrSlow: std::ffi::c_int,
    inSync: std::ffi::c_int,
) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn EV_StartLightStrobing(line: *mut line_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn EV_TurnTagLightsOff(line: *mut line_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn EV_LightTurnOn(line: *mut line_t, bright: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn T_Glow(g: *mut glow_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_SpawnGlowingLight(sector: *mut sector_t) {
    todo!("body not yet translated")
}
