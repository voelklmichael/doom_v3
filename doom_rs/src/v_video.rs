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
use crate::m_bbox::*;
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

static mut rcsid: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut screens: [*mut byte; (5) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut dirtybox: [std::ffi::c_int; (4) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut gammatable: [[byte; (256) as usize]; (5) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut usegamma: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn V_MarkRect(
    x: std::ffi::c_int,
    y: std::ffi::c_int,
    width: std::ffi::c_int,
    height: std::ffi::c_int,
) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn V_CopyRect(
    srcx: std::ffi::c_int,
    srcy: std::ffi::c_int,
    srcscrn: std::ffi::c_int,
    width: std::ffi::c_int,
    height: std::ffi::c_int,
    destx: std::ffi::c_int,
    desty: std::ffi::c_int,
    destscrn: std::ffi::c_int,
) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn V_DrawPatch(
    x: std::ffi::c_int,
    y: std::ffi::c_int,
    scrn: std::ffi::c_int,
    patch: *mut patch_t,
) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn V_DrawPatchFlipped(
    x: std::ffi::c_int,
    y: std::ffi::c_int,
    scrn: std::ffi::c_int,
    patch: *mut patch_t,
) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn V_DrawPatchDirect(
    x: std::ffi::c_int,
    y: std::ffi::c_int,
    scrn: std::ffi::c_int,
    patch: *mut patch_t,
) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn V_DrawBlock(
    x: std::ffi::c_int,
    y: std::ffi::c_int,
    scrn: std::ffi::c_int,
    width: std::ffi::c_int,
    height: std::ffi::c_int,
    src: *mut byte,
) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn V_GetBlock(
    x: std::ffi::c_int,
    y: std::ffi::c_int,
    scrn: std::ffi::c_int,
    width: std::ffi::c_int,
    height: std::ffi::c_int,
    dest: *mut byte,
) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn V_Init() {
    todo!("body not yet translated")
}
