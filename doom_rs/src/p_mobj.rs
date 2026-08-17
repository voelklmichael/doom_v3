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
use crate::hu_stuff::*;
use crate::i_system::*;
use crate::info::*;
use crate::m_fixed::*;
use crate::m_random::*;
use crate::p_local::*;
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
use crate::st_stuff::*;
use crate::tables::*;
use crate::z_zone::*;

pub const MF_SPECIAL: std::ffi::c_int = 1;
pub const MF_SOLID: std::ffi::c_int = 2;
pub const MF_SHOOTABLE: std::ffi::c_int = 4;
pub const MF_NOSECTOR: std::ffi::c_int = 8;
pub const MF_NOBLOCKMAP: std::ffi::c_int = 16;
pub const MF_AMBUSH: std::ffi::c_int = 32;
pub const MF_JUSTHIT: std::ffi::c_int = 64;
pub const MF_JUSTATTACKED: std::ffi::c_int = 128;
pub const MF_SPAWNCEILING: std::ffi::c_int = 256;
pub const MF_NOGRAVITY: std::ffi::c_int = 512;
pub const MF_DROPOFF: std::ffi::c_int = 0x400;
pub const MF_PICKUP: std::ffi::c_int = 0x800;
pub const MF_NOCLIP: std::ffi::c_int = 0x1000;
pub const MF_SLIDE: std::ffi::c_int = 0x2000;
pub const MF_FLOAT: std::ffi::c_int = 0x4000;
pub const MF_TELEPORT: std::ffi::c_int = 0x8000;
pub const MF_MISSILE: std::ffi::c_int = 0x10000;
pub const MF_DROPPED: std::ffi::c_int = 0x20000;
pub const MF_SHADOW: std::ffi::c_int = 0x40000;
pub const MF_NOBLOOD: std::ffi::c_int = 0x80000;
pub const MF_CORPSE: std::ffi::c_int = 0x100000;
pub const MF_INFLOAT: std::ffi::c_int = 0x200000;
pub const MF_COUNTKILL: std::ffi::c_int = 0x400000;
pub const MF_COUNTITEM: std::ffi::c_int = 0x800000;
pub const MF_SKULLFLY: std::ffi::c_int = 0x1000000;
pub const MF_NOTDMATCH: std::ffi::c_int = 0x2000000;
pub const MF_TRANSLATION: std::ffi::c_int = 0xc000000;
pub const MF_TRANSSHIFT: std::ffi::c_int = 26;

pub type mobjflag_t = std::ffi::c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mobj_t {
    pub thinker: thinker_t,
    pub x: fixed_t,
    pub y: fixed_t,
    pub z: fixed_t,
    pub snext: *mut mobj_s,
    pub sprev: *mut mobj_s,
    pub angle: angle_t,
    pub sprite: spritenum_t,
    pub frame: std::ffi::c_int,
    pub bnext: *mut mobj_s,
    pub bprev: *mut mobj_s,
    pub subsector: *mut subsector_s,
    pub floorz: fixed_t,
    pub ceilingz: fixed_t,
    pub radius: fixed_t,
    pub height: fixed_t,
    pub momx: fixed_t,
    pub momy: fixed_t,
    pub momz: fixed_t,
    pub validcount: std::ffi::c_int,
    pub type_: mobjtype_t,
    pub info: *mut mobjinfo_t,
    pub tics: std::ffi::c_int,
    pub state: *mut state_t,
    pub flags: std::ffi::c_int,
    pub health: std::ffi::c_int,
    pub movedir: std::ffi::c_int,
    pub movecount: std::ffi::c_int,
    pub target: *mut mobj_s,
    pub reactiontime: std::ffi::c_int,
    pub threshold: std::ffi::c_int,
    pub player: *mut player_s,
    pub lastlook: std::ffi::c_int,
    pub spawnpoint: mapthing_t,
    pub tracer: *mut mobj_s,
}

pub type mobj_s = mobj_t;

static mut rcsid: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

unsafe extern "C" {
    pub fn G_PlayerReborn(player: std::ffi::c_int);
}

pub static mut test: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn P_SetMobjState(mobj: *mut mobj_t, state: statenum_t) -> boolean {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_ExplodeMissile(mo: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_XYMovement(mo: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_ZMovement(mo: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_NightmareRespawn(mobj: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_MobjThinker(mobj: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_SpawnMobj(
    x: fixed_t,
    y: fixed_t,
    z: fixed_t,
    type_: mobjtype_t,
) -> *mut mobj_t {
    todo!("body not yet translated")
}

pub static mut itemrespawnque: [mapthing_t; (ITEMQUESIZE) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut itemrespawntime: [std::ffi::c_int; (ITEMQUESIZE) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut iquehead: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut iquetail: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn P_RemoveMobj(mobj: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_RespawnSpecials() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_SpawnPlayer(mthing: *mut mapthing_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_SpawnMapThing(mthing: *mut mapthing_t) {
    todo!("body not yet translated")
}

unsafe extern "C" {
    pub static mut attackrange: fixed_t;
}

pub unsafe extern "C" fn P_SpawnPuff(x: fixed_t, y: fixed_t, z: fixed_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_SpawnBlood(x: fixed_t, y: fixed_t, z: fixed_t, damage: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_CheckMissileSpawn(th: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_SpawnMissile(
    source: *mut mobj_t,
    dest: *mut mobj_t,
    type_: mobjtype_t,
) -> *mut mobj_t {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_SpawnPlayerMissile(source: *mut mobj_t, type_: mobjtype_t) {
    todo!("body not yet translated")
}
