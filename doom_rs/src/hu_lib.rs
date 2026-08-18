use crate::d_items::*;
use crate::d_player::*;
use crate::d_think::*;
use crate::d_ticcmd::*;
use crate::doomdata::*;
use crate::doomdef::*;
use crate::doomtype::*;
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
use crate::tables::*;
use crate::v_video::*;

pub const BG: std::ffi::c_int = 1;

pub const FG: std::ffi::c_int = 0;

pub const HU_CHARERASE: std::ffi::c_int = KEY_BACKSPACE;

pub const HU_MAXLINES: std::ffi::c_int = 4;

pub const HU_MAXLINELENGTH: std::ffi::c_int = 80;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hu_textline_t {
    pub x: std::ffi::c_int,
    pub y: std::ffi::c_int,
    pub f: *mut *mut patch_t,
    pub sc: std::ffi::c_int,
    pub l: [std::ffi::c_char; (HU_MAXLINELENGTH + 1) as usize],
    pub len: std::ffi::c_int,
    pub needsupdate: std::ffi::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hu_stext_t {
    pub l: [hu_textline_t; (HU_MAXLINES) as usize],
    pub h: std::ffi::c_int,
    pub cl: std::ffi::c_int,
    pub on: *mut boolean,
    pub laston: boolean,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hu_itext_t {
    pub l: hu_textline_t,
    pub lm: std::ffi::c_int,
    pub on: *mut boolean,
    pub laston: boolean,
}

static mut rcsid: [std::ffi::c_char; 49] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        104 as std::ffi::c_char,
        117 as std::ffi::c_char,
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
        51 as std::ffi::c_char,
        32 as std::ffi::c_char,
        49 as std::ffi::c_char,
        57 as std::ffi::c_char,
        57 as std::ffi::c_char,
        55 as std::ffi::c_char,
        47 as std::ffi::c_char,
        48 as std::ffi::c_char,
        49 as std::ffi::c_char,
        47 as std::ffi::c_char,
        50 as std::ffi::c_char,
        54 as std::ffi::c_char,
        32 as std::ffi::c_char,
        48 as std::ffi::c_char,
        55 as std::ffi::c_char,
        58 as std::ffi::c_char,
        52 as std::ffi::c_char,
        52 as std::ffi::c_char,
        58 as std::ffi::c_char,
        53 as std::ffi::c_char,
        56 as std::ffi::c_char,
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

pub const noterased: std::ffi::c_int = viewwindowx;

unsafe extern "C" {
    pub static mut automapactive: boolean;
}

pub unsafe extern "C" fn HUlib_init() {
    unsafe {
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn HUlib_clearTextLine(mut t: *mut hu_textline_t) {
    unsafe {
        (*t).len = 0;
        (*t).l[(0) as usize] = 0;
        (*t).needsupdate = true_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn HUlib_initTextLine(
    mut t: *mut hu_textline_t,
    mut x: std::ffi::c_int,
    mut y: std::ffi::c_int,
    mut f: *mut *mut patch_t,
    mut sc: std::ffi::c_int,
) {
    unsafe {
        (*t).x = x;
        (*t).y = y;
        (*t).f = f;
        (*t).sc = sc;
        HUlib_clearTextLine(t);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn HUlib_addCharToTextLine(
    mut t: *mut hu_textline_t,
    mut ch: std::ffi::c_char,
) -> boolean {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //
        //     if (t->len == HU_MAXLINELENGTH)
        // 	return false;
        //     else
        //     {
        // 	t->l[t->len++] = ch;
        // 	t->l[t->len] = 0;
        // 	t->needsupdate = 4;
        // 	return true;
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn HUlib_delCharFromTextLine(mut t: *mut hu_textline_t) -> boolean {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //
        //     if (!t->len) return false;
        //     else
        //     {
        // 	t->l[--t->len] = 0;
        // 	t->needsupdate = 4;
        // 	return true;
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn HUlib_drawTextLine(mut l: *mut hu_textline_t, mut drawcursor: boolean) {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut w: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut x: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut c: std::ffi::c_uchar = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        x = (*l).x;
        // TODO: for statement not yet translated:
        //
        //     for (i=0;i<l->len;i++)
        //     {
        // 	c = toupper(l->l[i]);
        // 	if (c != ' '
        // 	    && c >= l->sc
        // 	    && c <= '_')
        // 	{
        // 	    w = SHORT(l->f[c - l->sc]->width);
        // 	    if (x+w > SCREENWIDTH)
        // 		break;
        // 	    V_DrawPatchDirect(x, l->y, FG, l->f[c - l->sc]);
        // 	    x += w;
        // 	}
        // 	else
        // 	{
        // 	    x += 4;
        // 	    if (x >= SCREENWIDTH)
        // 		break;
        // 	}
        //     }
        todo!("for statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // draw the cursor if requested
        //     if (drawcursor
        // 	&& x + SHORT(l->f['_' - l->sc]->width) <= SCREENWIDTH)
        //     {
        // 	V_DrawPatchDirect(x, l->y, FG, l->f['_' - l->sc]);
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn HUlib_eraseTextLine(mut l: *mut hu_textline_t) {
    unsafe {
        let mut lh: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut y: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut yoffset: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        static mut lastautomapactive: boolean = unsafe { true_ };
        // TODO: if statement not yet translated:
        //
        //
        //     // Only erases when NOT in automap and the screen is reduced,
        //     // and the text must either need updating or refreshing
        //     // (because of a recent change back from the automap)
        //
        //     if (!automapactive &&
        // 	viewwindowx && l->needsupdate)
        //     {
        // 	lh = SHORT(l->f[0]->height) + 1;
        // 	for (y=l->y,yoffset=y*SCREENWIDTH ; y<l->y+lh ; y++,yoffset+=SCREENWIDTH)
        // 	{
        // 	    if (y < viewwindowy || y >= viewwindowy + viewheight)
        // 		R_VideoErase(yoffset, SCREENWIDTH); // erase entire line
        // 	    else
        // 	    {
        // 		R_VideoErase(yoffset, viewwindowx); // erase left border
        // 		R_VideoErase(yoffset + viewwindowx + viewwidth, viewwindowx);
        // 		// erase right border
        // 	    }
        // 	}
        //     }
        todo!("if statement not yet translated");
        lastautomapactive = automapactive;
        // TODO: if statement not yet translated:
        //
        //     if (l->needsupdate) l->needsupdate--;
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn HUlib_initSText(
    mut s: *mut hu_stext_t,
    mut x: std::ffi::c_int,
    mut y: std::ffi::c_int,
    mut h: std::ffi::c_int,
    mut font: *mut *mut patch_t,
    mut startchar: std::ffi::c_int,
    mut on: *mut boolean,
) {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        (*s).h = h;
        (*s).on = on;
        (*s).laston = true_;
        (*s).cl = 0;
        // TODO: for statement not yet translated:
        //
        //     for (i=0;i<h;i++)
        // 	HUlib_initTextLine(&s->l[i],
        // 			   x, y - i*(SHORT(font[0]->height)+1),
        // 			   font, startchar);
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn HUlib_addLineToSText(mut s: *mut hu_stext_t) {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     // add a clear line
        //     if (++s->cl == s->h)
        // 	s->cl = 0;
        todo!("if statement not yet translated");
        HUlib_clearTextLine((&((*s).l[((*s).cl) as usize]) as *const _ as *mut _));
        // TODO: for statement not yet translated:
        //
        //
        //     // everything needs updating
        //     for (i=0 ; i<s->h ; i++)
        // 	s->l[i].needsupdate = 4;
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn HUlib_addMessageToSText(
    mut s: *mut hu_stext_t,
    mut prefix: *mut std::ffi::c_char,
    mut msg: *mut std::ffi::c_char,
) {
    unsafe {
        HUlib_addLineToSText(s);
        // TODO: if statement not yet translated:
        //
        //     if (prefix)
        // 	while (*prefix)
        // 	    HUlib_addCharToTextLine(&s->l[s->cl], *(prefix++));
        todo!("if statement not yet translated");
        // TODO: while statement not yet translated:
        //
        //
        //     while (*msg)
        // 	HUlib_addCharToTextLine(&s->l[s->cl], *(msg++));
        todo!("while statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn HUlib_drawSText(mut s: *mut hu_stext_t) {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut idx: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut l: *mut hu_textline_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (!*s->on)
        // 	return; // if not on, don't draw
        todo!("if statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //     // draw everything
        //     for (i=0 ; i<s->h ; i++)
        //     {
        // 	idx = s->cl - i;
        // 	if (idx < 0)
        // 	    idx += s->h; // handle queue of lines
        //
        // 	l = &s->l[idx];
        //
        // 	// need a decision made here on whether to skip the draw
        // 	HUlib_drawTextLine(l, false); // no cursor, please
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn HUlib_eraseSText(mut s: *mut hu_stext_t) {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<s->h ; i++)
        //     {
        // 	if (s->laston && !*s->on)
        // 	    s->l[i].needsupdate = 4;
        // 	HUlib_eraseTextLine(&s->l[i]);
        //     }
        todo!("for statement not yet translated");
        (*s).laston = (*((*s).on));
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn HUlib_initIText(
    mut it: *mut hu_itext_t,
    mut x: std::ffi::c_int,
    mut y: std::ffi::c_int,
    mut font: *mut *mut patch_t,
    mut startchar: std::ffi::c_int,
    mut on: *mut boolean,
) {
    unsafe {
        (*it).lm = 0;
        (*it).on = on;
        (*it).laston = true_;
        HUlib_initTextLine((&((*it).l) as *const _ as *mut _), x, y, font, startchar);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn HUlib_delCharFromIText(mut it: *mut hu_itext_t) {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (it->l.len != it->lm)
        // 	HUlib_delCharFromTextLine(&it->l);
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn HUlib_eraseLineFromIText(mut it: *mut hu_itext_t) {
    unsafe {
        // TODO: while statement not yet translated:
        //
        //     while (it->lm != it->l.len)
        // 	HUlib_delCharFromTextLine(&it->l);
        todo!("while statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn HUlib_resetIText(mut it: *mut hu_itext_t) {
    unsafe {
        (*it).lm = 0;
        HUlib_clearTextLine((&((*it).l) as *const _ as *mut _));
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn HUlib_addPrefixToIText(
    mut it: *mut hu_itext_t,
    mut str: *mut std::ffi::c_char,
) {
    unsafe {
        // TODO: while statement not yet translated:
        //
        //     while (*str)
        // 	HUlib_addCharToTextLine(&it->l, *(str++));
        todo!("while statement not yet translated");
        (*it).lm = (*it).l.len;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn HUlib_keyInIText(
    mut it: *mut hu_itext_t,
    mut ch: std::ffi::c_uchar,
) -> boolean {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //
        //     if (ch >= ' ' && ch <= '_')
        //   	HUlib_addCharToTextLine(&it->l, (char) ch);
        //     else
        // 	if (ch == KEY_BACKSPACE)
        // 	    HUlib_delCharFromIText(it);
        // 	else
        // 	    if (ch != KEY_ENTER)
        // 		return false; // did not eat key
        todo!("if statement not yet translated");
        return true_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn HUlib_drawIText(mut it: *mut hu_itext_t) {
    unsafe {
        let mut l: *mut hu_textline_t = unsafe { (&((*it).l) as *const _ as *mut _) };
        // TODO: if statement not yet translated:
        //
        //
        //     if (!*it->on)
        // 	return;
        todo!("if statement not yet translated");
        HUlib_drawTextLine(l, true_);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn HUlib_eraseIText(mut it: *mut hu_itext_t) {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (it->laston && !*it->on)
        // 	it->l.needsupdate = 4;
        todo!("if statement not yet translated");
        HUlib_eraseTextLine((&((*it).l) as *const _ as *mut _));
        (*it).laston = (*((*it).on));
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}
