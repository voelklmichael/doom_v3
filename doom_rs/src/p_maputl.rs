use crate::d_items::*;
use crate::d_player::*;
use crate::d_think::*;
use crate::d_ticcmd::*;
use crate::doomdata::*;
use crate::doomdef::*;
use crate::doomtype::*;
use crate::info::*;
use crate::m_bbox::*;
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
use crate::tables::*;

static mut rcsid: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn P_AproxDistance(dx: fixed_t, dy: fixed_t) -> fixed_t {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_PointOnLineSide(
    x: fixed_t,
    y: fixed_t,
    line: *mut line_t,
) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_BoxOnLineSide(tmbox: *mut fixed_t, ld: *mut line_t) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_PointOnDivlineSide(
    x: fixed_t,
    y: fixed_t,
    line: *mut divline_t,
) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_MakeDivline(li: *mut line_t, dl: *mut divline_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_InterceptVector(v2: *mut divline_t, v1: *mut divline_t) -> fixed_t {
    todo!("body not yet translated")
}

pub static mut opentop: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut openbottom: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut openrange: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut lowfloor: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn P_LineOpening(linedef: *mut line_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_UnsetThingPosition(thing: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_SetThingPosition(thing: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_BlockLinesIterator(
    x: std::ffi::c_int,
    y: std::ffi::c_int,
    func: Option<unsafe extern "C" fn(*mut line_t) -> boolean>,
) -> boolean {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_BlockThingsIterator(
    x: std::ffi::c_int,
    y: std::ffi::c_int,
    func: Option<unsafe extern "C" fn(*mut mobj_t) -> boolean>,
) -> boolean {
    todo!("body not yet translated")
}

pub static mut intercepts: [intercept_t; (MAXINTERCEPTS) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut intercept_p: *mut intercept_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut trace: divline_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut earlyout: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut ptflags: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn PIT_AddLineIntercepts(ld: *mut line_t) -> boolean {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn PIT_AddThingIntercepts(thing: *mut mobj_t) -> boolean {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_TraverseIntercepts(func: traverser_t, maxfrac: fixed_t) -> boolean {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_PathTraverse(
    x1: fixed_t,
    y1: fixed_t,
    x2: fixed_t,
    y2: fixed_t,
    flags: std::ffi::c_int,
    trav: Option<unsafe extern "C" fn(*mut intercept_t) -> boolean>,
) -> boolean {
    todo!("body not yet translated")
}
