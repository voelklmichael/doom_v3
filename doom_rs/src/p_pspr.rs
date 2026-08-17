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

static mut rcsid: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn P_SetPsprite(
    player: *mut player_t,
    position: std::ffi::c_int,
    stnum: statenum_t,
) {
    todo!("body not yet translated")
}

pub static mut swingx: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut swingy: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn P_CalcSwing(player: *mut player_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_BringUpWeapon(player: *mut player_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_CheckAmmo(player: *mut player_t) -> boolean {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_FireWeapon(player: *mut player_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_DropWeapon(player: *mut player_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_WeaponReady(player: *mut player_t, psp: *mut pspdef_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_ReFire(player: *mut player_t, psp: *mut pspdef_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_CheckReload(player: *mut player_t, psp: *mut pspdef_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_Lower(player: *mut player_t, psp: *mut pspdef_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_Raise(player: *mut player_t, psp: *mut pspdef_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_GunFlash(player: *mut player_t, psp: *mut pspdef_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_Punch(player: *mut player_t, psp: *mut pspdef_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_Saw(player: *mut player_t, psp: *mut pspdef_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_FireMissile(player: *mut player_t, psp: *mut pspdef_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_FireBFG(player: *mut player_t, psp: *mut pspdef_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_FirePlasma(player: *mut player_t, psp: *mut pspdef_t) {
    todo!("body not yet translated")
}

pub static mut bulletslope: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn P_BulletSlope(mo: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_GunShot(mo: *mut mobj_t, accurate: boolean) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_FirePistol(player: *mut player_t, psp: *mut pspdef_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_FireShotgun(player: *mut player_t, psp: *mut pspdef_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_FireShotgun2(player: *mut player_t, psp: *mut pspdef_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_FireCGun(player: *mut player_t, psp: *mut pspdef_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_Light0(player: *mut player_t, psp: *mut pspdef_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_Light1(player: *mut player_t, psp: *mut pspdef_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_Light2(player: *mut player_t, psp: *mut pspdef_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_BFGSpray(mo: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_BFGsound(player: *mut player_t, psp: *mut pspdef_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_SetupPsprites(player: *mut player_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_MovePsprites(player: *mut player_t) {
    todo!("body not yet translated")
}
