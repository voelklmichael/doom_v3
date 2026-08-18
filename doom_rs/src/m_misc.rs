use crate::d_englsh::*;
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
use crate::dstrings::*;
use crate::hu_stuff::*;
use crate::i_system::*;
use crate::i_video::*;
use crate::info::*;
use crate::m_argv::*;
use crate::m_fixed::*;
use crate::m_swap::*;
use crate::p_mobj::*;
use crate::p_pspr::*;
use crate::r_data::*;
use crate::r_defs::*;
use crate::r_state::*;
use crate::tables::*;
use crate::v_video::*;
use crate::w_wad::*;
use crate::z_zone::*;

static mut rcsid: [std::ffi::c_char; 49] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        109 as std::ffi::c_char,
        95 as std::ffi::c_char,
        109 as std::ffi::c_char,
        105 as std::ffi::c_char,
        115 as std::ffi::c_char,
        99 as std::ffi::c_char,
        46 as std::ffi::c_char,
        99 as std::ffi::c_char,
        44 as std::ffi::c_char,
        118 as std::ffi::c_char,
        32 as std::ffi::c_char,
        49 as std::ffi::c_char,
        46 as std::ffi::c_char,
        54 as std::ffi::c_char,
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
        48 as std::ffi::c_char,
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
    pub static mut hu_font: [*mut patch_t; (HU_FONTSIZE) as usize];
}

pub unsafe extern "C" fn M_DrawText(
    x: std::ffi::c_int,
    y: std::ffi::c_int,
    direct: boolean,
    string: *mut std::ffi::c_char,
) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub const O_BINARY: std::ffi::c_int = 0;

pub unsafe extern "C" fn M_WriteFile(
    name: (), /* TODO: unparsed param type, needs manual translation */
    source: *mut std::ffi::c_void,
    length: std::ffi::c_int,
) -> boolean {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_ReadFile(
    name: (), /* TODO: unparsed param type, needs manual translation */
    buffer: *mut *mut byte,
) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub static mut usemouse: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut usejoystick: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

unsafe extern "C" {
    pub static mut key_right: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut key_left: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut key_up: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut key_down: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut key_strafeleft: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut key_straferight: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut key_fire: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut key_use: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut key_strafe: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut key_speed: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut mousebfire: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut mousebstrafe: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut mousebforward: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut joybfire: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut joybstrafe: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut joybuse: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut joybspeed: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut viewwidth: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut viewheight: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut mouseSensitivity: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut showMessages: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut detailLevel: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut screenblocks: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut numChannels: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut sndserver_filename: *mut std::ffi::c_char;
}

unsafe extern "C" {
    pub static mut mb_used: std::ffi::c_int;
}

pub static mut mousetype: *mut std::ffi::c_char = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut mousedev: *mut std::ffi::c_char = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

unsafe extern "C" {
    pub static mut chat_macros: *mut *mut std::ffi::c_char;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct default_t {
    pub name: *mut std::ffi::c_char,
    pub location: *mut std::ffi::c_int,
    pub defaultvalue: std::ffi::c_int,
    pub scantranslate: std::ffi::c_int,
    pub untranslated: std::ffi::c_int,
}

pub static mut defaults: *mut default_t /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut numdefaults: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut defaultfile: *mut std::ffi::c_char = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn M_SaveDefaults() {
    todo!("body not yet translated")
}

unsafe extern "C" {
    pub static mut scantokey: [byte; (128) as usize];
}

pub unsafe extern "C" fn M_LoadDefaults() {
    todo!("body not yet translated")
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcx_t {
    pub manufacturer: std::ffi::c_char,
    pub version: std::ffi::c_char,
    pub encoding: std::ffi::c_char,
    pub bits_per_pixel: std::ffi::c_char,
    pub xmin: std::ffi::c_ushort,
    pub ymin: std::ffi::c_ushort,
    pub xmax: std::ffi::c_ushort,
    pub ymax: std::ffi::c_ushort,
    pub hres: std::ffi::c_ushort,
    pub vres: std::ffi::c_ushort,
    pub palette: [std::ffi::c_uchar; (48) as usize],
    pub reserved: std::ffi::c_char,
    pub color_planes: std::ffi::c_char,
    pub bytes_per_line: std::ffi::c_ushort,
    pub palette_type: std::ffi::c_ushort,
    pub filler: [std::ffi::c_char; (58) as usize],
    pub data: std::ffi::c_uchar,
}

pub unsafe extern "C" fn WritePCXfile(
    filename: *mut std::ffi::c_char,
    data: *mut byte,
    width: std::ffi::c_int,
    height: std::ffi::c_int,
    palette: *mut byte,
) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_ScreenShot() {
    todo!("body not yet translated")
}
