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
use crate::i_system::*;
use crate::info::*;
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

static mut rcsid: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub const DI_EAST: std::ffi::c_int = 0;
pub const DI_NORTHEAST: std::ffi::c_int = DI_EAST + 1;
pub const DI_NORTH: std::ffi::c_int = DI_NORTHEAST + 1;
pub const DI_NORTHWEST: std::ffi::c_int = DI_NORTH + 1;
pub const DI_WEST: std::ffi::c_int = DI_NORTHWEST + 1;
pub const DI_SOUTHWEST: std::ffi::c_int = DI_WEST + 1;
pub const DI_SOUTH: std::ffi::c_int = DI_SOUTHWEST + 1;
pub const DI_SOUTHEAST: std::ffi::c_int = DI_SOUTH + 1;
pub const DI_NODIR: std::ffi::c_int = DI_SOUTHEAST + 1;
pub const NUMDIRS: std::ffi::c_int = DI_NODIR + 1;

pub type dirtype_t = std::ffi::c_int;

pub static mut opposite: *mut dirtype_t /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut diags: *mut dirtype_t /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut soundtarget: *mut mobj_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn P_RecursiveSound(sec: *mut sector_t, soundblocks: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_NoiseAlert(target: *mut mobj_t, emmiter: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_CheckMeleeRange(actor: *mut mobj_t) -> boolean {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_CheckMissileRange(actor: *mut mobj_t) -> boolean {
    todo!("body not yet translated")
}

pub static mut xspeed: [fixed_t; (8) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut yspeed: [fixed_t; (8) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub const MAXSPECIALCROSS: std::ffi::c_int = 8;

unsafe extern "C" {
    pub static mut spechit: [*mut line_t; (MAXSPECIALCROSS) as usize];
}

unsafe extern "C" {
    pub static mut numspechit: std::ffi::c_int;
}

pub unsafe extern "C" fn P_Move(actor: *mut mobj_t) -> boolean {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_TryWalk(actor: *mut mobj_t) -> boolean {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_NewChaseDir(actor: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_LookForPlayers(actor: *mut mobj_t, allaround: boolean) -> boolean {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_KeenDie(mo: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_Look(actor: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_Chase(actor: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_FaceTarget(actor: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_PosAttack(actor: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_SPosAttack(actor: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_CPosAttack(actor: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_CPosRefire(actor: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_SpidRefire(actor: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_BspiAttack(actor: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_TroopAttack(actor: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_SargAttack(actor: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_HeadAttack(actor: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_CyberAttack(actor: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_BruisAttack(actor: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_SkelMissile(actor: *mut mobj_t) {
    todo!("body not yet translated")
}

pub static mut TRACEANGLE: std::ffi::c_int = unsafe { 0xc000000 };

pub unsafe extern "C" fn A_Tracer(actor: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_SkelWhoosh(actor: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_SkelFist(actor: *mut mobj_t) {
    todo!("body not yet translated")
}

pub static mut corpsehit: *mut mobj_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut vileobj: *mut mobj_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut viletryx: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut viletryy: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn PIT_VileCheck(thing: *mut mobj_t) -> boolean {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_VileChase(actor: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_VileStart(actor: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_StartFire(actor: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_FireCrackle(actor: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_Fire(actor: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_VileTarget(actor: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_VileAttack(actor: *mut mobj_t) {
    todo!("body not yet translated")
}

pub const FATSPREAD: std::ffi::c_int = (ANG90 / 8);

pub unsafe extern "C" fn A_FatRaise(actor: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_FatAttack1(actor: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_FatAttack2(actor: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_FatAttack3(actor: *mut mobj_t) {
    todo!("body not yet translated")
}

pub const SKULLSPEED: std::ffi::c_int = (20 * FRACUNIT);

pub unsafe extern "C" fn A_SkullAttack(actor: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_PainShootSkull(actor: *mut mobj_t, angle: angle_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_PainAttack(actor: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_PainDie(actor: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_Scream(actor: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_XScream(actor: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_Pain(actor: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_Fall(actor: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_Explode(thingy: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_BossDeath(mo: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_Hoof(mo: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_Metal(mo: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_BabyMetal(mo: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_OpenShotgun2(player: *mut player_t, psp: *mut pspdef_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_LoadShotgun2(player: *mut player_t, psp: *mut pspdef_t) {
    todo!("body not yet translated")
}

unsafe extern "C" {
    pub fn A_ReFire(player: *mut player_t, psp: *mut pspdef_t);
}

pub unsafe extern "C" fn A_CloseShotgun2(player: *mut player_t, psp: *mut pspdef_t) {
    todo!("body not yet translated")
}

pub static mut braintargets: [*mut mobj_t; (32) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut numbraintargets: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut braintargeton: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn A_BrainAwake(mo: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_BrainPain(mo: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_BrainScream(mo: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_BrainExplode(mo: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_BrainDie(mo: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_BrainSpit(mo: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_SpawnSound(mo: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_SpawnFly(mo: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn A_PlayerScream(mo: *mut mobj_t) {
    todo!("body not yet translated")
}
