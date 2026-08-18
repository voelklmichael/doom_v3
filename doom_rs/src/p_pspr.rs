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
use crate::info::*;
use crate::m_fixed::*;
use crate::m_random::*;
use crate::p_local::*;
use crate::p_mobj::*;
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

pub const FF_FULLBRIGHT: std::ffi::c_int = 0x8000;

pub const FF_FRAMEMASK: std::ffi::c_int = 0x7fff;

pub const ps_weapon: std::ffi::c_int = 0;
pub const ps_flash: std::ffi::c_int = ps_weapon + 1;
pub const NUMPSPRITES: std::ffi::c_int = ps_flash + 1;

pub type psprnum_t = std::ffi::c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pspdef_t {
    pub state: *mut state_t,
    pub tics: std::ffi::c_int,
    pub sx: fixed_t,
    pub sy: fixed_t,
}

static mut rcsid: [std::ffi::c_char; 49] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        112 as std::ffi::c_char,
        95 as std::ffi::c_char,
        112 as std::ffi::c_char,
        115 as std::ffi::c_char,
        112 as std::ffi::c_char,
        114 as std::ffi::c_char,
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
        50 as std::ffi::c_char,
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

pub const LOWERSPEED: std::ffi::c_int = (FRACUNIT * 6);

pub const RAISESPEED: std::ffi::c_int = (FRACUNIT * 6);

pub const WEAPONBOTTOM: std::ffi::c_int = (128 * FRACUNIT);

pub const WEAPONTOP: std::ffi::c_int = (32 * FRACUNIT);

pub const BFGCELLS: std::ffi::c_int = 40;

pub unsafe extern "C" fn P_SetPsprite(
    player: *mut player_t,
    position: std::ffi::c_int,
    stnum: statenum_t,
) {
    unsafe { todo!("body not yet translated") }
}

pub static mut swingx: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut swingy: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn P_CalcSwing(player: *mut player_t) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn P_BringUpWeapon(player: *mut player_t) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn P_CheckAmmo(player: *mut player_t) -> boolean {
    unsafe { todo!("body not yet translated") }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_FireWeapon(player: *mut player_t) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn P_DropWeapon(player: *mut player_t) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn A_WeaponReady(player: *mut player_t, psp: *mut pspdef_t) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn A_ReFire(player: *mut player_t, psp: *mut pspdef_t) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn A_CheckReload(player: *mut player_t, psp: *mut pspdef_t) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn A_Lower(player: *mut player_t, psp: *mut pspdef_t) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn A_Raise(player: *mut player_t, psp: *mut pspdef_t) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn A_GunFlash(player: *mut player_t, psp: *mut pspdef_t) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn A_Punch(player: *mut player_t, psp: *mut pspdef_t) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn A_Saw(player: *mut player_t, psp: *mut pspdef_t) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn A_FireMissile(player: *mut player_t, psp: *mut pspdef_t) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn A_FireBFG(player: *mut player_t, psp: *mut pspdef_t) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn A_FirePlasma(player: *mut player_t, psp: *mut pspdef_t) {
    unsafe { todo!("body not yet translated") }
}

pub static mut bulletslope: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn P_BulletSlope(mo: *mut mobj_t) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn P_GunShot(mo: *mut mobj_t, accurate: boolean) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn A_FirePistol(player: *mut player_t, psp: *mut pspdef_t) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn A_FireShotgun(player: *mut player_t, psp: *mut pspdef_t) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn A_FireShotgun2(player: *mut player_t, psp: *mut pspdef_t) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn A_FireCGun(player: *mut player_t, psp: *mut pspdef_t) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn A_Light0(player: *mut player_t, psp: *mut pspdef_t) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn A_Light1(player: *mut player_t, psp: *mut pspdef_t) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn A_Light2(player: *mut player_t, psp: *mut pspdef_t) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn A_BFGSpray(mo: *mut mobj_t) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn A_BFGsound(player: *mut player_t, psp: *mut pspdef_t) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn P_SetupPsprites(player: *mut player_t) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn P_MovePsprites(player: *mut player_t) {
    unsafe { todo!("body not yet translated") }
}
