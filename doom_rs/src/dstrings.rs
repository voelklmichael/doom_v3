use crate::d_englsh::*;

pub const SAVEGAMENAME: *const std::ffi::c_char = (c"doomsav").as_ptr();

pub const DEVMAPS: *const std::ffi::c_char = (c"devmaps").as_ptr();

pub const DEVDATA: *const std::ffi::c_char = (c"devdata").as_ptr();

pub const NUM_QUITMESSAGES: std::ffi::c_int = 22;

static mut rcsid: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut endmsg: [*mut std::ffi::c_char; (NUM_QUITMESSAGES + 1) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated
