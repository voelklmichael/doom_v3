pub unsafe extern "C" fn SHORT(x: std::ffi::c_int) -> std::ffi::c_int {
    (x)
}

pub unsafe extern "C" fn LONG(x: std::ffi::c_int) -> std::ffi::c_int {
    (x)
}

static mut rcsid: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn SwapSHORT(x: std::ffi::c_ushort) -> std::ffi::c_ushort {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn SwapLONG(x: std::ffi::c_ulong) -> std::ffi::c_ulong {
    todo!("body not yet translated")
}
