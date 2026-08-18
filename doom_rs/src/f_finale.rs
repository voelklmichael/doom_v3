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

static mut rcsid: [std::ffi::c_char; 51] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        102 as std::ffi::c_char,
        95 as std::ffi::c_char,
        102 as std::ffi::c_char,
        105 as std::ffi::c_char,
        110 as std::ffi::c_char,
        97 as std::ffi::c_char,
        108 as std::ffi::c_char,
        101 as std::ffi::c_char,
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
        49 as std::ffi::c_char,
        58 as std::ffi::c_char,
        50 as std::ffi::c_char,
        54 as std::ffi::c_char,
        58 as std::ffi::c_char,
        51 as std::ffi::c_char,
        52 as std::ffi::c_char,
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

pub static mut finalestage: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut finalecount: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub const TEXTSPEED: std::ffi::c_int = 3;

pub const TEXTWAIT: std::ffi::c_int = 250;

pub static mut e1text: *mut std::ffi::c_char = unsafe { E1TEXT as *mut std::ffi::c_char };

pub static mut e2text: *mut std::ffi::c_char = unsafe { E2TEXT as *mut std::ffi::c_char };

pub static mut e3text: *mut std::ffi::c_char = unsafe { E3TEXT as *mut std::ffi::c_char };

pub static mut e4text: *mut std::ffi::c_char = unsafe { E4TEXT as *mut std::ffi::c_char };

pub static mut c1text: *mut std::ffi::c_char = unsafe { C1TEXT as *mut std::ffi::c_char };

pub static mut c2text: *mut std::ffi::c_char = unsafe { C2TEXT as *mut std::ffi::c_char };

pub static mut c3text: *mut std::ffi::c_char = unsafe { C3TEXT as *mut std::ffi::c_char };

pub static mut c4text: *mut std::ffi::c_char = unsafe { C4TEXT as *mut std::ffi::c_char };

pub static mut c5text: *mut std::ffi::c_char = unsafe { C5TEXT as *mut std::ffi::c_char };

pub static mut c6text: *mut std::ffi::c_char = unsafe { C6TEXT as *mut std::ffi::c_char };

pub static mut p1text: *mut std::ffi::c_char = unsafe { P1TEXT as *mut std::ffi::c_char };

pub static mut p2text: *mut std::ffi::c_char = unsafe { P2TEXT as *mut std::ffi::c_char };

pub static mut p3text: *mut std::ffi::c_char = unsafe { P3TEXT as *mut std::ffi::c_char };

pub static mut p4text: *mut std::ffi::c_char = unsafe { P4TEXT as *mut std::ffi::c_char };

pub static mut p5text: *mut std::ffi::c_char = unsafe { P5TEXT as *mut std::ffi::c_char };

pub static mut p6text: *mut std::ffi::c_char = unsafe { P6TEXT as *mut std::ffi::c_char };

pub static mut t1text: *mut std::ffi::c_char = unsafe { T1TEXT as *mut std::ffi::c_char };

pub static mut t2text: *mut std::ffi::c_char = unsafe { T2TEXT as *mut std::ffi::c_char };

pub static mut t3text: *mut std::ffi::c_char = unsafe { T3TEXT as *mut std::ffi::c_char };

pub static mut t4text: *mut std::ffi::c_char = unsafe { T4TEXT as *mut std::ffi::c_char };

pub static mut t5text: *mut std::ffi::c_char = unsafe { T5TEXT as *mut std::ffi::c_char };

pub static mut t6text: *mut std::ffi::c_char = unsafe { T6TEXT as *mut std::ffi::c_char };

pub static mut finaletext: *mut std::ffi::c_char = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut finaleflat: *mut std::ffi::c_char = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn F_StartFinale() {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn F_Responder(event: *mut event_t) -> boolean {
    unsafe { todo!("body not yet translated") }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn F_Ticker() {
    unsafe { todo!("body not yet translated") }
}

unsafe extern "C" {
    pub static mut hu_font: [*mut patch_t; (HU_FONTSIZE) as usize];
}

pub unsafe extern "C" fn F_TextWrite() {
    unsafe { todo!("body not yet translated") }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct castinfo_t {
    pub name: *mut std::ffi::c_char,
    pub type_: mobjtype_t,
}

pub static mut castorder: [castinfo_t; 18] = unsafe {
    [
        castinfo_t {
            name: CC_ZOMBIE as *mut std::ffi::c_char,
            type_: MT_POSSESSED,
        },
        castinfo_t {
            name: CC_SHOTGUN as *mut std::ffi::c_char,
            type_: MT_SHOTGUY,
        },
        castinfo_t {
            name: CC_HEAVY as *mut std::ffi::c_char,
            type_: MT_CHAINGUY,
        },
        castinfo_t {
            name: CC_IMP as *mut std::ffi::c_char,
            type_: MT_TROOP,
        },
        castinfo_t {
            name: CC_DEMON as *mut std::ffi::c_char,
            type_: MT_SERGEANT,
        },
        castinfo_t {
            name: CC_LOST as *mut std::ffi::c_char,
            type_: MT_SKULL,
        },
        castinfo_t {
            name: CC_CACO as *mut std::ffi::c_char,
            type_: MT_HEAD,
        },
        castinfo_t {
            name: CC_HELL as *mut std::ffi::c_char,
            type_: MT_KNIGHT,
        },
        castinfo_t {
            name: CC_BARON as *mut std::ffi::c_char,
            type_: MT_BRUISER,
        },
        castinfo_t {
            name: CC_ARACH as *mut std::ffi::c_char,
            type_: MT_BABY,
        },
        castinfo_t {
            name: CC_PAIN as *mut std::ffi::c_char,
            type_: MT_PAIN,
        },
        castinfo_t {
            name: CC_REVEN as *mut std::ffi::c_char,
            type_: MT_UNDEAD,
        },
        castinfo_t {
            name: CC_MANCU as *mut std::ffi::c_char,
            type_: MT_FATSO,
        },
        castinfo_t {
            name: CC_ARCH as *mut std::ffi::c_char,
            type_: MT_VILE,
        },
        castinfo_t {
            name: CC_SPIDER as *mut std::ffi::c_char,
            type_: MT_SPIDER,
        },
        castinfo_t {
            name: CC_CYBER as *mut std::ffi::c_char,
            type_: MT_CYBORG,
        },
        castinfo_t {
            name: CC_HERO as *mut std::ffi::c_char,
            type_: MT_PLAYER,
        },
        castinfo_t {
            name: std::ptr::null_mut(),
            type_: 0,
        },
    ]
};

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
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn F_CastTicker() {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn F_CastResponder(ev: *mut event_t) -> boolean {
    unsafe { todo!("body not yet translated") }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn F_CastPrint(text: *mut std::ffi::c_char) {
    unsafe { todo!("body not yet translated") }
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
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn F_DrawPatchCol(
    x: std::ffi::c_int,
    patch: *mut patch_t,
    col: std::ffi::c_int,
) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn F_BunnyScroll() {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn F_Drawer() {
    unsafe { todo!("body not yet translated") }
}
