use crate::d_event::*;
use crate::d_ticcmd::*;
use crate::doomdef::*;
use crate::doomtype::*;
use crate::i_system::*;

pub const PU_STATIC: std::ffi::c_int = 1;

pub const PU_SOUND: std::ffi::c_int = 2;

pub const PU_MUSIC: std::ffi::c_int = 3;

pub const PU_DAVE: std::ffi::c_int = 4;

pub const PU_LEVEL: std::ffi::c_int = 50;

pub const PU_LEVSPEC: std::ffi::c_int = 51;

pub const PU_PURGELEVEL: std::ffi::c_int = 100;

pub const PU_CACHE: std::ffi::c_int = 101;

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

/* TODO: statement-shaped macro body, needs manual translation:
#define Z_ChangeTag(...) \
{ \
      if (( (memblock_t *)( (byte *)(p) - sizeof(memblock_t)))->id!=0x1d4a11) \
      I_Error("Z_CT at "__FILE__":%i",__LINE__); \
      Z_ChangeTag2(p,t); \
};
*/

static mut rcsid: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub const ZONEID: std::ffi::c_int = 0x1d4a11;

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

pub const MINFRAGMENT: std::ffi::c_int = 64;

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
