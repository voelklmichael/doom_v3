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

static mut rcsid: [std::ffi::c_char; 49] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        109 as std::ffi::c_char,
        95 as std::ffi::c_char,
        98 as std::ffi::c_char,
        98 as std::ffi::c_char,
        111 as std::ffi::c_char,
        120 as std::ffi::c_char,
        46 as std::ffi::c_char,
        99 as std::ffi::c_char,
        44 as std::ffi::c_char,
        118 as std::ffi::c_char,
        32 as std::ffi::c_char,
        49 as std::ffi::c_char,
        46 as std::ffi::c_char,
        49 as std::ffi::c_char,
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

pub static mut mb_used: std::ffi::c_int = unsafe { 6 };

pub unsafe extern "C" fn I_Tactile(
    on: std::ffi::c_int,
    off: std::ffi::c_int,
    total: std::ffi::c_int,
) {
    unsafe { todo!("body not yet translated") }
}

pub static mut emptycmd: ticcmd_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn I_BaseTiccmd() -> *mut ticcmd_t {
    unsafe { todo!("body not yet translated") }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn I_GetHeapSize() -> std::ffi::c_int {
    unsafe { todo!("body not yet translated") }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn I_ZoneBase(size: *mut std::ffi::c_int) -> *mut byte {
    unsafe { todo!("body not yet translated") }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn I_GetTime() -> std::ffi::c_int {
    unsafe { todo!("body not yet translated") }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn I_Init() {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn I_Quit() {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn I_WaitVBL(count: std::ffi::c_int) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn I_BeginRead() {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn I_EndRead() {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn I_AllocLow(length: std::ffi::c_int) -> *mut byte {
    unsafe { todo!("body not yet translated") }
    todo!("fell off the end of a non-void C function")
}

unsafe extern "C" {
    pub static mut demorecording: boolean;
}

pub unsafe extern "C" fn I_Error(error: *mut std::ffi::c_char) {
    unsafe { todo!("body not yet translated") }
} // TODO: variadic definition not supported, C variadic marker dropped
