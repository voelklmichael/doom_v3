use crate::am_map::*;
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
use crate::g_game::*;
use crate::i_system::*;
use crate::i_video::*;
use crate::info::*;
use crate::m_cheat::*;
use crate::m_fixed::*;
use crate::m_random::*;
use crate::p_inter::*;
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
use crate::s_sound::*;
use crate::sounds::*;
use crate::st_lib::*;
use crate::tables::*;
use crate::v_video::*;
use crate::w_wad::*;
use crate::z_zone::*;

pub const ST_HEIGHT: std::ffi::c_int = (32 * SCREEN_MUL);

pub const ST_WIDTH: std::ffi::c_int = SCREENWIDTH;

pub const ST_Y: std::ffi::c_int = (SCREENHEIGHT - ST_HEIGHT);

pub const AutomapState: std::ffi::c_int = 0;
pub const FirstPersonState: std::ffi::c_int = AutomapState + 1;

pub type st_stateenum_t = std::ffi::c_int;

pub const StartChatState: std::ffi::c_int = 0;
pub const WaitDestState: std::ffi::c_int = StartChatState + 1;
pub const GetChatState: std::ffi::c_int = WaitDestState + 1;

pub type st_chatstateenum_t = std::ffi::c_int;

static mut rcsid: [std::ffi::c_char; 51] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        115 as std::ffi::c_char,
        116 as std::ffi::c_char,
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
        54 as std::ffi::c_char,
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

pub const STARTREDPALS: std::ffi::c_int = 1;

pub const STARTBONUSPALS: std::ffi::c_int = 9;

pub const NUMREDPALS: std::ffi::c_int = 8;

pub const NUMBONUSPALS: std::ffi::c_int = 4;

pub const RADIATIONPAL: std::ffi::c_int = 13;

pub const ST_FACEPROBABILITY: std::ffi::c_int = 96;

pub const ST_TOGGLECHAT: std::ffi::c_int = KEY_ENTER;

pub const ST_X: std::ffi::c_int = 0;

pub const ST_X2: std::ffi::c_int = 104;

pub const ST_FX: std::ffi::c_int = 143;

pub const ST_FY: std::ffi::c_int = 169;

pub const ST_TALLNUMWIDTH: std::ffi::c_int = ((*tallnum[(0) as usize]).width);

pub const ST_NUMPAINFACES: std::ffi::c_int = 5;

pub const ST_NUMSTRAIGHTFACES: std::ffi::c_int = 3;

pub const ST_NUMTURNFACES: std::ffi::c_int = 2;

pub const ST_NUMSPECIALFACES: std::ffi::c_int = 3;

pub const ST_FACESTRIDE: std::ffi::c_int =
    ((ST_NUMSTRAIGHTFACES + ST_NUMTURNFACES) + ST_NUMSPECIALFACES);

pub const ST_NUMEXTRAFACES: std::ffi::c_int = 2;

pub const ST_NUMFACES: std::ffi::c_int = ((ST_FACESTRIDE * ST_NUMPAINFACES) + ST_NUMEXTRAFACES);

pub const ST_TURNOFFSET: std::ffi::c_int = (ST_NUMSTRAIGHTFACES);

pub const ST_OUCHOFFSET: std::ffi::c_int = (ST_TURNOFFSET + ST_NUMTURNFACES);

pub const ST_EVILGRINOFFSET: std::ffi::c_int = (ST_OUCHOFFSET + 1);

pub const ST_RAMPAGEOFFSET: std::ffi::c_int = (ST_EVILGRINOFFSET + 1);

pub const ST_GODFACE: std::ffi::c_int = (ST_NUMPAINFACES * ST_FACESTRIDE);

pub const ST_DEADFACE: std::ffi::c_int = (ST_GODFACE + 1);

pub const ST_FACESX: std::ffi::c_int = 143;

pub const ST_FACESY: std::ffi::c_int = 168;

pub const ST_EVILGRINCOUNT: std::ffi::c_int = (2 * TICRATE);

pub const ST_STRAIGHTFACECOUNT: std::ffi::c_int = (TICRATE / 2);

pub const ST_TURNCOUNT: std::ffi::c_int = (1 * TICRATE);

pub const ST_OUCHCOUNT: std::ffi::c_int = (1 * TICRATE);

pub const ST_RAMPAGEDELAY: std::ffi::c_int = (2 * TICRATE);

pub const ST_MUCHPAIN: std::ffi::c_int = 20;

pub const ST_AMMOWIDTH: std::ffi::c_int = 3;

pub const ST_AMMOX: std::ffi::c_int = 44;

pub const ST_AMMOY: std::ffi::c_int = 171;

pub const ST_HEALTHWIDTH: std::ffi::c_int = 3;

pub const ST_HEALTHX: std::ffi::c_int = 90;

pub const ST_HEALTHY: std::ffi::c_int = 171;

pub const ST_ARMSX: std::ffi::c_int = 111;

pub const ST_ARMSY: std::ffi::c_int = 172;

pub const ST_ARMSBGX: std::ffi::c_int = 104;

pub const ST_ARMSBGY: std::ffi::c_int = 168;

pub const ST_ARMSXSPACE: std::ffi::c_int = 12;

pub const ST_ARMSYSPACE: std::ffi::c_int = 10;

pub const ST_FRAGSX: std::ffi::c_int = 138;

pub const ST_FRAGSY: std::ffi::c_int = 171;

pub const ST_FRAGSWIDTH: std::ffi::c_int = 2;

pub const ST_ARMORWIDTH: std::ffi::c_int = 3;

pub const ST_ARMORX: std::ffi::c_int = 221;

pub const ST_ARMORY: std::ffi::c_int = 171;

pub const ST_KEY0WIDTH: std::ffi::c_int = 8;

pub const ST_KEY0HEIGHT: std::ffi::c_int = 5;

pub const ST_KEY0X: std::ffi::c_int = 239;

pub const ST_KEY0Y: std::ffi::c_int = 171;

pub const ST_KEY1WIDTH: std::ffi::c_int = ST_KEY0WIDTH;

pub const ST_KEY1X: std::ffi::c_int = 239;

pub const ST_KEY1Y: std::ffi::c_int = 181;

pub const ST_KEY2WIDTH: std::ffi::c_int = ST_KEY0WIDTH;

pub const ST_KEY2X: std::ffi::c_int = 239;

pub const ST_KEY2Y: std::ffi::c_int = 191;

pub const ST_AMMO0WIDTH: std::ffi::c_int = 3;

pub const ST_AMMO0HEIGHT: std::ffi::c_int = 6;

pub const ST_AMMO0X: std::ffi::c_int = 288;

pub const ST_AMMO0Y: std::ffi::c_int = 173;

pub const ST_AMMO1WIDTH: std::ffi::c_int = ST_AMMO0WIDTH;

pub const ST_AMMO1X: std::ffi::c_int = 288;

pub const ST_AMMO1Y: std::ffi::c_int = 179;

pub const ST_AMMO2WIDTH: std::ffi::c_int = ST_AMMO0WIDTH;

pub const ST_AMMO2X: std::ffi::c_int = 288;

pub const ST_AMMO2Y: std::ffi::c_int = 191;

pub const ST_AMMO3WIDTH: std::ffi::c_int = ST_AMMO0WIDTH;

pub const ST_AMMO3X: std::ffi::c_int = 288;

pub const ST_AMMO3Y: std::ffi::c_int = 185;

pub const ST_MAXAMMO0WIDTH: std::ffi::c_int = 3;

pub const ST_MAXAMMO0HEIGHT: std::ffi::c_int = 5;

pub const ST_MAXAMMO0X: std::ffi::c_int = 314;

pub const ST_MAXAMMO0Y: std::ffi::c_int = 173;

pub const ST_MAXAMMO1WIDTH: std::ffi::c_int = ST_MAXAMMO0WIDTH;

pub const ST_MAXAMMO1X: std::ffi::c_int = 314;

pub const ST_MAXAMMO1Y: std::ffi::c_int = 179;

pub const ST_MAXAMMO2WIDTH: std::ffi::c_int = ST_MAXAMMO0WIDTH;

pub const ST_MAXAMMO2X: std::ffi::c_int = 314;

pub const ST_MAXAMMO2Y: std::ffi::c_int = 191;

pub const ST_MAXAMMO3WIDTH: std::ffi::c_int = ST_MAXAMMO0WIDTH;

pub const ST_MAXAMMO3X: std::ffi::c_int = 314;

pub const ST_MAXAMMO3Y: std::ffi::c_int = 185;

pub const ST_WEAPON0X: std::ffi::c_int = 110;

pub const ST_WEAPON0Y: std::ffi::c_int = 172;

pub const ST_WEAPON1X: std::ffi::c_int = 122;

pub const ST_WEAPON1Y: std::ffi::c_int = 172;

pub const ST_WEAPON2X: std::ffi::c_int = 134;

pub const ST_WEAPON2Y: std::ffi::c_int = 172;

pub const ST_WEAPON3X: std::ffi::c_int = 110;

pub const ST_WEAPON3Y: std::ffi::c_int = 181;

pub const ST_WEAPON4X: std::ffi::c_int = 122;

pub const ST_WEAPON4Y: std::ffi::c_int = 181;

pub const ST_WEAPON5X: std::ffi::c_int = 134;

pub const ST_WEAPON5Y: std::ffi::c_int = 181;

pub const ST_WPNSX: std::ffi::c_int = 109;

pub const ST_WPNSY: std::ffi::c_int = 191;

pub const ST_DETHX: std::ffi::c_int = 109;

pub const ST_DETHY: std::ffi::c_int = 191;

pub const ST_MSGTEXTX: std::ffi::c_int = 0;

pub const ST_MSGTEXTY: std::ffi::c_int = 0;

pub const ST_MSGWIDTH: std::ffi::c_int = 52;

pub const ST_MSGHEIGHT: std::ffi::c_int = 1;

pub const ST_OUTTEXTX: std::ffi::c_int = 0;

pub const ST_OUTTEXTY: std::ffi::c_int = 6;

pub const ST_OUTWIDTH: std::ffi::c_int = 52;

pub const ST_OUTHEIGHT: std::ffi::c_int = 1;

/* TODO: unparsed macro value, references an identifier with no known definition anywhere in this module's visible corpus (likely dead code never expanded in the original C):
#define ST_MAPWIDTH \
    (strlen(mapnames[(gameepisode-1)*9+(gamemap-1)]))
*/

/* TODO: unparsed macro value, references an identifier with no known definition anywhere in this module's visible corpus (likely dead code never expanded in the original C):
#define ST_MAPTITLEX \
    (SCREENWIDTH - ST_MAPWIDTH * ST_CHATFONTWIDTH)
*/

pub const ST_MAPTITLEY: std::ffi::c_int = 0;

pub const ST_MAPHEIGHT: std::ffi::c_int = 1;

static mut plyr: *mut player_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut st_firsttime: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut veryfirsttime: std::ffi::c_int = unsafe { 1 };

static mut lu_palette: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut st_clock: std::ffi::c_uint = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut st_msgcounter: std::ffi::c_int = unsafe { 0 };

static mut st_chatstate: st_chatstateenum_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut st_gamestate: st_stateenum_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut st_statusbaron: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut st_chat: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut st_oldchat: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut st_cursoron: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut st_notdeathmatch: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut st_armson: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut st_fragson: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut sbar: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut tallnum: [*mut patch_t; (10) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut tallpercent: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut shortnum: [*mut patch_t; (10) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut keys: [*mut patch_t; (NUMCARDS) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut faces: [*mut patch_t; (ST_NUMFACES) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut faceback: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut armsbg: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut arms: [[*mut patch_t; (2) as usize]; (6) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut w_ready: st_number_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut w_frags: st_number_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut w_health: st_percent_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut w_armsbg: st_binicon_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut w_arms: [st_multicon_t; (6) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut w_faces: st_multicon_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut w_keyboxes: [st_multicon_t; (3) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut w_armor: st_percent_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut w_ammo: [st_number_t; (4) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut w_maxammo: [st_number_t; (4) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut st_fragscount: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut st_oldhealth: std::ffi::c_int = unsafe { (-(1)) };

static mut oldweaponsowned: [boolean; (NUMWEAPONS) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut st_facecount: std::ffi::c_int = unsafe { 0 };

static mut st_faceindex: std::ffi::c_int = unsafe { 0 };

static mut keyboxes: [std::ffi::c_int; (3) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut st_randomnumber: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut cheat_mus_seq: [std::ffi::c_uchar; 9] =
    unsafe { [0xb2, 0x26, 0xb6, 0xae, 0xea, 1, 0, 0, 0xff] };

pub static mut cheat_choppers_seq: [std::ffi::c_uchar; 11] = unsafe {
    [
        0xb2, 0x26, 0xe2, 0x32, 0xf6, 0x2a, 0x2a, 0xa6, 0x6a, 0xea, 0xff,
    ]
};

pub static mut cheat_god_seq: [std::ffi::c_uchar; 6] =
    unsafe { [0xb2, 0x26, 0x26, 0xaa, 0x26, 0xff] };

pub static mut cheat_ammo_seq: [std::ffi::c_uchar; 6] =
    unsafe { [0xb2, 0x26, 0xf2, 0x66, 0xa2, 0xff] };

pub static mut cheat_ammonokey_seq: [std::ffi::c_uchar; 5] =
    unsafe { [0xb2, 0x26, 0x66, 0xa2, 0xff] };

pub static mut cheat_noclip_seq: [std::ffi::c_uchar; 11] = unsafe {
    [
        0xb2, 0x26, 0xea, 0x2a, 0xb2, 0xea, 0x2a, 0xf6, 0x2a, 0x26, 0xff,
    ]
};

pub static mut cheat_commercial_noclip_seq: [std::ffi::c_uchar; 7] =
    unsafe { [0xb2, 0x26, 0xe2, 0x36, 0xb2, 0x2a, 0xff] };

pub static mut cheat_powerup_seq: [[std::ffi::c_uchar; 10]; 7] = unsafe {
    [
        [0xb2, 0x26, 0x62, 0xa6, 0x32, 0xf6, 0x36, 0x26, 0x6e, 0xff],
        [0xb2, 0x26, 0x62, 0xa6, 0x32, 0xf6, 0x36, 0x26, 0xea, 0xff],
        [0xb2, 0x26, 0x62, 0xa6, 0x32, 0xf6, 0x36, 0x26, 0xb2, 0xff],
        [0xb2, 0x26, 0x62, 0xa6, 0x32, 0xf6, 0x36, 0x26, 0x6a, 0xff],
        [0xb2, 0x26, 0x62, 0xa6, 0x32, 0xf6, 0x36, 0x26, 0xa2, 0xff],
        [0xb2, 0x26, 0x62, 0xa6, 0x32, 0xf6, 0x36, 0x26, 0x36, 0xff],
        [
            0xb2,
            0x26,
            0x62,
            0xa6,
            0x32,
            0xf6,
            0x36,
            0x26,
            0xff,
            std::mem::zeroed(),
        ],
    ]
};

pub static mut cheat_clev_seq: [std::ffi::c_uchar; 10] =
    unsafe { [0xb2, 0x26, 0xe2, 0x36, 0xa6, 0x6e, 1, 0, 0, 0xff] };

pub static mut cheat_mypos_seq: [std::ffi::c_uchar; 8] =
    unsafe { [0xb2, 0x26, 0xb6, 0xba, 0x2a, 0xf6, 0xea, 0xff] };

pub static mut cheat_mus: cheatseq_t = unsafe {
    cheatseq_t {
        sequence: cheat_mus_seq.as_mut_ptr(),
        p: std::ptr::null_mut(),
    }
};

pub static mut cheat_god: cheatseq_t = unsafe {
    cheatseq_t {
        sequence: cheat_god_seq.as_mut_ptr(),
        p: std::ptr::null_mut(),
    }
};

pub static mut cheat_ammo: cheatseq_t = unsafe {
    cheatseq_t {
        sequence: cheat_ammo_seq.as_mut_ptr(),
        p: std::ptr::null_mut(),
    }
};

pub static mut cheat_ammonokey: cheatseq_t = unsafe {
    cheatseq_t {
        sequence: cheat_ammonokey_seq.as_mut_ptr(),
        p: std::ptr::null_mut(),
    }
};

pub static mut cheat_noclip: cheatseq_t = unsafe {
    cheatseq_t {
        sequence: cheat_noclip_seq.as_mut_ptr(),
        p: std::ptr::null_mut(),
    }
};

pub static mut cheat_commercial_noclip: cheatseq_t = unsafe {
    cheatseq_t {
        sequence: cheat_commercial_noclip_seq.as_mut_ptr(),
        p: std::ptr::null_mut(),
    }
};

pub static mut cheat_powerup: [cheatseq_t; 7] = unsafe {
    [
        cheatseq_t {
            sequence: cheat_powerup_seq[(0) as usize],
            p: std::ptr::null_mut(),
        },
        cheatseq_t {
            sequence: cheat_powerup_seq[(1) as usize],
            p: std::ptr::null_mut(),
        },
        cheatseq_t {
            sequence: cheat_powerup_seq[(2) as usize],
            p: std::ptr::null_mut(),
        },
        cheatseq_t {
            sequence: cheat_powerup_seq[(3) as usize],
            p: std::ptr::null_mut(),
        },
        cheatseq_t {
            sequence: cheat_powerup_seq[(4) as usize],
            p: std::ptr::null_mut(),
        },
        cheatseq_t {
            sequence: cheat_powerup_seq[(5) as usize],
            p: std::ptr::null_mut(),
        },
        cheatseq_t {
            sequence: cheat_powerup_seq[(6) as usize],
            p: std::ptr::null_mut(),
        },
    ]
};

pub static mut cheat_choppers: cheatseq_t = unsafe {
    cheatseq_t {
        sequence: cheat_choppers_seq.as_mut_ptr(),
        p: std::ptr::null_mut(),
    }
};

pub static mut cheat_clev: cheatseq_t = unsafe {
    cheatseq_t {
        sequence: cheat_clev_seq.as_mut_ptr(),
        p: std::ptr::null_mut(),
    }
};

pub static mut cheat_mypos: cheatseq_t = unsafe {
    cheatseq_t {
        sequence: cheat_mypos_seq.as_mut_ptr(),
        p: std::ptr::null_mut(),
    }
};

unsafe extern "C" {
    pub static mut mapnames: *mut *mut std::ffi::c_char;
}

pub unsafe extern "C" fn ST_refreshBackground() {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //
        //     if (st_statusbaron)
        //     {
        // 	V_DrawPatch(ST_X, 0, BG, sbar);
        //
        // 	if (netgame)
        // 	    V_DrawPatch(ST_FX, 0, BG, faceback);
        //
        // 	V_CopyRect(ST_X, 0, BG, ST_WIDTH, ST_HEIGHT, ST_X, ST_Y, FG);
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn ST_Responder(mut ev: *mut event_t) -> boolean {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //   // Filter automap on/off.
        //   if (ev->type == ev_keyup
        //       && ((ev->data1 & 0xffff0000) == AM_MSGHEADER))
        //   {
        //     switch(ev->data1)
        //     {
        //       case AM_MSGENTERED:
        // 	st_gamestate = AutomapState;
        // 	st_firsttime = true;
        // 	break;
        //
        //       case AM_MSGEXITED:
        // 	//	fprintf(stderr, "AM exited\n");
        // 	st_gamestate = FirstPersonState;
        // 	break;
        //     }
        //   }
        //
        //   // if a user keypress...
        //   else if (ev->type == ev_keydown)
        //   {
        //     if (!netgame)
        //     {
        //       // b. - enabled for more debug fun.
        //       // if (gameskill != sk_nightmare) {
        //
        //       // 'dqd' cheat for toggleable god mode
        //       if (cht_CheckCheat(&cheat_god, ev->data1))
        //       {
        // 	plyr->cheats ^= CF_GODMODE;
        // 	if (plyr->cheats & CF_GODMODE)
        // 	{
        // 	  if (plyr->mo)
        // 	    plyr->mo->health = 100;
        //
        // 	  plyr->health = 100;
        // 	  plyr->message = STSTR_DQDON;
        // 	}
        // 	else
        // 	  plyr->message = STSTR_DQDOFF;
        //       }
        //       // 'fa' cheat for killer fucking arsenal
        //       else if (cht_CheckCheat(&cheat_ammonokey, ev->data1))
        //       {
        // 	plyr->armorpoints = 200;
        // 	plyr->armortype = 2;
        //
        // 	for (i=0;i<NUMWEAPONS;i++)
        // 	  plyr->weaponowned[i] = true;
        //
        // 	for (i=0;i<NUMAMMO;i++)
        // 	  plyr->ammo[i] = plyr->maxammo[i];
        //
        // 	plyr->message = STSTR_FAADDED;
        //       }
        //       // 'kfa' cheat for key full ammo
        //       else if (cht_CheckCheat(&cheat_ammo, ev->data1))
        //       {
        // 	plyr->armorpoints = 200;
        // 	plyr->armortype = 2;
        //
        // 	for (i=0;i<NUMWEAPONS;i++)
        // 	  plyr->weaponowned[i] = true;
        //
        // 	for (i=0;i<NUMAMMO;i++)
        // 	  plyr->ammo[i] = plyr->maxammo[i];
        //
        // 	for (i=0;i<NUMCARDS;i++)
        // 	  plyr->cards[i] = true;
        //
        // 	plyr->message = STSTR_KFAADDED;
        //       }
        //       // 'mus' cheat for changing music
        //       else if (cht_CheckCheat(&cheat_mus, ev->data1))
        //       {
        //
        // 	char	buf[3];
        // 	int		musnum;
        //
        // 	plyr->message = STSTR_MUS;
        // 	cht_GetParam(&cheat_mus, buf);
        //
        // 	if (gamemode == commercial)
        // 	{
        // 	  musnum = mus_runnin + (buf[0]-'0')*10 + buf[1]-'0' - 1;
        //
        // 	  if (((buf[0]-'0')*10 + buf[1]-'0') > 35)
        // 	    plyr->message = STSTR_NOMUS;
        // 	  else
        // 	    S_ChangeMusic(musnum, 1);
        // 	}
        // 	else
        // 	{
        // 	  musnum = mus_e1m1 + (buf[0]-'1')*9 + (buf[1]-'1');
        //
        // 	  if (((buf[0]-'1')*9 + buf[1]-'1') > 31)
        // 	    plyr->message = STSTR_NOMUS;
        // 	  else
        // 	    S_ChangeMusic(musnum, 1);
        // 	}
        //       }
        //       // Simplified, accepting both "noclip" and "idspispopd".
        //       // no clipping mode cheat
        //       else if ( cht_CheckCheat(&cheat_noclip, ev->data1)
        // 		|| cht_CheckCheat(&cheat_commercial_noclip,ev->data1) )
        //       {
        // 	plyr->cheats ^= CF_NOCLIP;
        //
        // 	if (plyr->cheats & CF_NOCLIP)
        // 	  plyr->message = STSTR_NCON;
        // 	else
        // 	  plyr->message = STSTR_NCOFF;
        //       }
        //       // 'behold?' power-up cheats
        //       for (i=0;i<6;i++)
        //       {
        // 	if (cht_CheckCheat(&cheat_powerup[i], ev->data1))
        // 	{
        // 	  if (!plyr->powers[i])
        // 	    P_GivePower( plyr, i);
        // 	  else if (i!=pw_strength)
        // 	    plyr->powers[i] = 1;
        // 	  else
        // 	    plyr->powers[i] = 0;
        //
        // 	  plyr->message = STSTR_BEHOLDX;
        // 	}
        //       }
        //
        //       // 'behold' power-up menu
        //       if (cht_CheckCheat(&cheat_powerup[6], ev->data1))
        //       {
        // 	plyr->message = STSTR_BEHOLD;
        //       }
        //       // 'choppers' invulnerability & chainsaw
        //       else if (cht_CheckCheat(&cheat_choppers, ev->data1))
        //       {
        // 	plyr->weaponowned[wp_chainsaw] = true;
        // 	plyr->powers[pw_invulnerability] = true;
        // 	plyr->message = STSTR_CHOPPERS;
        //       }
        //       // 'mypos' for player position
        //       else if (cht_CheckCheat(&cheat_mypos, ev->data1))
        //       {
        // 	static char	buf[ST_MSGWIDTH];
        // 	sprintf(buf, "ang=0x%x;x,y=(0x%x,0x%x)",
        // 		players[consoleplayer].mo->angle,
        // 		players[consoleplayer].mo->x,
        // 		players[consoleplayer].mo->y);
        // 	plyr->message = buf;
        //       }
        //     }
        //
        //     // 'clev' change-level cheat
        //     if (cht_CheckCheat(&cheat_clev, ev->data1))
        //     {
        //       char		buf[3];
        //       int		epsd;
        //       int		map;
        //
        //       cht_GetParam(&cheat_clev, buf);
        //
        //       if (gamemode == commercial)
        //       {
        // 	epsd = 0;
        // 	map = (buf[0] - '0')*10 + buf[1] - '0';
        //       }
        //       else
        //       {
        // 	epsd = buf[0] - '0';
        // 	map = buf[1] - '0';
        //       }
        //
        //       // Catch invalid maps.
        //       if (epsd < 1)
        // 	return false;
        //
        //       if (map < 1)
        // 	return false;
        //
        //       // Ohmygod - this is not going to work.
        //       if ((gamemode == retail)
        // 	  && ((epsd > 4) || (map > 9)))
        // 	return false;
        //
        //       if ((gamemode == registered)
        // 	  && ((epsd > 3) || (map > 9)))
        // 	return false;
        //
        //       if ((gamemode == shareware)
        // 	  && ((epsd > 1) || (map > 9)))
        // 	return false;
        //
        //       if ((gamemode == commercial)
        // 	&& (( epsd > 1) || (map > 34)))
        // 	return false;
        //
        //       // So be it.
        //       plyr->message = STSTR_CLEV;
        //       G_DeferedInitNew(gameskill, epsd, map);
        //     }
        //   }
        todo!("if statement not yet translated");
        return false_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn ST_calcPainOffset() -> std::ffi::c_int {
    unsafe {
        let mut health: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        static mut lastcalc: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        static mut oldhealth: std::ffi::c_int = unsafe { (-(1)) };
        health = (if ((*plyr).health > 100) {
            100
        } else {
            (*plyr).health
        });
        // TODO: if statement not yet translated:
        //
        //
        //     if (health != oldhealth)
        //     {
        // 	lastcalc = ST_FACESTRIDE * (((100 - health) * ST_NUMPAINFACES) / 101);
        // 	oldhealth = health;
        //     }
        todo!("if statement not yet translated");
        return lastcalc;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn ST_updateFaceWidget() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut badguyangle: angle_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut diffang: angle_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        static mut lastattackdown: std::ffi::c_int = unsafe { (-(1)) };
        static mut priority: std::ffi::c_int = unsafe { 0 };
        let mut doevilgrin: boolean = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (priority < 10)
        //     {
        // 	// dead
        // 	if (!plyr->health)
        // 	{
        // 	    priority = 9;
        // 	    st_faceindex = ST_DEADFACE;
        // 	    st_facecount = 1;
        // 	}
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (priority < 9)
        //     {
        // 	if (plyr->bonuscount)
        // 	{
        // 	    // picking up bonus
        // 	    doevilgrin = false;
        //
        // 	    for (i=0;i<NUMWEAPONS;i++)
        // 	    {
        // 		if (oldweaponsowned[i] != plyr->weaponowned[i])
        // 		{
        // 		    doevilgrin = true;
        // 		    oldweaponsowned[i] = plyr->weaponowned[i];
        // 		}
        // 	    }
        // 	    if (doevilgrin)
        // 	    {
        // 		// evil grin if just picked up weapon
        // 		priority = 8;
        // 		st_facecount = ST_EVILGRINCOUNT;
        // 		st_faceindex = ST_calcPainOffset() + ST_EVILGRINOFFSET;
        // 	    }
        // 	}
        //
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (priority < 8)
        //     {
        // 	if (plyr->damagecount
        // 	    && plyr->attacker
        // 	    && plyr->attacker != plyr->mo)
        // 	{
        // 	    // being attacked
        // 	    priority = 7;
        //
        // 	    if (plyr->health - st_oldhealth > ST_MUCHPAIN)
        // 	    {
        // 		st_facecount = ST_TURNCOUNT;
        // 		st_faceindex = ST_calcPainOffset() + ST_OUCHOFFSET;
        // 	    }
        // 	    else
        // 	    {
        // 		badguyangle = R_PointToAngle2(plyr->mo->x,
        // 					      plyr->mo->y,
        // 					      plyr->attacker->x,
        // 					      plyr->attacker->y);
        //
        // 		if (badguyangle > plyr->mo->angle)
        // 		{
        // 		    // whether right or left
        // 		    diffang = badguyangle - plyr->mo->angle;
        // 		    i = diffang > ANG180;
        // 		}
        // 		else
        // 		{
        // 		    // whether left or right
        // 		    diffang = plyr->mo->angle - badguyangle;
        // 		    i = diffang <= ANG180;
        // 		} // confusing, aint it?
        //
        //
        // 		st_facecount = ST_TURNCOUNT;
        // 		st_faceindex = ST_calcPainOffset();
        //
        // 		if (diffang < ANG45)
        // 		{
        // 		    // head-on
        // 		    st_faceindex += ST_RAMPAGEOFFSET;
        // 		}
        // 		else if (i)
        // 		{
        // 		    // turn face right
        // 		    st_faceindex += ST_TURNOFFSET;
        // 		}
        // 		else
        // 		{
        // 		    // turn face left
        // 		    st_faceindex += ST_TURNOFFSET+1;
        // 		}
        // 	    }
        // 	}
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (priority < 7)
        //     {
        // 	// getting hurt because of your own damn stupidity
        // 	if (plyr->damagecount)
        // 	{
        // 	    if (plyr->health - st_oldhealth > ST_MUCHPAIN)
        // 	    {
        // 		priority = 7;
        // 		st_facecount = ST_TURNCOUNT;
        // 		st_faceindex = ST_calcPainOffset() + ST_OUCHOFFSET;
        // 	    }
        // 	    else
        // 	    {
        // 		priority = 6;
        // 		st_facecount = ST_TURNCOUNT;
        // 		st_faceindex = ST_calcPainOffset() + ST_RAMPAGEOFFSET;
        // 	    }
        //
        // 	}
        //
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (priority < 6)
        //     {
        // 	// rapid firing
        // 	if (plyr->attackdown)
        // 	{
        // 	    if (lastattackdown==-1)
        // 		lastattackdown = ST_RAMPAGEDELAY;
        // 	    else if (!--lastattackdown)
        // 	    {
        // 		priority = 5;
        // 		st_faceindex = ST_calcPainOffset() + ST_RAMPAGEOFFSET;
        // 		st_facecount = 1;
        // 		lastattackdown = 1;
        // 	    }
        // 	}
        // 	else
        // 	    lastattackdown = -1;
        //
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (priority < 5)
        //     {
        // 	// invulnerability
        // 	if ((plyr->cheats & CF_GODMODE)
        // 	    || plyr->powers[pw_invulnerability])
        // 	{
        // 	    priority = 4;
        //
        // 	    st_faceindex = ST_GODFACE;
        // 	    st_facecount = 1;
        //
        // 	}
        //
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // look left or look right if the facecount has timed out
        //     if (!st_facecount)
        //     {
        // 	st_faceindex = ST_calcPainOffset() + (st_randomnumber % 3);
        // 	st_facecount = ST_STRAIGHTFACECOUNT;
        // 	priority = 0;
        //     }
        todo!("if statement not yet translated");
        {
            let __macro_tmp = st_facecount;
            st_facecount -= 1;
            __macro_tmp
        };
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn ST_updateWidgets() {
    unsafe {
        static mut largeammo: std::ffi::c_int = unsafe { 1994 };
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     // must redirect the pointer if the ready weapon has changed.
        //     //  if (w_ready.data != plyr->readyweapon)
        //     //  {
        //     if (weaponinfo[plyr->readyweapon].ammo == am_noammo)
        // 	w_ready.num = &largeammo;
        //     else
        // 	w_ready.num = &plyr->ammo[weaponinfo[plyr->readyweapon].ammo];
        todo!("if statement not yet translated");
        w_ready.data = (*plyr).readyweapon;
        // TODO: for statement not yet translated:
        //
        //
        //     // if (*w_ready.on)
        //     //  STlib_updateNum(&w_ready, true);
        //     // refresh weapon change
        //     //  }
        //
        //     // update keycard multiple widgets
        //     for (i=0;i<3;i++)
        //     {
        // 	keyboxes[i] = plyr->cards[i] ? i : -1;
        //
        // 	if (plyr->cards[i+3])
        // 	    keyboxes[i] = i+3;
        //     }
        todo!("for statement not yet translated");
        ST_updateFaceWidget();
        st_notdeathmatch = (((deathmatch) == 0) as std::ffi::c_int);
        st_armson = (st_statusbaron && (((deathmatch) == 0) as std::ffi::c_int));
        st_fragson = (deathmatch && st_statusbaron);
        st_fragscount = 0;
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<MAXPLAYERS ; i++)
        //     {
        // 	if (i != consoleplayer)
        // 	    st_fragscount += plyr->frags[i];
        // 	else
        // 	    st_fragscount -= plyr->frags[i];
        //     }
        todo!("for statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // get rid of chat window if up because of message
        //     if (!--st_msgcounter)
        // 	st_chat = st_oldchat;
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn ST_Ticker() {
    unsafe {
        {
            let __macro_tmp = st_clock;
            st_clock += 1;
            __macro_tmp
        };
        st_randomnumber = M_Random();
        ST_updateWidgets();
        st_oldhealth = (*plyr).health;
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

static mut st_palette: std::ffi::c_int = unsafe { 0 };

pub unsafe extern "C" fn ST_doPaletteStuff() {
    unsafe {
        let mut palette: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut pal: *mut byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut cnt: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut bzc: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        cnt = (*plyr).damagecount;
        // TODO: if statement not yet translated:
        //
        //
        //     if (plyr->powers[pw_strength])
        //     {
        // 	// slowly fade the berzerk out
        //   	bzc = 12 - (plyr->powers[pw_strength]>>6);
        //
        // 	if (bzc > cnt)
        // 	    cnt = bzc;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (cnt)
        //     {
        // 	palette = (cnt+7)>>3;
        //
        // 	if (palette >= NUMREDPALS)
        // 	    palette = NUMREDPALS-1;
        //
        // 	palette += STARTREDPALS;
        //     }
        //
        //     else if (plyr->bonuscount)
        //     {
        // 	palette = (plyr->bonuscount+7)>>3;
        //
        // 	if (palette >= NUMBONUSPALS)
        // 	    palette = NUMBONUSPALS-1;
        //
        // 	palette += STARTBONUSPALS;
        //     }
        //
        //     else if ( plyr->powers[pw_ironfeet] > 4*32
        // 	      || plyr->powers[pw_ironfeet]&8)
        // 	palette = RADIATIONPAL;
        //     else
        // 	palette = 0;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (palette != st_palette)
        //     {
        // 	st_palette = palette;
        // 	pal = (byte *) W_CacheLumpNum (lu_palette, PU_CACHE)+palette*768;
        // 	I_SetPalette (pal);
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn ST_drawWidgets(mut refresh: boolean) {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        st_armson = (st_statusbaron && (((deathmatch) == 0) as std::ffi::c_int));
        st_fragson = (deathmatch && st_statusbaron);
        STlib_updateNum(
            (&(w_ready) as *const st_number_t as *mut st_number_t),
            refresh,
        );
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0;i<4;i++)
        //     {
        // 	STlib_updateNum(&w_ammo[i], refresh);
        // 	STlib_updateNum(&w_maxammo[i], refresh);
        //     }
        todo!("for statement not yet translated");
        STlib_updatePercent(
            (&(w_health) as *const st_percent_t as *mut st_percent_t),
            refresh,
        );
        STlib_updatePercent(
            (&(w_armor) as *const st_percent_t as *mut st_percent_t),
            refresh,
        );
        STlib_updateBinIcon(
            (&(w_armsbg) as *const st_binicon_t as *mut st_binicon_t),
            refresh,
        );
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0;i<6;i++)
        // 	STlib_updateMultIcon(&w_arms[i], refresh);
        todo!("for statement not yet translated");
        STlib_updateMultIcon(
            (&(w_faces) as *const st_multicon_t as *mut st_multicon_t),
            refresh,
        );
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0;i<3;i++)
        // 	STlib_updateMultIcon(&w_keyboxes[i], refresh);
        todo!("for statement not yet translated");
        STlib_updateNum(
            (&(w_frags) as *const st_number_t as *mut st_number_t),
            refresh,
        );
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn ST_doRefresh() {
    unsafe {
        st_firsttime = false_;
        ST_refreshBackground();
        ST_drawWidgets(true_);
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn ST_diffDraw() {
    unsafe {
        ST_drawWidgets(false_);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn ST_Drawer(mut fullscreen: boolean, mut refresh: boolean) {
    unsafe {
        st_statusbaron = ((((fullscreen) == 0) as std::ffi::c_int) || automapactive);
        st_firsttime = (st_firsttime || refresh);
        ST_doPaletteStuff();
        // TODO: if statement not yet translated:
        //
        //
        //     // If just after ST_Start(), refresh all
        //     if (st_firsttime) ST_doRefresh();
        //     // Otherwise, update as little as possible
        //     else ST_diffDraw();
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn ST_loadGraphics() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut j: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut facenum: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut namebuf: [std::ffi::c_char; (9) as usize] = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: for statement not yet translated:
        //
        //
        //     // Load the numbers, tall and short
        //     for (i=0;i<10;i++)
        //     {
        // 	sprintf(namebuf, "STTNUM%d", i);
        // 	tallnum[i] = (patch_t *) W_CacheLumpName(namebuf, PU_STATIC);
        //
        // 	sprintf(namebuf, "STYSNUM%d", i);
        // 	shortnum[i] = (patch_t *) W_CacheLumpName(namebuf, PU_STATIC);
        //     }
        todo!("for statement not yet translated");
        tallpercent = ((W_CacheLumpName((c"STTPRCNT").as_ptr(), PU_STATIC)) as *mut patch_t);
        // TODO: for statement not yet translated:
        //
        //
        //     // key cards
        //     for (i=0;i<NUMCARDS;i++)
        //     {
        // 	sprintf(namebuf, "STKEYS%d", i);
        // 	keys[i] = (patch_t *) W_CacheLumpName(namebuf, PU_STATIC);
        //     }
        todo!("for statement not yet translated");
        armsbg = ((W_CacheLumpName((c"STARMS").as_ptr(), PU_STATIC)) as *mut patch_t);
        // TODO: for statement not yet translated:
        //
        //
        //     // arms ownership widgets
        //     for (i=0;i<6;i++)
        //     {
        // 	sprintf(namebuf, "STGNUM%d", i+2);
        //
        // 	// gray #
        // 	arms[i][0] = (patch_t *) W_CacheLumpName(namebuf, PU_STATIC);
        //
        // 	// yellow #
        // 	arms[i][1] = shortnum[i+2];
        //     }
        todo!("for statement not yet translated");
        sprintf(namebuf, (c"STFB%d").as_ptr(), consoleplayer);
        faceback = ((W_CacheLumpName(namebuf, PU_STATIC)) as *mut patch_t);
        sbar = ((W_CacheLumpName((c"STBAR").as_ptr(), PU_STATIC)) as *mut patch_t);
        facenum = 0;
        // TODO: for statement not yet translated:
        //
        //     for (i=0;i<ST_NUMPAINFACES;i++)
        //     {
        // 	for (j=0;j<ST_NUMSTRAIGHTFACES;j++)
        // 	{
        // 	    sprintf(namebuf, "STFST%d%d", i, j);
        // 	    faces[facenum++] = W_CacheLumpName(namebuf, PU_STATIC);
        // 	}
        // 	sprintf(namebuf, "STFTR%d0", i);	// turn right
        // 	faces[facenum++] = W_CacheLumpName(namebuf, PU_STATIC);
        // 	sprintf(namebuf, "STFTL%d0", i);	// turn left
        // 	faces[facenum++] = W_CacheLumpName(namebuf, PU_STATIC);
        // 	sprintf(namebuf, "STFOUCH%d", i);	// ouch!
        // 	faces[facenum++] = W_CacheLumpName(namebuf, PU_STATIC);
        // 	sprintf(namebuf, "STFEVL%d", i);	// evil grin ;)
        // 	faces[facenum++] = W_CacheLumpName(namebuf, PU_STATIC);
        // 	sprintf(namebuf, "STFKILL%d", i);	// pissed off
        // 	faces[facenum++] = W_CacheLumpName(namebuf, PU_STATIC);
        //     }
        todo!("for statement not yet translated");
        faces[({
            let __macro_tmp = facenum;
            facenum += 1;
            __macro_tmp
        }) as usize] = W_CacheLumpName((c"STFGOD0").as_ptr(), PU_STATIC);
        faces[({
            let __macro_tmp = facenum;
            facenum += 1;
            __macro_tmp
        }) as usize] = W_CacheLumpName((c"STFDEAD0").as_ptr(), PU_STATIC);
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn ST_loadData() {
    unsafe {
        lu_palette = W_GetNumForName((c"PLAYPAL").as_ptr());
        ST_loadGraphics();
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn ST_unloadGraphics() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: for statement not yet translated:
        //
        //
        //     // unload the numbers, tall and short
        //     for (i=0;i<10;i++)
        //     {
        // 	Z_ChangeTag(tallnum[i], PU_CACHE);
        // 	Z_ChangeTag(shortnum[i], PU_CACHE);
        //     }
        todo!("for statement not yet translated");
        Z_ChangeTag(tallpercent, PU_CACHE);
        Z_ChangeTag(armsbg, PU_CACHE);
        // TODO: for statement not yet translated:
        //
        //
        //     // unload gray #'s
        //     for (i=0;i<6;i++)
        // 	Z_ChangeTag(arms[i][0], PU_CACHE);
        todo!("for statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //
        //     // unload the key cards
        //     for (i=0;i<NUMCARDS;i++)
        // 	Z_ChangeTag(keys[i], PU_CACHE);
        todo!("for statement not yet translated");
        Z_ChangeTag(sbar, PU_CACHE);
        Z_ChangeTag(faceback, PU_CACHE);
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0;i<ST_NUMFACES;i++)
        // 	Z_ChangeTag(faces[i], PU_CACHE);
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        //     // Note: nobody ain't seen no unloading
        //     //   of stminus yet. Dude.
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn ST_unloadData() {
    unsafe {
        ST_unloadGraphics();
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn ST_initData() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        st_firsttime = true_;
        plyr = (&(players[(consoleplayer) as usize]) as *const _ as *mut _);
        st_clock = 0;
        st_chatstate = StartChatState;
        st_gamestate = FirstPersonState;
        st_statusbaron = true_;
        st_oldchat = st_chat = false_;
        st_cursoron = false_;
        st_faceindex = 0;
        st_palette = (-(1));
        st_oldhealth = (-(1));
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0;i<NUMWEAPONS;i++)
        // 	oldweaponsowned[i] = plyr->weaponowned[i];
        todo!("for statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0;i<3;i++)
        // 	keyboxes[i] = -1;
        todo!("for statement not yet translated");
        STlib_init();
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn ST_createWidgets() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        STlib_initNum(
            (&(w_ready) as *const st_number_t as *mut st_number_t),
            ST_AMMOX,
            ST_AMMOY,
            tallnum,
            (&((*plyr).ammo[(weaponinfo[((*plyr).readyweapon) as usize].ammo) as usize]) as *const _
                as *mut _),
            (&(st_statusbaron) as *const boolean as *mut boolean),
            ST_AMMOWIDTH,
        );
        w_ready.data = (*plyr).readyweapon;
        STlib_initPercent(
            (&(w_health) as *const st_percent_t as *mut st_percent_t),
            ST_HEALTHX,
            ST_HEALTHY,
            tallnum,
            (&((*plyr).health) as *const _ as *mut _),
            (&(st_statusbaron) as *const boolean as *mut boolean),
            tallpercent,
        );
        STlib_initBinIcon(
            (&(w_armsbg) as *const st_binicon_t as *mut st_binicon_t),
            ST_ARMSBGX,
            ST_ARMSBGY,
            armsbg,
            (&(st_notdeathmatch) as *const boolean as *mut boolean),
            (&(st_statusbaron) as *const boolean as *mut boolean),
        );
        // TODO: for statement not yet translated:
        //
        //
        //     // weapons owned
        //     for(i=0;i<6;i++)
        //     {
        // 	STlib_initMultIcon(&w_arms[i],
        // 			   ST_ARMSX+(i%3)*ST_ARMSXSPACE,
        // 			   ST_ARMSY+(i/3)*ST_ARMSYSPACE,
        // 			   arms[i], (int *) &plyr->weaponowned[i+1],
        // 			   &st_armson);
        //     }
        todo!("for statement not yet translated");
        STlib_initNum(
            (&(w_frags) as *const st_number_t as *mut st_number_t),
            ST_FRAGSX,
            ST_FRAGSY,
            tallnum,
            (&(st_fragscount) as *const std::ffi::c_int as *mut std::ffi::c_int),
            (&(st_fragson) as *const boolean as *mut boolean),
            ST_FRAGSWIDTH,
        );
        STlib_initMultIcon(
            (&(w_faces) as *const st_multicon_t as *mut st_multicon_t),
            ST_FACESX,
            ST_FACESY,
            faces,
            (&(st_faceindex) as *const std::ffi::c_int as *mut std::ffi::c_int),
            (&(st_statusbaron) as *const boolean as *mut boolean),
        );
        STlib_initPercent(
            (&(w_armor) as *const st_percent_t as *mut st_percent_t),
            ST_ARMORX,
            ST_ARMORY,
            tallnum,
            (&((*plyr).armorpoints) as *const _ as *mut _),
            (&(st_statusbaron) as *const boolean as *mut boolean),
            tallpercent,
        );
        STlib_initMultIcon(
            (&(w_keyboxes[(0) as usize]) as *const _ as *mut _),
            ST_KEY0X,
            ST_KEY0Y,
            keys,
            (&(keyboxes[(0) as usize]) as *const _ as *mut _),
            (&(st_statusbaron) as *const boolean as *mut boolean),
        );
        STlib_initMultIcon(
            (&(w_keyboxes[(1) as usize]) as *const _ as *mut _),
            ST_KEY1X,
            ST_KEY1Y,
            keys,
            (&(keyboxes[(1) as usize]) as *const _ as *mut _),
            (&(st_statusbaron) as *const boolean as *mut boolean),
        );
        STlib_initMultIcon(
            (&(w_keyboxes[(2) as usize]) as *const _ as *mut _),
            ST_KEY2X,
            ST_KEY2Y,
            keys,
            (&(keyboxes[(2) as usize]) as *const _ as *mut _),
            (&(st_statusbaron) as *const boolean as *mut boolean),
        );
        STlib_initNum(
            (&(w_ammo[(0) as usize]) as *const _ as *mut _),
            ST_AMMO0X,
            ST_AMMO0Y,
            shortnum,
            (&((*plyr).ammo[(0) as usize]) as *const _ as *mut _),
            (&(st_statusbaron) as *const boolean as *mut boolean),
            ST_AMMO0WIDTH,
        );
        STlib_initNum(
            (&(w_ammo[(1) as usize]) as *const _ as *mut _),
            ST_AMMO1X,
            ST_AMMO1Y,
            shortnum,
            (&((*plyr).ammo[(1) as usize]) as *const _ as *mut _),
            (&(st_statusbaron) as *const boolean as *mut boolean),
            ST_AMMO1WIDTH,
        );
        STlib_initNum(
            (&(w_ammo[(2) as usize]) as *const _ as *mut _),
            ST_AMMO2X,
            ST_AMMO2Y,
            shortnum,
            (&((*plyr).ammo[(2) as usize]) as *const _ as *mut _),
            (&(st_statusbaron) as *const boolean as *mut boolean),
            ST_AMMO2WIDTH,
        );
        STlib_initNum(
            (&(w_ammo[(3) as usize]) as *const _ as *mut _),
            ST_AMMO3X,
            ST_AMMO3Y,
            shortnum,
            (&((*plyr).ammo[(3) as usize]) as *const _ as *mut _),
            (&(st_statusbaron) as *const boolean as *mut boolean),
            ST_AMMO3WIDTH,
        );
        STlib_initNum(
            (&(w_maxammo[(0) as usize]) as *const _ as *mut _),
            ST_MAXAMMO0X,
            ST_MAXAMMO0Y,
            shortnum,
            (&((*plyr).maxammo[(0) as usize]) as *const _ as *mut _),
            (&(st_statusbaron) as *const boolean as *mut boolean),
            ST_MAXAMMO0WIDTH,
        );
        STlib_initNum(
            (&(w_maxammo[(1) as usize]) as *const _ as *mut _),
            ST_MAXAMMO1X,
            ST_MAXAMMO1Y,
            shortnum,
            (&((*plyr).maxammo[(1) as usize]) as *const _ as *mut _),
            (&(st_statusbaron) as *const boolean as *mut boolean),
            ST_MAXAMMO1WIDTH,
        );
        STlib_initNum(
            (&(w_maxammo[(2) as usize]) as *const _ as *mut _),
            ST_MAXAMMO2X,
            ST_MAXAMMO2Y,
            shortnum,
            (&((*plyr).maxammo[(2) as usize]) as *const _ as *mut _),
            (&(st_statusbaron) as *const boolean as *mut boolean),
            ST_MAXAMMO2WIDTH,
        );
        STlib_initNum(
            (&(w_maxammo[(3) as usize]) as *const _ as *mut _),
            ST_MAXAMMO3X,
            ST_MAXAMMO3Y,
            shortnum,
            (&((*plyr).maxammo[(3) as usize]) as *const _ as *mut _),
            (&(st_statusbaron) as *const boolean as *mut boolean),
            ST_MAXAMMO3WIDTH,
        );
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

static mut st_stopped: boolean = unsafe { true_ };

pub unsafe extern "C" fn ST_Start() {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //
        //     if (!st_stopped)
        // 	ST_Stop();
        todo!("if statement not yet translated");
        ST_initData();
        ST_createWidgets();
        st_stopped = false_;
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn ST_Stop() {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (st_stopped)
        // 	return;
        todo!("if statement not yet translated");
        I_SetPalette(W_CacheLumpNum(lu_palette, PU_CACHE));
        st_stopped = true_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn ST_Init() {
    unsafe {
        veryfirsttime = 0;
        ST_loadData();
        screens[(4) as usize] = ((Z_Malloc((ST_WIDTH * ST_HEIGHT), PU_STATIC, 0)) as *mut byte);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}
