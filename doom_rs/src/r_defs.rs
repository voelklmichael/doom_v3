use crate::d_think::*;
use crate::doomdata::*;
use crate::doomdef::*;
use crate::doomtype::*;
use crate::info::*;
use crate::m_fixed::*;
use crate::p_mobj::*;
use crate::tables::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vertex_t {
    pub x: fixed_t,
    pub y: fixed_t,
}

pub static mut line_s: struct_ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

#[repr(C)]
#[derive(Copy, Clone)]
pub struct degenmobj_t {
    pub thinker: thinker_t,
    pub x: fixed_t,
    pub y: fixed_t,
    pub z: fixed_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sector_t {
    pub floorheight: fixed_t,
    pub ceilingheight: fixed_t,
    pub floorpic: std::ffi::c_short,
    pub ceilingpic: std::ffi::c_short,
    pub lightlevel: std::ffi::c_short,
    pub special: std::ffi::c_short,
    pub tag: std::ffi::c_short,
    pub soundtraversed: std::ffi::c_int,
    pub soundtarget: *mut mobj_t,
    pub blockbox: [std::ffi::c_int; (4) as usize],
    pub soundorg: degenmobj_t,
    pub validcount: std::ffi::c_int,
    pub thinglist: *mut mobj_t,
    pub specialdata: *mut std::ffi::c_void,
    pub linecount: std::ffi::c_int,
    pub lines: *mut *mut line_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct side_t {
    pub textureoffset: fixed_t,
    pub rowoffset: fixed_t,
    pub toptexture: std::ffi::c_short,
    pub bottomtexture: std::ffi::c_short,
    pub midtexture: std::ffi::c_short,
    pub sector: *mut sector_t,
}

pub const ST_HORIZONTAL: std::ffi::c_int = 0;
pub const ST_VERTICAL: std::ffi::c_int = ST_HORIZONTAL + 1;
pub const ST_POSITIVE: std::ffi::c_int = ST_VERTICAL + 1;
pub const ST_NEGATIVE: std::ffi::c_int = ST_POSITIVE + 1;

pub type slopetype_t = std::ffi::c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct line_t {
    pub v1: *mut vertex_t,
    pub v2: *mut vertex_t,
    pub dx: fixed_t,
    pub dy: fixed_t,
    pub flags: std::ffi::c_short,
    pub special: std::ffi::c_short,
    pub tag: std::ffi::c_short,
    pub sidenum: [std::ffi::c_short; (2) as usize],
    pub bbox: [fixed_t; (4) as usize],
    pub slopetype: slopetype_t,
    pub frontsector: *mut sector_t,
    pub backsector: *mut sector_t,
    pub validcount: std::ffi::c_int,
    pub specialdata: *mut std::ffi::c_void,
}

pub type line_s = line_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct subsector_t {
    pub sector: *mut sector_t,
    pub numlines: std::ffi::c_short,
    pub firstline: std::ffi::c_short,
}

pub type subsector_s = subsector_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seg_t {
    pub v1: *mut vertex_t,
    pub v2: *mut vertex_t,
    pub offset: fixed_t,
    pub angle: angle_t,
    pub sidedef: *mut side_t,
    pub linedef: *mut line_t,
    pub frontsector: *mut sector_t,
    pub backsector: *mut sector_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct node_t {
    pub x: fixed_t,
    pub y: fixed_t,
    pub dx: fixed_t,
    pub dy: fixed_t,
    pub bbox: [[fixed_t; (4) as usize]; (2) as usize],
    pub children: [std::ffi::c_ushort; (2) as usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct post_t {
    pub topdelta: byte,
    pub length: byte,
}

pub type column_t = post_t;

pub type lighttable_t = byte;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drawseg_t {
    pub curline: *mut seg_t,
    pub x1: std::ffi::c_int,
    pub x2: std::ffi::c_int,
    pub scale1: fixed_t,
    pub scale2: fixed_t,
    pub scalestep: fixed_t,
    pub silhouette: std::ffi::c_int,
    pub bsilheight: fixed_t,
    pub tsilheight: fixed_t,
    pub sprtopclip: *mut std::ffi::c_short,
    pub sprbottomclip: *mut std::ffi::c_short,
    pub maskedtexturecol: *mut std::ffi::c_short,
}

pub type drawseg_s = drawseg_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct patch_t {
    pub width: std::ffi::c_short,
    pub height: std::ffi::c_short,
    pub leftoffset: std::ffi::c_short,
    pub topoffset: std::ffi::c_short,
    pub columnofs: [std::ffi::c_int; (8) as usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vissprite_t {
    pub prev: *mut vissprite_s,
    pub next: *mut vissprite_s,
    pub x1: std::ffi::c_int,
    pub x2: std::ffi::c_int,
    pub gx: fixed_t,
    pub gy: fixed_t,
    pub gz: fixed_t,
    pub gzt: fixed_t,
    pub startfrac: fixed_t,
    pub scale: fixed_t,
    pub xiscale: fixed_t,
    pub texturemid: fixed_t,
    pub patch: std::ffi::c_int,
    pub colormap: *mut lighttable_t,
    pub mobjflags: std::ffi::c_int,
}

pub type vissprite_s = vissprite_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spriteframe_t {
    pub rotate: boolean,
    pub lump: [std::ffi::c_short; (8) as usize],
    pub flip: [byte; (8) as usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spritedef_t {
    pub numframes: std::ffi::c_int,
    pub spriteframes: *mut spriteframe_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct visplane_t {
    pub height: fixed_t,
    pub picnum: std::ffi::c_int,
    pub lightlevel: std::ffi::c_int,
    pub minx: std::ffi::c_int,
    pub maxx: std::ffi::c_int,
    pub pad1: byte,
    pub top: [byte; (SCREENWIDTH) as usize],
    pub pad2: byte,
    pub pad3: byte,
    pub bottom: [byte; (SCREENWIDTH) as usize],
    pub pad4: byte,
}
