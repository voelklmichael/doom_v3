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
use crate::st_stuff::*;
use crate::tables::*;
use crate::v_video::*;
use crate::w_wad::*;
use crate::z_zone::*;

pub const BG: std::ffi::c_int = 4;

pub const FG: std::ffi::c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_number_t {
    pub x: std::ffi::c_int,
    pub y: std::ffi::c_int,
    pub width: std::ffi::c_int,
    pub oldnum: std::ffi::c_int,
    pub num: *mut std::ffi::c_int,
    pub on: *mut boolean,
    pub p: *mut *mut patch_t,
    pub data: std::ffi::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_percent_t {
    pub n: st_number_t,
    pub p: *mut patch_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_multicon_t {
    pub x: std::ffi::c_int,
    pub y: std::ffi::c_int,
    pub oldinum: std::ffi::c_int,
    pub inum: *mut std::ffi::c_int,
    pub on: *mut boolean,
    pub p: *mut *mut patch_t,
    pub data: std::ffi::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_binicon_t {
    pub x: std::ffi::c_int,
    pub y: std::ffi::c_int,
    pub oldval: std::ffi::c_int,
    pub val: *mut boolean,
    pub on: *mut boolean,
    pub p: *mut patch_t,
    pub data: std::ffi::c_int,
}

static mut rcsid: [std::ffi::c_char; 49] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        115 as std::ffi::c_char,
        116 as std::ffi::c_char,
        95 as std::ffi::c_char,
        108 as std::ffi::c_char,
        105 as std::ffi::c_char,
        98 as std::ffi::c_char,
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
        54 as std::ffi::c_char,
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

unsafe extern "C" {
    pub static mut automapactive: boolean;
}

pub static mut sttminus: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn STlib_init() {
    unsafe {
        sttminus = ((W_CacheLumpName((c"STTMINUS").as_ptr(), PU_STATIC)) as *mut patch_t);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn STlib_initNum(
    mut n: *mut st_number_t,
    mut x: std::ffi::c_int,
    mut y: std::ffi::c_int,
    mut pl: *mut *mut patch_t,
    mut num: *mut std::ffi::c_int,
    mut on: *mut boolean,
    mut width: std::ffi::c_int,
) {
    unsafe {
        (*n).x = x;
        (*n).y = y;
        (*n).oldnum = 0;
        (*n).width = width;
        (*n).num = num;
        (*n).on = on;
        (*n).p = pl;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn STlib_drawNum(mut n: *mut st_number_t, mut refresh: boolean) {
    unsafe {
        let mut numdigits: std::ffi::c_int = unsafe { (*n).width };
        let mut num: std::ffi::c_int = unsafe { (*((*n).num)) };
        let mut w: std::ffi::c_int = unsafe { SHORT((*(*n).p[(0) as usize]).width) };
        let mut h: std::ffi::c_int = unsafe { SHORT((*(*n).p[(0) as usize]).height) };
        let mut x: std::ffi::c_int = unsafe { (*n).x };
        let mut neg: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        (*n).oldnum = (*((*n).num));
        neg = (num < 0);
        // TODO: if statement not yet translated:
        //
        //
        //     if (neg)
        //     {
        // 	if (numdigits == 2 && num < -9)
        // 	    num = -9;
        // 	else if (numdigits == 3 && num < -99)
        // 	    num = -99;
        //
        // 	num = -num;
        //     }
        todo!("if statement not yet translated");
        x = ((*n).x - (numdigits * w));
        // TODO: if statement not yet translated:
        //
        //
        //     if (n->y - ST_Y < 0)
        // 	I_Error("drawNum: n->y - ST_Y < 0");
        todo!("if statement not yet translated");
        V_CopyRect(x, ((*n).y - ST_Y), BG, (w * numdigits), h, x, (*n).y, FG);
        // TODO: if statement not yet translated:
        //
        //
        //     // if non-number, do not draw it
        //     if (num == 1994)
        // 	return;
        todo!("if statement not yet translated");
        x = (*n).x;
        // TODO: if statement not yet translated:
        //
        //
        //     // in the special case of 0, you draw 0
        //     if (!num)
        // 	V_DrawPatch(x - w, n->y, FG, n->p[ 0 ]);
        todo!("if statement not yet translated");
        // TODO: while statement not yet translated:
        //
        //
        //     // draw the new number
        //     while (num && numdigits--)
        //     {
        // 	x -= w;
        // 	V_DrawPatch(x, n->y, FG, n->p[ num % 10 ]);
        // 	num /= 10;
        //     }
        todo!("while statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // draw a minus sign if necessary
        //     if (neg)
        // 	V_DrawPatch(x - 8, n->y, FG, sttminus);
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn STlib_updateNum(mut n: *mut st_number_t, mut refresh: boolean) {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (*n->on) STlib_drawNum(n, refresh);
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn STlib_initPercent(
    mut p: *mut st_percent_t,
    mut x: std::ffi::c_int,
    mut y: std::ffi::c_int,
    mut pl: *mut *mut patch_t,
    mut num: *mut std::ffi::c_int,
    mut on: *mut boolean,
    mut percent: *mut patch_t,
) {
    unsafe {
        STlib_initNum((&((*p).n) as *const _ as *mut _), x, y, pl, num, on, 3);
        (*p).p = percent;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn STlib_updatePercent(
    mut per: *mut st_percent_t,
    mut refresh: std::ffi::c_int,
) {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (refresh && *per->n.on)
        // 	V_DrawPatch(per->n.x, per->n.y, FG, per->p);
        todo!("if statement not yet translated");
        STlib_updateNum((&((*per).n) as *const _ as *mut _), refresh);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn STlib_initMultIcon(
    mut i: *mut st_multicon_t,
    mut x: std::ffi::c_int,
    mut y: std::ffi::c_int,
    mut il: *mut *mut patch_t,
    mut inum: *mut std::ffi::c_int,
    mut on: *mut boolean,
) {
    unsafe {
        (*i).x = x;
        (*i).y = y;
        (*i).oldinum = (-(1));
        (*i).inum = inum;
        (*i).on = on;
        (*i).p = il;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn STlib_updateMultIcon(mut mi: *mut st_multicon_t, mut refresh: boolean) {
    unsafe {
        let mut w: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut h: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut x: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut y: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (*mi->on
        // 	&& (mi->oldinum != *mi->inum || refresh)
        // 	&& (*mi->inum!=-1))
        //     {
        // 	if (mi->oldinum != -1)
        // 	{
        // 	    x = mi->x - SHORT(mi->p[mi->oldinum]->leftoffset);
        // 	    y = mi->y - SHORT(mi->p[mi->oldinum]->topoffset);
        // 	    w = SHORT(mi->p[mi->oldinum]->width);
        // 	    h = SHORT(mi->p[mi->oldinum]->height);
        //
        // 	    if (y - ST_Y < 0)
        // 		I_Error("updateMultIcon: y - ST_Y < 0");
        //
        // 	    V_CopyRect(x, y-ST_Y, BG, w, h, x, y, FG);
        // 	}
        // 	V_DrawPatch(mi->x, mi->y, FG, mi->p[*mi->inum]);
        // 	mi->oldinum = *mi->inum;
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn STlib_initBinIcon(
    mut b: *mut st_binicon_t,
    mut x: std::ffi::c_int,
    mut y: std::ffi::c_int,
    mut i: *mut patch_t,
    mut val: *mut boolean,
    mut on: *mut boolean,
) {
    unsafe {
        (*b).x = x;
        (*b).y = y;
        (*b).oldval = 0;
        (*b).val = val;
        (*b).on = on;
        (*b).p = i;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn STlib_updateBinIcon(mut bi: *mut st_binicon_t, mut refresh: boolean) {
    unsafe {
        let mut x: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut y: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut w: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut h: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (*bi->on
        // 	&& (bi->oldval != *bi->val || refresh))
        //     {
        // 	x = bi->x - SHORT(bi->p->leftoffset);
        // 	y = bi->y - SHORT(bi->p->topoffset);
        // 	w = SHORT(bi->p->width);
        // 	h = SHORT(bi->p->height);
        //
        // 	if (y - ST_Y < 0)
        // 	    I_Error("updateBinIcon: y - ST_Y < 0");
        //
        // 	if (*bi->val)
        // 	    V_DrawPatch(bi->x, bi->y, FG, bi->p);
        // 	else
        // 	    V_CopyRect(x, y-ST_Y, BG, w, h, x, y, FG);
        //
        // 	bi->oldval = *bi->val;
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}
