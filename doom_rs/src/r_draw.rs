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
use crate::p_mobj::*;
use crate::p_pspr::*;
use crate::r_bsp::*;
use crate::r_data::*;
use crate::r_defs::*;
use crate::r_local::*;
use crate::r_main::*;
use crate::r_plane::*;
use crate::r_segs::*;
use crate::r_state::*;
use crate::r_things::*;
use crate::tables::*;
use crate::v_video::*;
use crate::w_wad::*;
use crate::z_zone::*;

unsafe extern "C" {
    pub fn R_DrawFuzzColumnLow();
}

unsafe extern "C" {
    pub fn R_DrawTranslatedColumnLow();
}

static mut rcsid: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut viewimage: *mut byte = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut viewwidth: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut scaledviewwidth: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut viewheight: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut viewwindowx: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut viewwindowy: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut ylookup: [*mut byte; (MAXHEIGHT) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut columnofs: [std::ffi::c_int; (MAXWIDTH) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut translations: [[byte; (256) as usize]; (3) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut dc_colormap: *mut lighttable_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut dc_x: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut dc_yl: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut dc_yh: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut dc_iscale: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut dc_texturemid: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut dc_source: *mut byte = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut dccount: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn R_DrawColumn() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_DrawColumnLow() {
    todo!("body not yet translated")
}

pub static mut fuzzoffset: [std::ffi::c_int; (FUZZTABLE) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut fuzzpos: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn R_DrawFuzzColumn() {
    todo!("body not yet translated")
}

pub static mut dc_translation: *mut byte = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut translationtables: *mut byte = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn R_DrawTranslatedColumn() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_InitTranslationTables() {
    todo!("body not yet translated")
}

pub static mut ds_y: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut ds_x1: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut ds_x2: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut ds_colormap: *mut lighttable_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut ds_xfrac: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut ds_yfrac: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut ds_xstep: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut ds_ystep: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut ds_source: *mut byte = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut dscount: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn R_DrawSpan() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_DrawSpanLow() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_InitBuffer(width: std::ffi::c_int, height: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_FillBackScreen() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_VideoErase(ofs: std::ffi::c_uint, count: std::ffi::c_int) {
    todo!("body not yet translated")
}

unsafe extern "C" {
    pub fn V_MarkRect(
        x: std::ffi::c_int,
        y: std::ffi::c_int,
        width: std::ffi::c_int,
        height: std::ffi::c_int,
    );
}

pub unsafe extern "C" fn R_DrawViewBorder() {
    todo!("body not yet translated")
}
