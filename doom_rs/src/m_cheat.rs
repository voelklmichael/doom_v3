pub unsafe extern "C" fn SCRAMBLE(a: std::ffi::c_int) -> std::ffi::c_int {
    ((((((((((a) & 1) << 7) + (((a) & 2) << 5)) + ((a) & 4)) + (((a) & 8) << 1))
        + (((a) & 16) >> 1))
        + ((a) & 32))
        + (((a) & 64) >> 5))
        + (((a) & 128) >> 7))
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cheatseq_t {
    pub sequence: *mut std::ffi::c_uchar,
    pub p: *mut std::ffi::c_uchar,
}

static mut rcsid: [std::ffi::c_char; 50] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        109 as std::ffi::c_char,
        95 as std::ffi::c_char,
        99 as std::ffi::c_char,
        104 as std::ffi::c_char,
        101 as std::ffi::c_char,
        97 as std::ffi::c_char,
        116 as std::ffi::c_char,
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
        49 as std::ffi::c_char,
        58 as std::ffi::c_char,
        50 as std::ffi::c_char,
        52 as std::ffi::c_char,
        58 as std::ffi::c_char,
        51 as std::ffi::c_char,
        52 as std::ffi::c_char,
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

static mut firsttime: std::ffi::c_int = unsafe { 1 };

static mut cheat_xlate_table: [std::ffi::c_uchar; (256) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn cht_CheckCheat(
    cht: *mut cheatseq_t,
    key: std::ffi::c_char,
) -> std::ffi::c_int {
    unsafe { todo!("body not yet translated") }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn cht_GetParam(cht: *mut cheatseq_t, buffer: *mut std::ffi::c_char) {
    unsafe { todo!("body not yet translated") }
}
