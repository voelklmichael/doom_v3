use crate::d_items::*;
use crate::d_net::*;
use crate::d_player::*;
use crate::d_think::*;
use crate::d_ticcmd::*;
use crate::doomdata::*;
use crate::doomdef::*;
use crate::doomtype::*;
use crate::info::*;
use crate::m_bbox::*;
use crate::m_fixed::*;
use crate::p_mobj::*;
use crate::p_pspr::*;
use crate::r_bsp::*;
use crate::r_data::*;
use crate::r_defs::*;
use crate::r_draw::*;
use crate::r_local::*;
use crate::r_plane::*;
use crate::r_segs::*;
use crate::r_sky::*;
use crate::r_state::*;
use crate::r_things::*;
use crate::tables::*;

unsafe extern "C" {
    pub static mut viewwidth: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut viewheight: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut viewwindowx: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut viewwindowy: std::ffi::c_int;
}

static mut rcsid: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut viewangleoffset: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut validcount: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut fixedcolormap: *mut lighttable_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

unsafe extern "C" {
    pub static mut walllights: *mut *mut lighttable_t;
}

pub static mut centerx: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut centery: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut centerxfrac: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut centeryfrac: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut projection: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut framecount: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut sscount: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut linecount: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut loopcount: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut viewx: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut viewy: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut viewz: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut viewangle: angle_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut viewcos: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut viewsin: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut viewplayer: *mut player_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut detailshift: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut clipangle: angle_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut viewangletox: [std::ffi::c_int; (FINEANGLES / 2) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut xtoviewangle: [angle_t; (SCREENWIDTH + 1) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut finecosine: *mut fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut scalelight: [[*mut lighttable_t; (MAXLIGHTSCALE) as usize]; (LIGHTLEVELS) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut scalelightfixed: [*mut lighttable_t; (MAXLIGHTSCALE) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut zlight: [[*mut lighttable_t; (MAXLIGHTZ) as usize]; (LIGHTLEVELS) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut extralight: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut colfunc: Option<unsafe extern "C" fn()> = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut basecolfunc: Option<unsafe extern "C" fn()> = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut fuzzcolfunc: Option<unsafe extern "C" fn()> = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut transcolfunc: Option<unsafe extern "C" fn()> = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut spanfunc: Option<unsafe extern "C" fn()> = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn R_AddPointToBox(
    x: std::ffi::c_int,
    y: std::ffi::c_int,
    box_: *mut fixed_t,
) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_PointOnSide(
    x: fixed_t,
    y: fixed_t,
    node: *mut node_t,
) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_PointOnSegSide(
    x: fixed_t,
    y: fixed_t,
    line: *mut seg_t,
) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_PointToAngle(x: fixed_t, y: fixed_t) -> angle_t {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_PointToAngle2(
    x1: fixed_t,
    y1: fixed_t,
    x2: fixed_t,
    y2: fixed_t,
) -> angle_t {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_PointToDist(x: fixed_t, y: fixed_t) -> fixed_t {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_InitPointToAngle() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_ScaleFromGlobalAngle(visangle: angle_t) -> fixed_t {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_InitTables() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_InitTextureMapping() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_InitLightTables() {
    todo!("body not yet translated")
}

pub static mut setsizeneeded: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut setblocks: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut setdetail: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn R_SetViewSize(blocks: std::ffi::c_int, detail: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_ExecuteSetViewSize() {
    todo!("body not yet translated")
}

unsafe extern "C" {
    pub static mut detailLevel: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut screenblocks: std::ffi::c_int;
}

pub unsafe extern "C" fn R_Init() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_PointInSubsector(x: fixed_t, y: fixed_t) -> *mut subsector_t {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_SetupFrame(player: *mut player_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_RenderPlayerView(player: *mut player_t) {
    todo!("body not yet translated")
}
