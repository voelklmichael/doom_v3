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
use crate::m_fixed::*;
use crate::m_swap::*;
use crate::p_local::*;
use crate::p_mobj::*;
use crate::p_pspr::*;
use crate::p_spec::*;
use crate::r_bsp::*;
use crate::r_defs::*;
use crate::r_draw::*;
use crate::r_local::*;
use crate::r_main::*;
use crate::r_plane::*;
use crate::r_segs::*;
use crate::r_sky::*;
use crate::r_state::*;
use crate::r_things::*;
use crate::tables::*;
use crate::w_wad::*;
use crate::z_zone::*;

static mut rcsid: [std::ffi::c_char; 49] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        114 as std::ffi::c_char,
        95 as std::ffi::c_char,
        100 as std::ffi::c_char,
        97 as std::ffi::c_char,
        116 as std::ffi::c_char,
        97 as std::ffi::c_char,
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
        49 as std::ffi::c_char,
        54 as std::ffi::c_char,
        58 as std::ffi::c_char,
        52 as std::ffi::c_char,
        55 as std::ffi::c_char,
        58 as std::ffi::c_char,
        53 as std::ffi::c_char,
        53 as std::ffi::c_char,
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mappatch_t {
    pub originx: std::ffi::c_short,
    pub originy: std::ffi::c_short,
    pub patch: std::ffi::c_short,
    pub stepdir: std::ffi::c_short,
    pub colormap: std::ffi::c_short,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct maptexture_t {
    pub name: [std::ffi::c_char; (8) as usize],
    pub masked: boolean,
    pub width: std::ffi::c_short,
    pub height: std::ffi::c_short,
    pub columndirectory: *mut *mut std::ffi::c_void,
    pub patchcount: std::ffi::c_short,
    pub patches: [mappatch_t; (1) as usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct texpatch_t {
    pub originx: std::ffi::c_int,
    pub originy: std::ffi::c_int,
    pub patch: std::ffi::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct texture_t {
    pub name: [std::ffi::c_char; (8) as usize],
    pub width: std::ffi::c_short,
    pub height: std::ffi::c_short,
    pub patchcount: std::ffi::c_short,
    pub patches: [texpatch_t; (1) as usize],
}

pub static mut firstflat: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut lastflat: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut numflats: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut firstpatch: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut lastpatch: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut numpatches: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut firstspritelump: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut lastspritelump: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut numspritelumps: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut numtextures: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut textures: *mut *mut texture_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut texturewidthmask: *mut std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut textureheight: *mut fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut texturecompositesize: *mut std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut texturecolumnlump: *mut *mut std::ffi::c_short = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut texturecolumnofs: *mut *mut std::ffi::c_ushort = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut texturecomposite: *mut *mut byte = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut flattranslation: *mut std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut texturetranslation: *mut std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut spritewidth: *mut fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut spriteoffset: *mut fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut spritetopoffset: *mut fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut colormaps: *mut lighttable_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn R_DrawColumnInCache(
    patch: *mut column_t,
    cache: *mut byte,
    originy: std::ffi::c_int,
    cacheheight: std::ffi::c_int,
) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_GenerateComposite(texnum: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_GenerateLookup(texnum: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_GetColumn(tex: std::ffi::c_int, col: std::ffi::c_int) -> *mut byte {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_InitTextures() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_InitFlats() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_InitSpriteLumps() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_InitColormaps() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_InitData() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_FlatNumForName(name: *mut std::ffi::c_char) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_CheckTextureNumForName(name: *mut std::ffi::c_char) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_TextureNumForName(name: *mut std::ffi::c_char) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub static mut flatmemory: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut texturememory: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut spritememory: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn R_PrecacheLevel() {
    todo!("body not yet translated")
}
