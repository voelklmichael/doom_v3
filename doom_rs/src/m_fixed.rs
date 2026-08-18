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

pub unsafe extern "C" fn FixedMul(mut a: fixed_t, mut b: fixed_t) -> fixed_t {
    unsafe {
        return ((((a) as std::ffi::c_longlong) * ((b) as std::ffi::c_longlong)) >> FRACBITS);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn FixedDiv(mut a: fixed_t, mut b: fixed_t) -> fixed_t {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if ( (abs(a)>>14) >= abs(b))
        // 	return (a^b)<0 ? MININT : MAXINT;
        todo!("if statement not yet translated");
        return FixedDiv2(a, b);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn FixedDiv2(mut a: fixed_t, mut b: fixed_t) -> fixed_t {
    unsafe {
        let mut c: std::ffi::c_double = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        c = ((((a) as std::ffi::c_double) / ((b) as std::ffi::c_double)) * ((FRACUNIT) as f64));
        // TODO: if statement not yet translated:
        //
        //
        //     if (c >= 2147483648.0 || c < -2147483648.0)
        // 	I_Error("FixedDiv: divide by zero");
        todo!("if statement not yet translated");
        return ((c) as fixed_t);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}
