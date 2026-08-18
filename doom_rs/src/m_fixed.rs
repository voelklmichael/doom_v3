use crate::d_event::*;
use crate::d_ticcmd::*;
use crate::doomtype::*;
use crate::i_system::*;

pub const FRACBITS: std::ffi::c_int = 16;

pub const FRACUNIT: std::ffi::c_int = (1 << FRACBITS);

pub type fixed_t = std::ffi::c_int;

static mut rcsid: [std::ffi::c_char; 49] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        109 as std::ffi::c_char,
        95 as std::ffi::c_char,
        98 as std::ffi::c_char,
        98 as std::ffi::c_char,
        111 as std::ffi::c_char,
        120 as std::ffi::c_char,
        46 as std::ffi::c_char,
        99 as std::ffi::c_char,
        44 as std::ffi::c_char,
        118 as std::ffi::c_char,
        32 as std::ffi::c_char,
        49 as std::ffi::c_char,
        46 as std::ffi::c_char,
        49 as std::ffi::c_char,
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
        50 as std::ffi::c_char,
        50 as std::ffi::c_char,
        58 as std::ffi::c_char,
        52 as std::ffi::c_char,
        53 as std::ffi::c_char,
        58 as std::ffi::c_char,
        49 as std::ffi::c_char,
        48 as std::ffi::c_char,
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

pub unsafe extern "C" fn FixedMul(a: fixed_t, b: fixed_t) -> fixed_t {
    unsafe { todo!("body not yet translated") }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn FixedDiv(a: fixed_t, b: fixed_t) -> fixed_t {
    unsafe { todo!("body not yet translated") }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn FixedDiv2(a: fixed_t, b: fixed_t) -> fixed_t {
    unsafe { todo!("body not yet translated") }
    todo!("fell off the end of a non-void C function")
}
