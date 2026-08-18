use crate::d_event::*;
use crate::d_items::*;
use crate::d_net::*;
use crate::d_player::*;
use crate::d_think::*;
use crate::d_ticcmd::*;
use crate::doomdata::*;
use crate::doomdef::*;
use crate::doomstat::*;
use crate::doomtype::*;
use crate::i_system::*;
use crate::info::*;
use crate::m_fixed::*;
use crate::p_mobj::*;
use crate::p_pspr::*;
use crate::r_bsp::*;
use crate::r_data::*;
use crate::r_defs::*;
use crate::r_local::*;
use crate::r_main::*;
use crate::r_plane::*;
use crate::r_segs::*;
use crate::r_state::*;
use crate::r_things::*;
use crate::tables::*;
use crate::v_video::*;
use crate::w_wad::*;
use crate::z_zone::*;

unsafe extern "C" {
    pub fn R_DrawFuzzColumnLow();
}

unsafe extern "C" {
    pub fn R_DrawTranslatedColumnLow();
}

static mut rcsid: [std::ffi::c_char; 49] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        114 as std::ffi::c_char,
        95 as std::ffi::c_char,
        100 as std::ffi::c_char,
        114 as std::ffi::c_char,
        97 as std::ffi::c_char,
        119 as std::ffi::c_char,
        46 as std::ffi::c_char,
        99 as std::ffi::c_char,
        44 as std::ffi::c_char,
        118 as std::ffi::c_char,
        32 as std::ffi::c_char,
        49 as std::ffi::c_char,
        46 as std::ffi::c_char,
        52 as std::ffi::c_char,
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
        49 as std::ffi::c_char,
        54 as std::ffi::c_char,
        58 as std::ffi::c_char,
        52 as std::ffi::c_char,
        55 as std::ffi::c_char,
        58 as std::ffi::c_char,
        53 as std::ffi::c_char,
        53 as std::ffi::c_char,
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

pub const MAXWIDTH: std::ffi::c_int = 1120;

pub const MAXHEIGHT: std::ffi::c_int = 832;

pub const SBARHEIGHT: std::ffi::c_int = 32;

pub static mut viewimage: *mut byte = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut viewwidth: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut scaledviewwidth: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut viewheight: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut viewwindowx: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut viewwindowy: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut ylookup: [*mut byte; (MAXHEIGHT) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut columnofs: [std::ffi::c_int; (MAXWIDTH) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut translations: [[byte; (256) as usize]; (3) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut dc_colormap: *mut lighttable_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut dc_x: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut dc_yl: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut dc_yh: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut dc_iscale: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut dc_texturemid: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut dc_source: *mut byte = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut dccount: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn R_DrawColumn() {
    unsafe {
        let mut count: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut dest: *mut byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut frac: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut fracstep: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        count = (dc_yh - dc_yl);
        // TODO: if statement not yet translated:
        //
        //
        //     // Zero length, column does not exceed a pixel.
        //     if (count < 0)
        // 	return;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //     if ((unsigned)dc_x >= SCREENWIDTH
        // 	|| dc_yl < 0
        // 	|| dc_yh >= SCREENHEIGHT)
        // 	I_Error ("R_DrawColumn: %i to %i at %i", dc_yl, dc_yh, dc_x);
        todo!("if statement not yet translated");
        dest = (ylookup[(dc_yl) as usize] + columnofs[(dc_x) as usize]);
        fracstep = dc_iscale;
        frac = (dc_texturemid + ((dc_yl - centery) * fracstep));
        // TODO: do-while statement not yet translated:
        //
        //
        //     // Inner loop that does the actual texture mapping,
        //     //  e.g. a DDA-lile scaling.
        //     // This is as fast as it gets.
        //     do
        //     {
        // 	// Re-map color indices from wall texture column
        // 	//  using a lighting/special effects LUT.
        // 	*dest = dc_colormap[dc_source[(frac>>FRACBITS)&127]];
        //
        // 	dest += SCREENWIDTH;
        // 	frac += fracstep;
        //
        //     } while (count--);
        todo!("do-while statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn R_DrawColumnLow() {
    unsafe {
        let mut count: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut dest: *mut byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut dest2: *mut byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut frac: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut fracstep: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        count = (dc_yh - dc_yl);
        // TODO: if statement not yet translated:
        //
        //
        //     // Zero length.
        //     if (count < 0)
        // 	return;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //     if ((unsigned)dc_x >= SCREENWIDTH
        // 	|| dc_yl < 0
        // 	|| dc_yh >= SCREENHEIGHT)
        //     {
        //
        // 	I_Error ("R_DrawColumn: %i to %i at %i", dc_yl, dc_yh, dc_x);
        //     }
        todo!("if statement not yet translated");
        dc_x <<= 1;
        dest = (ylookup[(dc_yl) as usize] + columnofs[(dc_x) as usize]);
        dest2 = (ylookup[(dc_yl) as usize] + columnofs[(dc_x + 1) as usize]);
        fracstep = dc_iscale;
        frac = (dc_texturemid + ((dc_yl - centery) * fracstep));
        // TODO: do-while statement not yet translated:
        //
        //
        //     do
        //     {
        // 	// Hack. Does not work corretly.
        // 	*dest2 = *dest = dc_colormap[dc_source[(frac>>FRACBITS)&127]];
        // 	dest += SCREENWIDTH;
        // 	dest2 += SCREENWIDTH;
        // 	frac += fracstep;
        //
        //     } while (count--);
        todo!("do-while statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub const FUZZTABLE: std::ffi::c_int = 50;

pub const FUZZOFF: std::ffi::c_int = (SCREENWIDTH);

pub static mut fuzzoffset: [std::ffi::c_int; (FUZZTABLE) as usize] = unsafe {
    [
        FUZZOFF,
        (-(FUZZOFF)),
        FUZZOFF,
        (-(FUZZOFF)),
        FUZZOFF,
        FUZZOFF,
        (-(FUZZOFF)),
        FUZZOFF,
        FUZZOFF,
        (-(FUZZOFF)),
        FUZZOFF,
        FUZZOFF,
        FUZZOFF,
        (-(FUZZOFF)),
        FUZZOFF,
        FUZZOFF,
        FUZZOFF,
        (-(FUZZOFF)),
        (-(FUZZOFF)),
        (-(FUZZOFF)),
        (-(FUZZOFF)),
        FUZZOFF,
        (-(FUZZOFF)),
        (-(FUZZOFF)),
        FUZZOFF,
        FUZZOFF,
        FUZZOFF,
        FUZZOFF,
        (-(FUZZOFF)),
        FUZZOFF,
        (-(FUZZOFF)),
        FUZZOFF,
        FUZZOFF,
        (-(FUZZOFF)),
        (-(FUZZOFF)),
        FUZZOFF,
        FUZZOFF,
        (-(FUZZOFF)),
        (-(FUZZOFF)),
        (-(FUZZOFF)),
        (-(FUZZOFF)),
        FUZZOFF,
        FUZZOFF,
        FUZZOFF,
        FUZZOFF,
        (-(FUZZOFF)),
        FUZZOFF,
        FUZZOFF,
        (-(FUZZOFF)),
        FUZZOFF,
    ]
};

pub static mut fuzzpos: std::ffi::c_int = unsafe { 0 };

pub unsafe extern "C" fn R_DrawFuzzColumn() {
    unsafe {
        let mut count: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut dest: *mut byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut frac: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut fracstep: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     // Adjust borders. Low...
        //     if (!dc_yl)
        // 	dc_yl = 1;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // .. and high.
        //     if (dc_yh == viewheight-1)
        // 	dc_yh = viewheight - 2;
        todo!("if statement not yet translated");
        count = (dc_yh - dc_yl);
        // TODO: if statement not yet translated:
        //
        //
        //     // Zero length.
        //     if (count < 0)
        // 	return;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //     if ((unsigned)dc_x >= SCREENWIDTH
        // 	|| dc_yl < 0 || dc_yh >= SCREENHEIGHT)
        //     {
        // 	I_Error ("R_DrawFuzzColumn: %i to %i at %i",
        // 		 dc_yl, dc_yh, dc_x);
        //     }
        todo!("if statement not yet translated");
        dest = (ylookup[(dc_yl) as usize] + columnofs[(dc_x) as usize]);
        fracstep = dc_iscale;
        frac = (dc_texturemid + ((dc_yl - centery) * fracstep));
        // TODO: do-while statement not yet translated:
        //
        //
        //     // Looks like an attempt at dithering,
        //     //  using the colormap #6 (of 0-31, a bit
        //     //  brighter than average).
        //     do
        //     {
        // 	// Lookup framebuffer, and retrieve
        // 	//  a pixel that is either one column
        // 	//  left or right of the current one.
        // 	// Add index from colormap to index.
        // 	*dest = colormaps[6*256+dest[fuzzoffset[fuzzpos]]];
        //
        // 	// Clamp table lookup index.
        // 	if (++fuzzpos == FUZZTABLE)
        // 	    fuzzpos = 0;
        //
        // 	dest += SCREENWIDTH;
        //
        // 	frac += fracstep;
        //     } while (count--);
        todo!("do-while statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub static mut dc_translation: *mut byte = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut translationtables: *mut byte = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn R_DrawTranslatedColumn() {
    unsafe {
        let mut count: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut dest: *mut byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut frac: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut fracstep: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        count = (dc_yh - dc_yl);
        // TODO: if statement not yet translated:
        //
        //     if (count < 0)
        // 	return;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //     if ((unsigned)dc_x >= SCREENWIDTH
        // 	|| dc_yl < 0
        // 	|| dc_yh >= SCREENHEIGHT)
        //     {
        // 	I_Error ( "R_DrawColumn: %i to %i at %i",
        // 		  dc_yl, dc_yh, dc_x);
        //     }
        todo!("if statement not yet translated");
        dest = (ylookup[(dc_yl) as usize] + columnofs[(dc_x) as usize]);
        fracstep = dc_iscale;
        frac = (dc_texturemid + ((dc_yl - centery) * fracstep));
        // TODO: do-while statement not yet translated:
        //
        //
        //     // Here we do an additional index re-mapping.
        //     do
        //     {
        // 	// Translation tables are used
        // 	//  to map certain colorramps to other ones,
        // 	//  used with PLAY sprites.
        // 	// Thus the "green" ramp of the player 0 sprite
        // 	//  is mapped to gray, red, black/indigo.
        // 	*dest = dc_colormap[dc_translation[dc_source[frac>>FRACBITS]]];
        // 	dest += SCREENWIDTH;
        //
        // 	frac += fracstep;
        //     } while (count--);
        todo!("do-while statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn R_InitTranslationTables() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        translationtables = Z_Malloc(((256 * 3) + 255), PU_STATIC, 0);
        translationtables =
            (((((translationtables) as std::ffi::c_int) + 255) & (!(255))) as *mut byte);
        // TODO: for statement not yet translated:
        //
        //
        //     // translate just the 16 green colors
        //     for (i=0 ; i<256 ; i++)
        //     {
        // 	if (i >= 0x70 && i<= 0x7f)
        // 	{
        // 	    // map green ramp to gray, brown, red
        // 	    translationtables[i] = 0x60 + (i&0xf);
        // 	    translationtables [i+256] = 0x40 + (i&0xf);
        // 	    translationtables [i+512] = 0x20 + (i&0xf);
        // 	}
        // 	else
        // 	{
        // 	    // Keep all other colors as is.
        // 	    translationtables[i] = translationtables[i+256]
        // 		= translationtables[i+512] = i;
        // 	}
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub static mut ds_y: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut ds_x1: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut ds_x2: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut ds_colormap: *mut lighttable_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut ds_xfrac: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut ds_yfrac: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut ds_xstep: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut ds_ystep: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut ds_source: *mut byte = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut dscount: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn R_DrawSpan() {
    unsafe {
        let mut xfrac: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut yfrac: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut dest: *mut byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut count: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut spot: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //     if (ds_x2 < ds_x1
        // 	|| ds_x1<0
        // 	|| ds_x2>=SCREENWIDTH
        // 	|| (unsigned)ds_y>SCREENHEIGHT)
        //     {
        // 	I_Error( "R_DrawSpan: %i to %i at %i",
        // 		 ds_x1,ds_x2,ds_y);
        //     }
        todo!("if statement not yet translated");
        xfrac = ds_xfrac;
        yfrac = ds_yfrac;
        dest = (ylookup[(ds_y) as usize] + columnofs[(ds_x1) as usize]);
        count = (ds_x2 - ds_x1);
        // TODO: do-while statement not yet translated:
        //
        //
        //     do
        //     {
        // 	// Current texture index in u,v.
        // 	spot = ((yfrac>>(16-6))&(63*64)) + ((xfrac>>16)&63);
        //
        // 	// Lookup pixel from flat texture tile,
        // 	//  re-index using light/colormap.
        // 	*dest++ = ds_colormap[ds_source[spot]];
        //
        // 	// Next step in u,v.
        // 	xfrac += ds_xstep;
        // 	yfrac += ds_ystep;
        //
        //     } while (count--);
        todo!("do-while statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn R_DrawSpanLow() {
    unsafe {
        let mut xfrac: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut yfrac: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut dest: *mut byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut count: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut spot: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //     if (ds_x2 < ds_x1
        // 	|| ds_x1<0
        // 	|| ds_x2>=SCREENWIDTH
        // 	|| (unsigned)ds_y>SCREENHEIGHT)
        //     {
        // 	I_Error( "R_DrawSpan: %i to %i at %i",
        // 		 ds_x1,ds_x2,ds_y);
        //     }
        todo!("if statement not yet translated");
        xfrac = ds_xfrac;
        yfrac = ds_yfrac;
        ds_x1 <<= 1;
        ds_x2 <<= 1;
        dest = (ylookup[(ds_y) as usize] + columnofs[(ds_x1) as usize]);
        count = (ds_x2 - ds_x1);
        // TODO: do-while statement not yet translated:
        //
        //     do
        //     {
        // 	spot = ((yfrac>>(16-6))&(63*64)) + ((xfrac>>16)&63);
        // 	// Lowres/blocky mode does it twice,
        // 	//  while scale is adjusted appropriately.
        // 	*dest++ = ds_colormap[ds_source[spot]];
        // 	*dest++ = ds_colormap[ds_source[spot]];
        //
        // 	xfrac += ds_xstep;
        // 	yfrac += ds_ystep;
        //
        //     } while (count--);
        todo!("do-while statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn R_InitBuffer(mut width: std::ffi::c_int, mut height: std::ffi::c_int) {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        viewwindowx = ((SCREENWIDTH - width) >> 1);
        // TODO: for statement not yet translated:
        //
        //
        //     // Column offset. For windows.
        //     for (i=0 ; i<width ; i++)
        // 	columnofs[i] = viewwindowx + i;
        todo!("for statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // Samw with base row offset.
        //     if (width == SCREENWIDTH)
        // 	viewwindowy = 0;
        //     else
        // 	viewwindowy = (SCREENHEIGHT-SBARHEIGHT-height) >> 1;
        todo!("if statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //
        //     // Preclaculate all row offsets.
        //     for (i=0 ; i<height ; i++)
        // 	ylookup[i] = screens[0] + (i+viewwindowy)*SCREENWIDTH;
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn R_FillBackScreen() {
    unsafe {
        let mut src: *mut byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut dest: *mut byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut x: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut y: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut patch: *mut patch_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut name1: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { (c"FLOOR7_2").as_ptr() };
        let mut name2: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { (c"GRNROCK").as_ptr() };
        let mut name: *mut std::ffi::c_char = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (scaledviewwidth == 320)
        // 	return;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if ( gamemode == commercial)
        // 	name = name2;
        //     else
        // 	name = name1;
        todo!("if statement not yet translated");
        src = W_CacheLumpName(name, PU_CACHE);
        dest = screens[(1) as usize];
        // TODO: for statement not yet translated:
        //
        //
        //     for (y=0 ; y<SCREENHEIGHT-SBARHEIGHT ; y++)
        //     {
        // 	for (x=0 ; x<SCREENWIDTH/64 ; x++)
        // 	{
        // 	    memcpy (dest, src+((y&63)<<6), 64);
        // 	    dest += 64;
        // 	}
        //
        // 	if (SCREENWIDTH&63)
        // 	{
        // 	    memcpy (dest, src+((y&63)<<6), SCREENWIDTH&63);
        // 	    dest += (SCREENWIDTH&63);
        // 	}
        //     }
        todo!("for statement not yet translated");
        patch = W_CacheLumpName((c"brdr_t").as_ptr(), PU_CACHE);
        // TODO: for statement not yet translated:
        //
        //
        //     for (x=0 ; x<scaledviewwidth ; x+=8)
        // 	V_DrawPatch (viewwindowx+x,viewwindowy-8,1,patch);
        todo!("for statement not yet translated");
        patch = W_CacheLumpName((c"brdr_b").as_ptr(), PU_CACHE);
        // TODO: for statement not yet translated:
        //
        //
        //     for (x=0 ; x<scaledviewwidth ; x+=8)
        // 	V_DrawPatch (viewwindowx+x,viewwindowy+viewheight,1,patch);
        todo!("for statement not yet translated");
        patch = W_CacheLumpName((c"brdr_l").as_ptr(), PU_CACHE);
        // TODO: for statement not yet translated:
        //
        //
        //     for (y=0 ; y<viewheight ; y+=8)
        // 	V_DrawPatch (viewwindowx-8,viewwindowy+y,1,patch);
        todo!("for statement not yet translated");
        patch = W_CacheLumpName((c"brdr_r").as_ptr(), PU_CACHE);
        // TODO: for statement not yet translated:
        //
        //
        //     for (y=0 ; y<viewheight ; y+=8)
        // 	V_DrawPatch (viewwindowx+scaledviewwidth,viewwindowy+y,1,patch);
        todo!("for statement not yet translated");
        V_DrawPatch(
            (viewwindowx - 8),
            (viewwindowy - 8),
            1,
            W_CacheLumpName((c"brdr_tl").as_ptr(), PU_CACHE),
        );
        V_DrawPatch(
            (viewwindowx + scaledviewwidth),
            (viewwindowy - 8),
            1,
            W_CacheLumpName((c"brdr_tr").as_ptr(), PU_CACHE),
        );
        V_DrawPatch(
            (viewwindowx - 8),
            (viewwindowy + viewheight),
            1,
            W_CacheLumpName((c"brdr_bl").as_ptr(), PU_CACHE),
        );
        V_DrawPatch(
            (viewwindowx + scaledviewwidth),
            (viewwindowy + viewheight),
            1,
            W_CacheLumpName((c"brdr_br").as_ptr(), PU_CACHE),
        );
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn R_VideoErase(mut ofs: std::ffi::c_uint, mut count: std::ffi::c_int) {
    unsafe {
        memcpy(
            (screens[(0) as usize] + ofs),
            (screens[(1) as usize] + ofs),
            count,
        );
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

unsafe extern "C" {
    pub fn V_MarkRect(
        x: std::ffi::c_int,
        y: std::ffi::c_int,
        width: std::ffi::c_int,
        height: std::ffi::c_int,
    );
}

pub unsafe extern "C" fn R_DrawViewBorder() {
    unsafe {
        let mut top: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut side: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut ofs: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (scaledviewwidth == SCREENWIDTH)
        // 	return;
        todo!("if statement not yet translated");
        top = (((SCREENHEIGHT - SBARHEIGHT) - viewheight) / 2);
        side = ((SCREENWIDTH - scaledviewwidth) / 2);
        R_VideoErase(0, ((top * SCREENWIDTH) + side));
        ofs = (((viewheight + top) * SCREENWIDTH) - side);
        R_VideoErase(ofs, ((top * SCREENWIDTH) + side));
        ofs = (((top * SCREENWIDTH) + SCREENWIDTH) - side);
        side <<= 1;
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=1 ; i<viewheight ; i++)
        //     {
        // 	R_VideoErase (ofs, side);
        // 	ofs += SCREENWIDTH;
        //     }
        todo!("for statement not yet translated");
        V_MarkRect(0, 0, SCREENWIDTH, (SCREENHEIGHT - SBARHEIGHT));
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}
