use crate::doomdef::*;
use crate::doomtype::*;

pub const ML_LABEL: std::ffi::c_int = 0;
pub const ML_THINGS: std::ffi::c_int = ML_LABEL + 1;
pub const ML_LINEDEFS: std::ffi::c_int = ML_THINGS + 1;
pub const ML_SIDEDEFS: std::ffi::c_int = ML_LINEDEFS + 1;
pub const ML_VERTEXES: std::ffi::c_int = ML_SIDEDEFS + 1;
pub const ML_SEGS: std::ffi::c_int = ML_VERTEXES + 1;
pub const ML_SSECTORS: std::ffi::c_int = ML_SEGS + 1;
pub const ML_NODES: std::ffi::c_int = ML_SSECTORS + 1;
pub const ML_SECTORS: std::ffi::c_int = ML_NODES + 1;
pub const ML_REJECT: std::ffi::c_int = ML_SECTORS + 1;
pub const ML_BLOCKMAP: std::ffi::c_int = ML_REJECT + 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mapvertex_t {
    pub x: std::ffi::c_short,
    pub y: std::ffi::c_short,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mapsidedef_t {
    pub textureoffset: std::ffi::c_short,
    pub rowoffset: std::ffi::c_short,
    pub toptexture: [std::ffi::c_char; (8) as usize],
    pub bottomtexture: [std::ffi::c_char; (8) as usize],
    pub midtexture: [std::ffi::c_char; (8) as usize],
    pub sector: std::ffi::c_short,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct maplinedef_t {
    pub v1: std::ffi::c_short,
    pub v2: std::ffi::c_short,
    pub flags: std::ffi::c_short,
    pub special: std::ffi::c_short,
    pub tag: std::ffi::c_short,
    pub sidenum: [std::ffi::c_short; (2) as usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mapsector_t {
    pub floorheight: std::ffi::c_short,
    pub ceilingheight: std::ffi::c_short,
    pub floorpic: [std::ffi::c_char; (8) as usize],
    pub ceilingpic: [std::ffi::c_char; (8) as usize],
    pub lightlevel: std::ffi::c_short,
    pub special: std::ffi::c_short,
    pub tag: std::ffi::c_short,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mapsubsector_t {
    pub numsegs: std::ffi::c_short,
    pub firstseg: std::ffi::c_short,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mapseg_t {
    pub v1: std::ffi::c_short,
    pub v2: std::ffi::c_short,
    pub angle: std::ffi::c_short,
    pub linedef: std::ffi::c_short,
    pub side: std::ffi::c_short,
    pub offset: std::ffi::c_short,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mapnode_t {
    pub x: std::ffi::c_short,
    pub y: std::ffi::c_short,
    pub dx: std::ffi::c_short,
    pub dy: std::ffi::c_short,
    pub bbox: [[std::ffi::c_short; (4) as usize]; (2) as usize],
    pub children: [std::ffi::c_ushort; (2) as usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mapthing_t {
    pub x: std::ffi::c_short,
    pub y: std::ffi::c_short,
    pub angle: std::ffi::c_short,
    pub type_: std::ffi::c_short,
    pub options: std::ffi::c_short,
}
