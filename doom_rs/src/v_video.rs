use crate::d_event::*;
use crate::d_items::*;
use crate::d_player::*;
use crate::d_think::*;
use crate::d_ticcmd::*;
use crate::doomdata::*;
use crate::doomdef::*;
use crate::doomtype::*;
use crate::i_system::*;
use crate::info::*;
use crate::m_bbox::*;
use crate::m_fixed::*;
use crate::m_swap::*;
use crate::p_mobj::*;
use crate::p_pspr::*;
use crate::r_bsp::*;
use crate::r_data::*;
use crate::r_defs::*;
use crate::r_draw::*;
use crate::r_local::*;
use crate::r_main::*;
use crate::r_plane::*;
use crate::r_segs::*;
use crate::r_state::*;
use crate::r_things::*;
use crate::tables::*;

pub const CENTERY: std::ffi::c_int = (SCREENHEIGHT / 2);

static mut rcsid: [std::ffi::c_char; 50] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        118 as std::ffi::c_char,
        95 as std::ffi::c_char,
        118 as std::ffi::c_char,
        105 as std::ffi::c_char,
        100 as std::ffi::c_char,
        101 as std::ffi::c_char,
        111 as std::ffi::c_char,
        46 as std::ffi::c_char,
        99 as std::ffi::c_char,
        44 as std::ffi::c_char,
        118 as std::ffi::c_char,
        32 as std::ffi::c_char,
        49 as std::ffi::c_char,
        46 as std::ffi::c_char,
        53 as std::ffi::c_char,
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
        51 as std::ffi::c_char,
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

pub static mut screens: [*mut byte; (5) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut dirtybox: [std::ffi::c_int; (4) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut gammatable: [[byte; 256]; 5] = unsafe {
    [
        [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
            47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68,
            69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90,
            91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109,
            110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126,
            127, 128, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142,
            143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159,
            160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176,
            177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193,
            194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210,
            211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227,
            228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244,
            245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255,
        ],
        [
            2, 4, 5, 7, 8, 10, 11, 12, 14, 15, 16, 18, 19, 20, 21, 23, 24, 25, 26, 27, 29, 30, 31,
            32, 33, 34, 36, 37, 38, 39, 40, 41, 42, 44, 45, 46, 47, 48, 49, 50, 51, 52, 54, 55, 56,
            57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79,
            80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100,
            101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117,
            118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 129, 130, 131, 132, 133,
            134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 148, 149,
            150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 163, 164, 165,
            166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 175, 176, 177, 178, 179, 180, 181,
            182, 183, 184, 185, 186, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 196,
            197, 198, 199, 200, 201, 202, 203, 204, 205, 205, 206, 207, 208, 209, 210, 211, 212,
            213, 214, 214, 215, 216, 217, 218, 219, 220, 221, 222, 222, 223, 224, 225, 226, 227,
            228, 229, 230, 230, 231, 232, 233, 234, 235, 236, 237, 237, 238, 239, 240, 241, 242,
            243, 244, 245, 245, 246, 247, 248, 249, 250, 251, 252, 252, 253, 254, 255,
        ],
        [
            4, 7, 9, 11, 13, 15, 17, 19, 21, 22, 24, 26, 27, 29, 30, 32, 33, 35, 36, 38, 39, 40,
            42, 43, 45, 46, 47, 48, 50, 51, 52, 54, 55, 56, 57, 59, 60, 61, 62, 63, 65, 66, 67, 68,
            69, 70, 72, 73, 74, 75, 76, 77, 78, 79, 80, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92,
            93, 94, 95, 96, 97, 98, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111,
            112, 113, 114, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127,
            128, 129, 130, 131, 132, 133, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143,
            144, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 153, 154, 155, 156, 157, 158,
            159, 160, 160, 161, 162, 163, 164, 165, 166, 166, 167, 168, 169, 170, 171, 172, 172,
            173, 174, 175, 176, 177, 178, 178, 179, 180, 181, 182, 183, 183, 184, 185, 186, 187,
            188, 188, 189, 190, 191, 192, 193, 193, 194, 195, 196, 197, 197, 198, 199, 200, 201,
            201, 202, 203, 204, 205, 206, 206, 207, 208, 209, 210, 210, 211, 212, 213, 213, 214,
            215, 216, 217, 217, 218, 219, 220, 221, 221, 222, 223, 224, 224, 225, 226, 227, 228,
            228, 229, 230, 231, 231, 232, 233, 234, 235, 235, 236, 237, 238, 238, 239, 240, 241,
            241, 242, 243, 244, 244, 245, 246, 247, 247, 248, 249, 250, 251, 251, 252, 253, 254,
            254, 255,
        ],
        [
            8, 12, 16, 19, 22, 24, 27, 29, 31, 34, 36, 38, 40, 41, 43, 45, 47, 49, 50, 52, 53, 55,
            57, 58, 60, 61, 63, 64, 65, 67, 68, 70, 71, 72, 74, 75, 76, 77, 79, 80, 81, 82, 84, 85,
            86, 87, 88, 90, 91, 92, 93, 94, 95, 96, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107,
            108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124,
            125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 135, 136, 137, 138, 139, 140,
            141, 142, 143, 143, 144, 145, 146, 147, 148, 149, 150, 150, 151, 152, 153, 154, 155,
            155, 156, 157, 158, 159, 160, 160, 161, 162, 163, 164, 165, 165, 166, 167, 168, 169,
            169, 170, 171, 172, 173, 173, 174, 175, 176, 176, 177, 178, 179, 180, 180, 181, 182,
            183, 183, 184, 185, 186, 186, 187, 188, 189, 189, 190, 191, 192, 192, 193, 194, 195,
            195, 196, 197, 197, 198, 199, 200, 200, 201, 202, 202, 203, 204, 205, 205, 206, 207,
            207, 208, 209, 210, 210, 211, 212, 212, 213, 214, 214, 215, 216, 216, 217, 218, 219,
            219, 220, 221, 221, 222, 223, 223, 224, 225, 225, 226, 227, 227, 228, 229, 229, 230,
            231, 231, 232, 233, 233, 234, 235, 235, 236, 237, 237, 238, 238, 239, 240, 240, 241,
            242, 242, 243, 244, 244, 245, 246, 246, 247, 247, 248, 249, 249, 250, 251, 251, 252,
            253, 253, 254, 254, 255,
        ],
        [
            16, 23, 28, 32, 36, 39, 42, 45, 48, 50, 53, 55, 57, 60, 62, 64, 66, 68, 69, 71, 73, 75,
            76, 78, 80, 81, 83, 84, 86, 87, 89, 90, 92, 93, 94, 96, 97, 98, 100, 101, 102, 103,
            105, 106, 107, 108, 109, 110, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122,
            123, 124, 125, 126, 128, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139,
            140, 141, 142, 143, 143, 144, 145, 146, 147, 148, 149, 150, 150, 151, 152, 153, 154,
            155, 155, 156, 157, 158, 159, 159, 160, 161, 162, 163, 163, 164, 165, 166, 166, 167,
            168, 169, 169, 170, 171, 172, 172, 173, 174, 175, 175, 176, 177, 177, 178, 179, 180,
            180, 181, 182, 182, 183, 184, 184, 185, 186, 187, 187, 188, 189, 189, 190, 191, 191,
            192, 193, 193, 194, 195, 195, 196, 196, 197, 198, 198, 199, 200, 200, 201, 202, 202,
            203, 203, 204, 205, 205, 206, 207, 207, 208, 208, 209, 210, 210, 211, 211, 212, 213,
            213, 214, 214, 215, 216, 216, 217, 217, 218, 219, 219, 220, 220, 221, 221, 222, 223,
            223, 224, 224, 225, 225, 226, 227, 227, 228, 228, 229, 229, 230, 230, 231, 232, 232,
            233, 233, 234, 234, 235, 235, 236, 236, 237, 237, 238, 239, 239, 240, 240, 241, 241,
            242, 242, 243, 243, 244, 244, 245, 245, 246, 246, 247, 247, 248, 248, 249, 249, 250,
            250, 251, 251, 252, 252, 253, 254, 254, 255, 255,
        ],
    ]
};

pub static mut usegamma: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn V_MarkRect(
    mut x: std::ffi::c_int,
    mut y: std::ffi::c_int,
    mut width: std::ffi::c_int,
    mut height: std::ffi::c_int,
) {
    unsafe {
        M_AddToBox(dirtybox, x, y);
        M_AddToBox(dirtybox, ((x + width) - 1), ((y + height) - 1));
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn V_CopyRect(
    mut srcx: std::ffi::c_int,
    mut srcy: std::ffi::c_int,
    mut srcscrn: std::ffi::c_int,
    mut width: std::ffi::c_int,
    mut height: std::ffi::c_int,
    mut destx: std::ffi::c_int,
    mut desty: std::ffi::c_int,
    mut destscrn: std::ffi::c_int,
) {
    unsafe {
        let mut src: *mut byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut dest: *mut byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //     if (srcx<0
        // 	||srcx+width >SCREENWIDTH
        // 	|| srcy<0
        // 	|| srcy+height>SCREENHEIGHT
        // 	||destx<0||destx+width >SCREENWIDTH
        // 	|| desty<0
        // 	|| desty+height>SCREENHEIGHT
        // 	|| (unsigned)srcscrn>4
        // 	|| (unsigned)destscrn>4)
        //     {
        // 	I_Error ("Bad V_CopyRect");
        //     }
        todo!("if statement not yet translated");
        V_MarkRect(destx, desty, width, height);
        src = ((screens[(srcscrn) as usize] + (SCREENWIDTH * srcy)) + srcx);
        dest = ((screens[(destscrn) as usize] + (SCREENWIDTH * desty)) + destx);
        // TODO: for statement not yet translated:
        //
        //
        //     for ( ; height>0 ; height--)
        //     {
        // 	memcpy (dest, src, width);
        // 	src += SCREENWIDTH;
        // 	dest += SCREENWIDTH;
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn V_DrawPatch(
    mut x: std::ffi::c_int,
    mut y: std::ffi::c_int,
    mut scrn: std::ffi::c_int,
    mut patch: *mut patch_t,
) {
    unsafe {
        let mut count: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut col: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut column: *mut column_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut desttop: *mut byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut dest: *mut byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut source: *mut byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut w: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        y -= SHORT((*patch).topoffset);
        x -= SHORT((*patch).leftoffset);
        // TODO: if statement not yet translated:
        //     if (x<0
        // 	||x+SHORT(patch->width) >SCREENWIDTH
        // 	|| y<0
        // 	|| y+SHORT(patch->height)>SCREENHEIGHT
        // 	|| (unsigned)scrn>4)
        //     {
        //       fprintf( stderr, "Patch at %d,%d exceeds LFB\n", x,y );
        //       // No I_Error abort - what is up with TNT.WAD?
        //       fprintf( stderr, "V_DrawPatch: bad patch (ignored)\n");
        //       return;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (!scrn)
        // 	V_MarkRect (x, y, SHORT(patch->width), SHORT(patch->height));
        todo!("if statement not yet translated");
        col = 0;
        desttop = ((screens[(scrn) as usize] + (y * SCREENWIDTH)) + x);
        w = SHORT((*patch).width);
        // TODO: for statement not yet translated:
        //
        //
        //     for ( ; col<w ; x++, col++, desttop++)
        //     {
        // 	column = (column_t *)((byte *)patch + LONG(patch->columnofs[col]));
        //
        // 	// step through the posts in a column
        // 	while (column->topdelta != 0xff )
        // 	{
        // 	    source = (byte *)column + 3;
        // 	    dest = desttop + column->topdelta*SCREENWIDTH;
        // 	    count = column->length;
        //
        // 	    while (count--)
        // 	    {
        // 		*dest = *source++;
        // 		dest += SCREENWIDTH;
        // 	    }
        // 	    column = (column_t *)(  (byte *)column + column->length
        // 				    + 4 );
        // 	}
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn V_DrawPatchFlipped(
    mut x: std::ffi::c_int,
    mut y: std::ffi::c_int,
    mut scrn: std::ffi::c_int,
    mut patch: *mut patch_t,
) {
    unsafe {
        let mut count: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut col: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut column: *mut column_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut desttop: *mut byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut dest: *mut byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut source: *mut byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut w: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        y -= SHORT((*patch).topoffset);
        x -= SHORT((*patch).leftoffset);
        // TODO: if statement not yet translated:
        //     if (x<0
        // 	||x+SHORT(patch->width) >SCREENWIDTH
        // 	|| y<0
        // 	|| y+SHORT(patch->height)>SCREENHEIGHT
        // 	|| (unsigned)scrn>4)
        //     {
        //       fprintf( stderr, "Patch origin %d,%d exceeds LFB\n", x,y );
        //       I_Error ("Bad V_DrawPatch in V_DrawPatchFlipped");
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (!scrn)
        // 	V_MarkRect (x, y, SHORT(patch->width), SHORT(patch->height));
        todo!("if statement not yet translated");
        col = 0;
        desttop = ((screens[(scrn) as usize] + (y * SCREENWIDTH)) + x);
        w = SHORT((*patch).width);
        // TODO: for statement not yet translated:
        //
        //
        //     for ( ; col<w ; x++, col++, desttop++)
        //     {
        // 	column = (column_t *)((byte *)patch + LONG(patch->columnofs[w-1-col]));
        //
        // 	// step through the posts in a column
        // 	while (column->topdelta != 0xff )
        // 	{
        // 	    source = (byte *)column + 3;
        // 	    dest = desttop + column->topdelta*SCREENWIDTH;
        // 	    count = column->length;
        //
        // 	    while (count--)
        // 	    {
        // 		*dest = *source++;
        // 		dest += SCREENWIDTH;
        // 	    }
        // 	    column = (column_t *)(  (byte *)column + column->length
        // 				    + 4 );
        // 	}
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn V_DrawPatchDirect(
    mut x: std::ffi::c_int,
    mut y: std::ffi::c_int,
    mut scrn: std::ffi::c_int,
    mut patch: *mut patch_t,
) {
    unsafe {
        V_DrawPatch(x, y, scrn, patch);
        // TODO: statement not yet translated:
        //
        //
        //     /*
        //     int		count;
        //     int		col;
        //     column_t*	column;
        //     byte*	desttop;
        //     byte*	dest;
        //     byte*	source;
        //     int		w;
        //
        //     y -= SHORT(patch->topoffset);
        //     x -= SHORT(patch->leftoffset);
        //
        // #ifdef RANGECHECK
        //     if (x<0
        // 	||x+SHORT(patch->width) >SCREENWIDTH
        // 	|| y<0
        // 	|| y+SHORT(patch->height)>SCREENHEIGHT
        // 	|| (unsigned)scrn>4)
        //     {
        // 	I_Error ("Bad V_DrawPatchDirect");
        //     }
        // #endif
        //
        //     //	V_MarkRect (x, y, SHORT(patch->width), SHORT(patch->height));
        //     desttop = destscreen + y*SCREENWIDTH/4 + (x>>2);
        //
        //     w = SHORT(patch->width);
        //     for ( col = 0 ; col<w ; col++)
        //     {
        // 	outp (SC_INDEX+1,1<<(x&3));
        // 	column = (column_t *)((byte *)patch + LONG(patch->columnofs[col]));
        //
        // 	// step through the posts in a column
        //
        // 	while (column->topdelta != 0xff )
        // 	{
        // 	    source = (byte *)column + 3;
        // 	    dest = desttop + column->topdelta*SCREENWIDTH/4;
        // 	    count = column->length;
        //
        // 	    while (count--)
        // 	    {
        // 		*dest = *source++;
        // 		dest += SCREENWIDTH/4;
        // 	    }
        // 	    column = (column_t *)(  (byte *)column + column->length
        // 				    + 4 );
        // 	}
        // 	if ( ((++x)&3) == 0 )
        // 	    desttop++;	// go to next byte, not next plane
        //     }*/
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn V_DrawBlock(
    mut x: std::ffi::c_int,
    mut y: std::ffi::c_int,
    mut scrn: std::ffi::c_int,
    mut width: std::ffi::c_int,
    mut height: std::ffi::c_int,
    mut src: *mut byte,
) {
    unsafe {
        let mut dest: *mut byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //     if (x<0
        // 	||x+width >SCREENWIDTH
        // 	|| y<0
        // 	|| y+height>SCREENHEIGHT
        // 	|| (unsigned)scrn>4 )
        //     {
        // 	I_Error ("Bad V_DrawBlock");
        //     }
        todo!("if statement not yet translated");
        V_MarkRect(x, y, width, height);
        dest = ((screens[(scrn) as usize] + (y * SCREENWIDTH)) + x);
        // TODO: while statement not yet translated:
        //
        //
        //     while (height--)
        //     {
        // 	memcpy (dest, src, width);
        // 	src += width;
        // 	dest += SCREENWIDTH;
        //     }
        todo!("while statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn V_GetBlock(
    mut x: std::ffi::c_int,
    mut y: std::ffi::c_int,
    mut scrn: std::ffi::c_int,
    mut width: std::ffi::c_int,
    mut height: std::ffi::c_int,
    mut dest: *mut byte,
) {
    unsafe {
        let mut src: *mut byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //     if (x<0
        // 	||x+width >SCREENWIDTH
        // 	|| y<0
        // 	|| y+height>SCREENHEIGHT
        // 	|| (unsigned)scrn>4 )
        //     {
        // 	I_Error ("Bad V_DrawBlock");
        //     }
        todo!("if statement not yet translated");
        src = ((screens[(scrn) as usize] + (y * SCREENWIDTH)) + x);
        // TODO: while statement not yet translated:
        //
        //
        //     while (height--)
        //     {
        // 	memcpy (dest, src, width);
        // 	src += SCREENWIDTH;
        // 	dest += width;
        //     }
        todo!("while statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn V_Init() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut base: *mut byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        base = I_AllocLow(((SCREENWIDTH * SCREENHEIGHT) * 4));
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<4 ; i++)
        // 	screens[i] = base + i*SCREENWIDTH*SCREENHEIGHT;
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}
