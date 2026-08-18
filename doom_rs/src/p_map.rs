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
use crate::m_bbox::*;
use crate::m_fixed::*;
use crate::m_random::*;
use crate::p_local::*;
use crate::p_mobj::*;
use crate::p_pspr::*;
use crate::p_spec::*;
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
use crate::s_sound::*;
use crate::sounds::*;
use crate::tables::*;

static mut rcsid: [std::ffi::c_char; 48] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        112 as std::ffi::c_char,
        95 as std::ffi::c_char,
        109 as std::ffi::c_char,
        97 as std::ffi::c_char,
        112 as std::ffi::c_char,
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
        50 as std::ffi::c_char,
        58 as std::ffi::c_char,
        52 as std::ffi::c_char,
        53 as std::ffi::c_char,
        58 as std::ffi::c_char,
        49 as std::ffi::c_char,
        49 as std::ffi::c_char,
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

pub static mut tmbbox: [fixed_t; (4) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut tmthing: *mut mobj_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut tmflags: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut tmx: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut tmy: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut floatok: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut tmfloorz: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut tmceilingz: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut tmdropoffz: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut ceilingline: *mut line_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub const MAXSPECIALCROSS: std::ffi::c_int = 8;

pub static mut spechit: [*mut line_t; (MAXSPECIALCROSS) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut numspechit: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn PIT_StompThing(thing: *mut mobj_t) -> boolean {
    unsafe { todo!("body not yet translated") }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_TeleportMove(thing: *mut mobj_t, x: fixed_t, y: fixed_t) -> boolean {
    unsafe { todo!("body not yet translated") }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn PIT_CheckLine(ld: *mut line_t) -> boolean {
    unsafe { todo!("body not yet translated") }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn PIT_CheckThing(thing: *mut mobj_t) -> boolean {
    unsafe { todo!("body not yet translated") }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_CheckPosition(thing: *mut mobj_t, x: fixed_t, y: fixed_t) -> boolean {
    unsafe { todo!("body not yet translated") }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_TryMove(thing: *mut mobj_t, x: fixed_t, y: fixed_t) -> boolean {
    unsafe { todo!("body not yet translated") }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_ThingHeightClip(thing: *mut mobj_t) -> boolean {
    unsafe { todo!("body not yet translated") }
    todo!("fell off the end of a non-void C function")
}

pub static mut bestslidefrac: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut secondslidefrac: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut bestslideline: *mut line_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut secondslideline: *mut line_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut slidemo: *mut mobj_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut tmxmove: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut tmymove: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn P_HitSlideLine(ld: *mut line_t) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn PTR_SlideTraverse(in_: *mut intercept_t) -> boolean {
    unsafe { todo!("body not yet translated") }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_SlideMove(mo: *mut mobj_t) {
    unsafe { todo!("body not yet translated") }
}

pub static mut linetarget: *mut mobj_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut shootthing: *mut mobj_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut shootz: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut la_damage: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut attackrange: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut aimslope: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

unsafe extern "C" {
    pub static mut topslope: fixed_t;
}

unsafe extern "C" {
    pub static mut bottomslope: fixed_t;
}

pub unsafe extern "C" fn PTR_AimTraverse(in_: *mut intercept_t) -> boolean {
    unsafe { todo!("body not yet translated") }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn PTR_ShootTraverse(in_: *mut intercept_t) -> boolean {
    unsafe { todo!("body not yet translated") }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_AimLineAttack(
    t1: *mut mobj_t,
    angle: angle_t,
    distance: fixed_t,
) -> fixed_t {
    unsafe { todo!("body not yet translated") }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_LineAttack(
    t1: *mut mobj_t,
    angle: angle_t,
    distance: fixed_t,
    slope: fixed_t,
    damage: std::ffi::c_int,
) {
    unsafe { todo!("body not yet translated") }
}

pub static mut usething: *mut mobj_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn PTR_UseTraverse(in_: *mut intercept_t) -> boolean {
    unsafe { todo!("body not yet translated") }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_UseLines(player: *mut player_t) {
    unsafe { todo!("body not yet translated") }
}

pub static mut bombsource: *mut mobj_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut bombspot: *mut mobj_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut bombdamage: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn PIT_RadiusAttack(thing: *mut mobj_t) -> boolean {
    unsafe { todo!("body not yet translated") }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_RadiusAttack(
    spot: *mut mobj_t,
    source: *mut mobj_t,
    damage: std::ffi::c_int,
) {
    unsafe { todo!("body not yet translated") }
}

pub static mut crushchange: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut nofit: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn PIT_ChangeSector(thing: *mut mobj_t) -> boolean {
    unsafe { todo!("body not yet translated") }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_ChangeSector(sector: *mut sector_t, crunch: boolean) -> boolean {
    unsafe { todo!("body not yet translated") }
    todo!("fell off the end of a non-void C function")
}
