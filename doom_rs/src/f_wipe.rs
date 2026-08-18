use crate::d_items::*;
use crate::d_player::*;
use crate::d_think::*;
use crate::d_ticcmd::*;
use crate::doomdata::*;
use crate::doomdef::*;
use crate::doomtype::*;
use crate::i_video::*;
use crate::info::*;
use crate::m_fixed::*;
use crate::m_random::*;
use crate::p_mobj::*;
use crate::p_pspr::*;
use crate::r_data::*;
use crate::r_defs::*;
use crate::r_state::*;
use crate::tables::*;
use crate::v_video::*;
use crate::z_zone::*;

pub const wipe_ColorXForm: std::ffi::c_int = 0;
pub const wipe_Melt: std::ffi::c_int = wipe_ColorXForm + 1;
pub const wipe_NUMWIPES: std::ffi::c_int = wipe_Melt + 1;

static mut rcsid: [std::ffi::c_char; 49] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        102 as std::ffi::c_char,
        95 as std::ffi::c_char,
        119 as std::ffi::c_char,
        105 as std::ffi::c_char,
        112 as std::ffi::c_char,
        101 as std::ffi::c_char,
        46 as std::ffi::c_char,
        99 as std::ffi::c_char,
        44 as std::ffi::c_char,
        118 as std::ffi::c_char,
        32 as std::ffi::c_char,
        49 as std::ffi::c_char,
        46 as std::ffi::c_char,
        50 as std::ffi::c_char,
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
        48 as std::ffi::c_char,
        57 as std::ffi::c_char,
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

static mut go: boolean = unsafe { 0 };

static mut wipe_scr_start: *mut byte = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut wipe_scr_end: *mut byte = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut wipe_scr: *mut byte = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn wipe_shittyColMajorXform(
    mut array: *mut std::ffi::c_short,
    mut width: std::ffi::c_int,
    mut height: std::ffi::c_int,
) {
    unsafe {
        let mut x: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut y: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut dest: *mut std::ffi::c_short = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        dest = ((Z_Malloc(((width * height) * 2), PU_STATIC, 0)) as *mut std::ffi::c_short);
        // TODO: for statement not yet translated:
        //
        //
        //     for(y=0;y<height;y++)
        // 	for(x=0;x<width;x++)
        // 	    dest[x*height+y] = array[y*width+x];
        todo!("for statement not yet translated");
        memcpy(array, dest, ((width * height) * 2));
        Z_Free(dest);
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn wipe_initColorXForm(
    mut width: std::ffi::c_int,
    mut height: std::ffi::c_int,
    mut ticks: std::ffi::c_int,
) -> std::ffi::c_int {
    unsafe {
        memcpy(wipe_scr, wipe_scr_start, (width * height));
        return 0;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn wipe_doColorXForm(
    mut width: std::ffi::c_int,
    mut height: std::ffi::c_int,
    mut ticks: std::ffi::c_int,
) -> std::ffi::c_int {
    unsafe {
        let mut changed: boolean = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut w: *mut byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut e: *mut byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut newval: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        changed = false_;
        w = wipe_scr;
        e = wipe_scr_end;
        // TODO: while statement not yet translated:
        //
        //
        //     while (w!=wipe_scr+width*height)
        //     {
        // 	if (*w != *e)
        // 	{
        // 	    if (*w > *e)
        // 	    {
        // 		newval = *w - ticks;
        // 		if (newval < *e)
        // 		    *w = *e;
        // 		else
        // 		    *w = newval;
        // 		changed = true;
        // 	    }
        // 	    else if (*w < *e)
        // 	    {
        // 		newval = *w + ticks;
        // 		if (newval > *e)
        // 		    *w = *e;
        // 		else
        // 		    *w = newval;
        // 		changed = true;
        // 	    }
        // 	}
        // 	w++;
        // 	e++;
        //     }
        todo!("while statement not yet translated");
        return (((changed) == 0) as std::ffi::c_int);
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn wipe_exitColorXForm(
    mut width: std::ffi::c_int,
    mut height: std::ffi::c_int,
    mut ticks: std::ffi::c_int,
) -> std::ffi::c_int {
    unsafe {
        return 0;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

static mut y: *mut std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn wipe_initMelt(
    mut width: std::ffi::c_int,
    mut height: std::ffi::c_int,
    mut ticks: std::ffi::c_int,
) -> std::ffi::c_int {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut r: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        memcpy(wipe_scr, wipe_scr_start, (width * height));
        wipe_shittyColMajorXform(
            ((wipe_scr_start) as *mut std::ffi::c_short),
            (width / 2),
            height,
        );
        wipe_shittyColMajorXform(
            ((wipe_scr_end) as *mut std::ffi::c_short),
            (width / 2),
            height,
        );
        y = ((Z_Malloc(
            (width * std::mem::size_of::<std::ffi::c_int>()),
            PU_STATIC,
            0,
        )) as *mut std::ffi::c_int);
        y[(0) as usize] = (-(M_Random() % 16));
        // TODO: for statement not yet translated:
        //
        //     for (i=1;i<width;i++)
        //     {
        // 	r = (M_Random()%3) - 1;
        // 	y[i] = y[i-1] + r;
        // 	if (y[i] > 0) y[i] = 0;
        // 	else if (y[i] == -16) y[i] = -15;
        //     }
        todo!("for statement not yet translated");
        return 0;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn wipe_doMelt(
    mut width: std::ffi::c_int,
    mut height: std::ffi::c_int,
    mut ticks: std::ffi::c_int,
) -> std::ffi::c_int {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut j: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut dy: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut idx: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut s: *mut std::ffi::c_short = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut d: *mut std::ffi::c_short = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut done: boolean = unsafe { true_ };
        width /= 2;
        // TODO: while statement not yet translated:
        //
        //
        //     while (ticks--)
        //     {
        // 	for (i=0;i<width;i++)
        // 	{
        // 	    if (y[i]<0)
        // 	    {
        // 		y[i]++; done = false;
        // 	    }
        // 	    else if (y[i] < height)
        // 	    {
        // 		dy = (y[i] < 16) ? y[i]+1 : 8;
        // 		if (y[i]+dy >= height) dy = height - y[i];
        // 		s = &((short *)wipe_scr_end)[i*height+y[i]];
        // 		d = &((short *)wipe_scr)[y[i]*width+i];
        // 		idx = 0;
        // 		for (j=dy;j;j--)
        // 		{
        // 		    d[idx] = *(s++);
        // 		    idx += width;
        // 		}
        // 		y[i] += dy;
        // 		s = &((short *)wipe_scr_start)[i*height];
        // 		d = &((short *)wipe_scr)[y[i]*width+i];
        // 		idx = 0;
        // 		for (j=height-y[i];j;j--)
        // 		{
        // 		    d[idx] = *(s++);
        // 		    idx += width;
        // 		}
        // 		done = false;
        // 	    }
        // 	}
        //     }
        todo!("while statement not yet translated");
        return done;
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn wipe_exitMelt(
    mut width: std::ffi::c_int,
    mut height: std::ffi::c_int,
    mut ticks: std::ffi::c_int,
) -> std::ffi::c_int {
    unsafe {
        Z_Free(y);
        return 0;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn wipe_StartScreen(
    mut x: std::ffi::c_int,
    mut y: std::ffi::c_int,
    mut width: std::ffi::c_int,
    mut height: std::ffi::c_int,
) -> std::ffi::c_int {
    unsafe {
        wipe_scr_start = screens[(2) as usize];
        I_ReadScreen(wipe_scr_start);
        return 0;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn wipe_EndScreen(
    mut x: std::ffi::c_int,
    mut y: std::ffi::c_int,
    mut width: std::ffi::c_int,
    mut height: std::ffi::c_int,
) -> std::ffi::c_int {
    unsafe {
        wipe_scr_end = screens[(3) as usize];
        I_ReadScreen(wipe_scr_end);
        V_DrawBlock(x, y, 0, width, height, wipe_scr_start);
        return 0;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn wipe_ScreenWipe(
    mut wipeno: std::ffi::c_int,
    mut x: std::ffi::c_int,
    mut y: std::ffi::c_int,
    mut width: std::ffi::c_int,
    mut height: std::ffi::c_int,
    mut ticks: std::ffi::c_int,
) -> std::ffi::c_int {
    unsafe {
        let mut rc: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        static mut wipes: *mut Option<unsafe extern "C" fn(std::ffi::c_int, std::ffi::c_int, std::ffi::c_int) -> std::ffi::c_int> /* TODO: was unsized array */ = unsafe { std::mem::zeroed() /* TODO: initializer not yet translated */ };
        // TODO: statement not yet translated:
        //
        //
        //     void V_MarkRect(int, int, int, int);
        todo!("statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // initial stuff
        //     if (!go)
        //     {
        // 	go = 1;
        // 	// wipe_scr = (byte *) Z_Malloc(width*height, PU_STATIC, 0); // DEBUG
        // 	wipe_scr = screens[0];
        // 	(*wipes[wipeno*3])(width, height, ticks);
        //     }
        todo!("if statement not yet translated");
        V_MarkRect(0, 0, width, height);
        rc = (*(wipes[((wipeno * 3) + 1) as usize]))(width, height, ticks);
        // TODO: if statement not yet translated:
        //
        //     //  V_DrawBlock(x, y, 0, width, height, wipe_scr); // DEBUG
        //
        //     // final stuff
        //     if (rc)
        //     {
        // 	go = 0;
        // 	(*wipes[wipeno*3+2])(width, height, ticks);
        //     }
        todo!("if statement not yet translated");
        return (((go) == 0) as std::ffi::c_int);
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}
