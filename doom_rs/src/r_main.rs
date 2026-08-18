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

pub const LIGHTLEVELS: std::ffi::c_int = 16;

pub const LIGHTSEGSHIFT: std::ffi::c_int = 4;

pub const MAXLIGHTSCALE: std::ffi::c_int = 48;

pub const LIGHTSCALESHIFT: std::ffi::c_int = 12;

pub const MAXLIGHTZ: std::ffi::c_int = 128;

pub const LIGHTZSHIFT: std::ffi::c_int = 20;

pub const NUMCOLORMAPS: std::ffi::c_int = 32;

static mut rcsid: [std::ffi::c_char; 49] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        114 as std::ffi::c_char,
        95 as std::ffi::c_char,
        109 as std::ffi::c_char,
        97 as std::ffi::c_char,
        105 as std::ffi::c_char,
        110 as std::ffi::c_char,
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

pub const FIELDOFVIEW: std::ffi::c_int = 2048;

pub static mut viewangleoffset: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut validcount: std::ffi::c_int = unsafe { 1 };

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

pub static mut finecosine: *mut fixed_t =
    unsafe { (&(finesine[(FINEANGLES / 4) as usize]) as *const _ as *mut _) };

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
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn R_PointOnSide(
    x: fixed_t,
    y: fixed_t,
    node: *mut node_t,
) -> std::ffi::c_int {
    unsafe { todo!("body not yet translated") }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn R_PointOnSegSide(
    x: fixed_t,
    y: fixed_t,
    line: *mut seg_t,
) -> std::ffi::c_int {
    unsafe { todo!("body not yet translated") }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn R_PointToAngle(x: fixed_t, y: fixed_t) -> angle_t {
    unsafe { todo!("body not yet translated") }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn R_PointToAngle2(
    x1: fixed_t,
    y1: fixed_t,
    x2: fixed_t,
    y2: fixed_t,
) -> angle_t {
    unsafe { todo!("body not yet translated") }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn R_PointToDist(x: fixed_t, y: fixed_t) -> fixed_t {
    unsafe { todo!("body not yet translated") }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn R_InitPointToAngle() {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn R_ScaleFromGlobalAngle(visangle: angle_t) -> fixed_t {
    unsafe { todo!("body not yet translated") }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn R_InitTables() {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn R_InitTextureMapping() {
    unsafe { todo!("body not yet translated") }
}

pub const DISTMAP: std::ffi::c_int = 2;

pub unsafe extern "C" fn R_InitLightTables() {
    unsafe { todo!("body not yet translated") }
}

pub static mut setsizeneeded: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut setblocks: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut setdetail: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn R_SetViewSize(blocks: std::ffi::c_int, detail: std::ffi::c_int) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn R_ExecuteSetViewSize() {
    unsafe { todo!("body not yet translated") }
}

unsafe extern "C" {
    pub static mut detailLevel: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut screenblocks: std::ffi::c_int;
}

pub unsafe extern "C" fn R_Init() {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn R_PointInSubsector(x: fixed_t, y: fixed_t) -> *mut subsector_t {
    unsafe { todo!("body not yet translated") }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn R_SetupFrame(player: *mut player_t) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn R_RenderPlayerView(player: *mut player_t) {
    unsafe { todo!("body not yet translated") }
}
