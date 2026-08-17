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
use crate::i_system::*;
use crate::info::*;
use crate::m_cheat::*;
use crate::m_fixed::*;
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
use crate::st_stuff::*;
use crate::tables::*;
use crate::v_video::*;
use crate::w_wad::*;
use crate::z_zone::*;

static mut rcsid: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpoint_t {
    // TODO: unparsed multi-declarator field, needs manual translation: y: /* unrecognized type: int x, */,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fline_t {
    // TODO: unparsed multi-declarator field, needs manual translation: b: /* unrecognized type: fpoint_t a, */,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpoint_t {
    // TODO: unparsed multi-declarator field, needs manual translation: x,y
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mline_t {
    // TODO: unparsed multi-declarator field, needs manual translation: b: /* unrecognized type: mpoint_t a, */,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct islope_t {
    // TODO: unparsed multi-declarator field, needs manual translation: islp: /* unrecognized type: fixed_t slp, */,
}

pub static mut player_arrow: *mut mline_t /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut cheat_player_arrow: *mut mline_t /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut triangle_guy: *mut mline_t /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut thintriangle_guy: *mut mline_t /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut cheating: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut grid: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut leveljuststarted: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut automapactive: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut finit_width: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut finit_height: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut f_x: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut f_y: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut f_w: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut f_h: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut lightlev: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut fb: *mut byte = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut amclock: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut m_paninc: mpoint_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut mtof_zoommul: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut ftom_zoommul: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

// TODO: unparsed multi-declarator variable, needs manual translation: m_y: /* unrecognized type: fixed_t m_x, */,

// TODO: unparsed multi-declarator variable, needs manual translation: m_y2: /* unrecognized type: fixed_t m_x2, */,

static mut m_w: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut m_h: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut min_x: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut min_y: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut max_x: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut max_y: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut max_w: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut max_h: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut min_w: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut min_h: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut min_scale_mtof: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut max_scale_mtof: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

// TODO: unparsed multi-declarator variable, needs manual translation: old_m_h: /* unrecognized type: fixed_t old_m_w, */,

// TODO: unparsed multi-declarator variable, needs manual translation: old_m_y: /* unrecognized type: fixed_t old_m_x, */,

static mut f_oldloc: mpoint_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut scale_mtof: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut scale_ftom: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut plr: *mut player_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut marknums: [*mut patch_t; (10) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut markpoints: [mpoint_t; (AM_NUMMARKPOINTS) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut markpointnum: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut followplayer: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut cheat_amap_seq: *mut std::ffi::c_uchar /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut cheat_amap: cheatseq_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut stopped: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

unsafe extern "C" {
    pub static mut viewactive: boolean;
}

unsafe extern "C" {
    pub fn V_MarkRect(
        x: std::ffi::c_int,
        y: std::ffi::c_int,
        width: std::ffi::c_int,
        height: std::ffi::c_int,
    );
}

pub unsafe extern "C" fn AM_getIslope(ml: *mut mline_t, is: *mut islope_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn AM_activateNewScale() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn AM_saveScaleAndLoc() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn AM_restoreScaleAndLoc() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn AM_addMark() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn AM_findMinMaxBoundaries() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn AM_changeWindowLoc() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn AM_initVariables() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn AM_loadPics() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn AM_unloadPics() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn AM_clearMarks() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn AM_LevelInit() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn AM_Stop() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn AM_Start() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn AM_minOutWindowScale() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn AM_maxOutWindowScale() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn AM_Responder(ev: *mut event_t) -> boolean {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn AM_changeWindowScale() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn AM_doFollowPlayer() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn AM_updateLightLev() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn AM_Ticker() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn AM_clearFB(color: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn AM_clipMline(ml: *mut mline_t, fl: *mut fline_t) -> boolean {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn AM_drawFline(fl: *mut fline_t, color: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn AM_drawMline(ml: *mut mline_t, color: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn AM_drawGrid(color: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn AM_drawWalls() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn AM_rotate(x: *mut fixed_t, y: *mut fixed_t, a: angle_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn AM_drawLineCharacter(
    lineguy: *mut mline_t,
    lineguylines: std::ffi::c_int,
    scale: fixed_t,
    angle: angle_t,
    color: std::ffi::c_int,
    x: fixed_t,
    y: fixed_t,
) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn AM_drawPlayers() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn AM_drawThings(colors: std::ffi::c_int, colorrange: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn AM_drawMarks() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn AM_drawCrosshair(color: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn AM_Drawer() {
    todo!("body not yet translated")
}
