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

pub unsafe extern "C" fn AM_getIslope(ml: *mut mline_t, is: *mut islope_t) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn AM_activateNewScale() {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn AM_saveScaleAndLoc() {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn AM_restoreScaleAndLoc() {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn AM_addMark() {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn AM_findMinMaxBoundaries() {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn AM_changeWindowLoc() {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn AM_initVariables() {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn AM_loadPics() {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn AM_unloadPics() {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn AM_clearMarks() {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn AM_LevelInit() {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn AM_Stop() {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn AM_Start() {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn AM_minOutWindowScale() {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn AM_maxOutWindowScale() {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn AM_Responder(ev: *mut event_t) -> boolean {
    unsafe { todo!("body not yet translated") }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn AM_changeWindowScale() {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn AM_doFollowPlayer() {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn AM_updateLightLev() {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn AM_Ticker() {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn AM_clearFB(color: std::ffi::c_int) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn AM_clipMline(ml: *mut mline_t, fl: *mut fline_t) -> boolean {
    unsafe { todo!("body not yet translated") }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn AM_drawFline(fl: *mut fline_t, color: std::ffi::c_int) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn AM_drawMline(ml: *mut mline_t, color: std::ffi::c_int) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn AM_drawGrid(color: std::ffi::c_int) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn AM_drawWalls() {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn AM_rotate(x: *mut fixed_t, y: *mut fixed_t, a: angle_t) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn AM_drawLineCharacter(
    lineguy: *mut mline_t,
    lineguylines: std::ffi::c_int,
    scale: fixed_t,
    angle: angle_t,
    color: std::ffi::c_int,
    x: fixed_t,
    y: fixed_t,
) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn AM_drawPlayers() {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn AM_drawThings(colors: std::ffi::c_int, colorrange: std::ffi::c_int) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn AM_drawMarks() {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn AM_drawCrosshair(color: std::ffi::c_int) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn AM_Drawer() {
    unsafe { todo!("body not yet translated") }
}
