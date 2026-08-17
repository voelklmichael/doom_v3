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
use crate::tables::*;
use crate::w_wad::*;
use crate::z_zone::*;

unsafe extern "C" {
    pub fn R_AddPSprites();
}

unsafe extern "C" {
    pub fn R_DrawSprites();
}

unsafe extern "C" {
    pub fn R_ClipVisSprite(vis: *mut vissprite_t, xl: std::ffi::c_int, xh: std::ffi::c_int);
}

static mut rcsid: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

#[repr(C)]
#[derive(Copy, Clone)]
pub struct maskdraw_t {
    pub x1: std::ffi::c_int,
    pub x2: std::ffi::c_int,
    pub column: std::ffi::c_int,
    pub topclip: std::ffi::c_int,
    pub bottomclip: std::ffi::c_int,
}

pub static mut pspritescale: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut pspriteiscale: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut spritelights: *mut *mut lighttable_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut negonearray: [std::ffi::c_short; (SCREENWIDTH) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut screenheightarray: [std::ffi::c_short; (SCREENWIDTH) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut sprites: *mut spritedef_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut numsprites: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut sprtemp: [spriteframe_t; (29) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut maxframe: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut spritename: *mut std::ffi::c_char = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn R_InstallSpriteLump(
    lump: std::ffi::c_int,
    frame: std::ffi::c_uint,
    rotation: std::ffi::c_uint,
    flipped: boolean,
) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_InitSpriteDefs(namelist: *mut *mut std::ffi::c_char) {
    todo!("body not yet translated")
}

pub static mut vissprites: [vissprite_t; (MAXVISSPRITES) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut vissprite_p: *mut vissprite_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut newvissprite: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn R_InitSprites(namelist: *mut *mut std::ffi::c_char) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_ClearSprites() {
    todo!("body not yet translated")
}

pub static mut overflowsprite: vissprite_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn R_NewVisSprite() -> *mut vissprite_t {
    todo!("body not yet translated")
}

pub static mut mfloorclip: *mut std::ffi::c_short = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut mceilingclip: *mut std::ffi::c_short = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut spryscale: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut sprtopscreen: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn R_DrawMaskedColumn(column: *mut column_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_DrawVisSprite(
    vis: *mut vissprite_t,
    x1: std::ffi::c_int,
    x2: std::ffi::c_int,
) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_ProjectSprite(thing: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_AddSprites(sec: *mut sector_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_DrawPSprite(psp: *mut pspdef_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_DrawPlayerSprites() {
    todo!("body not yet translated")
}

pub static mut vsprsortedhead: vissprite_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn R_SortVisSprites() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_DrawSprite(spr: *mut vissprite_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_DrawMasked() {
    todo!("body not yet translated")
}
