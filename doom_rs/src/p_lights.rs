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

static mut rcsid: [std::ffi::c_char; 51] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        112 as std::ffi::c_char,
        95 as std::ffi::c_char,
        108 as std::ffi::c_char,
        105 as std::ffi::c_char,
        103 as std::ffi::c_char,
        104 as std::ffi::c_char,
        116 as std::ffi::c_char,
        115 as std::ffi::c_char,
        46 as std::ffi::c_char,
        99 as std::ffi::c_char,
        44 as std::ffi::c_char,
        118 as std::ffi::c_char,
        32 as std::ffi::c_char,
        49 as std::ffi::c_char,
        46 as std::ffi::c_char,
        53 as std::ffi::c_char,
        32 as std::ffi::c_char,
        49 as std::ffi::c_char,
        57 as std::ffi::c_char,
        57 as std::ffi::c_char,
        55 as std::ffi::c_char,
        47 as std::ffi::c_char,
        48 as std::ffi::c_char,
        50 as std::ffi::c_char,
        47 as std::ffi::c_char,
        48 as std::ffi::c_char,
        51 as std::ffi::c_char,
        32 as std::ffi::c_char,
        50 as std::ffi::c_char,
        50 as std::ffi::c_char,
        58 as std::ffi::c_char,
        52 as std::ffi::c_char,
        53 as std::ffi::c_char,
        58 as std::ffi::c_char,
        49 as std::ffi::c_char,
        49 as std::ffi::c_char,
        32 as std::ffi::c_char,
        98 as std::ffi::c_char,
        49 as std::ffi::c_char,
        32 as std::ffi::c_char,
        69 as std::ffi::c_char,
        120 as std::ffi::c_char,
        112 as std::ffi::c_char,
        32 as std::ffi::c_char,
        36 as std::ffi::c_char,
        0,
    ]
};

pub unsafe extern "C" fn T_FireFlicker(flick: *mut fireflicker_t) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn P_SpawnFireFlicker(sector: *mut sector_t) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn T_LightFlash(flash: *mut lightflash_t) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn P_SpawnLightFlash(sector: *mut sector_t) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn T_StrobeFlash(flash: *mut strobe_t) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn P_SpawnStrobeFlash(
    sector: *mut sector_t,
    fastOrSlow: std::ffi::c_int,
    inSync: std::ffi::c_int,
) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn EV_StartLightStrobing(line: *mut line_t) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn EV_TurnTagLightsOff(line: *mut line_t) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn EV_LightTurnOn(line: *mut line_t, bright: std::ffi::c_int) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn T_Glow(g: *mut glow_t) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn P_SpawnGlowingLight(sector: *mut sector_t) {
    unsafe { todo!("body not yet translated") }
}
