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

static mut rcsid: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut firsttime: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut cheat_xlate_table: [std::ffi::c_uchar; (256) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn cht_CheckCheat(
    cht: *mut cheatseq_t,
    key: std::ffi::c_char,
) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn cht_GetParam(cht: *mut cheatseq_t, buffer: *mut std::ffi::c_char) {
    todo!("body not yet translated")
}
