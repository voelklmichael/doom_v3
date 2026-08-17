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
use crate::g_game::*;
use crate::i_sound::*;
use crate::i_video::*;
use crate::info::*;
use crate::m_fixed::*;
use crate::m_misc::*;
use crate::p_mobj::*;
use crate::p_pspr::*;
use crate::sounds::*;
use crate::tables::*;

unsafe extern "C" {
    pub fn I_StartFrame();
}

unsafe extern "C" {
    pub fn I_StartTic();
}

static mut rcsid: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut mb_used: std::ffi::c_int = unsafe { 6 };

pub unsafe extern "C" fn I_Tactile(
    on: std::ffi::c_int,
    off: std::ffi::c_int,
    total: std::ffi::c_int,
) {
    todo!("body not yet translated")
}

pub static mut emptycmd: ticcmd_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn I_BaseTiccmd() -> *mut ticcmd_t {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn I_GetHeapSize() -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn I_ZoneBase(size: *mut std::ffi::c_int) -> *mut byte {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn I_GetTime() -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn I_Init() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn I_Quit() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn I_WaitVBL(count: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn I_BeginRead() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn I_EndRead() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn I_AllocLow(length: std::ffi::c_int) -> *mut byte {
    todo!("body not yet translated")
}

unsafe extern "C" {
    pub static mut demorecording: boolean;
}

pub unsafe extern "C" fn I_Error(error: *mut std::ffi::c_char) {
    todo!("body not yet translated")
} // TODO: variadic definition not supported, C variadic marker dropped
