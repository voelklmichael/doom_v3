use crate::d_items::*;
use crate::d_player::*;
use crate::d_think::*;
use crate::d_ticcmd::*;
use crate::doomdata::*;
use crate::doomdef::*;
use crate::doomtype::*;
use crate::info::*;
use crate::m_fixed::*;
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
use crate::tables::*;

pub const FLOATSPEED: std::ffi::c_int = (FRACUNIT * 4);

pub const MAXHEALTH: std::ffi::c_int = 100;

pub const VIEWHEIGHT: std::ffi::c_int = (41 * FRACUNIT);

pub const MAPBLOCKUNITS: std::ffi::c_int = 128;

pub const MAPBLOCKSIZE: std::ffi::c_int = (MAPBLOCKUNITS * FRACUNIT);

pub const MAPBLOCKSHIFT: std::ffi::c_int = (FRACBITS + 7);

pub const MAPBMASK: std::ffi::c_int = (MAPBLOCKSIZE - 1);

pub const MAPBTOFRAC: std::ffi::c_int = (MAPBLOCKSHIFT - FRACBITS);

pub const PLAYERRADIUS: std::ffi::c_int = (16 * FRACUNIT);

pub const MAXRADIUS: std::ffi::c_int = (32 * FRACUNIT);

pub const GRAVITY: std::ffi::c_int = FRACUNIT;

pub const MAXMOVE: std::ffi::c_int = (30 * FRACUNIT);

pub const USERANGE: std::ffi::c_int = (64 * FRACUNIT);

pub const MELEERANGE: std::ffi::c_int = (64 * FRACUNIT);

pub const MISSILERANGE: std::ffi::c_int = ((32 * 64) * FRACUNIT);

pub const BASETHRESHOLD: std::ffi::c_int = 100;

unsafe extern "C" {
    pub static mut thinkercap: thinker_t;
}

unsafe extern "C" {
    pub fn P_InitThinkers();
}

unsafe extern "C" {
    pub fn P_AddThinker(thinker: *mut thinker_t);
}

unsafe extern "C" {
    pub fn P_RemoveThinker(thinker: *mut thinker_t);
}

unsafe extern "C" {
    pub fn P_SetupPsprites(curplayer: *mut player_t);
}

unsafe extern "C" {
    pub fn P_MovePsprites(curplayer: *mut player_t);
}

unsafe extern "C" {
    pub fn P_DropWeapon(player: *mut player_t);
}

unsafe extern "C" {
    pub fn P_PlayerThink(player: *mut player_t);
}

pub const ONFLOORZ: std::ffi::c_int = MININT;

pub const ONCEILINGZ: std::ffi::c_int = MAXINT;

pub const ITEMQUESIZE: std::ffi::c_int = 128;

unsafe extern "C" {
    pub static mut itemrespawnque: [mapthing_t; (ITEMQUESIZE) as usize];
}

unsafe extern "C" {
    pub static mut itemrespawntime: [std::ffi::c_int; (ITEMQUESIZE) as usize];
}

unsafe extern "C" {
    pub static mut iquehead: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut iquetail: std::ffi::c_int;
}

unsafe extern "C" {
    pub fn P_RespawnSpecials();
}

unsafe extern "C" {
    pub fn P_SpawnMobj(x: fixed_t, y: fixed_t, z: fixed_t, type_: mobjtype_t) -> *mut mobj_t;
}

unsafe extern "C" {
    pub fn P_RemoveMobj(th: *mut mobj_t);
}

unsafe extern "C" {
    pub fn P_SetMobjState(mobj: *mut mobj_t, state: statenum_t) -> boolean;
}

unsafe extern "C" {
    pub fn P_MobjThinker(mobj: *mut mobj_t);
}

unsafe extern "C" {
    pub fn P_SpawnPuff(x: fixed_t, y: fixed_t, z: fixed_t);
}

unsafe extern "C" {
    pub fn P_SpawnBlood(x: fixed_t, y: fixed_t, z: fixed_t, damage: std::ffi::c_int);
}

unsafe extern "C" {
    pub fn P_SpawnMissile(source: *mut mobj_t, dest: *mut mobj_t, type_: mobjtype_t)
    -> *mut mobj_t;
}

unsafe extern "C" {
    pub fn P_SpawnPlayerMissile(source: *mut mobj_t, type_: mobjtype_t);
}

unsafe extern "C" {
    pub fn P_NoiseAlert(target: *mut mobj_t, emmiter: *mut mobj_t);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct divline_t {
    pub x: fixed_t,
    pub y: fixed_t,
    pub dx: fixed_t,
    pub dy: fixed_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union intercept_t_d {
    pub thing: *mut mobj_t,
    pub line: *mut line_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intercept_t {
    pub frac: fixed_t,
    pub isaline: boolean,
    pub d: intercept_t_d,
}

pub const MAXINTERCEPTS: std::ffi::c_int = 128;

unsafe extern "C" {
    pub static mut intercepts: [intercept_t; (MAXINTERCEPTS) as usize];
}

unsafe extern "C" {
    pub static mut intercept_p: *mut intercept_t;
}

pub type traverser_t = Option<unsafe extern "C" fn(*mut intercept_t) -> boolean>;

unsafe extern "C" {
    pub fn P_AproxDistance(dx: fixed_t, dy: fixed_t) -> fixed_t;
}

unsafe extern "C" {
    pub fn P_PointOnLineSide(x: fixed_t, y: fixed_t, line: *mut line_t) -> std::ffi::c_int;
}

unsafe extern "C" {
    pub fn P_PointOnDivlineSide(x: fixed_t, y: fixed_t, line: *mut divline_t) -> std::ffi::c_int;
}

unsafe extern "C" {
    pub fn P_MakeDivline(li: *mut line_t, dl: *mut divline_t);
}

unsafe extern "C" {
    pub fn P_InterceptVector(v2: *mut divline_t, v1: *mut divline_t) -> fixed_t;
}

unsafe extern "C" {
    pub fn P_BoxOnLineSide(tmbox: *mut fixed_t, ld: *mut line_t) -> std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut opentop: fixed_t;
}

unsafe extern "C" {
    pub static mut openbottom: fixed_t;
}

unsafe extern "C" {
    pub static mut openrange: fixed_t;
}

unsafe extern "C" {
    pub static mut lowfloor: fixed_t;
}

unsafe extern "C" {
    pub fn P_LineOpening(linedef: *mut line_t);
}

unsafe extern "C" {
    pub fn P_BlockLinesIterator(
        x: std::ffi::c_int,
        y: std::ffi::c_int,
        func: Option<unsafe extern "C" fn(*mut line_t) -> boolean>,
    ) -> boolean;
}

unsafe extern "C" {
    pub fn P_BlockThingsIterator(
        x: std::ffi::c_int,
        y: std::ffi::c_int,
        func: Option<unsafe extern "C" fn(*mut mobj_t) -> boolean>,
    ) -> boolean;
}

pub const PT_ADDLINES: std::ffi::c_int = 1;

pub const PT_ADDTHINGS: std::ffi::c_int = 2;

pub const PT_EARLYOUT: std::ffi::c_int = 4;

unsafe extern "C" {
    pub static mut trace: divline_t;
}

unsafe extern "C" {
    pub fn P_PathTraverse(
        x1: fixed_t,
        y1: fixed_t,
        x2: fixed_t,
        y2: fixed_t,
        flags: std::ffi::c_int,
        trav: Option<unsafe extern "C" fn(*mut intercept_t) -> boolean>,
    ) -> boolean;
}

unsafe extern "C" {
    pub fn P_UnsetThingPosition(thing: *mut mobj_t);
}

unsafe extern "C" {
    pub fn P_SetThingPosition(thing: *mut mobj_t);
}

unsafe extern "C" {
    pub static mut floatok: boolean;
}

unsafe extern "C" {
    pub static mut tmfloorz: fixed_t;
}

unsafe extern "C" {
    pub static mut tmceilingz: fixed_t;
}

unsafe extern "C" {
    pub static mut ceilingline: *mut line_t;
}

unsafe extern "C" {
    pub fn P_CheckPosition(thing: *mut mobj_t, x: fixed_t, y: fixed_t) -> boolean;
}

unsafe extern "C" {
    pub fn P_TryMove(thing: *mut mobj_t, x: fixed_t, y: fixed_t) -> boolean;
}

unsafe extern "C" {
    pub fn P_TeleportMove(thing: *mut mobj_t, x: fixed_t, y: fixed_t) -> boolean;
}

unsafe extern "C" {
    pub fn P_SlideMove(mo: *mut mobj_t);
}

unsafe extern "C" {
    pub fn P_CheckSight(t1: *mut mobj_t, t2: *mut mobj_t) -> boolean;
}

unsafe extern "C" {
    pub fn P_UseLines(player: *mut player_t);
}

unsafe extern "C" {
    pub fn P_ChangeSector(sector: *mut sector_t, crunch: boolean) -> boolean;
}

unsafe extern "C" {
    pub static mut linetarget: *mut mobj_t;
}

unsafe extern "C" {
    pub fn P_AimLineAttack(t1: *mut mobj_t, angle: angle_t, distance: fixed_t) -> fixed_t;
}

unsafe extern "C" {
    pub fn P_LineAttack(
        t1: *mut mobj_t,
        angle: angle_t,
        distance: fixed_t,
        slope: fixed_t,
        damage: std::ffi::c_int,
    );
}

unsafe extern "C" {
    pub fn P_RadiusAttack(spot: *mut mobj_t, source: *mut mobj_t, damage: std::ffi::c_int);
}

unsafe extern "C" {
    pub static mut rejectmatrix: *mut byte;
}

unsafe extern "C" {
    pub static mut blockmaplump: *mut std::ffi::c_short;
}

unsafe extern "C" {
    pub static mut blockmap: *mut std::ffi::c_short;
}

unsafe extern "C" {
    pub static mut bmapwidth: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut bmapheight: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut bmaporgx: fixed_t;
}

unsafe extern "C" {
    pub static mut bmaporgy: fixed_t;
}

unsafe extern "C" {
    pub static mut blocklinks: *mut *mut mobj_t;
}

unsafe extern "C" {
    pub static mut maxammo: [std::ffi::c_int; (NUMAMMO) as usize];
}

unsafe extern "C" {
    pub static mut clipammo: [std::ffi::c_int; (NUMAMMO) as usize];
}

unsafe extern "C" {
    pub fn P_TouchSpecialThing(special: *mut mobj_t, toucher: *mut mobj_t);
}

unsafe extern "C" {
    pub fn P_DamageMobj(
        target: *mut mobj_t,
        inflictor: *mut mobj_t,
        source: *mut mobj_t,
        damage: std::ffi::c_int,
    );
}
