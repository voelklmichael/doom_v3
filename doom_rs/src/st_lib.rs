use crate::d_event::*;
use crate::d_items::*;
use crate::d_player::*;
use crate::d_think::*;
use crate::d_ticcmd::*;
use crate::doomdata::*;
use crate::doomdef::*;
use crate::doomtype::*;
use crate::i_system::*;
use crate::info::*;
use crate::m_fixed::*;
use crate::m_swap::*;
use crate::p_mobj::*;
use crate::p_pspr::*;
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
use crate::st_stuff::*;
use crate::tables::*;
use crate::v_video::*;
use crate::w_wad::*;
use crate::z_zone::*;

pub const BG: std::ffi::c_int = 4;

pub const FG: std::ffi::c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_number_t {
    pub x: std::ffi::c_int,
    pub y: std::ffi::c_int,
    pub width: std::ffi::c_int,
    pub oldnum: std::ffi::c_int,
    pub num: *mut std::ffi::c_int,
    pub on: *mut boolean,
    pub p: *mut *mut patch_t,
    pub data: std::ffi::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_percent_t {
    pub n: st_number_t,
    pub p: *mut patch_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_multicon_t {
    pub x: std::ffi::c_int,
    pub y: std::ffi::c_int,
    pub oldinum: std::ffi::c_int,
    pub inum: *mut std::ffi::c_int,
    pub on: *mut boolean,
    pub p: *mut *mut patch_t,
    pub data: std::ffi::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_binicon_t {
    pub x: std::ffi::c_int,
    pub y: std::ffi::c_int,
    pub oldval: std::ffi::c_int,
    pub val: *mut boolean,
    pub on: *mut boolean,
    pub p: *mut patch_t,
    pub data: std::ffi::c_int,
}

static mut rcsid: [std::ffi::c_char; 49] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        115 as std::ffi::c_char,
        116 as std::ffi::c_char,
        95 as std::ffi::c_char,
        108 as std::ffi::c_char,
        105 as std::ffi::c_char,
        98 as std::ffi::c_char,
        46 as std::ffi::c_char,
        99 as std::ffi::c_char,
        44 as std::ffi::c_char,
        118 as std::ffi::c_char,
        32 as std::ffi::c_char,
        49 as std::ffi::c_char,
        46 as std::ffi::c_char,
        52 as std::ffi::c_char,
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
        49 as std::ffi::c_char,
        54 as std::ffi::c_char,
        58 as std::ffi::c_char,
        52 as std::ffi::c_char,
        55 as std::ffi::c_char,
        58 as std::ffi::c_char,
        53 as std::ffi::c_char,
        54 as std::ffi::c_char,
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

unsafe extern "C" {
    pub static mut automapactive: boolean;
}

pub static mut sttminus: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn STlib_init() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn STlib_initNum(
    n: *mut st_number_t,
    x: std::ffi::c_int,
    y: std::ffi::c_int,
    pl: *mut *mut patch_t,
    num: *mut std::ffi::c_int,
    on: *mut boolean,
    width: std::ffi::c_int,
) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn STlib_drawNum(n: *mut st_number_t, refresh: boolean) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn STlib_updateNum(n: *mut st_number_t, refresh: boolean) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn STlib_initPercent(
    p: *mut st_percent_t,
    x: std::ffi::c_int,
    y: std::ffi::c_int,
    pl: *mut *mut patch_t,
    num: *mut std::ffi::c_int,
    on: *mut boolean,
    percent: *mut patch_t,
) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn STlib_updatePercent(per: *mut st_percent_t, refresh: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn STlib_initMultIcon(
    i: *mut st_multicon_t,
    x: std::ffi::c_int,
    y: std::ffi::c_int,
    il: *mut *mut patch_t,
    inum: *mut std::ffi::c_int,
    on: *mut boolean,
) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn STlib_updateMultIcon(mi: *mut st_multicon_t, refresh: boolean) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn STlib_initBinIcon(
    b: *mut st_binicon_t,
    x: std::ffi::c_int,
    y: std::ffi::c_int,
    i: *mut patch_t,
    val: *mut boolean,
    on: *mut boolean,
) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn STlib_updateBinIcon(bi: *mut st_binicon_t, refresh: boolean) {
    todo!("body not yet translated")
}
