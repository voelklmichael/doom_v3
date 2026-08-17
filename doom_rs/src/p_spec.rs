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
use crate::m_argv::*;
use crate::m_fixed::*;
use crate::m_random::*;
use crate::p_local::*;
use crate::p_mobj::*;
use crate::p_pspr::*;
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
use crate::w_wad::*;
use crate::z_zone::*;

unsafe extern "C" {
    pub fn P_UseSpecialLine(
        thing: *mut mobj_t,
        line: *mut line_t,
        side: std::ffi::c_int,
    ) -> boolean;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fireflicker_t {
    pub thinker: thinker_t,
    pub sector: *mut sector_t,
    pub count: std::ffi::c_int,
    pub maxlight: std::ffi::c_int,
    pub minlight: std::ffi::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lightflash_t {
    pub thinker: thinker_t,
    pub sector: *mut sector_t,
    pub count: std::ffi::c_int,
    pub maxlight: std::ffi::c_int,
    pub minlight: std::ffi::c_int,
    pub maxtime: std::ffi::c_int,
    pub mintime: std::ffi::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct strobe_t {
    pub thinker: thinker_t,
    pub sector: *mut sector_t,
    pub count: std::ffi::c_int,
    pub minlight: std::ffi::c_int,
    pub maxlight: std::ffi::c_int,
    pub darktime: std::ffi::c_int,
    pub brighttime: std::ffi::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct glow_t {
    pub thinker: thinker_t,
    pub sector: *mut sector_t,
    pub minlight: std::ffi::c_int,
    pub maxlight: std::ffi::c_int,
    pub direction: std::ffi::c_int,
}

unsafe extern "C" {
    pub fn P_SpawnFireFlicker(sector: *mut sector_t);
}

unsafe extern "C" {
    pub fn T_LightFlash(flash: *mut lightflash_t);
}

unsafe extern "C" {
    pub fn P_SpawnLightFlash(sector: *mut sector_t);
}

unsafe extern "C" {
    pub fn T_StrobeFlash(flash: *mut strobe_t);
}

unsafe extern "C" {
    pub fn P_SpawnStrobeFlash(
        sector: *mut sector_t,
        fastOrSlow: std::ffi::c_int,
        inSync: std::ffi::c_int,
    );
}

unsafe extern "C" {
    pub fn EV_StartLightStrobing(line: *mut line_t);
}

unsafe extern "C" {
    pub fn EV_TurnTagLightsOff(line: *mut line_t);
}

unsafe extern "C" {
    pub fn EV_LightTurnOn(line: *mut line_t, bright: std::ffi::c_int);
}

unsafe extern "C" {
    pub fn T_Glow(g: *mut glow_t);
}

unsafe extern "C" {
    pub fn P_SpawnGlowingLight(sector: *mut sector_t);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct switchlist_t {
    pub name1: [std::ffi::c_char; (9) as usize],
    pub name2: [std::ffi::c_char; (9) as usize],
    pub episode: std::ffi::c_short,
}

pub const top: std::ffi::c_int = 0;
pub const middle: std::ffi::c_int = top + 1;
pub const bottom: std::ffi::c_int = middle + 1;

pub type bwhere_e = std::ffi::c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct button_t {
    pub line: *mut line_t,
    pub where_: bwhere_e,
    pub btexture: std::ffi::c_int,
    pub btimer: std::ffi::c_int,
    pub soundorg: *mut mobj_t,
}

unsafe extern "C" {
    pub static mut buttonlist: [button_t; (MAXBUTTONS) as usize];
}

unsafe extern "C" {
    pub fn P_ChangeSwitchTexture(line: *mut line_t, useAgain: std::ffi::c_int);
}

unsafe extern "C" {
    pub fn P_InitSwitchList();
}

pub const up: std::ffi::c_int = 0;
pub const down: std::ffi::c_int = up + 1;
pub const waiting: std::ffi::c_int = down + 1;
pub const in_stasis: std::ffi::c_int = waiting + 1;

pub type plat_e = std::ffi::c_int;

pub const perpetualRaise: std::ffi::c_int = 0;
pub const downWaitUpStay: std::ffi::c_int = perpetualRaise + 1;
pub const raiseAndChange: std::ffi::c_int = downWaitUpStay + 1;
pub const raiseToNearestAndChange: std::ffi::c_int = raiseAndChange + 1;
pub const blazeDWUS: std::ffi::c_int = raiseToNearestAndChange + 1;

pub type plattype_e = std::ffi::c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct plat_t {
    pub thinker: thinker_t,
    pub sector: *mut sector_t,
    pub speed: fixed_t,
    pub low: fixed_t,
    pub high: fixed_t,
    pub wait: std::ffi::c_int,
    pub count: std::ffi::c_int,
    pub status: plat_e,
    pub oldstatus: plat_e,
    pub crush: boolean,
    pub tag: std::ffi::c_int,
    pub type_: plattype_e,
}

unsafe extern "C" {
    pub static mut activeplats: [*mut plat_t; (MAXPLATS) as usize];
}

unsafe extern "C" {
    pub fn T_PlatRaise(plat: *mut plat_t);
}

unsafe extern "C" {
    pub fn EV_DoPlat(
        line: *mut line_t,
        type_: plattype_e,
        amount: std::ffi::c_int,
    ) -> std::ffi::c_int;
}

unsafe extern "C" {
    pub fn P_AddActivePlat(plat: *mut plat_t);
}

unsafe extern "C" {
    pub fn P_RemoveActivePlat(plat: *mut plat_t);
}

unsafe extern "C" {
    pub fn EV_StopPlat(line: *mut line_t);
}

unsafe extern "C" {
    pub fn P_ActivateInStasis(tag: std::ffi::c_int);
}

pub const normal: std::ffi::c_int = 0;
pub const close30ThenOpen: std::ffi::c_int = normal + 1;
pub const close: std::ffi::c_int = close30ThenOpen + 1;
pub const open: std::ffi::c_int = close + 1;
pub const raiseIn5Mins: std::ffi::c_int = open + 1;
pub const blazeRaise: std::ffi::c_int = raiseIn5Mins + 1;
pub const blazeOpen: std::ffi::c_int = blazeRaise + 1;
pub const blazeClose: std::ffi::c_int = blazeOpen + 1;

pub type vldoor_e = std::ffi::c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vldoor_t {
    pub thinker: thinker_t,
    pub type_: vldoor_e,
    pub sector: *mut sector_t,
    pub topheight: fixed_t,
    pub speed: fixed_t,
    pub direction: std::ffi::c_int,
    pub topwait: std::ffi::c_int,
    pub topcountdown: std::ffi::c_int,
}

unsafe extern "C" {
    pub fn EV_VerticalDoor(line: *mut line_t, thing: *mut mobj_t);
}

unsafe extern "C" {
    pub fn EV_DoDoor(line: *mut line_t, type_: vldoor_e) -> std::ffi::c_int;
}

unsafe extern "C" {
    pub fn EV_DoLockedDoor(
        line: *mut line_t,
        type_: vldoor_e,
        thing: *mut mobj_t,
    ) -> std::ffi::c_int;
}

unsafe extern "C" {
    pub fn T_VerticalDoor(door: *mut vldoor_t);
}

unsafe extern "C" {
    pub fn P_SpawnDoorCloseIn30(sec: *mut sector_t);
}

unsafe extern "C" {
    pub fn P_SpawnDoorRaiseIn5Mins(sec: *mut sector_t, secnum: std::ffi::c_int);
}

pub const lowerToFloor: std::ffi::c_int = 0;
pub const raiseToHighest: std::ffi::c_int = lowerToFloor + 1;
pub const lowerAndCrush: std::ffi::c_int = raiseToHighest + 1;
pub const crushAndRaise: std::ffi::c_int = lowerAndCrush + 1;
pub const fastCrushAndRaise: std::ffi::c_int = crushAndRaise + 1;
pub const silentCrushAndRaise: std::ffi::c_int = fastCrushAndRaise + 1;

pub type ceiling_e = std::ffi::c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceiling_t {
    pub thinker: thinker_t,
    pub type_: ceiling_e,
    pub sector: *mut sector_t,
    pub bottomheight: fixed_t,
    pub topheight: fixed_t,
    pub speed: fixed_t,
    pub crush: boolean,
    pub direction: std::ffi::c_int,
    pub tag: std::ffi::c_int,
    pub olddirection: std::ffi::c_int,
}

unsafe extern "C" {
    pub static mut activeceilings: [*mut ceiling_t; (MAXCEILINGS) as usize];
}

unsafe extern "C" {
    pub fn EV_DoCeiling(line: *mut line_t, type_: ceiling_e) -> std::ffi::c_int;
}

unsafe extern "C" {
    pub fn T_MoveCeiling(ceiling: *mut ceiling_t);
}

unsafe extern "C" {
    pub fn P_AddActiveCeiling(c: *mut ceiling_t);
}

unsafe extern "C" {
    pub fn P_RemoveActiveCeiling(c: *mut ceiling_t);
}

unsafe extern "C" {
    pub fn EV_CeilingCrushStop(line: *mut line_t) -> std::ffi::c_int;
}

unsafe extern "C" {
    pub fn P_ActivateInStasisCeiling(line: *mut line_t);
}

pub const lowerFloor: std::ffi::c_int = 0;
pub const lowerFloorToLowest: std::ffi::c_int = lowerFloor + 1;
pub const turboLower: std::ffi::c_int = lowerFloorToLowest + 1;
pub const raiseFloor: std::ffi::c_int = turboLower + 1;
pub const raiseFloorToNearest: std::ffi::c_int = raiseFloor + 1;
pub const raiseToTexture: std::ffi::c_int = raiseFloorToNearest + 1;
pub const lowerAndChange: std::ffi::c_int = raiseToTexture + 1;
pub const raiseFloor24: std::ffi::c_int = lowerAndChange + 1;
pub const raiseFloor24AndChange: std::ffi::c_int = raiseFloor24 + 1;
pub const raiseFloorCrush: std::ffi::c_int = raiseFloor24AndChange + 1;
pub const raiseFloorTurbo: std::ffi::c_int = raiseFloorCrush + 1;
pub const donutRaise: std::ffi::c_int = raiseFloorTurbo + 1;
pub const raiseFloor512: std::ffi::c_int = donutRaise + 1;

pub type floor_e = std::ffi::c_int;

pub const build8: std::ffi::c_int = 0;
pub const turbo16: std::ffi::c_int = build8 + 1;

pub type stair_e = std::ffi::c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct floormove_t {
    pub thinker: thinker_t,
    pub type_: floor_e,
    pub crush: boolean,
    pub sector: *mut sector_t,
    pub direction: std::ffi::c_int,
    pub newspecial: std::ffi::c_int,
    pub texture: std::ffi::c_short,
    pub floordestheight: fixed_t,
    pub speed: fixed_t,
}

pub const ok: std::ffi::c_int = 0;
pub const crushed: std::ffi::c_int = ok + 1;
pub const pastdest: std::ffi::c_int = crushed + 1;

pub type result_e = std::ffi::c_int;

unsafe extern "C" {
    pub fn T_MovePlane(
        sector: *mut sector_t,
        speed: fixed_t,
        dest: fixed_t,
        crush: boolean,
        floorOrCeiling: std::ffi::c_int,
        direction: std::ffi::c_int,
    ) -> result_e;
}

unsafe extern "C" {
    pub fn EV_BuildStairs(line: *mut line_t, type_: stair_e) -> std::ffi::c_int;
}

unsafe extern "C" {
    pub fn EV_DoFloor(line: *mut line_t, floortype: floor_e) -> std::ffi::c_int;
}

unsafe extern "C" {
    pub fn T_MoveFloor(floor: *mut floormove_t);
}

unsafe extern "C" {
    pub fn EV_Teleport(
        line: *mut line_t,
        side: std::ffi::c_int,
        thing: *mut mobj_t,
    ) -> std::ffi::c_int;
}

static mut rcsid: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

#[repr(C)]
#[derive(Copy, Clone)]
pub struct anim_t {
    pub istexture: boolean,
    pub picnum: std::ffi::c_int,
    pub basepic: std::ffi::c_int,
    pub numpics: std::ffi::c_int,
    pub speed: std::ffi::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct animdef_t {
    pub istexture: boolean,
    pub endname: [std::ffi::c_char; (9) as usize],
    pub startname: [std::ffi::c_char; (9) as usize],
    pub speed: std::ffi::c_int,
}

pub static mut animdefs: *mut animdef_t /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut anims: [anim_t; (MAXANIMS) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut lastanim: *mut anim_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn P_InitPicAnims() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn getSide(
    currentSector: std::ffi::c_int,
    line: std::ffi::c_int,
    side: std::ffi::c_int,
) -> *mut side_t {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn getSector(
    currentSector: std::ffi::c_int,
    line: std::ffi::c_int,
    side: std::ffi::c_int,
) -> *mut sector_t {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn twoSided(
    sector: std::ffi::c_int,
    line: std::ffi::c_int,
) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn getNextSector(line: *mut line_t, sec: *mut sector_t) -> *mut sector_t {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_FindLowestFloorSurrounding(sec: *mut sector_t) -> fixed_t {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_FindHighestFloorSurrounding(sec: *mut sector_t) -> fixed_t {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_FindNextHighestFloor(
    sec: *mut sector_t,
    currentheight: std::ffi::c_int,
) -> fixed_t {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_FindLowestCeilingSurrounding(sec: *mut sector_t) -> fixed_t {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_FindHighestCeilingSurrounding(sec: *mut sector_t) -> fixed_t {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_FindSectorFromLineTag(
    line: *mut line_t,
    start: std::ffi::c_int,
) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_FindMinSurroundingLight(
    sector: *mut sector_t,
    max: std::ffi::c_int,
) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_CrossSpecialLine(
    linenum: std::ffi::c_int,
    side: std::ffi::c_int,
    thing: *mut mobj_t,
) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_ShootSpecialLine(thing: *mut mobj_t, line: *mut line_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_PlayerInSpecialSector(player: *mut player_t) {
    todo!("body not yet translated")
}

pub static mut levelTimer: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut levelTimeCount: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn P_UpdateSpecials() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn EV_DoDonut(line: *mut line_t) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub static mut numlinespecials: std::ffi::c_short = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut linespeciallist: [*mut line_t; (MAXLINEANIMS) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn P_SpawnSpecials() {
    todo!("body not yet translated")
}
