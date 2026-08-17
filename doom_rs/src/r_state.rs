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
use crate::r_data::*;
use crate::r_defs::*;
use crate::tables::*;

unsafe extern "C" {
    pub static mut textureheight: *mut fixed_t;
}

unsafe extern "C" {
    pub static mut spritewidth: *mut fixed_t;
}

unsafe extern "C" {
    pub static mut spriteoffset: *mut fixed_t;
}

unsafe extern "C" {
    pub static mut spritetopoffset: *mut fixed_t;
}

unsafe extern "C" {
    pub static mut colormaps: *mut lighttable_t;
}

unsafe extern "C" {
    pub static mut viewwidth: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut scaledviewwidth: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut viewheight: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut firstflat: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut flattranslation: *mut std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut texturetranslation: *mut std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut firstspritelump: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut lastspritelump: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut numspritelumps: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut numsprites: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut sprites: *mut spritedef_t;
}

unsafe extern "C" {
    pub static mut numvertexes: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut vertexes: *mut vertex_t;
}

unsafe extern "C" {
    pub static mut numsegs: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut segs: *mut seg_t;
}

unsafe extern "C" {
    pub static mut numsectors: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut sectors: *mut sector_t;
}

unsafe extern "C" {
    pub static mut numsubsectors: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut subsectors: *mut subsector_t;
}

unsafe extern "C" {
    pub static mut numnodes: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut nodes: *mut node_t;
}

unsafe extern "C" {
    pub static mut numlines: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut lines: *mut line_t;
}

unsafe extern "C" {
    pub static mut numsides: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut sides: *mut side_t;
}

unsafe extern "C" {
    pub static mut viewx: fixed_t;
}

unsafe extern "C" {
    pub static mut viewy: fixed_t;
}

unsafe extern "C" {
    pub static mut viewz: fixed_t;
}

unsafe extern "C" {
    pub static mut viewangle: angle_t;
}

unsafe extern "C" {
    pub static mut viewplayer: *mut player_t;
}

unsafe extern "C" {
    pub static mut clipangle: angle_t;
}

unsafe extern "C" {
    pub static mut viewangletox: [std::ffi::c_int; (FINEANGLES / 2) as usize];
}

unsafe extern "C" {
    pub static mut xtoviewangle: [angle_t; (SCREENWIDTH + 1) as usize];
}

unsafe extern "C" {
    pub static mut rw_distance: fixed_t;
}

unsafe extern "C" {
    pub static mut rw_normalangle: angle_t;
}

unsafe extern "C" {
    pub static mut rw_angle1: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut sscount: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut floorplane: *mut visplane_t;
}

unsafe extern "C" {
    pub static mut ceilingplane: *mut visplane_t;
}
