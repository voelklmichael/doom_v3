use crate::m_fixed::*;

pub const BOXTOP: std::ffi::c_int = 0;
pub const BOXBOTTOM: std::ffi::c_int = BOXTOP + 1;
pub const BOXLEFT: std::ffi::c_int = BOXBOTTOM + 1;
pub const BOXRIGHT: std::ffi::c_int = BOXLEFT + 1;

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

pub unsafe extern "C" fn M_ClearBox(mut box_: *mut fixed_t) {
    unsafe {
        box_[(BOXTOP) as usize] = box_[(BOXRIGHT) as usize] = std::ffi::c_int::MIN;
        box_[(BOXBOTTOM) as usize] = box_[(BOXLEFT) as usize] = std::ffi::c_int::MAX;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_AddToBox(mut box_: *mut fixed_t, mut x: fixed_t, mut y: fixed_t) {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (x<box[BOXLEFT])
        // 	box[BOXLEFT] = x;
        //     else if (x>box[BOXRIGHT])
        // 	box[BOXRIGHT] = x;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (y<box[BOXBOTTOM])
        // 	box[BOXBOTTOM] = y;
        //     else if (y>box[BOXTOP])
        // 	box[BOXTOP] = y;
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}
