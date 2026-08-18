use crate::d_englsh::*;
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
use crate::dstrings::*;
use crate::i_system::*;
use crate::info::*;
use crate::m_cheat::*;
use crate::m_fixed::*;
use crate::p_local::*;
use crate::p_mobj::*;
use crate::p_pspr::*;
use crate::p_spec::*;
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

pub const AM_MSGHEADER: std::ffi::c_int =
    (((b'a' as std::ffi::c_int) << 24) + ((b'm' as std::ffi::c_int) << 16));

pub const AM_MSGENTERED: std::ffi::c_int = (AM_MSGHEADER | ((b'e' as std::ffi::c_int) << 8));

pub const AM_MSGEXITED: std::ffi::c_int = (AM_MSGHEADER | ((b'x' as std::ffi::c_int) << 8));

static mut rcsid: [std::ffi::c_char; 49] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        97 as std::ffi::c_char,
        109 as std::ffi::c_char,
        95 as std::ffi::c_char,
        109 as std::ffi::c_char,
        97 as std::ffi::c_char,
        112 as std::ffi::c_char,
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
        50 as std::ffi::c_char,
        49 as std::ffi::c_char,
        58 as std::ffi::c_char,
        50 as std::ffi::c_char,
        52 as std::ffi::c_char,
        58 as std::ffi::c_char,
        51 as std::ffi::c_char,
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

pub const REDS: std::ffi::c_int = (256 - (5 * 16));

pub const REDRANGE: std::ffi::c_int = 16;

pub const BLUES: std::ffi::c_int = ((256 - (4 * 16)) + 8);

pub const BLUERANGE: std::ffi::c_int = 8;

pub const GREENS: std::ffi::c_int = (7 * 16);

pub const GREENRANGE: std::ffi::c_int = 16;

pub const GRAYS: std::ffi::c_int = (6 * 16);

pub const GRAYSRANGE: std::ffi::c_int = 16;

pub const BROWNS: std::ffi::c_int = (4 * 16);

pub const BROWNRANGE: std::ffi::c_int = 16;

pub const YELLOWS: std::ffi::c_int = ((256 - 32) + 7);

pub const YELLOWRANGE: std::ffi::c_int = 1;

pub const BLACK: std::ffi::c_int = 0;

pub const WHITE: std::ffi::c_int = (256 - 47);

pub const BACKGROUND: std::ffi::c_int = BLACK;

pub const YOURCOLORS: std::ffi::c_int = WHITE;

pub const YOURRANGE: std::ffi::c_int = 0;

pub const WALLCOLORS: std::ffi::c_int = REDS;

pub const WALLRANGE: std::ffi::c_int = REDRANGE;

pub const TSWALLCOLORS: std::ffi::c_int = GRAYS;

pub const TSWALLRANGE: std::ffi::c_int = GRAYSRANGE;

pub const FDWALLCOLORS: std::ffi::c_int = BROWNS;

pub const FDWALLRANGE: std::ffi::c_int = BROWNRANGE;

pub const CDWALLCOLORS: std::ffi::c_int = YELLOWS;

pub const CDWALLRANGE: std::ffi::c_int = YELLOWRANGE;

pub const THINGCOLORS: std::ffi::c_int = GREENS;

pub const THINGRANGE: std::ffi::c_int = GREENRANGE;

pub const SECRETWALLCOLORS: std::ffi::c_int = WALLCOLORS;

pub const SECRETWALLRANGE: std::ffi::c_int = WALLRANGE;

pub const GRIDCOLORS: std::ffi::c_int = (GRAYS + (GRAYSRANGE / 2));

pub const GRIDRANGE: std::ffi::c_int = 0;

pub const XHAIRCOLORS: std::ffi::c_int = GRAYS;

pub const FB: std::ffi::c_int = 0;

pub const AM_PANDOWNKEY: std::ffi::c_int = KEY_DOWNARROW;

pub const AM_PANUPKEY: std::ffi::c_int = KEY_UPARROW;

pub const AM_PANRIGHTKEY: std::ffi::c_int = KEY_RIGHTARROW;

pub const AM_PANLEFTKEY: std::ffi::c_int = KEY_LEFTARROW;

pub const AM_ZOOMINKEY: std::ffi::c_int = (b'=' as std::ffi::c_int);

pub const AM_ZOOMOUTKEY: std::ffi::c_int = (b'-' as std::ffi::c_int);

pub const AM_STARTKEY: std::ffi::c_int = KEY_TAB;

pub const AM_ENDKEY: std::ffi::c_int = KEY_TAB;

pub const AM_GOBIGKEY: std::ffi::c_int = (b'0' as std::ffi::c_int);

pub const AM_FOLLOWKEY: std::ffi::c_int = (b'f' as std::ffi::c_int);

pub const AM_GRIDKEY: std::ffi::c_int = (b'g' as std::ffi::c_int);

pub const AM_MARKKEY: std::ffi::c_int = (b'm' as std::ffi::c_int);

pub const AM_CLEARMARKKEY: std::ffi::c_int = (b'c' as std::ffi::c_int);

pub const AM_NUMMARKPOINTS: std::ffi::c_int = 10;

pub const INITSCALEMTOF: std::ffi::c_double = (0.2 * ((FRACUNIT) as f64));

pub const F_PANINC: std::ffi::c_int = 4;

pub const M_ZOOMIN: std::ffi::c_int = ((1.02 * ((FRACUNIT) as f64)) as std::ffi::c_int);

pub const M_ZOOMOUT: std::ffi::c_int = ((((FRACUNIT) as f64) / 1.02) as std::ffi::c_int);

pub unsafe extern "C" fn FTOM(x: std::ffi::c_int) -> std::ffi::c_int {
    FixedMul(((x) << 16), scale_ftom)
}

pub unsafe extern "C" fn MTOF(x: std::ffi::c_int) -> std::ffi::c_int {
    (FixedMul((x), scale_mtof) >> 16)
}

pub unsafe extern "C" fn CXMTOF(x: std::ffi::c_int) -> std::ffi::c_int {
    (f_x + MTOF(((x) - m_x)))
}

pub unsafe extern "C" fn CYMTOF(y: std::ffi::c_int) -> std::ffi::c_int {
    (f_y + (f_h - MTOF(((y) - m_y))))
}

pub const LINE_NEVERSEE: std::ffi::c_int = ML_DONTDRAW;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpoint_t {
    pub x: std::ffi::c_int,
    pub y: std::ffi::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fline_t {
    pub a: fpoint_t,
    pub b: fpoint_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpoint_t {
    pub x: fixed_t,
    pub y: fixed_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mline_t {
    pub a: mpoint_t,
    pub b: mpoint_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct islope_t {
    pub slp: fixed_t,
    pub islp: fixed_t,
}

pub static mut player_arrow: [mline_t; 7] = unsafe {
    [
        mline_t {
            a: mpoint_t {
                x: ((-(R)) + (R / 8)),
                y: 0,
            },
            b: mpoint_t { x: R, y: 0 },
        },
        mline_t {
            a: mpoint_t { x: R, y: 0 },
            b: mpoint_t {
                x: (R - (R / 2)),
                y: (R / 4),
            },
        },
        mline_t {
            a: mpoint_t { x: R, y: 0 },
            b: mpoint_t {
                x: (R - (R / 2)),
                y: ((-(R)) / 4),
            },
        },
        mline_t {
            a: mpoint_t {
                x: ((-(R)) + (R / 8)),
                y: 0,
            },
            b: mpoint_t {
                x: ((-(R)) - (R / 8)),
                y: (R / 4),
            },
        },
        mline_t {
            a: mpoint_t {
                x: ((-(R)) + (R / 8)),
                y: 0,
            },
            b: mpoint_t {
                x: ((-(R)) - (R / 8)),
                y: ((-(R)) / 4),
            },
        },
        mline_t {
            a: mpoint_t {
                x: ((-(R)) + ((3 * R) / 8)),
                y: 0,
            },
            b: mpoint_t {
                x: ((-(R)) + (R / 8)),
                y: (R / 4),
            },
        },
        mline_t {
            a: mpoint_t {
                x: ((-(R)) + ((3 * R) / 8)),
                y: 0,
            },
            b: mpoint_t {
                x: ((-(R)) + (R / 8)),
                y: ((-(R)) / 4),
            },
        },
    ]
};

pub const NUMPLYRLINES: std::ffi::c_int =
    (std::mem::size_of_val(&(player_arrow)) / std::mem::size_of::<mline_t>()) as std::ffi::c_int;

pub static mut cheat_player_arrow: [mline_t; 16] = unsafe {
    [
        mline_t {
            a: mpoint_t {
                x: ((-(R)) + (R / 8)),
                y: 0,
            },
            b: mpoint_t { x: R, y: 0 },
        },
        mline_t {
            a: mpoint_t { x: R, y: 0 },
            b: mpoint_t {
                x: (R - (R / 2)),
                y: (R / 6),
            },
        },
        mline_t {
            a: mpoint_t { x: R, y: 0 },
            b: mpoint_t {
                x: (R - (R / 2)),
                y: ((-(R)) / 6),
            },
        },
        mline_t {
            a: mpoint_t {
                x: ((-(R)) + (R / 8)),
                y: 0,
            },
            b: mpoint_t {
                x: ((-(R)) - (R / 8)),
                y: (R / 6),
            },
        },
        mline_t {
            a: mpoint_t {
                x: ((-(R)) + (R / 8)),
                y: 0,
            },
            b: mpoint_t {
                x: ((-(R)) - (R / 8)),
                y: ((-(R)) / 6),
            },
        },
        mline_t {
            a: mpoint_t {
                x: ((-(R)) + ((3 * R) / 8)),
                y: 0,
            },
            b: mpoint_t {
                x: ((-(R)) + (R / 8)),
                y: (R / 6),
            },
        },
        mline_t {
            a: mpoint_t {
                x: ((-(R)) + ((3 * R) / 8)),
                y: 0,
            },
            b: mpoint_t {
                x: ((-(R)) + (R / 8)),
                y: ((-(R)) / 6),
            },
        },
        mline_t {
            a: mpoint_t {
                x: ((-(R)) / 2),
                y: 0,
            },
            b: mpoint_t {
                x: ((-(R)) / 2),
                y: ((-(R)) / 6),
            },
        },
        mline_t {
            a: mpoint_t {
                x: ((-(R)) / 2),
                y: ((-(R)) / 6),
            },
            b: mpoint_t {
                x: (((-(R)) / 2) + (R / 6)),
                y: ((-(R)) / 6),
            },
        },
        mline_t {
            a: mpoint_t {
                x: (((-(R)) / 2) + (R / 6)),
                y: ((-(R)) / 6),
            },
            b: mpoint_t {
                x: (((-(R)) / 2) + (R / 6)),
                y: (R / 4),
            },
        },
        mline_t {
            a: mpoint_t {
                x: ((-(R)) / 6),
                y: 0,
            },
            b: mpoint_t {
                x: ((-(R)) / 6),
                y: ((-(R)) / 6),
            },
        },
        mline_t {
            a: mpoint_t {
                x: ((-(R)) / 6),
                y: ((-(R)) / 6),
            },
            b: mpoint_t {
                x: 0,
                y: ((-(R)) / 6),
            },
        },
        mline_t {
            a: mpoint_t {
                x: 0,
                y: ((-(R)) / 6),
            },
            b: mpoint_t { x: 0, y: (R / 4) },
        },
        mline_t {
            a: mpoint_t {
                x: (R / 6),
                y: (R / 4),
            },
            b: mpoint_t {
                x: (R / 6),
                y: ((-(R)) / 7),
            },
        },
        mline_t {
            a: mpoint_t {
                x: (R / 6),
                y: ((-(R)) / 7),
            },
            b: mpoint_t {
                x: ((R / 6) + (R / 32)),
                y: (((-(R)) / 7) - (R / 32)),
            },
        },
        mline_t {
            a: mpoint_t {
                x: ((R / 6) + (R / 32)),
                y: (((-(R)) / 7) - (R / 32)),
            },
            b: mpoint_t {
                x: ((R / 6) + (R / 10)),
                y: ((-(R)) / 7),
            },
        },
    ]
};

pub const NUMCHEATPLYRLINES: std::ffi::c_int = (std::mem::size_of_val(&(cheat_player_arrow))
    / std::mem::size_of::<mline_t>())
    as std::ffi::c_int;

pub static mut triangle_guy: [mline_t; 3] = unsafe {
    [
        mline_t {
            a: mpoint_t {
                x: ((-(0.867)) * ((R) as f64)) as std::ffi::c_int,
                y: ((-(0.5)) * ((R) as f64)) as std::ffi::c_int,
            },
            b: mpoint_t {
                x: (0.867 * ((R) as f64)) as std::ffi::c_int,
                y: ((-(0.5)) * ((R) as f64)) as std::ffi::c_int,
            },
        },
        mline_t {
            a: mpoint_t {
                x: (0.867 * ((R) as f64)) as std::ffi::c_int,
                y: ((-(0.5)) * ((R) as f64)) as std::ffi::c_int,
            },
            b: mpoint_t { x: 0, y: R },
        },
        mline_t {
            a: mpoint_t { x: 0, y: R },
            b: mpoint_t {
                x: ((-(0.867)) * ((R) as f64)) as std::ffi::c_int,
                y: ((-(0.5)) * ((R) as f64)) as std::ffi::c_int,
            },
        },
    ]
};

pub const NUMTRIANGLEGUYLINES: std::ffi::c_int =
    (std::mem::size_of_val(&(triangle_guy)) / std::mem::size_of::<mline_t>()) as std::ffi::c_int;

pub const R: std::ffi::c_int = (FRACUNIT);

pub static mut thintriangle_guy: [mline_t; 3] = unsafe {
    [
        mline_t {
            a: mpoint_t {
                x: ((-(0.5)) * ((R) as f64)) as std::ffi::c_int,
                y: ((-(0.7)) * ((R) as f64)) as std::ffi::c_int,
            },
            b: mpoint_t { x: R, y: 0 },
        },
        mline_t {
            a: mpoint_t { x: R, y: 0 },
            b: mpoint_t {
                x: ((-(0.5)) * ((R) as f64)) as std::ffi::c_int,
                y: (0.7 * ((R) as f64)) as std::ffi::c_int,
            },
        },
        mline_t {
            a: mpoint_t {
                x: ((-(0.5)) * ((R) as f64)) as std::ffi::c_int,
                y: (0.7 * ((R) as f64)) as std::ffi::c_int,
            },
            b: mpoint_t {
                x: ((-(0.5)) * ((R) as f64)) as std::ffi::c_int,
                y: ((-(0.7)) * ((R) as f64)) as std::ffi::c_int,
            },
        },
    ]
};

pub const NUMTHINTRIANGLEGUYLINES: std::ffi::c_int = (std::mem::size_of_val(&(thintriangle_guy))
    / std::mem::size_of::<mline_t>())
    as std::ffi::c_int;

static mut cheating: std::ffi::c_int = unsafe { 0 };

static mut grid: std::ffi::c_int = unsafe { 0 };

static mut leveljuststarted: std::ffi::c_int = unsafe { 1 };

pub static mut automapactive: boolean = unsafe { false_ };

static mut finit_width: std::ffi::c_int = unsafe { SCREENWIDTH };

static mut finit_height: std::ffi::c_int = unsafe { (SCREENHEIGHT - 32) };

static mut f_x: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut f_y: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut f_w: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut f_h: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut lightlev: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut fb: *mut byte = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut amclock: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut m_paninc: mpoint_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut mtof_zoommul: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut ftom_zoommul: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut m_x: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut m_y: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut m_x2: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut m_y2: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut m_w: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut m_h: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut min_x: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut min_y: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut max_x: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut max_y: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut max_w: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut max_h: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut min_w: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut min_h: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut min_scale_mtof: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut max_scale_mtof: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut old_m_w: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut old_m_h: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut old_m_x: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut old_m_y: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut f_oldloc: mpoint_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut scale_mtof: fixed_t = unsafe { INITSCALEMTOF };

static mut scale_ftom: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut plr: *mut player_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut marknums: [*mut patch_t; (10) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut markpoints: [mpoint_t; (AM_NUMMARKPOINTS) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut markpointnum: std::ffi::c_int = unsafe { 0 };

static mut followplayer: std::ffi::c_int = unsafe { 1 };

static mut cheat_amap_seq: [std::ffi::c_uchar; 5] = unsafe { [0xb2, 0x26, 0x26, 0x2e, 0xff] };

static mut cheat_amap: cheatseq_t = unsafe {
    cheatseq_t {
        sequence: cheat_amap_seq.as_mut_ptr(),
        p: std::ptr::null_mut(),
    }
};

static mut stopped: boolean = unsafe { true_ };

unsafe extern "C" {
    pub static mut viewactive: boolean;
}

unsafe extern "C" {
    pub fn V_MarkRect(
        x: std::ffi::c_int,
        y: std::ffi::c_int,
        width: std::ffi::c_int,
        height: std::ffi::c_int,
    );
}

pub unsafe extern "C" fn AM_getIslope(mut ml: *mut mline_t, mut is: *mut islope_t) {
    unsafe {
        let mut dx: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut dy: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        dy = ((*ml).a.y - (*ml).b.y);
        dx = ((*ml).b.x - (*ml).a.x);
        // TODO: if statement not yet translated:
        //
        //     if (!dy) is->islp = (dx<0?-MAXINT:MAXINT);
        //     else is->islp = FixedDiv(dx, dy);
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (!dx) is->slp = (dy<0?-MAXINT:MAXINT);
        //     else is->slp = FixedDiv(dy, dx);
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn AM_activateNewScale() {
    unsafe {
        m_x += (m_w / 2);
        m_y += (m_h / 2);
        m_w = FTOM(f_w);
        m_h = FTOM(f_h);
        m_x -= (m_w / 2);
        m_y -= (m_h / 2);
        m_x2 = (m_x + m_w);
        m_y2 = (m_y + m_h);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn AM_saveScaleAndLoc() {
    unsafe {
        old_m_x = m_x;
        old_m_y = m_y;
        old_m_w = m_w;
        old_m_h = m_h;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn AM_restoreScaleAndLoc() {
    unsafe {
        m_w = old_m_w;
        m_h = old_m_h;
        // TODO: if statement not yet translated:
        //
        //     if (!followplayer)
        //     {
        // 	m_x = old_m_x;
        // 	m_y = old_m_y;
        //     } else {
        // 	m_x = plr->mo->x - m_w/2;
        // 	m_y = plr->mo->y - m_h/2;
        //     }
        todo!("if statement not yet translated");
        m_x2 = (m_x + m_w);
        m_y2 = (m_y + m_h);
        scale_mtof = FixedDiv((f_w << FRACBITS), m_w);
        scale_ftom = FixedDiv(FRACUNIT, scale_mtof);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn AM_addMark() {
    unsafe {
        markpoints[(markpointnum) as usize].x = (m_x + (m_w / 2));
        markpoints[(markpointnum) as usize].y = (m_y + (m_h / 2));
        markpointnum = ((markpointnum + 1) % AM_NUMMARKPOINTS);
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn AM_findMinMaxBoundaries() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut a: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut b: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        min_x = min_y = std::ffi::c_int::MAX;
        max_x = max_y = (-(std::ffi::c_int::MAX));
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0;i<numvertexes;i++)
        //     {
        // 	if (vertexes[i].x < min_x)
        // 	    min_x = vertexes[i].x;
        // 	else if (vertexes[i].x > max_x)
        // 	    max_x = vertexes[i].x;
        //
        // 	if (vertexes[i].y < min_y)
        // 	    min_y = vertexes[i].y;
        // 	else if (vertexes[i].y > max_y)
        // 	    max_y = vertexes[i].y;
        //     }
        todo!("for statement not yet translated");
        max_w = (max_x - min_x);
        max_h = (max_y - min_y);
        min_w = (2 * PLAYERRADIUS);
        min_h = (2 * PLAYERRADIUS);
        a = FixedDiv((f_w << FRACBITS), max_w);
        b = FixedDiv((f_h << FRACBITS), max_h);
        min_scale_mtof = (if (a < b) { a } else { b });
        max_scale_mtof = FixedDiv((f_h << FRACBITS), (2 * PLAYERRADIUS));
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn AM_changeWindowLoc() {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (m_paninc.x || m_paninc.y)
        //     {
        // 	followplayer = 0;
        // 	f_oldloc.x = MAXINT;
        //     }
        todo!("if statement not yet translated");
        m_x += m_paninc.x;
        m_y += m_paninc.y;
        // TODO: if statement not yet translated:
        //
        //
        //     if (m_x + m_w/2 > max_x)
        // 	m_x = max_x - m_w/2;
        //     else if (m_x + m_w/2 < min_x)
        // 	m_x = min_x - m_w/2;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (m_y + m_h/2 > max_y)
        // 	m_y = max_y - m_h/2;
        //     else if (m_y + m_h/2 < min_y)
        // 	m_y = min_y - m_h/2;
        todo!("if statement not yet translated");
        m_x2 = (m_x + m_w);
        m_y2 = (m_y + m_h);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn AM_initVariables() {
    unsafe {
        let mut pnum: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        static mut st_notify: event_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        automapactive = true_;
        fb = screens[(0) as usize];
        f_oldloc.x = std::ffi::c_int::MAX;
        amclock = 0;
        lightlev = 0;
        m_paninc.x = m_paninc.y = 0;
        ftom_zoommul = FRACUNIT;
        mtof_zoommul = FRACUNIT;
        m_w = FTOM(f_w);
        m_h = FTOM(f_h);
        // TODO: if statement not yet translated:
        //
        //
        //     // find player to center on initially
        //     if (!playeringame[pnum = consoleplayer])
        // 	for (pnum=0;pnum<MAXPLAYERS;pnum++)
        // 	    if (playeringame[pnum])
        // 		break;
        todo!("if statement not yet translated");
        plr = (&(players[(pnum) as usize]) as *const _ as *mut _);
        m_x = ((*(*plr).mo).x - (m_w / 2));
        m_y = ((*(*plr).mo).y - (m_h / 2));
        AM_changeWindowLoc();
        old_m_x = m_x;
        old_m_y = m_y;
        old_m_w = m_w;
        old_m_h = m_h;
        ST_Responder((&(st_notify) as *const _ as *mut _));
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn AM_loadPics() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut namebuf: [std::ffi::c_char; (9) as usize] = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0;i<10;i++)
        //     {
        // 	sprintf(namebuf, "AMMNUM%d", i);
        // 	marknums[i] = W_CacheLumpName(namebuf, PU_STATIC);
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn AM_unloadPics() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0;i<10;i++)
        // 	Z_ChangeTag(marknums[i], PU_CACHE);
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn AM_clearMarks() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0;i<AM_NUMMARKPOINTS;i++)
        // 	markpoints[i].x = -1; // means empty
        todo!("for statement not yet translated");
        markpointnum = 0;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn AM_LevelInit() {
    unsafe {
        leveljuststarted = 0;
        f_x = f_y = 0;
        f_w = finit_width;
        f_h = finit_height;
        AM_clearMarks();
        AM_findMinMaxBoundaries();
        scale_mtof = FixedDiv(
            min_scale_mtof,
            ((0.7 * ((FRACUNIT) as f64)) as std::ffi::c_int),
        );
        // TODO: if statement not yet translated:
        //
        //     if (scale_mtof > max_scale_mtof)
        // 	scale_mtof = min_scale_mtof;
        todo!("if statement not yet translated");
        scale_ftom = FixedDiv(FRACUNIT, scale_mtof);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn AM_Stop() {
    unsafe {
        static mut st_notify: event_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        AM_unloadPics();
        automapactive = false_;
        ST_Responder((&(st_notify) as *const _ as *mut _));
        stopped = true_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn AM_Start() {
    unsafe {
        static mut lastlevel: std::ffi::c_int = unsafe { (-(1)) };
        static mut lastepisode: std::ffi::c_int = unsafe { (-(1)) };
        // TODO: if statement not yet translated:
        //
        //
        //     if (!stopped) AM_Stop();
        todo!("if statement not yet translated");
        stopped = false_;
        // TODO: if statement not yet translated:
        //
        //     if (lastlevel != gamemap || lastepisode != gameepisode)
        //     {
        // 	AM_LevelInit();
        // 	lastlevel = gamemap;
        // 	lastepisode = gameepisode;
        //     }
        todo!("if statement not yet translated");
        AM_initVariables();
        AM_loadPics();
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn AM_minOutWindowScale() {
    unsafe {
        scale_mtof = min_scale_mtof;
        scale_ftom = FixedDiv(FRACUNIT, scale_mtof);
        AM_activateNewScale();
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn AM_maxOutWindowScale() {
    unsafe {
        scale_mtof = max_scale_mtof;
        scale_ftom = FixedDiv(FRACUNIT, scale_mtof);
        AM_activateNewScale();
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn AM_Responder(mut ev: *mut event_t) -> boolean {
    unsafe {
        let mut rc: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        static mut cheatstate: std::ffi::c_int = unsafe { 0 };
        static mut bigstate: std::ffi::c_int = unsafe { 0 };
        static mut buffer: [std::ffi::c_char; (20) as usize] = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        rc = false_;
        // TODO: if statement not yet translated:
        //
        //
        //     if (!automapactive)
        //     {
        // 	if (ev->type == ev_keydown && ev->data1 == AM_STARTKEY)
        // 	{
        // 	    AM_Start ();
        // 	    viewactive = false;
        // 	    rc = true;
        // 	}
        //     }
        //
        //     else if (ev->type == ev_keydown)
        //     {
        //
        // 	rc = true;
        // 	switch(ev->data1)
        // 	{
        // 	  case AM_PANRIGHTKEY: // pan right
        // 	    if (!followplayer) m_paninc.x = FTOM(F_PANINC);
        // 	    else rc = false;
        // 	    break;
        // 	  case AM_PANLEFTKEY: // pan left
        // 	    if (!followplayer) m_paninc.x = -FTOM(F_PANINC);
        // 	    else rc = false;
        // 	    break;
        // 	  case AM_PANUPKEY: // pan up
        // 	    if (!followplayer) m_paninc.y = FTOM(F_PANINC);
        // 	    else rc = false;
        // 	    break;
        // 	  case AM_PANDOWNKEY: // pan down
        // 	    if (!followplayer) m_paninc.y = -FTOM(F_PANINC);
        // 	    else rc = false;
        // 	    break;
        // 	  case AM_ZOOMOUTKEY: // zoom out
        // 	    mtof_zoommul = M_ZOOMOUT;
        // 	    ftom_zoommul = M_ZOOMIN;
        // 	    break;
        // 	  case AM_ZOOMINKEY: // zoom in
        // 	    mtof_zoommul = M_ZOOMIN;
        // 	    ftom_zoommul = M_ZOOMOUT;
        // 	    break;
        // 	  case AM_ENDKEY:
        // 	    bigstate = 0;
        // 	    viewactive = true;
        // 	    AM_Stop ();
        // 	    break;
        // 	  case AM_GOBIGKEY:
        // 	    bigstate = !bigstate;
        // 	    if (bigstate)
        // 	    {
        // 		AM_saveScaleAndLoc();
        // 		AM_minOutWindowScale();
        // 	    }
        // 	    else AM_restoreScaleAndLoc();
        // 	    break;
        // 	  case AM_FOLLOWKEY:
        // 	    followplayer = !followplayer;
        // 	    f_oldloc.x = MAXINT;
        // 	    plr->message = followplayer ? AMSTR_FOLLOWON : AMSTR_FOLLOWOFF;
        // 	    break;
        // 	  case AM_GRIDKEY:
        // 	    grid = !grid;
        // 	    plr->message = grid ? AMSTR_GRIDON : AMSTR_GRIDOFF;
        // 	    break;
        // 	  case AM_MARKKEY:
        // 	    sprintf(buffer, "%s %d", AMSTR_MARKEDSPOT, markpointnum);
        // 	    plr->message = buffer;
        // 	    AM_addMark();
        // 	    break;
        // 	  case AM_CLEARMARKKEY:
        // 	    AM_clearMarks();
        // 	    plr->message = AMSTR_MARKSCLEARED;
        // 	    break;
        // 	  default:
        // 	    cheatstate=0;
        // 	    rc = false;
        // 	}
        // 	if (!deathmatch && cht_CheckCheat(&cheat_amap, ev->data1))
        // 	{
        // 	    rc = false;
        // 	    cheating = (cheating+1) % 3;
        // 	}
        //     }
        //
        //     else if (ev->type == ev_keyup)
        //     {
        // 	rc = false;
        // 	switch (ev->data1)
        // 	{
        // 	  case AM_PANRIGHTKEY:
        // 	    if (!followplayer) m_paninc.x = 0;
        // 	    break;
        // 	  case AM_PANLEFTKEY:
        // 	    if (!followplayer) m_paninc.x = 0;
        // 	    break;
        // 	  case AM_PANUPKEY:
        // 	    if (!followplayer) m_paninc.y = 0;
        // 	    break;
        // 	  case AM_PANDOWNKEY:
        // 	    if (!followplayer) m_paninc.y = 0;
        // 	    break;
        // 	  case AM_ZOOMOUTKEY:
        // 	  case AM_ZOOMINKEY:
        // 	    mtof_zoommul = FRACUNIT;
        // 	    ftom_zoommul = FRACUNIT;
        // 	    break;
        // 	}
        //     }
        todo!("if statement not yet translated");
        return rc;
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn AM_changeWindowScale() {
    unsafe {
        scale_mtof = FixedMul(scale_mtof, mtof_zoommul);
        scale_ftom = FixedDiv(FRACUNIT, scale_mtof);
        // TODO: if statement not yet translated:
        //
        //
        //     if (scale_mtof < min_scale_mtof)
        // 	AM_minOutWindowScale();
        //     else if (scale_mtof > max_scale_mtof)
        // 	AM_maxOutWindowScale();
        //     else
        // 	AM_activateNewScale();
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn AM_doFollowPlayer() {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //
        //     if (f_oldloc.x != plr->mo->x || f_oldloc.y != plr->mo->y)
        //     {
        // 	m_x = FTOM(MTOF(plr->mo->x)) - m_w/2;
        // 	m_y = FTOM(MTOF(plr->mo->y)) - m_h/2;
        // 	m_x2 = m_x + m_w;
        // 	m_y2 = m_y + m_h;
        // 	f_oldloc.x = plr->mo->x;
        // 	f_oldloc.y = plr->mo->y;
        //
        // 	//  m_x = FTOM(MTOF(plr->mo->x - m_w/2));
        // 	//  m_y = FTOM(MTOF(plr->mo->y - m_h/2));
        // 	//  m_x = plr->mo->x - m_w/2;
        // 	//  m_y = plr->mo->y - m_h/2;
        //
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn AM_updateLightLev() {
    unsafe {
        static mut nexttic: std::ffi::c_int = unsafe { 0 };
        static mut litelevels: *mut std::ffi::c_int /* TODO: was unsized array */ = unsafe { std::mem::zeroed() /* TODO: initializer not yet translated */ };
        static mut litelevelscnt: std::ffi::c_int = unsafe { 0 };
        // TODO: if statement not yet translated:
        //
        //
        //     // Change light level
        //     if (amclock>nexttic)
        //     {
        // 	lightlev = litelevels[litelevelscnt++];
        // 	if (litelevelscnt == sizeof(litelevels)/sizeof(int)) litelevelscnt = 0;
        // 	nexttic = amclock + 6 - (amclock % 6);
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn AM_Ticker() {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //
        //     if (!automapactive)
        // 	return;
        todo!("if statement not yet translated");
        {
            let __macro_tmp = amclock;
            amclock += 1;
            __macro_tmp
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (followplayer)
        // 	AM_doFollowPlayer();
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // Change the zoom if necessary
        //     if (ftom_zoommul != FRACUNIT)
        // 	AM_changeWindowScale();
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // Change x,y location
        //     if (m_paninc.x || m_paninc.y)
        // 	AM_changeWindowLoc();
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        //     // Update light level
        //     // AM_updateLightLev();
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn AM_clearFB(mut color: std::ffi::c_int) {
    unsafe {
        memset(fb, color, (f_w * f_h));
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn AM_clipMline(mut ml: *mut mline_t, mut fl: *mut fline_t) -> boolean {
    unsafe {
        // TODO: statement not yet translated:
        //
        //     enum
        //     {
        // 	LEFT	=1,
        // 	RIGHT	=2,
        // 	BOTTOM	=4,
        // 	TOP	=8
        //     };
        todo!("statement not yet translated");
        let mut outcode1: std::ffi::c_int = unsafe { 0 };
        let mut outcode2: std::ffi::c_int = unsafe { 0 };
        let mut outside: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut tmp: fpoint_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut dx: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut dy: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // C preprocessor directive at statement position (not executable, nothing lost):
        //
        //
        //
        // #define DOOUTCODE(oc, mx, my) \
        //     (oc) = 0; \
        //     if ((my) < 0) (oc) |= TOP; \
        //     else if ((my) >= f_h) (oc) |= BOTTOM; \
        //     if ((mx) < 0) (oc) |= LEFT; \
        //     else if ((mx) >= f_w) (oc) |= RIGHT;
        // TODO: if statement not yet translated:
        //
        //
        //     // do trivial rejects and outcodes
        //     if (ml->a.y > m_y2)
        // 	outcode1 = TOP;
        //     else if (ml->a.y < m_y)
        // 	outcode1 = BOTTOM;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (ml->b.y > m_y2)
        // 	outcode2 = TOP;
        //     else if (ml->b.y < m_y)
        // 	outcode2 = BOTTOM;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (outcode1 & outcode2)
        // 	return false; // trivially outside
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (ml->a.x < m_x)
        // 	outcode1 |= LEFT;
        //     else if (ml->a.x > m_x2)
        // 	outcode1 |= RIGHT;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (ml->b.x < m_x)
        // 	outcode2 |= LEFT;
        //     else if (ml->b.x > m_x2)
        // 	outcode2 |= RIGHT;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (outcode1 & outcode2)
        // 	return false; // trivially outside
        todo!("if statement not yet translated");
        (*fl).a.x = CXMTOF((*ml).a.x);
        (*fl).a.y = CYMTOF((*ml).a.y);
        (*fl).b.x = CXMTOF((*ml).b.x);
        (*fl).b.y = CYMTOF((*ml).b.y);
        DOOUTCODE(outcode1, (*fl).a.x, (*fl).a.y);
        DOOUTCODE(outcode2, (*fl).b.x, (*fl).b.y);
        // TODO: if statement not yet translated:
        //
        //
        //     if (outcode1 & outcode2)
        // 	return false;
        todo!("if statement not yet translated");
        // TODO: while statement not yet translated:
        //
        //
        //     while (outcode1 | outcode2)
        //     {
        // 	// may be partially inside box
        // 	// find an outside point
        // 	if (outcode1)
        // 	    outside = outcode1;
        // 	else
        // 	    outside = outcode2;
        //
        // 	// clip to each side
        // 	if (outside & TOP)
        // 	{
        // 	    dy = fl->a.y - fl->b.y;
        // 	    dx = fl->b.x - fl->a.x;
        // 	    tmp.x = fl->a.x + (dx*(fl->a.y))/dy;
        // 	    tmp.y = 0;
        // 	}
        // 	else if (outside & BOTTOM)
        // 	{
        // 	    dy = fl->a.y - fl->b.y;
        // 	    dx = fl->b.x - fl->a.x;
        // 	    tmp.x = fl->a.x + (dx*(fl->a.y-f_h))/dy;
        // 	    tmp.y = f_h-1;
        // 	}
        // 	else if (outside & RIGHT)
        // 	{
        // 	    dy = fl->b.y - fl->a.y;
        // 	    dx = fl->b.x - fl->a.x;
        // 	    tmp.y = fl->a.y + (dy*(f_w-1 - fl->a.x))/dx;
        // 	    tmp.x = f_w-1;
        // 	}
        // 	else if (outside & LEFT)
        // 	{
        // 	    dy = fl->b.y - fl->a.y;
        // 	    dx = fl->b.x - fl->a.x;
        // 	    tmp.y = fl->a.y + (dy*(-fl->a.x))/dx;
        // 	    tmp.x = 0;
        // 	}
        //
        // 	if (outside == outcode1)
        // 	{
        // 	    fl->a = tmp;
        // 	    DOOUTCODE(outcode1, fl->a.x, fl->a.y);
        // 	}
        // 	else
        // 	{
        // 	    fl->b = tmp;
        // 	    DOOUTCODE(outcode2, fl->b.x, fl->b.y);
        // 	}
        //
        // 	if (outcode1 & outcode2)
        // 	    return false; // trivially outside
        //     }
        todo!("while statement not yet translated");
        return true_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn AM_drawFline(mut fl: *mut fline_t, mut color: std::ffi::c_int) {
    unsafe {
        let mut x: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut y: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut dx: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut dy: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut sx: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut sy: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut ax: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut ay: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut d: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        static mut fuck: std::ffi::c_int = unsafe { 0 };
        // TODO: if statement not yet translated:
        //
        //
        //     // For debugging only
        //     if (      fl->a.x < 0 || fl->a.x >= f_w
        // 	   || fl->a.y < 0 || fl->a.y >= f_h
        // 	   || fl->b.x < 0 || fl->b.x >= f_w
        // 	   || fl->b.y < 0 || fl->b.y >= f_h)
        //     {
        // 	fprintf(stderr, "fuck %d \r", fuck++);
        // 	return;
        //     }
        todo!("if statement not yet translated");
        // C preprocessor directive at statement position (not executable, nothing lost):
        //
        //
        // #define PUTDOT(xx,yy,cc) fb[(yy)*f_w+(xx)]=(cc)
        dx = ((*fl).b.x - (*fl).a.x);
        ax = (2 * (if (dx < 0) { (-(dx)) } else { dx }));
        sx = (if (dx < 0) { (-(1)) } else { 1 });
        dy = ((*fl).b.y - (*fl).a.y);
        ay = (2 * (if (dy < 0) { (-(dy)) } else { dy }));
        sy = (if (dy < 0) { (-(1)) } else { 1 });
        x = (*fl).a.x;
        y = (*fl).a.y;
        // TODO: if statement not yet translated:
        //
        //
        //     if (ax > ay)
        //     {
        // 	d = ay - ax/2;
        // 	while (1)
        // 	{
        // 	    PUTDOT(x,y,color);
        // 	    if (x == fl->b.x) return;
        // 	    if (d>=0)
        // 	    {
        // 		y += sy;
        // 		d -= ax;
        // 	    }
        // 	    x += sx;
        // 	    d += ay;
        // 	}
        //     }
        //     else
        //     {
        // 	d = ax - ay/2;
        // 	while (1)
        // 	{
        // 	    PUTDOT(x, y, color);
        // 	    if (y == fl->b.y) return;
        // 	    if (d >= 0)
        // 	    {
        // 		x += sx;
        // 		d -= ay;
        // 	    }
        // 	    y += sy;
        // 	    d += ax;
        // 	}
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn AM_drawMline(mut ml: *mut mline_t, mut color: std::ffi::c_int) {
    unsafe {
        static mut fl: fline_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (AM_clipMline(ml, &fl))
        // 	AM_drawFline(&fl, color); // draws it on frame buffer using fb coords
        todo!("if statement not yet translated");
    }
}

pub unsafe extern "C" fn AM_drawGrid(mut color: std::ffi::c_int) {
    unsafe {
        let mut x: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut y: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut start: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut end: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut ml: mline_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        start = m_x;
        // TODO: if statement not yet translated:
        //
        //     if ((start-bmaporgx)%(MAPBLOCKUNITS<<FRACBITS))
        // 	start += (MAPBLOCKUNITS<<FRACBITS)
        // 	    - ((start-bmaporgx)%(MAPBLOCKUNITS<<FRACBITS));
        todo!("if statement not yet translated");
        end = (m_x + m_w);
        ml.a.y = m_y;
        ml.b.y = (m_y + m_h);
        // TODO: for statement not yet translated:
        //
        //     for (x=start; x<end; x+=(MAPBLOCKUNITS<<FRACBITS))
        //     {
        // 	ml.a.x = x;
        // 	ml.b.x = x;
        // 	AM_drawMline(&ml, color);
        //     }
        todo!("for statement not yet translated");
        start = m_y;
        // TODO: if statement not yet translated:
        //
        //     if ((start-bmaporgy)%(MAPBLOCKUNITS<<FRACBITS))
        // 	start += (MAPBLOCKUNITS<<FRACBITS)
        // 	    - ((start-bmaporgy)%(MAPBLOCKUNITS<<FRACBITS));
        todo!("if statement not yet translated");
        end = (m_y + m_h);
        ml.a.x = m_x;
        ml.b.x = (m_x + m_w);
        // TODO: for statement not yet translated:
        //
        //     for (y=start; y<end; y+=(MAPBLOCKUNITS<<FRACBITS))
        //     {
        // 	ml.a.y = y;
        // 	ml.b.y = y;
        // 	AM_drawMline(&ml, color);
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn AM_drawWalls() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        static mut l: mline_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0;i<numlines;i++)
        //     {
        // 	l.a.x = lines[i].v1->x;
        // 	l.a.y = lines[i].v1->y;
        // 	l.b.x = lines[i].v2->x;
        // 	l.b.y = lines[i].v2->y;
        // 	if (cheating || (lines[i].flags & ML_MAPPED))
        // 	{
        // 	    if ((lines[i].flags & LINE_NEVERSEE) && !cheating)
        // 		continue;
        // 	    if (!lines[i].backsector)
        // 	    {
        // 		AM_drawMline(&l, WALLCOLORS+lightlev);
        // 	    }
        // 	    else
        // 	    {
        // 		if (lines[i].special == 39)
        // 		{ // teleporters
        // 		    AM_drawMline(&l, WALLCOLORS+WALLRANGE/2);
        // 		}
        // 		else if (lines[i].flags & ML_SECRET) // secret door
        // 		{
        // 		    if (cheating) AM_drawMline(&l, SECRETWALLCOLORS + lightlev);
        // 		    else AM_drawMline(&l, WALLCOLORS+lightlev);
        // 		}
        // 		else if (lines[i].backsector->floorheight
        // 			   != lines[i].frontsector->floorheight) {
        // 		    AM_drawMline(&l, FDWALLCOLORS + lightlev); // floor level change
        // 		}
        // 		else if (lines[i].backsector->ceilingheight
        // 			   != lines[i].frontsector->ceilingheight) {
        // 		    AM_drawMline(&l, CDWALLCOLORS+lightlev); // ceiling level change
        // 		}
        // 		else if (cheating) {
        // 		    AM_drawMline(&l, TSWALLCOLORS+lightlev);
        // 		}
        // 	    }
        // 	}
        // 	else if (plr->powers[pw_allmap])
        // 	{
        // 	    if (!(lines[i].flags & LINE_NEVERSEE)) AM_drawMline(&l, GRAYS+3);
        // 	}
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn AM_rotate(mut x: *mut fixed_t, mut y: *mut fixed_t, mut a: angle_t) {
    unsafe {
        let mut tmpx: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        tmpx = (FixedMul((*(x)), finecosine[(a >> ANGLETOFINESHIFT) as usize])
            - FixedMul((*(y)), finesine[(a >> ANGLETOFINESHIFT) as usize]));
        (*(y)) = (FixedMul((*(x)), finesine[(a >> ANGLETOFINESHIFT) as usize])
            + FixedMul((*(y)), finecosine[(a >> ANGLETOFINESHIFT) as usize]));
        (*(x)) = tmpx;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn AM_drawLineCharacter(
    mut lineguy: *mut mline_t,
    mut lineguylines: std::ffi::c_int,
    mut scale: fixed_t,
    mut angle: angle_t,
    mut color: std::ffi::c_int,
    mut x: fixed_t,
    mut y: fixed_t,
) {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut l: mline_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0;i<lineguylines;i++)
        //     {
        // 	l.a.x = lineguy[i].a.x;
        // 	l.a.y = lineguy[i].a.y;
        //
        // 	if (scale)
        // 	{
        // 	    l.a.x = FixedMul(scale, l.a.x);
        // 	    l.a.y = FixedMul(scale, l.a.y);
        // 	}
        //
        // 	if (angle)
        // 	    AM_rotate(&l.a.x, &l.a.y, angle);
        //
        // 	l.a.x += x;
        // 	l.a.y += y;
        //
        // 	l.b.x = lineguy[i].b.x;
        // 	l.b.y = lineguy[i].b.y;
        //
        // 	if (scale)
        // 	{
        // 	    l.b.x = FixedMul(scale, l.b.x);
        // 	    l.b.y = FixedMul(scale, l.b.y);
        // 	}
        //
        // 	if (angle)
        // 	    AM_rotate(&l.b.x, &l.b.y, angle);
        //
        // 	l.b.x += x;
        // 	l.b.y += y;
        //
        // 	AM_drawMline(&l, color);
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn AM_drawPlayers() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut p: *mut player_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        static mut their_colors: *mut std::ffi::c_int /* TODO: was unsized array */ = unsafe { std::mem::zeroed() /* TODO: initializer not yet translated */ };
        let mut their_color: std::ffi::c_int = unsafe { (-(1)) };
        let mut color: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (!netgame)
        //     {
        // 	if (cheating)
        // 	    AM_drawLineCharacter
        // 		(cheat_player_arrow, NUMCHEATPLYRLINES, 0,
        // 		 plr->mo->angle, WHITE, plr->mo->x, plr->mo->y);
        // 	else
        // 	    AM_drawLineCharacter
        // 		(player_arrow, NUMPLYRLINES, 0, plr->mo->angle,
        // 		 WHITE, plr->mo->x, plr->mo->y);
        // 	return;
        //     }
        todo!("if statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0;i<MAXPLAYERS;i++)
        //     {
        // 	their_color++;
        // 	p = &players[i];
        //
        // 	if ( (deathmatch && !singledemo) && p != plr)
        // 	    continue;
        //
        // 	if (!playeringame[i])
        // 	    continue;
        //
        // 	if (p->powers[pw_invisibility])
        // 	    color = 246; // *close* to black
        // 	else
        // 	    color = their_colors[their_color];
        //
        // 	AM_drawLineCharacter
        // 	    (player_arrow, NUMPLYRLINES, 0, p->mo->angle,
        // 	     color, p->mo->x, p->mo->y);
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn AM_drawThings(
    mut colors: std::ffi::c_int,
    mut colorrange: std::ffi::c_int,
) {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut t: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0;i<numsectors;i++)
        //     {
        // 	t = sectors[i].thinglist;
        // 	while (t)
        // 	{
        // 	    AM_drawLineCharacter
        // 		(thintriangle_guy, NUMTHINTRIANGLEGUYLINES,
        // 		 16<<FRACBITS, t->angle, colors+lightlev, t->x, t->y);
        // 	    t = t->snext;
        // 	}
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn AM_drawMarks() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut fx: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut fy: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut w: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut h: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0;i<AM_NUMMARKPOINTS;i++)
        //     {
        // 	if (markpoints[i].x != -1)
        // 	{
        // 	    //      w = SHORT(marknums[i]->width);
        // 	    //      h = SHORT(marknums[i]->height);
        // 	    w = 5; // because something's wrong with the wad, i guess
        // 	    h = 6; // because something's wrong with the wad, i guess
        // 	    fx = CXMTOF(markpoints[i].x);
        // 	    fy = CYMTOF(markpoints[i].y);
        // 	    if (fx >= f_x && fx <= f_w - w && fy >= f_y && fy <= f_h - h)
        // 		V_DrawPatch(fx, fy, FB, marknums[i]);
        // 	}
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn AM_drawCrosshair(mut color: std::ffi::c_int) {
    unsafe {
        fb[((f_w * (f_h + 1)) / 2) as usize] = color;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn AM_Drawer() {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (!automapactive) return;
        todo!("if statement not yet translated");
        AM_clearFB(BACKGROUND);
        // TODO: if statement not yet translated:
        //
        //     if (grid)
        // 	AM_drawGrid(GRIDCOLORS);
        todo!("if statement not yet translated");
        AM_drawWalls();
        AM_drawPlayers();
        // TODO: if statement not yet translated:
        //
        //     if (cheating==2)
        // 	AM_drawThings(THINGCOLORS, THINGRANGE);
        todo!("if statement not yet translated");
        AM_drawCrosshair(XHAIRCOLORS);
        AM_drawMarks();
        V_MarkRect(f_x, f_y, f_w, f_h);
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}
