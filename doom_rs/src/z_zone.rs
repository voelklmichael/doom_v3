use crate::d_event::*;
use crate::d_ticcmd::*;
use crate::doomdef::*;
use crate::doomtype::*;
use crate::i_system::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct memblock_t {
    pub size: std::ffi::c_int,
    pub user: *mut *mut std::ffi::c_void,
    pub tag: std::ffi::c_int,
    pub id: std::ffi::c_int,
    pub next: *mut memblock_s,
    pub prev: *mut memblock_s,
}

pub type memblock_s = memblock_t;

static mut rcsid: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

#[repr(C)]
#[derive(Copy, Clone)]
pub struct memzone_t {
    pub size: std::ffi::c_int,
    pub blocklist: memblock_t,
    pub rover: *mut memblock_t,
}

pub static mut mainzone: *mut memzone_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn Z_ClearZone(zone: *mut memzone_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn Z_Init() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn Z_Free(ptr: *mut std::ffi::c_void) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn Z_Malloc(
    size: std::ffi::c_int,
    tag: std::ffi::c_int,
    user: *mut std::ffi::c_void,
) -> *mut std::ffi::c_void {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn Z_FreeTags(lowtag: std::ffi::c_int, hightag: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn Z_DumpHeap(lowtag: std::ffi::c_int, hightag: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn Z_FileDumpHeap(f: *mut FILE) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn Z_CheckHeap() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn Z_ChangeTag2(ptr: *mut std::ffi::c_void, tag: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn Z_FreeMemory() -> std::ffi::c_int {
    todo!("body not yet translated")
}
