use crate::d_items::*;
use crate::d_player::*;
use crate::d_think::*;
use crate::d_ticcmd::*;
use crate::doomdata::*;
use crate::doomdef::*;
use crate::doomtype::*;
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
use crate::tables::*;
use crate::v_video::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hu_textline_t {
    pub x: std::ffi::c_int,
    pub y: std::ffi::c_int,
    pub f: *mut *mut patch_t,
    pub sc: std::ffi::c_int,
    pub l: [std::ffi::c_char; (HU_MAXLINELENGTH + 1) as usize],
    pub len: std::ffi::c_int,
    pub needsupdate: std::ffi::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hu_stext_t {
    pub l: [hu_textline_t; (HU_MAXLINES) as usize],
    pub h: std::ffi::c_int,
    pub cl: std::ffi::c_int,
    pub on: *mut boolean,
    pub laston: boolean,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hu_itext_t {
    pub l: hu_textline_t,
    pub lm: std::ffi::c_int,
    pub on: *mut boolean,
    pub laston: boolean,
}

static mut rcsid: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

unsafe extern "C" {
    pub static mut automapactive: boolean;
}

pub unsafe extern "C" fn HUlib_init() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn HUlib_clearTextLine(t: *mut hu_textline_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn HUlib_initTextLine(
    t: *mut hu_textline_t,
    x: std::ffi::c_int,
    y: std::ffi::c_int,
    f: *mut *mut patch_t,
    sc: std::ffi::c_int,
) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn HUlib_addCharToTextLine(
    t: *mut hu_textline_t,
    ch: std::ffi::c_char,
) -> boolean {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn HUlib_delCharFromTextLine(t: *mut hu_textline_t) -> boolean {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn HUlib_drawTextLine(l: *mut hu_textline_t, drawcursor: boolean) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn HUlib_eraseTextLine(l: *mut hu_textline_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn HUlib_initSText(
    s: *mut hu_stext_t,
    x: std::ffi::c_int,
    y: std::ffi::c_int,
    h: std::ffi::c_int,
    font: *mut *mut patch_t,
    startchar: std::ffi::c_int,
    on: *mut boolean,
) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn HUlib_addLineToSText(s: *mut hu_stext_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn HUlib_addMessageToSText(
    s: *mut hu_stext_t,
    prefix: *mut std::ffi::c_char,
    msg: *mut std::ffi::c_char,
) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn HUlib_drawSText(s: *mut hu_stext_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn HUlib_eraseSText(s: *mut hu_stext_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn HUlib_initIText(
    it: *mut hu_itext_t,
    x: std::ffi::c_int,
    y: std::ffi::c_int,
    font: *mut *mut patch_t,
    startchar: std::ffi::c_int,
    on: *mut boolean,
) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn HUlib_delCharFromIText(it: *mut hu_itext_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn HUlib_eraseLineFromIText(it: *mut hu_itext_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn HUlib_resetIText(it: *mut hu_itext_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn HUlib_addPrefixToIText(it: *mut hu_itext_t, str: *mut std::ffi::c_char) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn HUlib_keyInIText(it: *mut hu_itext_t, ch: std::ffi::c_uchar) -> boolean {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn HUlib_drawIText(it: *mut hu_itext_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn HUlib_eraseIText(it: *mut hu_itext_t) {
    todo!("body not yet translated")
}
