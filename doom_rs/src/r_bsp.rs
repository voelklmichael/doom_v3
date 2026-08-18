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
use crate::i_system::*;
use crate::info::*;
use crate::m_bbox::*;
use crate::m_fixed::*;
use crate::p_mobj::*;
use crate::p_pspr::*;
use crate::r_data::*;
use crate::r_defs::*;
use crate::r_main::*;
use crate::r_plane::*;
use crate::r_state::*;
use crate::r_things::*;
use crate::tables::*;

unsafe extern "C" {
    pub static mut rw_x: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut rw_stopx: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut segtextured: boolean;
}

unsafe extern "C" {
    pub static mut markfloor: boolean;
}

unsafe extern "C" {
    pub static mut markceiling: boolean;
}

unsafe extern "C" {
    pub static mut skymap: boolean;
}

unsafe extern "C" {
    pub static mut hscalelight: *mut *mut lighttable_t;
}

unsafe extern "C" {
    pub static mut vscalelight: *mut *mut lighttable_t;
}

unsafe extern "C" {
    pub static mut dscalelight: *mut *mut lighttable_t;
}

pub type drawfunc_t = Option<unsafe extern "C" fn(std::ffi::c_int, std::ffi::c_int)>;

static mut rcsid: [std::ffi::c_char; 48] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        114 as std::ffi::c_char,
        95 as std::ffi::c_char,
        98 as std::ffi::c_char,
        115 as std::ffi::c_char,
        112 as std::ffi::c_char,
        46 as std::ffi::c_char,
        99 as std::ffi::c_char,
        44 as std::ffi::c_char,
        118 as std::ffi::c_char,
        32 as std::ffi::c_char,
        49 as std::ffi::c_char,
        46 as std::ffi::c_char,
        52 as std::ffi::c_char,
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

pub static mut curline: *mut seg_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut sidedef: *mut side_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut linedef: *mut line_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut frontsector: *mut sector_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut backsector: *mut sector_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut drawsegs: [drawseg_t; (MAXDRAWSEGS) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut ds_p: *mut drawseg_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

unsafe extern "C" {
    pub fn R_StoreWallRange(start: std::ffi::c_int, stop: std::ffi::c_int);
}

pub unsafe extern "C" fn R_ClearDrawSegs() {
    todo!("body not yet translated")
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cliprange_t {
    pub first: std::ffi::c_int,
    pub last: std::ffi::c_int,
}

pub const MAXSEGS: std::ffi::c_int = 32;

pub static mut newend: *mut cliprange_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut solidsegs: [cliprange_t; (MAXSEGS) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn R_ClipSolidWallSegment(first: std::ffi::c_int, last: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_ClipPassWallSegment(first: std::ffi::c_int, last: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_ClearClipSegs() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_AddLine(line: *mut seg_t) {
    todo!("body not yet translated")
}

pub static mut checkcoord: [[std::ffi::c_int; (4) as usize]; (12) as usize] = unsafe {
    [
        [3, 0, 2, 1],
        [3, 0, 2, 0],
        [3, 1, 2, 0],
        [0],
        [2, 0, 2, 1],
        [0, 0, 0, 0],
        [3, 1, 3, 0],
        [0],
        [2, 0, 3, 1],
        [2, 1, 3, 1],
        [2, 1, 3, 0],
    ]
};

pub unsafe extern "C" fn R_CheckBBox(bspcoord: *mut fixed_t) -> boolean {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_Subsector(num: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_RenderBSPNode(bspnum: std::ffi::c_int) {
    todo!("body not yet translated")
}
