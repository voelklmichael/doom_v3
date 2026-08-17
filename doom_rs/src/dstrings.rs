use crate::d_englsh::*;

static mut rcsid: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut endmsg: [*mut std::ffi::c_char; (NUM_QUITMESSAGES + 1) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated
