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
use crate::g_game::*;
use crate::i_system::*;
use crate::info::*;
use crate::m_fixed::*;
use crate::m_random::*;
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
use crate::s_sound::*;
use crate::sounds::*;
use crate::tables::*;
use crate::v_video::*;
use crate::w_wad::*;
use crate::z_zone::*;

pub const NoState: std::ffi::c_int = -1;
pub const StatCount: std::ffi::c_int = NoState + 1;
pub const ShowNextLoc: std::ffi::c_int = StatCount + 1;

pub type stateenum_t = std::ffi::c_int;

static mut rcsid: [std::ffi::c_char; 51] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        119 as std::ffi::c_char,
        105 as std::ffi::c_char,
        95 as std::ffi::c_char,
        115 as std::ffi::c_char,
        116 as std::ffi::c_char,
        117 as std::ffi::c_char,
        102 as std::ffi::c_char,
        102 as std::ffi::c_char,
        46 as std::ffi::c_char,
        99 as std::ffi::c_char,
        44 as std::ffi::c_char,
        118 as std::ffi::c_char,
        32 as std::ffi::c_char,
        49 as std::ffi::c_char,
        46 as std::ffi::c_char,
        55 as std::ffi::c_char,
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

pub const NUMEPISODES: std::ffi::c_int = 4;

pub const NUMMAPS: std::ffi::c_int = 9;

pub const WI_TITLEY: std::ffi::c_int = 2;

pub const WI_SPACINGY: std::ffi::c_int = 33;

pub const SP_STATSX: std::ffi::c_int = 50;

pub const SP_STATSY: std::ffi::c_int = 50;

pub const SP_TIMEX: std::ffi::c_int = 16;

pub const SP_TIMEY: std::ffi::c_int = (SCREENHEIGHT - 32);

pub const NG_STATSY: std::ffi::c_int = 50;

pub const NG_STATSX: std::ffi::c_int =
    ((32 + (SHORT((*star).width) / 2)) + (32 * (((dofrags) == 0) as std::ffi::c_int)));

pub const NG_SPACINGX: std::ffi::c_int = 64;

pub const DM_MATRIXX: std::ffi::c_int = 42;

pub const DM_MATRIXY: std::ffi::c_int = 68;

pub const DM_SPACINGX: std::ffi::c_int = 40;

pub const DM_TOTALSX: std::ffi::c_int = 269;

pub const DM_KILLERSX: std::ffi::c_int = 10;

pub const DM_KILLERSY: std::ffi::c_int = 100;

pub const DM_VICTIMSX: std::ffi::c_int = 5;

pub const DM_VICTIMSY: std::ffi::c_int = 50;

pub const ANIM_ALWAYS: std::ffi::c_int = 0;
pub const ANIM_RANDOM: std::ffi::c_int = ANIM_ALWAYS + 1;
pub const ANIM_LEVEL: std::ffi::c_int = ANIM_RANDOM + 1;

pub type animenum_t = std::ffi::c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct point_t {
    pub x: std::ffi::c_int,
    pub y: std::ffi::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct anim_t {
    pub type_: animenum_t,
    pub period: std::ffi::c_int,
    pub nanims: std::ffi::c_int,
    pub loc: point_t,
    pub data1: std::ffi::c_int,
    pub data2: std::ffi::c_int,
    pub p: [*mut patch_t; (3) as usize],
    pub nexttic: std::ffi::c_int,
    pub lastdrawn: std::ffi::c_int,
    pub ctr: std::ffi::c_int,
    pub state: std::ffi::c_int,
}

static mut lnodes: [[point_t; (NUMMAPS) as usize]; (NUMEPISODES) as usize] = unsafe {
    [
        [
            point_t { x: 185, y: 164 },
            point_t { x: 148, y: 143 },
            point_t { x: 69, y: 122 },
            point_t { x: 209, y: 102 },
            point_t { x: 116, y: 89 },
            point_t { x: 166, y: 55 },
            point_t { x: 71, y: 56 },
            point_t { x: 135, y: 29 },
            point_t { x: 71, y: 24 },
        ],
        [
            point_t { x: 254, y: 25 },
            point_t { x: 97, y: 50 },
            point_t { x: 188, y: 64 },
            point_t { x: 128, y: 78 },
            point_t { x: 214, y: 92 },
            point_t { x: 133, y: 130 },
            point_t { x: 208, y: 136 },
            point_t { x: 148, y: 140 },
            point_t { x: 235, y: 158 },
        ],
        [
            point_t { x: 156, y: 168 },
            point_t { x: 48, y: 154 },
            point_t { x: 174, y: 95 },
            point_t { x: 265, y: 75 },
            point_t { x: 130, y: 48 },
            point_t { x: 279, y: 23 },
            point_t { x: 198, y: 48 },
            point_t { x: 140, y: 25 },
            point_t { x: 281, y: 136 },
        ],
    ]
};

static mut epsd0animinfo: [anim_t; 10] = unsafe {
    [
        anim_t {
            type_: ANIM_ALWAYS,
            period: (TICRATE / 3),
            nanims: 3,
            loc: point_t { x: 224, y: 104 },
            ..ZEROED_anim_t
        },
        anim_t {
            type_: ANIM_ALWAYS,
            period: (TICRATE / 3),
            nanims: 3,
            loc: point_t { x: 184, y: 160 },
            ..ZEROED_anim_t
        },
        anim_t {
            type_: ANIM_ALWAYS,
            period: (TICRATE / 3),
            nanims: 3,
            loc: point_t { x: 112, y: 136 },
            ..ZEROED_anim_t
        },
        anim_t {
            type_: ANIM_ALWAYS,
            period: (TICRATE / 3),
            nanims: 3,
            loc: point_t { x: 72, y: 112 },
            ..ZEROED_anim_t
        },
        anim_t {
            type_: ANIM_ALWAYS,
            period: (TICRATE / 3),
            nanims: 3,
            loc: point_t { x: 88, y: 96 },
            ..ZEROED_anim_t
        },
        anim_t {
            type_: ANIM_ALWAYS,
            period: (TICRATE / 3),
            nanims: 3,
            loc: point_t { x: 64, y: 48 },
            ..ZEROED_anim_t
        },
        anim_t {
            type_: ANIM_ALWAYS,
            period: (TICRATE / 3),
            nanims: 3,
            loc: point_t { x: 192, y: 40 },
            ..ZEROED_anim_t
        },
        anim_t {
            type_: ANIM_ALWAYS,
            period: (TICRATE / 3),
            nanims: 3,
            loc: point_t { x: 136, y: 16 },
            ..ZEROED_anim_t
        },
        anim_t {
            type_: ANIM_ALWAYS,
            period: (TICRATE / 3),
            nanims: 3,
            loc: point_t { x: 80, y: 16 },
            ..ZEROED_anim_t
        },
        anim_t {
            type_: ANIM_ALWAYS,
            period: (TICRATE / 3),
            nanims: 3,
            loc: point_t { x: 64, y: 24 },
            ..ZEROED_anim_t
        },
    ]
};

static mut epsd1animinfo: [anim_t; 9] = unsafe {
    [
        anim_t {
            type_: ANIM_LEVEL,
            period: (TICRATE / 3),
            nanims: 1,
            loc: point_t { x: 128, y: 136 },
            data1: 1,
            ..ZEROED_anim_t
        },
        anim_t {
            type_: ANIM_LEVEL,
            period: (TICRATE / 3),
            nanims: 1,
            loc: point_t { x: 128, y: 136 },
            data1: 2,
            ..ZEROED_anim_t
        },
        anim_t {
            type_: ANIM_LEVEL,
            period: (TICRATE / 3),
            nanims: 1,
            loc: point_t { x: 128, y: 136 },
            data1: 3,
            ..ZEROED_anim_t
        },
        anim_t {
            type_: ANIM_LEVEL,
            period: (TICRATE / 3),
            nanims: 1,
            loc: point_t { x: 128, y: 136 },
            data1: 4,
            ..ZEROED_anim_t
        },
        anim_t {
            type_: ANIM_LEVEL,
            period: (TICRATE / 3),
            nanims: 1,
            loc: point_t { x: 128, y: 136 },
            data1: 5,
            ..ZEROED_anim_t
        },
        anim_t {
            type_: ANIM_LEVEL,
            period: (TICRATE / 3),
            nanims: 1,
            loc: point_t { x: 128, y: 136 },
            data1: 6,
            ..ZEROED_anim_t
        },
        anim_t {
            type_: ANIM_LEVEL,
            period: (TICRATE / 3),
            nanims: 1,
            loc: point_t { x: 128, y: 136 },
            data1: 7,
            ..ZEROED_anim_t
        },
        anim_t {
            type_: ANIM_LEVEL,
            period: (TICRATE / 3),
            nanims: 3,
            loc: point_t { x: 192, y: 144 },
            data1: 8,
            ..ZEROED_anim_t
        },
        anim_t {
            type_: ANIM_LEVEL,
            period: (TICRATE / 3),
            nanims: 1,
            loc: point_t { x: 128, y: 136 },
            data1: 8,
            ..ZEROED_anim_t
        },
    ]
};

static mut epsd2animinfo: [anim_t; 6] = unsafe {
    [
        anim_t {
            type_: ANIM_ALWAYS,
            period: (TICRATE / 3),
            nanims: 3,
            loc: point_t { x: 104, y: 168 },
            ..ZEROED_anim_t
        },
        anim_t {
            type_: ANIM_ALWAYS,
            period: (TICRATE / 3),
            nanims: 3,
            loc: point_t { x: 40, y: 136 },
            ..ZEROED_anim_t
        },
        anim_t {
            type_: ANIM_ALWAYS,
            period: (TICRATE / 3),
            nanims: 3,
            loc: point_t { x: 160, y: 96 },
            ..ZEROED_anim_t
        },
        anim_t {
            type_: ANIM_ALWAYS,
            period: (TICRATE / 3),
            nanims: 3,
            loc: point_t { x: 104, y: 80 },
            ..ZEROED_anim_t
        },
        anim_t {
            type_: ANIM_ALWAYS,
            period: (TICRATE / 3),
            nanims: 3,
            loc: point_t { x: 120, y: 32 },
            ..ZEROED_anim_t
        },
        anim_t {
            type_: ANIM_ALWAYS,
            period: (TICRATE / 4),
            nanims: 3,
            loc: point_t { x: 40, y: 0 },
            ..ZEROED_anim_t
        },
    ]
};

static mut NUMANIMS: [std::ffi::c_int; (NUMEPISODES) as usize] = unsafe {
    [
        (std::mem::size_of_val(&(epsd0animinfo)) / std::mem::size_of::<anim_t>())
            as std::ffi::c_int,
        (std::mem::size_of_val(&(epsd1animinfo)) / std::mem::size_of::<anim_t>())
            as std::ffi::c_int,
        (std::mem::size_of_val(&(epsd2animinfo)) / std::mem::size_of::<anim_t>())
            as std::ffi::c_int,
    ]
};

static mut anims: [*mut anim_t; (NUMEPISODES) as usize] = unsafe {
    [
        epsd0animinfo.as_mut_ptr(),
        epsd1animinfo.as_mut_ptr(),
        epsd2animinfo.as_mut_ptr(),
    ]
};

pub const FB: std::ffi::c_int = 0;

pub const SP_KILLS: std::ffi::c_int = 0;

pub const SP_ITEMS: std::ffi::c_int = 2;

pub const SP_SECRET: std::ffi::c_int = 4;

pub const SP_FRAGS: std::ffi::c_int = 6;

pub const SP_TIME: std::ffi::c_int = 8;

/* TODO: unparsed macro value, references an identifier with no known definition anywhere in this module's visible corpus (likely dead code never expanded in the original C):
#define SP_PAR ST_TIME
*/

pub const SP_PAUSE: std::ffi::c_int = 1;

pub const SHOWNEXTLOCDELAY: std::ffi::c_int = 4;

static mut acceleratestage: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut me: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut state: stateenum_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut wbs: *mut wbstartstruct_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut plrs: *mut wbplayerstruct_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut cnt: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut bcnt: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut firstrefresh: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut cnt_kills: [std::ffi::c_int; (MAXPLAYERS) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut cnt_items: [std::ffi::c_int; (MAXPLAYERS) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut cnt_secret: [std::ffi::c_int; (MAXPLAYERS) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut cnt_time: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut cnt_par: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut cnt_pause: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut NUMCMAPS: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut bg: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut yah: [*mut patch_t; (2) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut splat: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut percent: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut colon: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut num: [*mut patch_t; (10) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut wiminus: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut finished: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut entering: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut sp_secret: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut kills: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut secret: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut items: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut frags: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut time: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut par: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut sucks: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut killers: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut victims: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut total: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut star: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut bstar: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut p: [*mut patch_t; (MAXPLAYERS) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut bp: [*mut patch_t; (MAXPLAYERS) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut lnames: *mut *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn WI_slamBackground() {
    unsafe {
        memcpy(
            screens[(0) as usize],
            screens[(1) as usize],
            (SCREENWIDTH * SCREENHEIGHT),
        );
        V_MarkRect(0, 0, SCREENWIDTH, SCREENHEIGHT);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn WI_Responder(mut ev: *mut event_t) -> boolean {
    unsafe {
        return false_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn WI_drawLF() {
    unsafe {
        let mut y: std::ffi::c_int = unsafe { WI_TITLEY };
        V_DrawPatch(
            ((SCREENWIDTH - SHORT((*lnames[((*wbs).last) as usize]).width)) / 2),
            y,
            FB,
            lnames[((*wbs).last) as usize],
        );
        y += ((5 * SHORT((*lnames[((*wbs).last) as usize]).height)) / 4);
        V_DrawPatch(
            ((SCREENWIDTH - SHORT((*finished).width)) / 2),
            y,
            FB,
            finished,
        );
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn WI_drawEL() {
    unsafe {
        let mut y: std::ffi::c_int = unsafe { WI_TITLEY };
        V_DrawPatch(
            ((SCREENWIDTH - SHORT((*entering).width)) / 2),
            y,
            FB,
            entering,
        );
        y += ((5 * SHORT((*lnames[((*wbs).next) as usize]).height)) / 4);
        V_DrawPatch(
            ((SCREENWIDTH - SHORT((*lnames[((*wbs).next) as usize]).width)) / 2),
            y,
            FB,
            lnames[((*wbs).next) as usize],
        );
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn WI_drawOnLnode(
    mut n: std::ffi::c_int,
    mut c: *mut *mut patch_t, /* TODO: was unsized array */
) {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut left: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut top: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut right: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut bottom: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut fits: boolean = unsafe { false_ };
        i = 0;
        // TODO: do-while statement not yet translated:
        //
        //     do
        //     {
        // 	left = lnodes[wbs->epsd][n].x - SHORT(c[i]->leftoffset);
        // 	top = lnodes[wbs->epsd][n].y - SHORT(c[i]->topoffset);
        // 	right = left + SHORT(c[i]->width);
        // 	bottom = top + SHORT(c[i]->height);
        //
        // 	if (left >= 0
        // 	    && right < SCREENWIDTH
        // 	    && top >= 0
        // 	    && bottom < SCREENHEIGHT)
        // 	{
        // 	    fits = true;
        // 	}
        // 	else
        // 	{
        // 	    i++;
        // 	}
        //     } while (!fits && i!=2);
        todo!("do-while statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (fits && i<2)
        //     {
        // 	V_DrawPatch(lnodes[wbs->epsd][n].x, lnodes[wbs->epsd][n].y,
        // 		    FB, c[i]);
        //     }
        //     else
        //     {
        // 	// DEBUG
        // 	printf("Could not place patch on level %d", n+1);
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn WI_initAnimatedBack() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut a: *mut anim_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (gamemode == commercial)
        // 	return;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (wbs->epsd > 2)
        // 	return;
        todo!("if statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0;i<NUMANIMS[wbs->epsd];i++)
        //     {
        // 	a = &anims[wbs->epsd][i];
        //
        // 	// init variables
        // 	a->ctr = -1;
        //
        // 	// specify the next time to draw it
        // 	if (a->type == ANIM_ALWAYS)
        // 	    a->nexttic = bcnt + 1 + (M_Random()%a->period);
        // 	else if (a->type == ANIM_RANDOM)
        // 	    a->nexttic = bcnt + 1 + a->data2+(M_Random()%a->data1);
        // 	else if (a->type == ANIM_LEVEL)
        // 	    a->nexttic = bcnt + 1;
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn WI_updateAnimatedBack() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut a: *mut anim_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (gamemode == commercial)
        // 	return;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (wbs->epsd > 2)
        // 	return;
        todo!("if statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0;i<NUMANIMS[wbs->epsd];i++)
        //     {
        // 	a = &anims[wbs->epsd][i];
        //
        // 	if (bcnt == a->nexttic)
        // 	{
        // 	    switch (a->type)
        // 	    {
        // 	      case ANIM_ALWAYS:
        // 		if (++a->ctr >= a->nanims) a->ctr = 0;
        // 		a->nexttic = bcnt + a->period;
        // 		break;
        //
        // 	      case ANIM_RANDOM:
        // 		a->ctr++;
        // 		if (a->ctr == a->nanims)
        // 		{
        // 		    a->ctr = -1;
        // 		    a->nexttic = bcnt+a->data2+(M_Random()%a->data1);
        // 		}
        // 		else a->nexttic = bcnt + a->period;
        // 		break;
        //
        // 	      case ANIM_LEVEL:
        // 		// gawd-awful hack for level anims
        // 		if (!(state == StatCount && i == 7)
        // 		    && wbs->next == a->data1)
        // 		{
        // 		    a->ctr++;
        // 		    if (a->ctr == a->nanims) a->ctr--;
        // 		    a->nexttic = bcnt + a->period;
        // 		}
        // 		break;
        // 	    }
        // 	}
        //
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn WI_drawAnimatedBack() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut a: *mut anim_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (commercial)
        // 	return;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (wbs->epsd > 2)
        // 	return;
        todo!("if statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<NUMANIMS[wbs->epsd] ; i++)
        //     {
        // 	a = &anims[wbs->epsd][i];
        //
        // 	if (a->ctr >= 0)
        // 	    V_DrawPatch(a->loc.x, a->loc.y, FB, a->p[a->ctr]);
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn WI_drawNum(
    mut x: std::ffi::c_int,
    mut y: std::ffi::c_int,
    mut n: std::ffi::c_int,
    mut digits: std::ffi::c_int,
) -> std::ffi::c_int {
    unsafe {
        let mut fontwidth: std::ffi::c_int = unsafe { SHORT((*num[(0) as usize]).width) };
        let mut neg: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut temp: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (digits < 0)
        //     {
        // 	if (!n)
        // 	{
        // 	    // make variable-length zeros 1 digit long
        // 	    digits = 1;
        // 	}
        // 	else
        // 	{
        // 	    // figure out # of digits in #
        // 	    digits = 0;
        // 	    temp = n;
        //
        // 	    while (temp)
        // 	    {
        // 		temp /= 10;
        // 		digits++;
        // 	    }
        // 	}
        //     }
        todo!("if statement not yet translated");
        neg = (n < 0);
        // TODO: if statement not yet translated:
        //
        //     if (neg)
        // 	n = -n;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // if non-number, do not draw it
        //     if (n == 1994)
        // 	return 0;
        todo!("if statement not yet translated");
        // TODO: while statement not yet translated:
        //
        //
        //     // draw the new number
        //     while (digits--)
        //     {
        // 	x -= fontwidth;
        // 	V_DrawPatch(x, y, FB, num[ n % 10 ]);
        // 	n /= 10;
        //     }
        todo!("while statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // draw a minus sign if necessary
        //     if (neg)
        // 	V_DrawPatch(x-=8, y, FB, wiminus);
        todo!("if statement not yet translated");
        return x;
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn WI_drawPercent(
    mut x: std::ffi::c_int,
    mut y: std::ffi::c_int,
    mut p: std::ffi::c_int,
) {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (p < 0)
        // 	return;
        todo!("if statement not yet translated");
        V_DrawPatch(x, y, FB, percent);
        WI_drawNum(x, y, p, (-(1)));
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn WI_drawTime(
    mut x: std::ffi::c_int,
    mut y: std::ffi::c_int,
    mut t: std::ffi::c_int,
) {
    unsafe {
        let mut div: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut n: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (t<0)
        // 	return;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (t <= 61*59)
        //     {
        // 	div = 1;
        //
        // 	do
        // 	{
        // 	    n = (t / div) % 60;
        // 	    x = WI_drawNum(x, y, n, 2) - SHORT(colon->width);
        // 	    div *= 60;
        //
        // 	    // draw
        // 	    if (div==60 || t / div)
        // 		V_DrawPatch(x, y, FB, colon);
        //
        // 	} while (t / div);
        //     }
        //     else
        //     {
        // 	// "sucks"
        // 	V_DrawPatch(x - SHORT(sucks->width), y, FB, sucks);
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn WI_End() {
    unsafe {
        // TODO: statement not yet translated:
        //
        //     void WI_unloadData(void);
        todo!("statement not yet translated");
        WI_unloadData();
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn WI_initNoState() {
    unsafe {
        state = NoState;
        acceleratestage = 0;
        cnt = 10;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn WI_updateNoState() {
    unsafe {
        WI_updateAnimatedBack();
        // TODO: if statement not yet translated:
        //
        //
        //     if (!--cnt)
        //     {
        // 	WI_End();
        // 	G_WorldDone();
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

static mut snl_pointeron: boolean = unsafe { false_ };

pub unsafe extern "C" fn WI_initShowNextLoc() {
    unsafe {
        state = ShowNextLoc;
        acceleratestage = 0;
        cnt = (SHOWNEXTLOCDELAY * TICRATE);
        WI_initAnimatedBack();
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn WI_updateShowNextLoc() {
    unsafe {
        WI_updateAnimatedBack();
        // TODO: if statement not yet translated:
        //
        //
        //     if (!--cnt || acceleratestage)
        // 	WI_initNoState();
        //     else
        // 	snl_pointeron = (cnt & 31) < 20;
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn WI_drawShowNextLoc() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut last: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        WI_slamBackground();
        WI_drawAnimatedBack();
        // TODO: if statement not yet translated:
        //
        //
        //     if ( gamemode != commercial)
        //     {
        //   	if (wbs->epsd > 2)
        // 	{
        // 	    WI_drawEL();
        // 	    return;
        // 	}
        //
        // 	last = (wbs->last == 8) ? wbs->next - 1 : wbs->last;
        //
        // 	// draw a splat on taken cities.
        // 	for (i=0 ; i<=last ; i++)
        // 	    WI_drawOnLnode(i, &splat);
        //
        // 	// splat the secret level?
        // 	if (wbs->didsecret)
        // 	    WI_drawOnLnode(8, &splat);
        //
        // 	// draw flashing ptr
        // 	if (snl_pointeron)
        // 	    WI_drawOnLnode(wbs->next, yah);
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // draws which level you are entering..
        //     if ( (gamemode != commercial)
        // 	 || wbs->next != 30)
        // 	WI_drawEL();
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn WI_drawNoState() {
    unsafe {
        snl_pointeron = true_;
        WI_drawShowNextLoc();
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn WI_fragSum(mut playernum: std::ffi::c_int) -> std::ffi::c_int {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut frags: std::ffi::c_int = unsafe { 0 };
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<MAXPLAYERS ; i++)
        //     {
        // 	if (playeringame[i]
        // 	    && i!=playernum)
        // 	{
        // 	    frags += plrs[playernum].frags[i];
        // 	}
        //     }
        todo!("for statement not yet translated");
        frags -= plrs[(playernum) as usize].frags[(playernum) as usize];
        return frags;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

static mut dm_state: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut dm_frags: [[std::ffi::c_int; (MAXPLAYERS) as usize]; (MAXPLAYERS) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut dm_totals: [std::ffi::c_int; (MAXPLAYERS) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn WI_initDeathmatchStats() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut j: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        state = StatCount;
        acceleratestage = 0;
        dm_state = 1;
        cnt_pause = TICRATE;
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<MAXPLAYERS ; i++)
        //     {
        // 	if (playeringame[i])
        // 	{
        // 	    for (j=0 ; j<MAXPLAYERS ; j++)
        // 		if (playeringame[j])
        // 		    dm_frags[i][j] = 0;
        //
        // 	    dm_totals[i] = 0;
        // 	}
        //     }
        todo!("for statement not yet translated");
        WI_initAnimatedBack();
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn WI_updateDeathmatchStats() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut j: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut stillticking: boolean = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        WI_updateAnimatedBack();
        // TODO: if statement not yet translated:
        //
        //
        //     if (acceleratestage && dm_state != 4)
        //     {
        // 	acceleratestage = 0;
        //
        // 	for (i=0 ; i<MAXPLAYERS ; i++)
        // 	{
        // 	    if (playeringame[i])
        // 	    {
        // 		for (j=0 ; j<MAXPLAYERS ; j++)
        // 		    if (playeringame[j])
        // 			dm_frags[i][j] = plrs[i].frags[j];
        //
        // 		dm_totals[i] = WI_fragSum(i);
        // 	    }
        // 	}
        //
        //
        // 	S_StartSound(0, sfx_barexp);
        // 	dm_state = 4;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //
        //     if (dm_state == 2)
        //     {
        // 	if (!(bcnt&3))
        // 	    S_StartSound(0, sfx_pistol);
        //
        // 	stillticking = false;
        //
        // 	for (i=0 ; i<MAXPLAYERS ; i++)
        // 	{
        // 	    if (playeringame[i])
        // 	    {
        // 		for (j=0 ; j<MAXPLAYERS ; j++)
        // 		{
        // 		    if (playeringame[j]
        // 			&& dm_frags[i][j] != plrs[i].frags[j])
        // 		    {
        // 			if (plrs[i].frags[j] < 0)
        // 			    dm_frags[i][j]--;
        // 			else
        // 			    dm_frags[i][j]++;
        //
        // 			if (dm_frags[i][j] > 99)
        // 			    dm_frags[i][j] = 99;
        //
        // 			if (dm_frags[i][j] < -99)
        // 			    dm_frags[i][j] = -99;
        //
        // 			stillticking = true;
        // 		    }
        // 		}
        // 		dm_totals[i] = WI_fragSum(i);
        //
        // 		if (dm_totals[i] > 99)
        // 		    dm_totals[i] = 99;
        //
        // 		if (dm_totals[i] < -99)
        // 		    dm_totals[i] = -99;
        // 	    }
        //
        // 	}
        // 	if (!stillticking)
        // 	{
        // 	    S_StartSound(0, sfx_barexp);
        // 	    dm_state++;
        // 	}
        //
        //     }
        //     else if (dm_state == 4)
        //     {
        // 	if (acceleratestage)
        // 	{
        // 	    S_StartSound(0, sfx_slop);
        //
        // 	    if ( gamemode == commercial)
        // 		WI_initNoState();
        // 	    else
        // 		WI_initShowNextLoc();
        // 	}
        //     }
        //     else if (dm_state & 1)
        //     {
        // 	if (!--cnt_pause)
        // 	{
        // 	    dm_state++;
        // 	    cnt_pause = TICRATE;
        // 	}
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn WI_drawDeathmatchStats() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut j: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut x: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut y: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut w: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut lh: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        lh = WI_SPACINGY;
        WI_slamBackground();
        WI_drawAnimatedBack();
        WI_drawLF();
        V_DrawPatch(
            (DM_TOTALSX - (SHORT((*total).width) / 2)),
            ((DM_MATRIXY - WI_SPACINGY) + 10),
            FB,
            total,
        );
        V_DrawPatch(DM_KILLERSX, DM_KILLERSY, FB, killers);
        V_DrawPatch(DM_VICTIMSX, DM_VICTIMSY, FB, victims);
        x = (DM_MATRIXX + DM_SPACINGX);
        y = DM_MATRIXY;
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<MAXPLAYERS ; i++)
        //     {
        // 	if (playeringame[i])
        // 	{
        // 	    V_DrawPatch(x-SHORT(p[i]->width)/2,
        // 			DM_MATRIXY - WI_SPACINGY,
        // 			FB,
        // 			p[i]);
        //
        // 	    V_DrawPatch(DM_MATRIXX-SHORT(p[i]->width)/2,
        // 			y,
        // 			FB,
        // 			p[i]);
        //
        // 	    if (i == me)
        // 	    {
        // 		V_DrawPatch(x-SHORT(p[i]->width)/2,
        // 			    DM_MATRIXY - WI_SPACINGY,
        // 			    FB,
        // 			    bstar);
        //
        // 		V_DrawPatch(DM_MATRIXX-SHORT(p[i]->width)/2,
        // 			    y,
        // 			    FB,
        // 			    star);
        // 	    }
        // 	}
        // 	else
        // 	{
        // 	    // V_DrawPatch(x-SHORT(bp[i]->width)/2,
        // 	    //   DM_MATRIXY - WI_SPACINGY, FB, bp[i]);
        // 	    // V_DrawPatch(DM_MATRIXX-SHORT(bp[i]->width)/2,
        // 	    //   y, FB, bp[i]);
        // 	}
        // 	x += DM_SPACINGX;
        // 	y += WI_SPACINGY;
        //     }
        todo!("for statement not yet translated");
        y = (DM_MATRIXY + 10);
        w = SHORT((*num[(0) as usize]).width);
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<MAXPLAYERS ; i++)
        //     {
        // 	x = DM_MATRIXX + DM_SPACINGX;
        //
        // 	if (playeringame[i])
        // 	{
        // 	    for (j=0 ; j<MAXPLAYERS ; j++)
        // 	    {
        // 		if (playeringame[j])
        // 		    WI_drawNum(x+w, y, dm_frags[i][j], 2);
        //
        // 		x += DM_SPACINGX;
        // 	    }
        // 	    WI_drawNum(DM_TOTALSX+w, y, dm_totals[i], 2);
        // 	}
        // 	y += WI_SPACINGY;
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

static mut cnt_frags: [std::ffi::c_int; (MAXPLAYERS) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut dofrags: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut ng_state: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn WI_initNetgameStats() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        state = StatCount;
        acceleratestage = 0;
        ng_state = 1;
        cnt_pause = TICRATE;
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<MAXPLAYERS ; i++)
        //     {
        // 	if (!playeringame[i])
        // 	    continue;
        //
        // 	cnt_kills[i] = cnt_items[i] = cnt_secret[i] = cnt_frags[i] = 0;
        //
        // 	dofrags += WI_fragSum(i);
        //     }
        todo!("for statement not yet translated");
        dofrags = (((((dofrags) == 0) as std::ffi::c_int) == 0) as std::ffi::c_int);
        WI_initAnimatedBack();
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn WI_updateNetgameStats() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut fsum: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut stillticking: boolean = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        WI_updateAnimatedBack();
        // TODO: if statement not yet translated:
        //
        //
        //     if (acceleratestage && ng_state != 10)
        //     {
        // 	acceleratestage = 0;
        //
        // 	for (i=0 ; i<MAXPLAYERS ; i++)
        // 	{
        // 	    if (!playeringame[i])
        // 		continue;
        //
        // 	    cnt_kills[i] = (plrs[i].skills * 100) / wbs->maxkills;
        // 	    cnt_items[i] = (plrs[i].sitems * 100) / wbs->maxitems;
        // 	    cnt_secret[i] = (plrs[i].ssecret * 100) / wbs->maxsecret;
        //
        // 	    if (dofrags)
        // 		cnt_frags[i] = WI_fragSum(i);
        // 	}
        // 	S_StartSound(0, sfx_barexp);
        // 	ng_state = 10;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (ng_state == 2)
        //     {
        // 	if (!(bcnt&3))
        // 	    S_StartSound(0, sfx_pistol);
        //
        // 	stillticking = false;
        //
        // 	for (i=0 ; i<MAXPLAYERS ; i++)
        // 	{
        // 	    if (!playeringame[i])
        // 		continue;
        //
        // 	    cnt_kills[i] += 2;
        //
        // 	    if (cnt_kills[i] >= (plrs[i].skills * 100) / wbs->maxkills)
        // 		cnt_kills[i] = (plrs[i].skills * 100) / wbs->maxkills;
        // 	    else
        // 		stillticking = true;
        // 	}
        //
        // 	if (!stillticking)
        // 	{
        // 	    S_StartSound(0, sfx_barexp);
        // 	    ng_state++;
        // 	}
        //     }
        //     else if (ng_state == 4)
        //     {
        // 	if (!(bcnt&3))
        // 	    S_StartSound(0, sfx_pistol);
        //
        // 	stillticking = false;
        //
        // 	for (i=0 ; i<MAXPLAYERS ; i++)
        // 	{
        // 	    if (!playeringame[i])
        // 		continue;
        //
        // 	    cnt_items[i] += 2;
        // 	    if (cnt_items[i] >= (plrs[i].sitems * 100) / wbs->maxitems)
        // 		cnt_items[i] = (plrs[i].sitems * 100) / wbs->maxitems;
        // 	    else
        // 		stillticking = true;
        // 	}
        // 	if (!stillticking)
        // 	{
        // 	    S_StartSound(0, sfx_barexp);
        // 	    ng_state++;
        // 	}
        //     }
        //     else if (ng_state == 6)
        //     {
        // 	if (!(bcnt&3))
        // 	    S_StartSound(0, sfx_pistol);
        //
        // 	stillticking = false;
        //
        // 	for (i=0 ; i<MAXPLAYERS ; i++)
        // 	{
        // 	    if (!playeringame[i])
        // 		continue;
        //
        // 	    cnt_secret[i] += 2;
        //
        // 	    if (cnt_secret[i] >= (plrs[i].ssecret * 100) / wbs->maxsecret)
        // 		cnt_secret[i] = (plrs[i].ssecret * 100) / wbs->maxsecret;
        // 	    else
        // 		stillticking = true;
        // 	}
        //
        // 	if (!stillticking)
        // 	{
        // 	    S_StartSound(0, sfx_barexp);
        // 	    ng_state += 1 + 2*!dofrags;
        // 	}
        //     }
        //     else if (ng_state == 8)
        //     {
        // 	if (!(bcnt&3))
        // 	    S_StartSound(0, sfx_pistol);
        //
        // 	stillticking = false;
        //
        // 	for (i=0 ; i<MAXPLAYERS ; i++)
        // 	{
        // 	    if (!playeringame[i])
        // 		continue;
        //
        // 	    cnt_frags[i] += 1;
        //
        // 	    if (cnt_frags[i] >= (fsum = WI_fragSum(i)))
        // 		cnt_frags[i] = fsum;
        // 	    else
        // 		stillticking = true;
        // 	}
        //
        // 	if (!stillticking)
        // 	{
        // 	    S_StartSound(0, sfx_pldeth);
        // 	    ng_state++;
        // 	}
        //     }
        //     else if (ng_state == 10)
        //     {
        // 	if (acceleratestage)
        // 	{
        // 	    S_StartSound(0, sfx_sgcock);
        // 	    if ( gamemode == commercial )
        // 		WI_initNoState();
        // 	    else
        // 		WI_initShowNextLoc();
        // 	}
        //     }
        //     else if (ng_state & 1)
        //     {
        // 	if (!--cnt_pause)
        // 	{
        // 	    ng_state++;
        // 	    cnt_pause = TICRATE;
        // 	}
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn WI_drawNetgameStats() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut x: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut y: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut pwidth: std::ffi::c_int = unsafe { SHORT((*percent).width) };
        WI_slamBackground();
        WI_drawAnimatedBack();
        WI_drawLF();
        V_DrawPatch(
            ((NG_STATSX + NG_SPACINGX) - SHORT((*kills).width)),
            NG_STATSY,
            FB,
            kills,
        );
        V_DrawPatch(
            ((NG_STATSX + (2 * NG_SPACINGX)) - SHORT((*items).width)),
            NG_STATSY,
            FB,
            items,
        );
        V_DrawPatch(
            ((NG_STATSX + (3 * NG_SPACINGX)) - SHORT((*secret).width)),
            NG_STATSY,
            FB,
            secret,
        );
        // TODO: if statement not yet translated:
        //
        //
        //     if (dofrags)
        // 	V_DrawPatch(NG_STATSX+4*NG_SPACINGX-SHORT(frags->width),
        // 		    NG_STATSY, FB, frags);
        todo!("if statement not yet translated");
        y = (NG_STATSY + SHORT((*kills).height));
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<MAXPLAYERS ; i++)
        //     {
        // 	if (!playeringame[i])
        // 	    continue;
        //
        // 	x = NG_STATSX;
        // 	V_DrawPatch(x-SHORT(p[i]->width), y, FB, p[i]);
        //
        // 	if (i == me)
        // 	    V_DrawPatch(x-SHORT(p[i]->width), y, FB, star);
        //
        // 	x += NG_SPACINGX;
        // 	WI_drawPercent(x-pwidth, y+10, cnt_kills[i]);	x += NG_SPACINGX;
        // 	WI_drawPercent(x-pwidth, y+10, cnt_items[i]);	x += NG_SPACINGX;
        // 	WI_drawPercent(x-pwidth, y+10, cnt_secret[i]);	x += NG_SPACINGX;
        //
        // 	if (dofrags)
        // 	    WI_drawNum(x, y+10, cnt_frags[i], -1);
        //
        // 	y += WI_SPACINGY;
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

static mut sp_state: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn WI_initStats() {
    unsafe {
        state = StatCount;
        acceleratestage = 0;
        sp_state = 1;
        cnt_kills[(0) as usize] = cnt_items[(0) as usize] = cnt_secret[(0) as usize] = (-(1));
        cnt_time = cnt_par = (-(1));
        cnt_pause = TICRATE;
        WI_initAnimatedBack();
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn WI_updateStats() {
    unsafe {
        WI_updateAnimatedBack();
        // TODO: if statement not yet translated:
        //
        //
        //     if (acceleratestage && sp_state != 10)
        //     {
        // 	acceleratestage = 0;
        // 	cnt_kills[0] = (plrs[me].skills * 100) / wbs->maxkills;
        // 	cnt_items[0] = (plrs[me].sitems * 100) / wbs->maxitems;
        // 	cnt_secret[0] = (plrs[me].ssecret * 100) / wbs->maxsecret;
        // 	cnt_time = plrs[me].stime / TICRATE;
        // 	cnt_par = wbs->partime / TICRATE;
        // 	S_StartSound(0, sfx_barexp);
        // 	sp_state = 10;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (sp_state == 2)
        //     {
        // 	cnt_kills[0] += 2;
        //
        // 	if (!(bcnt&3))
        // 	    S_StartSound(0, sfx_pistol);
        //
        // 	if (cnt_kills[0] >= (plrs[me].skills * 100) / wbs->maxkills)
        // 	{
        // 	    cnt_kills[0] = (plrs[me].skills * 100) / wbs->maxkills;
        // 	    S_StartSound(0, sfx_barexp);
        // 	    sp_state++;
        // 	}
        //     }
        //     else if (sp_state == 4)
        //     {
        // 	cnt_items[0] += 2;
        //
        // 	if (!(bcnt&3))
        // 	    S_StartSound(0, sfx_pistol);
        //
        // 	if (cnt_items[0] >= (plrs[me].sitems * 100) / wbs->maxitems)
        // 	{
        // 	    cnt_items[0] = (plrs[me].sitems * 100) / wbs->maxitems;
        // 	    S_StartSound(0, sfx_barexp);
        // 	    sp_state++;
        // 	}
        //     }
        //     else if (sp_state == 6)
        //     {
        // 	cnt_secret[0] += 2;
        //
        // 	if (!(bcnt&3))
        // 	    S_StartSound(0, sfx_pistol);
        //
        // 	if (cnt_secret[0] >= (plrs[me].ssecret * 100) / wbs->maxsecret)
        // 	{
        // 	    cnt_secret[0] = (plrs[me].ssecret * 100) / wbs->maxsecret;
        // 	    S_StartSound(0, sfx_barexp);
        // 	    sp_state++;
        // 	}
        //     }
        //
        //     else if (sp_state == 8)
        //     {
        // 	if (!(bcnt&3))
        // 	    S_StartSound(0, sfx_pistol);
        //
        // 	cnt_time += 3;
        //
        // 	if (cnt_time >= plrs[me].stime / TICRATE)
        // 	    cnt_time = plrs[me].stime / TICRATE;
        //
        // 	cnt_par += 3;
        //
        // 	if (cnt_par >= wbs->partime / TICRATE)
        // 	{
        // 	    cnt_par = wbs->partime / TICRATE;
        //
        // 	    if (cnt_time >= plrs[me].stime / TICRATE)
        // 	    {
        // 		S_StartSound(0, sfx_barexp);
        // 		sp_state++;
        // 	    }
        // 	}
        //     }
        //     else if (sp_state == 10)
        //     {
        // 	if (acceleratestage)
        // 	{
        // 	    S_StartSound(0, sfx_sgcock);
        //
        // 	    if (gamemode == commercial)
        // 		WI_initNoState();
        // 	    else
        // 		WI_initShowNextLoc();
        // 	}
        //     }
        //     else if (sp_state & 1)
        //     {
        // 	if (!--cnt_pause)
        // 	{
        // 	    sp_state++;
        // 	    cnt_pause = TICRATE;
        // 	}
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn WI_drawStats() {
    unsafe {
        let mut lh: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        lh = ((3 * SHORT((*num[(0) as usize]).height)) / 2);
        WI_slamBackground();
        WI_drawAnimatedBack();
        WI_drawLF();
        V_DrawPatch(SP_STATSX, SP_STATSY, FB, kills);
        WI_drawPercent(
            (SCREENWIDTH - SP_STATSX),
            SP_STATSY,
            cnt_kills[(0) as usize],
        );
        V_DrawPatch(SP_STATSX, (SP_STATSY + lh), FB, items);
        WI_drawPercent(
            (SCREENWIDTH - SP_STATSX),
            (SP_STATSY + lh),
            cnt_items[(0) as usize],
        );
        V_DrawPatch(SP_STATSX, (SP_STATSY + (2 * lh)), FB, sp_secret);
        WI_drawPercent(
            (SCREENWIDTH - SP_STATSX),
            (SP_STATSY + (2 * lh)),
            cnt_secret[(0) as usize],
        );
        V_DrawPatch(SP_TIMEX, SP_TIMEY, FB, time);
        WI_drawTime(((SCREENWIDTH / 2) - SP_TIMEX), SP_TIMEY, cnt_time);
        // TODO: if statement not yet translated:
        //
        //
        //     if (wbs->epsd < 3)
        //     {
        // 	V_DrawPatch(SCREENWIDTH/2 + SP_TIMEX, SP_TIMEY, FB, par);
        // 	WI_drawTime(SCREENWIDTH - SP_TIMEX, SP_TIMEY, cnt_par);
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn WI_checkForAccelerate() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut player: *mut player_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: for statement not yet translated:
        //
        //
        //     // check for button presses to skip delays
        //     for (i=0, player = players ; i<MAXPLAYERS ; i++, player++)
        //     {
        // 	if (playeringame[i])
        // 	{
        // 	    if (player->cmd.buttons & BT_ATTACK)
        // 	    {
        // 		if (!player->attackdown)
        // 		    acceleratestage = 1;
        // 		player->attackdown = true;
        // 	    }
        // 	    else
        // 		player->attackdown = false;
        // 	    if (player->cmd.buttons & BT_USE)
        // 	    {
        // 		if (!player->usedown)
        // 		    acceleratestage = 1;
        // 		player->usedown = true;
        // 	    }
        // 	    else
        // 		player->usedown = false;
        // 	}
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn WI_Ticker() {
    unsafe {
        {
            let __macro_tmp = bcnt;
            bcnt += 1;
            __macro_tmp
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (bcnt == 1)
        //     {
        // 	// intermission music
        //   	if ( gamemode == commercial )
        // 	  S_ChangeMusic(mus_dm2int, true);
        // 	else
        // 	  S_ChangeMusic(mus_inter, true);
        //     }
        todo!("if statement not yet translated");
        WI_checkForAccelerate();
        // TODO: switch statement not yet translated:
        //
        //
        //     switch (state)
        //     {
        //       case StatCount:
        // 	if (deathmatch) WI_updateDeathmatchStats();
        // 	else if (netgame) WI_updateNetgameStats();
        // 	else WI_updateStats();
        // 	break;
        //
        //       case ShowNextLoc:
        // 	WI_updateShowNextLoc();
        // 	break;
        //
        //       case NoState:
        // 	WI_updateNoState();
        // 	break;
        //     }
        todo!("switch statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn WI_loadData() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut j: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut name: [std::ffi::c_char; (9) as usize] = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut a: *mut anim_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (gamemode == commercial)
        // 	strcpy(name, "INTERPIC");
        //     else
        // 	sprintf(name, "WIMAP%d", wbs->epsd);
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if ( gamemode == retail )
        //     {
        //       if (wbs->epsd == 3)
        // 	strcpy(name,"INTERPIC");
        //     }
        todo!("if statement not yet translated");
        bg = W_CacheLumpName(name, PU_CACHE);
        V_DrawPatch(0, 0, 1, bg);
        // TODO: if statement not yet translated:
        //
        //
        //
        //     // UNUSED unsigned char *pic = screens[1];
        //     // if (gamemode == commercial)
        //     // {
        //     // darken the background image
        //     // while (pic != screens[1] + SCREENHEIGHT*SCREENWIDTH)
        //     // {
        //     //   *pic = colormaps[256*25 + *pic];
        //     //   pic++;
        //     // }
        //     //}
        //
        //     if (gamemode == commercial)
        //     {
        // 	NUMCMAPS = 32;
        // 	lnames = (patch_t **) Z_Malloc(sizeof(patch_t*) * NUMCMAPS,
        // 				       PU_STATIC, 0);
        // 	for (i=0 ; i<NUMCMAPS ; i++)
        // 	{
        // 	    sprintf(name, "CWILV%2.2d", i);
        // 	    lnames[i] = W_CacheLumpName(name, PU_STATIC);
        // 	}
        //     }
        //     else
        //     {
        // 	lnames = (patch_t **) Z_Malloc(sizeof(patch_t*) * NUMMAPS,
        // 				       PU_STATIC, 0);
        // 	for (i=0 ; i<NUMMAPS ; i++)
        // 	{
        // 	    sprintf(name, "WILV%d%d", wbs->epsd, i);
        // 	    lnames[i] = W_CacheLumpName(name, PU_STATIC);
        // 	}
        //
        // 	// you are here
        // 	yah[0] = W_CacheLumpName("WIURH0", PU_STATIC);
        //
        // 	// you are here (alt.)
        // 	yah[1] = W_CacheLumpName("WIURH1", PU_STATIC);
        //
        // 	// splat
        // 	splat = W_CacheLumpName("WISPLAT", PU_STATIC);
        //
        // 	if (wbs->epsd < 3)
        // 	{
        // 	    for (j=0;j<NUMANIMS[wbs->epsd];j++)
        // 	    {
        // 		a = &anims[wbs->epsd][j];
        // 		for (i=0;i<a->nanims;i++)
        // 		{
        // 		    // MONDO HACK!
        // 		    if (wbs->epsd != 1 || j != 8)
        // 		    {
        // 			// animations
        // 			sprintf(name, "WIA%d%.2d%.2d", wbs->epsd, j, i);
        // 			a->p[i] = W_CacheLumpName(name, PU_STATIC);
        // 		    }
        // 		    else
        // 		    {
        // 			// HACK ALERT!
        // 			a->p[i] = anims[1][4].p[i];
        // 		    }
        // 		}
        // 	    }
        // 	}
        //     }
        todo!("if statement not yet translated");
        wiminus = W_CacheLumpName((c"WIMINUS").as_ptr(), PU_STATIC);
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0;i<10;i++)
        //     {
        // 	 // numbers 0-9
        // 	sprintf(name, "WINUM%d", i);
        // 	num[i] = W_CacheLumpName(name, PU_STATIC);
        //     }
        todo!("for statement not yet translated");
        percent = W_CacheLumpName((c"WIPCNT").as_ptr(), PU_STATIC);
        finished = W_CacheLumpName((c"WIF").as_ptr(), PU_STATIC);
        entering = W_CacheLumpName((c"WIENTER").as_ptr(), PU_STATIC);
        kills = W_CacheLumpName((c"WIOSTK").as_ptr(), PU_STATIC);
        secret = W_CacheLumpName((c"WIOSTS").as_ptr(), PU_STATIC);
        sp_secret = W_CacheLumpName((c"WISCRT2").as_ptr(), PU_STATIC);
        // TODO: if statement not yet translated:
        //
        //
        //     // Yuck.
        //     if (french)
        //     {
        // 	// "items"
        // 	if (netgame && !deathmatch)
        // 	    items = W_CacheLumpName("WIOBJ", PU_STATIC);
        //   	else
        // 	    items = W_CacheLumpName("WIOSTI", PU_STATIC);
        //     } else
        // 	items = W_CacheLumpName("WIOSTI", PU_STATIC);
        todo!("if statement not yet translated");
        frags = W_CacheLumpName((c"WIFRGS").as_ptr(), PU_STATIC);
        colon = W_CacheLumpName((c"WICOLON").as_ptr(), PU_STATIC);
        time = W_CacheLumpName((c"WITIME").as_ptr(), PU_STATIC);
        sucks = W_CacheLumpName((c"WISUCKS").as_ptr(), PU_STATIC);
        par = W_CacheLumpName((c"WIPAR").as_ptr(), PU_STATIC);
        killers = W_CacheLumpName((c"WIKILRS").as_ptr(), PU_STATIC);
        victims = W_CacheLumpName((c"WIVCTMS").as_ptr(), PU_STATIC);
        total = W_CacheLumpName((c"WIMSTT").as_ptr(), PU_STATIC);
        star = W_CacheLumpName((c"STFST01").as_ptr(), PU_STATIC);
        bstar = W_CacheLumpName((c"STFDEAD0").as_ptr(), PU_STATIC);
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<MAXPLAYERS ; i++)
        //     {
        // 	// "1,2,3,4"
        // 	sprintf(name, "STPB%d", i);
        // 	p[i] = W_CacheLumpName(name, PU_STATIC);
        //
        // 	// "1,2,3,4"
        // 	sprintf(name, "WIBP%d", i+1);
        // 	bp[i] = W_CacheLumpName(name, PU_STATIC);
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn WI_unloadData() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut j: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        Z_ChangeTag(wiminus, PU_CACHE);
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<10 ; i++)
        // 	Z_ChangeTag(num[i], PU_CACHE);
        todo!("for statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (gamemode == commercial)
        //     {
        //   	for (i=0 ; i<NUMCMAPS ; i++)
        // 	    Z_ChangeTag(lnames[i], PU_CACHE);
        //     }
        //     else
        //     {
        // 	Z_ChangeTag(yah[0], PU_CACHE);
        // 	Z_ChangeTag(yah[1], PU_CACHE);
        //
        // 	Z_ChangeTag(splat, PU_CACHE);
        //
        // 	for (i=0 ; i<NUMMAPS ; i++)
        // 	    Z_ChangeTag(lnames[i], PU_CACHE);
        //
        // 	if (wbs->epsd < 3)
        // 	{
        // 	    for (j=0;j<NUMANIMS[wbs->epsd];j++)
        // 	    {
        // 		if (wbs->epsd != 1 || j != 8)
        // 		    for (i=0;i<anims[wbs->epsd][j].nanims;i++)
        // 			Z_ChangeTag(anims[wbs->epsd][j].p[i], PU_CACHE);
        // 	    }
        // 	}
        //     }
        todo!("if statement not yet translated");
        Z_Free(lnames);
        Z_ChangeTag(percent, PU_CACHE);
        Z_ChangeTag(colon, PU_CACHE);
        Z_ChangeTag(finished, PU_CACHE);
        Z_ChangeTag(entering, PU_CACHE);
        Z_ChangeTag(kills, PU_CACHE);
        Z_ChangeTag(secret, PU_CACHE);
        Z_ChangeTag(sp_secret, PU_CACHE);
        Z_ChangeTag(items, PU_CACHE);
        Z_ChangeTag(frags, PU_CACHE);
        Z_ChangeTag(time, PU_CACHE);
        Z_ChangeTag(sucks, PU_CACHE);
        Z_ChangeTag(par, PU_CACHE);
        Z_ChangeTag(victims, PU_CACHE);
        Z_ChangeTag(killers, PU_CACHE);
        Z_ChangeTag(total, PU_CACHE);
        // TODO: for statement not yet translated:
        //
        //     //  Z_ChangeTag(star, PU_CACHE);
        //     //  Z_ChangeTag(bstar, PU_CACHE);
        //
        //     for (i=0 ; i<MAXPLAYERS ; i++)
        // 	Z_ChangeTag(p[i], PU_CACHE);
        todo!("for statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<MAXPLAYERS ; i++)
        // 	Z_ChangeTag(bp[i], PU_CACHE);
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn WI_Drawer() {
    unsafe {
        // TODO: switch statement not yet translated:
        //
        //     switch (state)
        //     {
        //       case StatCount:
        // 	if (deathmatch)
        // 	    WI_drawDeathmatchStats();
        // 	else if (netgame)
        // 	    WI_drawNetgameStats();
        // 	else
        // 	    WI_drawStats();
        // 	break;
        //
        //       case ShowNextLoc:
        // 	WI_drawShowNextLoc();
        // 	break;
        //
        //       case NoState:
        // 	WI_drawNoState();
        // 	break;
        //     }
        todo!("switch statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn WI_initVariables(mut wbstartstruct: *mut wbstartstruct_t) {
    unsafe {
        wbs = wbstartstruct;
        acceleratestage = 0;
        cnt = bcnt = 0;
        firstrefresh = 1;
        me = (*wbs).pnum;
        plrs = (*wbs).plyr;
        // TODO: if statement not yet translated:
        //
        //
        //     if (!wbs->maxkills)
        // 	wbs->maxkills = 1;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (!wbs->maxitems)
        // 	wbs->maxitems = 1;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (!wbs->maxsecret)
        // 	wbs->maxsecret = 1;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if ( gamemode != retail )
        //       if (wbs->epsd > 2)
        // 	wbs->epsd -= 3;
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn WI_Start(mut wbstartstruct: *mut wbstartstruct_t) {
    unsafe {
        WI_initVariables(wbstartstruct);
        WI_loadData();
        // TODO: if statement not yet translated:
        //
        //
        //     if (deathmatch)
        // 	WI_initDeathmatchStats();
        //     else if (netgame)
        // 	WI_initNetgameStats();
        //     else
        // 	WI_initStats();
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

const ZEROED_anim_t: anim_t = unsafe { std::mem::zeroed() };
