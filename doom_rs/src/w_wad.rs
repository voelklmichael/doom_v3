use crate::d_event::*;
use crate::d_ticcmd::*;
use crate::doomtype::*;
use crate::i_system::*;
use crate::m_swap::*;
use crate::z_zone::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wadinfo_t {
    pub identification: [std::ffi::c_char; (4) as usize],
    pub numlumps: std::ffi::c_int,
    pub infotableofs: std::ffi::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct filelump_t {
    pub filepos: std::ffi::c_int,
    pub size: std::ffi::c_int,
    pub name: [std::ffi::c_char; (8) as usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lumpinfo_t {
    pub name: [std::ffi::c_char; (8) as usize],
    pub handle: std::ffi::c_int,
    pub position: std::ffi::c_int,
    pub size: std::ffi::c_int,
}

static mut rcsid: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut lumpinfo: *mut lumpinfo_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut numlumps: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut lumpcache: *mut *mut std::ffi::c_void = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn strupr(s: *mut std::ffi::c_char) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn filelength(handle: std::ffi::c_int) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn ExtractFileBase(path: *mut std::ffi::c_char, dest: *mut std::ffi::c_char) {
    todo!("body not yet translated")
}

pub static mut reloadlump: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut reloadname: *mut std::ffi::c_char = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn W_AddFile(filename: *mut std::ffi::c_char) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn W_Reload() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn W_InitMultipleFiles(filenames: *mut *mut std::ffi::c_char) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn W_InitFile(filename: *mut std::ffi::c_char) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn W_NumLumps() -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn W_CheckNumForName(name: *mut std::ffi::c_char) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn W_GetNumForName(name: *mut std::ffi::c_char) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn W_LumpLength(lump: std::ffi::c_int) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn W_ReadLump(lump: std::ffi::c_int, dest: *mut std::ffi::c_void) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn W_CacheLumpNum(
    lump: std::ffi::c_int,
    tag: std::ffi::c_int,
) -> *mut std::ffi::c_void {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn W_CacheLumpName(
    name: *mut std::ffi::c_char,
    tag: std::ffi::c_int,
) -> *mut std::ffi::c_void {
    todo!("body not yet translated")
}

pub static mut info: [[std::ffi::c_int; (10) as usize]; (2500) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut profilecount: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn W_Profile() {
    todo!("body not yet translated")
}
