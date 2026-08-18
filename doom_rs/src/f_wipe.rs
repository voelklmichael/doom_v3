use crate::d_items::*;
use crate::d_player::*;
use crate::d_think::*;
use crate::d_ticcmd::*;
use crate::doomdata::*;
use crate::doomdef::*;
use crate::doomtype::*;
use crate::i_video::*;
use crate::info::*;
use crate::m_fixed::*;
use crate::m_random::*;
use crate::p_mobj::*;
use crate::p_pspr::*;
use crate::r_data::*;
use crate::r_defs::*;
use crate::r_state::*;
use crate::tables::*;
use crate::v_video::*;
use crate::z_zone::*;

pub const wipe_ColorXForm: std::ffi::c_int = 0;
pub const wipe_Melt: std::ffi::c_int = wipe_ColorXForm + 1;
pub const wipe_NUMWIPES: std::ffi::c_int = wipe_Melt + 1;

static mut rcsid: [std::ffi::c_char; 49] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        102 as std::ffi::c_char,
        95 as std::ffi::c_char,
        119 as std::ffi::c_char,
        105 as std::ffi::c_char,
        112 as std::ffi::c_char,
        101 as std::ffi::c_char,
        46 as std::ffi::c_char,
        99 as std::ffi::c_char,
        44 as std::ffi::c_char,
        118 as std::ffi::c_char,
        32 as std::ffi::c_char,
        49 as std::ffi::c_char,
        46 as std::ffi::c_char,
        50 as std::ffi::c_char,
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
        48 as std::ffi::c_char,
        57 as std::ffi::c_char,
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

static mut go: boolean = unsafe { 0 };

static mut wipe_scr_start: *mut byte = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut wipe_scr_end: *mut byte = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut wipe_scr: *mut byte = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn wipe_shittyColMajorXform(
    array: *mut std::ffi::c_short,
    width: std::ffi::c_int,
    height: std::ffi::c_int,
) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn wipe_initColorXForm(
    width: std::ffi::c_int,
    height: std::ffi::c_int,
    ticks: std::ffi::c_int,
) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn wipe_doColorXForm(
    width: std::ffi::c_int,
    height: std::ffi::c_int,
    ticks: std::ffi::c_int,
) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn wipe_exitColorXForm(
    width: std::ffi::c_int,
    height: std::ffi::c_int,
    ticks: std::ffi::c_int,
) -> std::ffi::c_int {
    todo!("body not yet translated")
}

static mut y: *mut std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn wipe_initMelt(
    width: std::ffi::c_int,
    height: std::ffi::c_int,
    ticks: std::ffi::c_int,
) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn wipe_doMelt(
    width: std::ffi::c_int,
    height: std::ffi::c_int,
    ticks: std::ffi::c_int,
) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn wipe_exitMelt(
    width: std::ffi::c_int,
    height: std::ffi::c_int,
    ticks: std::ffi::c_int,
) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn wipe_StartScreen(
    x: std::ffi::c_int,
    y: std::ffi::c_int,
    width: std::ffi::c_int,
    height: std::ffi::c_int,
) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn wipe_EndScreen(
    x: std::ffi::c_int,
    y: std::ffi::c_int,
    width: std::ffi::c_int,
    height: std::ffi::c_int,
) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn wipe_ScreenWipe(
    wipeno: std::ffi::c_int,
    x: std::ffi::c_int,
    y: std::ffi::c_int,
    width: std::ffi::c_int,
    height: std::ffi::c_int,
    ticks: std::ffi::c_int,
) -> std::ffi::c_int {
    todo!("body not yet translated")
}
