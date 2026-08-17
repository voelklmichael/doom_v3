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
use crate::info::*;
use crate::m_fixed::*;
use crate::m_swap::*;
use crate::p_mobj::*;
use crate::p_pspr::*;
use crate::r_data::*;
use crate::r_defs::*;
use crate::r_state::*;
use crate::s_sound::*;
use crate::sounds::*;
use crate::tables::*;
use crate::v_video::*;
use crate::w_wad::*;
use crate::z_zone::*;

static mut rcsid: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut finalestage: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut finalecount: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub const TEXTSPEED: std::ffi::c_int = 3;

pub const TEXTWAIT: std::ffi::c_int = 250;

pub static mut e1text: *mut std::ffi::c_char = unsafe { E1TEXT };

pub static mut e2text: *mut std::ffi::c_char = unsafe { E2TEXT };

pub static mut e3text: *mut std::ffi::c_char = unsafe { E3TEXT };

pub static mut e4text: *mut std::ffi::c_char = unsafe { E4TEXT };

pub static mut c1text: *mut std::ffi::c_char = unsafe { C1TEXT };

pub static mut c2text: *mut std::ffi::c_char = unsafe { C2TEXT };

pub static mut c3text: *mut std::ffi::c_char = unsafe { C3TEXT };

pub static mut c4text: *mut std::ffi::c_char = unsafe { C4TEXT };

pub static mut c5text: *mut std::ffi::c_char = unsafe { C5TEXT };

pub static mut c6text: *mut std::ffi::c_char = unsafe { C6TEXT };

pub static mut p1text: *mut std::ffi::c_char = unsafe { P1TEXT };

pub static mut p2text: *mut std::ffi::c_char = unsafe { P2TEXT };

pub static mut p3text: *mut std::ffi::c_char = unsafe { P3TEXT };

pub static mut p4text: *mut std::ffi::c_char = unsafe { P4TEXT };

pub static mut p5text: *mut std::ffi::c_char = unsafe { P5TEXT };

pub static mut p6text: *mut std::ffi::c_char = unsafe { P6TEXT };

pub static mut t1text: *mut std::ffi::c_char = unsafe { T1TEXT };

pub static mut t2text: *mut std::ffi::c_char = unsafe { T2TEXT };

pub static mut t3text: *mut std::ffi::c_char = unsafe { T3TEXT };

pub static mut t4text: *mut std::ffi::c_char = unsafe { T4TEXT };

pub static mut t5text: *mut std::ffi::c_char = unsafe { T5TEXT };

pub static mut t6text: *mut std::ffi::c_char = unsafe { T6TEXT };

pub static mut finaletext: *mut std::ffi::c_char = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut finaleflat: *mut std::ffi::c_char = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn F_StartFinale() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn F_Responder(event: *mut event_t) -> boolean {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn F_Ticker() {
    todo!("body not yet translated")
}

unsafe extern "C" {
    pub static mut hu_font: [*mut patch_t; (HU_FONTSIZE) as usize];
}

pub unsafe extern "C" fn F_TextWrite() {
    todo!("body not yet translated")
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct castinfo_t {
    pub name: *mut std::ffi::c_char,
    pub type_: mobjtype_t,
}

pub static mut castorder: *mut castinfo_t /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut castnum: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut casttics: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut caststate: *mut state_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut castdeath: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut castframes: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut castonmelee: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut castattacking: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

unsafe extern "C" {
    pub static mut wipegamestate: gamestate_t;
}

pub unsafe extern "C" fn F_StartCast() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn F_CastTicker() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn F_CastResponder(ev: *mut event_t) -> boolean {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn F_CastPrint(text: *mut std::ffi::c_char) {
    todo!("body not yet translated")
}

unsafe extern "C" {
    pub fn V_DrawPatchFlipped(
        x: std::ffi::c_int,
        y: std::ffi::c_int,
        scrn: std::ffi::c_int,
        patch: *mut patch_t,
    );
}

pub unsafe extern "C" fn F_CastDrawer() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn F_DrawPatchCol(
    x: std::ffi::c_int,
    patch: *mut patch_t,
    col: std::ffi::c_int,
) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn F_BunnyScroll() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn F_Drawer() {
    todo!("body not yet translated")
}
