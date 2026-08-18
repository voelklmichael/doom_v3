use crate::d_englsh::*;
use crate::d_event::*;
use crate::d_items::*;
use crate::d_main::*;
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
use crate::hu_stuff::*;
use crate::i_system::*;
use crate::i_video::*;
use crate::info::*;
use crate::m_argv::*;
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
use crate::s_sound::*;
use crate::sounds::*;
use crate::tables::*;
use crate::v_video::*;
use crate::w_wad::*;
use crate::z_zone::*;

static mut rcsid: [std::ffi::c_char; 49] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        109 as std::ffi::c_char,
        95 as std::ffi::c_char,
        109 as std::ffi::c_char,
        101 as std::ffi::c_char,
        110 as std::ffi::c_char,
        117 as std::ffi::c_char,
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

unsafe extern "C" {
    pub static mut hu_font: [*mut patch_t; (HU_FONTSIZE) as usize];
}

unsafe extern "C" {
    pub static mut message_dontfuckwithme: boolean;
}

unsafe extern "C" {
    pub static mut chat_on: boolean;
}

pub static mut mouseSensitivity: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut showMessages: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut detailLevel: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut screenblocks: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut screenSize: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut quickSaveSlot: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut messageToPrint: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut messageString: *mut std::ffi::c_char = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut messx: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut messy: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut messageLastMenuActive: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut messageNeedsInput: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut messageRoutine: Option<unsafe extern "C" fn(std::ffi::c_int)> =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub const SAVESTRINGSIZE: std::ffi::c_int = 24;

pub static mut gammamsg: [[std::ffi::c_char; (26) as usize]; (5) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut saveStringEnter: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut saveSlot: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut saveCharIndex: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut saveOldString: [std::ffi::c_char; (SAVESTRINGSIZE) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut inhelpscreens: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut menuactive: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub const SKULLXOFF: std::ffi::c_int = (-(32));

pub const LINEHEIGHT: std::ffi::c_int = 16;

unsafe extern "C" {
    pub static mut sendpause: boolean;
}

pub static mut savegamestrings: [[std::ffi::c_char; (SAVESTRINGSIZE) as usize]; (10) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut endstring: [std::ffi::c_char; (160) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

#[repr(C)]
#[derive(Copy, Clone)]
pub struct menuitem_t {
    pub status: std::ffi::c_short,
    pub name: [std::ffi::c_char; (10) as usize],
    pub routine: Option<unsafe extern "C" fn(std::ffi::c_int)>,
    pub alphaKey: std::ffi::c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct menu_t {
    pub numitems: std::ffi::c_short,
    pub prevMenu: *mut menu_s,
    pub menuitems: *mut menuitem_t,
    pub routine: Option<unsafe extern "C" fn()>,
    pub x: std::ffi::c_short,
    pub y: std::ffi::c_short,
    pub lastOn: std::ffi::c_short,
}

pub type menu_s = menu_t;

pub static mut itemOn: std::ffi::c_short = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut skullAnimCounter: std::ffi::c_short = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut whichSkull: std::ffi::c_short = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut skullName: [[std::ffi::c_char; 9]; 2] = unsafe {
    [
        [
            77 as std::ffi::c_char,
            95 as std::ffi::c_char,
            83 as std::ffi::c_char,
            75 as std::ffi::c_char,
            85 as std::ffi::c_char,
            76 as std::ffi::c_char,
            76 as std::ffi::c_char,
            49 as std::ffi::c_char,
            0,
        ],
        [
            77 as std::ffi::c_char,
            95 as std::ffi::c_char,
            83 as std::ffi::c_char,
            75 as std::ffi::c_char,
            85 as std::ffi::c_char,
            76 as std::ffi::c_char,
            76 as std::ffi::c_char,
            50 as std::ffi::c_char,
            0,
        ],
    ]
};

pub static mut currentMenu: *mut menu_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

unsafe extern "C" {
    pub fn M_StartGame(choice: std::ffi::c_int);
}

pub const newgame: std::ffi::c_int = 0;
pub const options: std::ffi::c_int = newgame + 1;
pub const loadgame: std::ffi::c_int = options + 1;
pub const savegame: std::ffi::c_int = loadgame + 1;
pub const readthis: std::ffi::c_int = savegame + 1;
pub const quitdoom: std::ffi::c_int = readthis + 1;
pub const main_end: std::ffi::c_int = quitdoom + 1;

pub type main_e = std::ffi::c_int;

pub static mut MainMenu: [menuitem_t; 6] = unsafe {
    [
        menuitem_t {
            status: 1,
            name: [
                77 as std::ffi::c_char,
                95 as std::ffi::c_char,
                78 as std::ffi::c_char,
                71 as std::ffi::c_char,
                65 as std::ffi::c_char,
                77 as std::ffi::c_char,
                69 as std::ffi::c_char,
                0,
                0,
                0,
            ],
            routine: Some(M_NewGame),
            alphaKey: (b'n' as std::ffi::c_int),
        },
        menuitem_t {
            status: 1,
            name: [
                77 as std::ffi::c_char,
                95 as std::ffi::c_char,
                79 as std::ffi::c_char,
                80 as std::ffi::c_char,
                84 as std::ffi::c_char,
                73 as std::ffi::c_char,
                79 as std::ffi::c_char,
                78 as std::ffi::c_char,
                0,
                0,
            ],
            routine: Some(M_Options),
            alphaKey: (b'o' as std::ffi::c_int),
        },
        menuitem_t {
            status: 1,
            name: [
                77 as std::ffi::c_char,
                95 as std::ffi::c_char,
                76 as std::ffi::c_char,
                79 as std::ffi::c_char,
                65 as std::ffi::c_char,
                68 as std::ffi::c_char,
                71 as std::ffi::c_char,
                0,
                0,
                0,
            ],
            routine: Some(M_LoadGame),
            alphaKey: (b'l' as std::ffi::c_int),
        },
        menuitem_t {
            status: 1,
            name: [
                77 as std::ffi::c_char,
                95 as std::ffi::c_char,
                83 as std::ffi::c_char,
                65 as std::ffi::c_char,
                86 as std::ffi::c_char,
                69 as std::ffi::c_char,
                71 as std::ffi::c_char,
                0,
                0,
                0,
            ],
            routine: Some(M_SaveGame),
            alphaKey: (b's' as std::ffi::c_int),
        },
        menuitem_t {
            status: 1,
            name: [
                77 as std::ffi::c_char,
                95 as std::ffi::c_char,
                82 as std::ffi::c_char,
                68 as std::ffi::c_char,
                84 as std::ffi::c_char,
                72 as std::ffi::c_char,
                73 as std::ffi::c_char,
                83 as std::ffi::c_char,
                0,
                0,
            ],
            routine: Some(M_ReadThis),
            alphaKey: (b'r' as std::ffi::c_int),
        },
        menuitem_t {
            status: 1,
            name: [
                77 as std::ffi::c_char,
                95 as std::ffi::c_char,
                81 as std::ffi::c_char,
                85 as std::ffi::c_char,
                73 as std::ffi::c_char,
                84 as std::ffi::c_char,
                71 as std::ffi::c_char,
                0,
                0,
                0,
            ],
            routine: Some(M_QuitDOOM),
            alphaKey: (b'q' as std::ffi::c_int),
        },
    ]
};

pub static mut MainDef: menu_t = unsafe {
    menu_t {
        numitems: main_end,
        prevMenu: std::ptr::null_mut(),
        menuitems: MainMenu.as_mut_ptr(),
        routine: Some(M_DrawMainMenu),
        x: 97,
        y: 64,
        lastOn: 0,
    }
};

pub const ep1: std::ffi::c_int = 0;
pub const ep2: std::ffi::c_int = ep1 + 1;
pub const ep3: std::ffi::c_int = ep2 + 1;
pub const ep4: std::ffi::c_int = ep3 + 1;
pub const ep_end: std::ffi::c_int = ep4 + 1;

pub type episodes_e = std::ffi::c_int;

pub static mut EpisodeMenu: [menuitem_t; 4] = unsafe {
    [
        menuitem_t {
            status: 1,
            name: [
                77 as std::ffi::c_char,
                95 as std::ffi::c_char,
                69 as std::ffi::c_char,
                80 as std::ffi::c_char,
                73 as std::ffi::c_char,
                49 as std::ffi::c_char,
                0,
                0,
                0,
                0,
            ],
            routine: Some(M_Episode),
            alphaKey: (b'k' as std::ffi::c_int),
        },
        menuitem_t {
            status: 1,
            name: [
                77 as std::ffi::c_char,
                95 as std::ffi::c_char,
                69 as std::ffi::c_char,
                80 as std::ffi::c_char,
                73 as std::ffi::c_char,
                50 as std::ffi::c_char,
                0,
                0,
                0,
                0,
            ],
            routine: Some(M_Episode),
            alphaKey: (b't' as std::ffi::c_int),
        },
        menuitem_t {
            status: 1,
            name: [
                77 as std::ffi::c_char,
                95 as std::ffi::c_char,
                69 as std::ffi::c_char,
                80 as std::ffi::c_char,
                73 as std::ffi::c_char,
                51 as std::ffi::c_char,
                0,
                0,
                0,
                0,
            ],
            routine: Some(M_Episode),
            alphaKey: (b'i' as std::ffi::c_int),
        },
        menuitem_t {
            status: 1,
            name: [
                77 as std::ffi::c_char,
                95 as std::ffi::c_char,
                69 as std::ffi::c_char,
                80 as std::ffi::c_char,
                73 as std::ffi::c_char,
                52 as std::ffi::c_char,
                0,
                0,
                0,
                0,
            ],
            routine: Some(M_Episode),
            alphaKey: (b't' as std::ffi::c_int),
        },
    ]
};

pub static mut EpiDef: menu_t = unsafe {
    menu_t {
        numitems: ep_end,
        prevMenu: (&(MainDef) as *const menu_t as *mut menu_t),
        menuitems: EpisodeMenu.as_mut_ptr(),
        routine: Some(M_DrawEpisode),
        x: 48,
        y: 63,
        lastOn: ep1,
    }
};

pub const killthings: std::ffi::c_int = 0;
pub const toorough: std::ffi::c_int = killthings + 1;
pub const hurtme: std::ffi::c_int = toorough + 1;
pub const violence: std::ffi::c_int = hurtme + 1;
pub const nightmare: std::ffi::c_int = violence + 1;
pub const newg_end: std::ffi::c_int = nightmare + 1;

pub type newgame_e = std::ffi::c_int;

pub static mut NewGameMenu: [menuitem_t; 5] = unsafe {
    [
        menuitem_t {
            status: 1,
            name: [
                77 as std::ffi::c_char,
                95 as std::ffi::c_char,
                74 as std::ffi::c_char,
                75 as std::ffi::c_char,
                73 as std::ffi::c_char,
                76 as std::ffi::c_char,
                76 as std::ffi::c_char,
                0,
                0,
                0,
            ],
            routine: Some(M_ChooseSkill),
            alphaKey: (b'i' as std::ffi::c_int),
        },
        menuitem_t {
            status: 1,
            name: [
                77 as std::ffi::c_char,
                95 as std::ffi::c_char,
                82 as std::ffi::c_char,
                79 as std::ffi::c_char,
                85 as std::ffi::c_char,
                71 as std::ffi::c_char,
                72 as std::ffi::c_char,
                0,
                0,
                0,
            ],
            routine: Some(M_ChooseSkill),
            alphaKey: (b'h' as std::ffi::c_int),
        },
        menuitem_t {
            status: 1,
            name: [
                77 as std::ffi::c_char,
                95 as std::ffi::c_char,
                72 as std::ffi::c_char,
                85 as std::ffi::c_char,
                82 as std::ffi::c_char,
                84 as std::ffi::c_char,
                0,
                0,
                0,
                0,
            ],
            routine: Some(M_ChooseSkill),
            alphaKey: (b'h' as std::ffi::c_int),
        },
        menuitem_t {
            status: 1,
            name: [
                77 as std::ffi::c_char,
                95 as std::ffi::c_char,
                85 as std::ffi::c_char,
                76 as std::ffi::c_char,
                84 as std::ffi::c_char,
                82 as std::ffi::c_char,
                65 as std::ffi::c_char,
                0,
                0,
                0,
            ],
            routine: Some(M_ChooseSkill),
            alphaKey: (b'u' as std::ffi::c_int),
        },
        menuitem_t {
            status: 1,
            name: [
                77 as std::ffi::c_char,
                95 as std::ffi::c_char,
                78 as std::ffi::c_char,
                77 as std::ffi::c_char,
                65 as std::ffi::c_char,
                82 as std::ffi::c_char,
                69 as std::ffi::c_char,
                0,
                0,
                0,
            ],
            routine: Some(M_ChooseSkill),
            alphaKey: (b'n' as std::ffi::c_int),
        },
    ]
};

pub static mut NewDef: menu_t = unsafe {
    menu_t {
        numitems: newg_end,
        prevMenu: (&(EpiDef) as *const menu_t as *mut menu_t),
        menuitems: NewGameMenu.as_mut_ptr(),
        routine: Some(M_DrawNewGame),
        x: 48,
        y: 63,
        lastOn: hurtme,
    }
};

pub const endgame: std::ffi::c_int = 0;
pub const messages: std::ffi::c_int = endgame + 1;
pub const detail: std::ffi::c_int = messages + 1;
pub const scrnsize: std::ffi::c_int = detail + 1;
pub const option_empty1: std::ffi::c_int = scrnsize + 1;
pub const mousesens: std::ffi::c_int = option_empty1 + 1;
pub const option_empty2: std::ffi::c_int = mousesens + 1;
pub const soundvol: std::ffi::c_int = option_empty2 + 1;
pub const opt_end: std::ffi::c_int = soundvol + 1;

pub type options_e = std::ffi::c_int;

pub static mut OptionsMenu: [menuitem_t; 8] = unsafe {
    [
        menuitem_t {
            status: 1,
            name: [
                77 as std::ffi::c_char,
                95 as std::ffi::c_char,
                69 as std::ffi::c_char,
                78 as std::ffi::c_char,
                68 as std::ffi::c_char,
                71 as std::ffi::c_char,
                65 as std::ffi::c_char,
                77 as std::ffi::c_char,
                0,
                0,
            ],
            routine: Some(M_EndGame),
            alphaKey: (b'e' as std::ffi::c_int),
        },
        menuitem_t {
            status: 1,
            name: [
                77 as std::ffi::c_char,
                95 as std::ffi::c_char,
                77 as std::ffi::c_char,
                69 as std::ffi::c_char,
                83 as std::ffi::c_char,
                83 as std::ffi::c_char,
                71 as std::ffi::c_char,
                0,
                0,
                0,
            ],
            routine: Some(M_ChangeMessages),
            alphaKey: (b'm' as std::ffi::c_int),
        },
        menuitem_t {
            status: 1,
            name: [
                77 as std::ffi::c_char,
                95 as std::ffi::c_char,
                68 as std::ffi::c_char,
                69 as std::ffi::c_char,
                84 as std::ffi::c_char,
                65 as std::ffi::c_char,
                73 as std::ffi::c_char,
                76 as std::ffi::c_char,
                0,
                0,
            ],
            routine: Some(M_ChangeDetail),
            alphaKey: (b'g' as std::ffi::c_int),
        },
        menuitem_t {
            status: 2,
            name: [
                77 as std::ffi::c_char,
                95 as std::ffi::c_char,
                83 as std::ffi::c_char,
                67 as std::ffi::c_char,
                82 as std::ffi::c_char,
                78 as std::ffi::c_char,
                83 as std::ffi::c_char,
                90 as std::ffi::c_char,
                0,
                0,
            ],
            routine: Some(M_SizeDisplay),
            alphaKey: (b's' as std::ffi::c_int),
        },
        menuitem_t {
            status: (-(1)),
            name: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            routine: None,
            ..ZEROED_menuitem_t
        },
        menuitem_t {
            status: 2,
            name: [
                77 as std::ffi::c_char,
                95 as std::ffi::c_char,
                77 as std::ffi::c_char,
                83 as std::ffi::c_char,
                69 as std::ffi::c_char,
                78 as std::ffi::c_char,
                83 as std::ffi::c_char,
                0,
                0,
                0,
            ],
            routine: Some(M_ChangeSensitivity),
            alphaKey: (b'm' as std::ffi::c_int),
        },
        menuitem_t {
            status: (-(1)),
            name: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            routine: None,
            ..ZEROED_menuitem_t
        },
        menuitem_t {
            status: 1,
            name: [
                77 as std::ffi::c_char,
                95 as std::ffi::c_char,
                83 as std::ffi::c_char,
                86 as std::ffi::c_char,
                79 as std::ffi::c_char,
                76 as std::ffi::c_char,
                0,
                0,
                0,
                0,
            ],
            routine: Some(M_Sound),
            alphaKey: (b's' as std::ffi::c_int),
        },
    ]
};

pub static mut OptionsDef: menu_t = unsafe {
    menu_t {
        numitems: opt_end,
        prevMenu: (&(MainDef) as *const menu_t as *mut menu_t),
        menuitems: OptionsMenu.as_mut_ptr(),
        routine: Some(M_DrawOptions),
        x: 60,
        y: 37,
        lastOn: 0,
    }
};

pub const rdthsempty1: std::ffi::c_int = 0;
pub const read1_end: std::ffi::c_int = rdthsempty1 + 1;

pub type read_e = std::ffi::c_int;

pub static mut ReadMenu1: [menuitem_t; 1] = unsafe {
    [menuitem_t {
        status: 1,
        name: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        routine: Some(M_ReadThis2),
        alphaKey: 0,
    }]
};

pub static mut ReadDef1: menu_t = unsafe {
    menu_t {
        numitems: read1_end,
        prevMenu: (&(MainDef) as *const menu_t as *mut menu_t),
        menuitems: ReadMenu1.as_mut_ptr(),
        routine: Some(M_DrawReadThis1),
        x: 280,
        y: 185,
        lastOn: 0,
    }
};

pub const rdthsempty2: std::ffi::c_int = 0;
pub const read2_end: std::ffi::c_int = rdthsempty2 + 1;

pub type read_e2 = std::ffi::c_int;

pub static mut ReadMenu2: [menuitem_t; 1] = unsafe {
    [menuitem_t {
        status: 1,
        name: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        routine: Some(M_FinishReadThis),
        alphaKey: 0,
    }]
};

pub static mut ReadDef2: menu_t = unsafe {
    menu_t {
        numitems: read2_end,
        prevMenu: (&(ReadDef1) as *const menu_t as *mut menu_t),
        menuitems: ReadMenu2.as_mut_ptr(),
        routine: Some(M_DrawReadThis2),
        x: 330,
        y: 175,
        lastOn: 0,
    }
};

pub const sfx_vol: std::ffi::c_int = 0;
pub const sfx_empty1: std::ffi::c_int = sfx_vol + 1;
pub const music_vol: std::ffi::c_int = sfx_empty1 + 1;
pub const sfx_empty2: std::ffi::c_int = music_vol + 1;
pub const sound_end: std::ffi::c_int = sfx_empty2 + 1;

pub type sound_e = std::ffi::c_int;

pub static mut SoundMenu: [menuitem_t; 4] = unsafe {
    [
        menuitem_t {
            status: 2,
            name: [
                77 as std::ffi::c_char,
                95 as std::ffi::c_char,
                83 as std::ffi::c_char,
                70 as std::ffi::c_char,
                88 as std::ffi::c_char,
                86 as std::ffi::c_char,
                79 as std::ffi::c_char,
                76 as std::ffi::c_char,
                0,
                0,
            ],
            routine: Some(M_SfxVol),
            alphaKey: (b's' as std::ffi::c_int),
        },
        menuitem_t {
            status: (-(1)),
            name: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            routine: None,
            ..ZEROED_menuitem_t
        },
        menuitem_t {
            status: 2,
            name: [
                77 as std::ffi::c_char,
                95 as std::ffi::c_char,
                77 as std::ffi::c_char,
                85 as std::ffi::c_char,
                83 as std::ffi::c_char,
                86 as std::ffi::c_char,
                79 as std::ffi::c_char,
                76 as std::ffi::c_char,
                0,
                0,
            ],
            routine: Some(M_MusicVol),
            alphaKey: (b'm' as std::ffi::c_int),
        },
        menuitem_t {
            status: (-(1)),
            name: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            routine: None,
            ..ZEROED_menuitem_t
        },
    ]
};

pub static mut SoundDef: menu_t = unsafe {
    menu_t {
        numitems: sound_end,
        prevMenu: (&(OptionsDef) as *const menu_t as *mut menu_t),
        menuitems: SoundMenu.as_mut_ptr(),
        routine: Some(M_DrawSound),
        x: 80,
        y: 64,
        lastOn: 0,
    }
};

pub const load1: std::ffi::c_int = 0;
pub const load2: std::ffi::c_int = load1 + 1;
pub const load3: std::ffi::c_int = load2 + 1;
pub const load4: std::ffi::c_int = load3 + 1;
pub const load5: std::ffi::c_int = load4 + 1;
pub const load6: std::ffi::c_int = load5 + 1;
pub const load_end: std::ffi::c_int = load6 + 1;

pub type load_e = std::ffi::c_int;

pub static mut LoadMenu: [menuitem_t; 6] = unsafe {
    [
        menuitem_t {
            status: 1,
            name: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            routine: Some(M_LoadSelect),
            alphaKey: (b'1' as std::ffi::c_int),
        },
        menuitem_t {
            status: 1,
            name: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            routine: Some(M_LoadSelect),
            alphaKey: (b'2' as std::ffi::c_int),
        },
        menuitem_t {
            status: 1,
            name: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            routine: Some(M_LoadSelect),
            alphaKey: (b'3' as std::ffi::c_int),
        },
        menuitem_t {
            status: 1,
            name: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            routine: Some(M_LoadSelect),
            alphaKey: (b'4' as std::ffi::c_int),
        },
        menuitem_t {
            status: 1,
            name: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            routine: Some(M_LoadSelect),
            alphaKey: (b'5' as std::ffi::c_int),
        },
        menuitem_t {
            status: 1,
            name: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            routine: Some(M_LoadSelect),
            alphaKey: (b'6' as std::ffi::c_int),
        },
    ]
};

pub static mut LoadDef: menu_t = unsafe {
    menu_t {
        numitems: load_end,
        prevMenu: (&(MainDef) as *const menu_t as *mut menu_t),
        menuitems: LoadMenu.as_mut_ptr(),
        routine: Some(M_DrawLoad),
        x: 80,
        y: 54,
        lastOn: 0,
    }
};

pub static mut SaveMenu: [menuitem_t; 6] = unsafe {
    [
        menuitem_t {
            status: 1,
            name: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            routine: Some(M_SaveSelect),
            alphaKey: (b'1' as std::ffi::c_int),
        },
        menuitem_t {
            status: 1,
            name: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            routine: Some(M_SaveSelect),
            alphaKey: (b'2' as std::ffi::c_int),
        },
        menuitem_t {
            status: 1,
            name: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            routine: Some(M_SaveSelect),
            alphaKey: (b'3' as std::ffi::c_int),
        },
        menuitem_t {
            status: 1,
            name: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            routine: Some(M_SaveSelect),
            alphaKey: (b'4' as std::ffi::c_int),
        },
        menuitem_t {
            status: 1,
            name: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            routine: Some(M_SaveSelect),
            alphaKey: (b'5' as std::ffi::c_int),
        },
        menuitem_t {
            status: 1,
            name: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            routine: Some(M_SaveSelect),
            alphaKey: (b'6' as std::ffi::c_int),
        },
    ]
};

pub static mut SaveDef: menu_t = unsafe {
    menu_t {
        numitems: load_end,
        prevMenu: (&(MainDef) as *const menu_t as *mut menu_t),
        menuitems: SaveMenu.as_mut_ptr(),
        routine: Some(M_DrawSave),
        x: 80,
        y: 54,
        lastOn: 0,
    }
};

pub unsafe extern "C" fn M_ReadSaveStrings() {
    unsafe {
        let mut handle: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut count: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut name: [std::ffi::c_char; (256) as usize] = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: for statement not yet translated:
        //
        //
        //     for (i = 0;i < load_end;i++)
        //     {
        // 	if (M_CheckParm("-cdrom"))
        // 	    sprintf(name,"c:\\doomdata\\"SAVEGAMENAME"%d.dsg",i);
        // 	else
        // 	    sprintf(name,SAVEGAMENAME"%d.dsg",i);
        //
        // 	handle = open (name, O_RDONLY | 0, 0666);
        // 	if (handle == -1)
        // 	{
        // 	    strcpy(&savegamestrings[i][0],EMPTYSTRING);
        // 	    LoadMenu[i].status = 0;
        // 	    continue;
        // 	}
        // 	count = read (handle, &savegamestrings[i], SAVESTRINGSIZE);
        // 	close (handle);
        // 	LoadMenu[i].status = 1;
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_DrawLoad() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        V_DrawPatchDirect(72, 28, 0, W_CacheLumpName((c"M_LOADG").as_ptr(), PU_CACHE));
        // TODO: for statement not yet translated:
        //
        //     for (i = 0;i < load_end; i++)
        //     {
        // 	M_DrawSaveLoadBorder(LoadDef.x,LoadDef.y+LINEHEIGHT*i);
        // 	M_WriteText(LoadDef.x,LoadDef.y+LINEHEIGHT*i,savegamestrings[i]);
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_DrawSaveLoadBorder(mut x: std::ffi::c_int, mut y: std::ffi::c_int) {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        V_DrawPatchDirect(
            (x - 8),
            (y + 7),
            0,
            W_CacheLumpName((c"M_LSLEFT").as_ptr(), PU_CACHE),
        );
        // TODO: for statement not yet translated:
        //
        //
        //     for (i = 0;i < 24;i++)
        //     {
        // 	V_DrawPatchDirect (x,y+7,0,W_CacheLumpName("M_LSCNTR",PU_CACHE));
        // 	x += 8;
        //     }
        todo!("for statement not yet translated");
        V_DrawPatchDirect(
            x,
            (y + 7),
            0,
            W_CacheLumpName((c"M_LSRGHT").as_ptr(), PU_CACHE),
        );
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_LoadSelect(mut choice: std::ffi::c_int) {
    unsafe {
        let mut name: [std::ffi::c_char; (256) as usize] = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (M_CheckParm("-cdrom"))
        // 	sprintf(name,"c:\\doomdata\\"SAVEGAMENAME"%d.dsg",choice);
        //     else
        // 	sprintf(name,SAVEGAMENAME"%d.dsg",choice);
        todo!("if statement not yet translated");
        G_LoadGame(name);
        M_ClearMenus();
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_LoadGame(mut choice: std::ffi::c_int) {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (netgame)
        //     {
        // 	M_StartMessage(LOADNET,NULL,false);
        // 	return;
        //     }
        todo!("if statement not yet translated");
        M_SetupNextMenu((&(LoadDef) as *const menu_t as *mut menu_t));
        M_ReadSaveStrings();
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_DrawSave() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        V_DrawPatchDirect(72, 28, 0, W_CacheLumpName((c"M_SAVEG").as_ptr(), PU_CACHE));
        // TODO: for statement not yet translated:
        //
        //     for (i = 0;i < load_end; i++)
        //     {
        // 	M_DrawSaveLoadBorder(LoadDef.x,LoadDef.y+LINEHEIGHT*i);
        // 	M_WriteText(LoadDef.x,LoadDef.y+LINEHEIGHT*i,savegamestrings[i]);
        //     }
        todo!("for statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (saveStringEnter)
        //     {
        // 	i = M_StringWidth(savegamestrings[saveSlot]);
        // 	M_WriteText(LoadDef.x + i,LoadDef.y+LINEHEIGHT*saveSlot,"_");
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_DoSave(mut slot: std::ffi::c_int) {
    unsafe {
        G_SaveGame(slot, savegamestrings[(slot) as usize]);
        M_ClearMenus();
        // TODO: if statement not yet translated:
        //
        //
        //     // PICK QUICKSAVE SLOT YET?
        //     if (quickSaveSlot == -2)
        // 	quickSaveSlot = slot;
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_SaveSelect(mut choice: std::ffi::c_int) {
    unsafe {
        saveStringEnter = 1;
        saveSlot = choice;
        strcpy(saveOldString, savegamestrings[(choice) as usize]);
        // TODO: if statement not yet translated:
        //
        //     if (!strcmp(savegamestrings[choice],EMPTYSTRING))
        // 	savegamestrings[choice][0] = 0;
        todo!("if statement not yet translated");
        saveCharIndex = strlen(savegamestrings[(choice) as usize]);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_SaveGame(mut choice: std::ffi::c_int) {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (!usergame)
        //     {
        // 	M_StartMessage(SAVEDEAD,NULL,false);
        // 	return;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (gamestate != GS_LEVEL)
        // 	return;
        todo!("if statement not yet translated");
        M_SetupNextMenu((&(SaveDef) as *const menu_t as *mut menu_t));
        M_ReadSaveStrings();
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub static mut tempstring: [std::ffi::c_char; (80) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn M_QuickSaveResponse(mut ch: std::ffi::c_int) {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (ch == 'y')
        //     {
        // 	M_DoSave(quickSaveSlot);
        // 	S_StartSound(NULL,sfx_swtchx);
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_QuickSave() {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (!usergame)
        //     {
        // 	S_StartSound(NULL,sfx_oof);
        // 	return;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (gamestate != GS_LEVEL)
        // 	return;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (quickSaveSlot < 0)
        //     {
        // 	M_StartControlPanel();
        // 	M_ReadSaveStrings();
        // 	M_SetupNextMenu(&SaveDef);
        // 	quickSaveSlot = -2;	// means to pick a slot now
        // 	return;
        //     }
        todo!("if statement not yet translated");
        sprintf(
            tempstring,
            QSPROMPT,
            savegamestrings[(quickSaveSlot) as usize],
        );
        M_StartMessage(tempstring, M_QuickSaveResponse, true_);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_QuickLoadResponse(mut ch: std::ffi::c_int) {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (ch == 'y')
        //     {
        // 	M_LoadSelect(quickSaveSlot);
        // 	S_StartSound(NULL,sfx_swtchx);
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_QuickLoad() {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (netgame)
        //     {
        // 	M_StartMessage(QLOADNET,NULL,false);
        // 	return;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (quickSaveSlot < 0)
        //     {
        // 	M_StartMessage(QSAVESPOT,NULL,false);
        // 	return;
        //     }
        todo!("if statement not yet translated");
        sprintf(
            tempstring,
            QLPROMPT,
            savegamestrings[(quickSaveSlot) as usize],
        );
        M_StartMessage(tempstring, M_QuickLoadResponse, true_);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_DrawReadThis1() {
    unsafe {
        inhelpscreens = true_;
        // TODO: switch statement not yet translated:
        //
        //     switch ( gamemode )
        //     {
        //       case commercial:
        // 	V_DrawPatchDirect (0,0,0,W_CacheLumpName("HELP",PU_CACHE));
        // 	break;
        //       case shareware:
        //       case registered:
        //       case retail:
        // 	V_DrawPatchDirect (0,0,0,W_CacheLumpName("HELP1",PU_CACHE));
        // 	break;
        //       default:
        // 	break;
        //     }
        todo!("switch statement not yet translated");
        return;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_DrawReadThis2() {
    unsafe {
        inhelpscreens = true_;
        // TODO: switch statement not yet translated:
        //
        //     switch ( gamemode )
        //     {
        //       case retail:
        //       case commercial:
        // 	// This hack keeps us from having to change menus.
        // 	V_DrawPatchDirect (0,0,0,W_CacheLumpName("CREDIT",PU_CACHE));
        // 	break;
        //       case shareware:
        //       case registered:
        // 	V_DrawPatchDirect (0,0,0,W_CacheLumpName("HELP2",PU_CACHE));
        // 	break;
        //       default:
        // 	break;
        //     }
        todo!("switch statement not yet translated");
        return;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_DrawSound() {
    unsafe {
        V_DrawPatchDirect(60, 38, 0, W_CacheLumpName((c"M_SVOL").as_ptr(), PU_CACHE));
        M_DrawThermo(
            SoundDef.x,
            (SoundDef.y + (LINEHEIGHT * (sfx_vol + 1))),
            16,
            snd_SfxVolume,
        );
        M_DrawThermo(
            SoundDef.x,
            (SoundDef.y + (LINEHEIGHT * (music_vol + 1))),
            16,
            snd_MusicVolume,
        );
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_Sound(mut choice: std::ffi::c_int) {
    unsafe {
        M_SetupNextMenu((&(SoundDef) as *const menu_t as *mut menu_t));
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_SfxVol(mut choice: std::ffi::c_int) {
    unsafe {
        // TODO: switch statement not yet translated:
        //
        //     switch(choice)
        //     {
        //       case 0:
        // 	if (snd_SfxVolume)
        // 	    snd_SfxVolume--;
        // 	break;
        //       case 1:
        // 	if (snd_SfxVolume < 15)
        // 	    snd_SfxVolume++;
        // 	break;
        //     }
        todo!("switch statement not yet translated");
        S_SetSfxVolume(snd_SfxVolume);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_MusicVol(mut choice: std::ffi::c_int) {
    unsafe {
        // TODO: switch statement not yet translated:
        //
        //     switch(choice)
        //     {
        //       case 0:
        // 	if (snd_MusicVolume)
        // 	    snd_MusicVolume--;
        // 	break;
        //       case 1:
        // 	if (snd_MusicVolume < 15)
        // 	    snd_MusicVolume++;
        // 	break;
        //     }
        todo!("switch statement not yet translated");
        S_SetMusicVolume(snd_MusicVolume);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_DrawMainMenu() {
    unsafe {
        V_DrawPatchDirect(94, 2, 0, W_CacheLumpName((c"M_DOOM").as_ptr(), PU_CACHE));
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_DrawNewGame() {
    unsafe {
        V_DrawPatchDirect(96, 14, 0, W_CacheLumpName((c"M_NEWG").as_ptr(), PU_CACHE));
        V_DrawPatchDirect(54, 38, 0, W_CacheLumpName((c"M_SKILL").as_ptr(), PU_CACHE));
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_NewGame(mut choice: std::ffi::c_int) {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (netgame && !demoplayback)
        //     {
        // 	M_StartMessage(NEWGAME,NULL,false);
        // 	return;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if ( gamemode == commercial )
        // 	M_SetupNextMenu(&NewDef);
        //     else
        // 	M_SetupNextMenu(&EpiDef);
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub static mut epi: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn M_DrawEpisode() {
    unsafe {
        V_DrawPatchDirect(54, 38, 0, W_CacheLumpName((c"M_EPISOD").as_ptr(), PU_CACHE));
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_VerifyNightmare(mut ch: std::ffi::c_int) {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (ch != 'y')
        // 	return;
        todo!("if statement not yet translated");
        G_DeferedInitNew(nightmare, (epi + 1), 1);
        M_ClearMenus();
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_ChooseSkill(mut choice: std::ffi::c_int) {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (choice == nightmare)
        //     {
        // 	M_StartMessage(NIGHTMARE,M_VerifyNightmare,true);
        // 	return;
        //     }
        todo!("if statement not yet translated");
        G_DeferedInitNew(choice, (epi + 1), 1);
        M_ClearMenus();
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_Episode(mut choice: std::ffi::c_int) {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if ( (gamemode == shareware)
        // 	 && choice)
        //     {
        // 	M_StartMessage(SWSTRING,NULL,false);
        // 	M_SetupNextMenu(&ReadDef1);
        // 	return;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // Yet another hack...
        //     if ( (gamemode == registered)
        // 	 && (choice > 2))
        //     {
        //       fprintf( stderr,
        // 	       "M_Episode: 4th episode requires UltimateDOOM\n");
        //       choice = 0;
        //     }
        todo!("if statement not yet translated");
        epi = choice;
        M_SetupNextMenu((&(NewDef) as *const menu_t as *mut menu_t));
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub static mut detailNames: [[std::ffi::c_char; 9]; 2] = unsafe {
    [
        [
            77 as std::ffi::c_char,
            95 as std::ffi::c_char,
            71 as std::ffi::c_char,
            68 as std::ffi::c_char,
            72 as std::ffi::c_char,
            73 as std::ffi::c_char,
            71 as std::ffi::c_char,
            72 as std::ffi::c_char,
            0,
        ],
        [
            77 as std::ffi::c_char,
            95 as std::ffi::c_char,
            71 as std::ffi::c_char,
            68 as std::ffi::c_char,
            76 as std::ffi::c_char,
            79 as std::ffi::c_char,
            87 as std::ffi::c_char,
            0,
            0,
        ],
    ]
};

pub static mut msgNames: [[std::ffi::c_char; 9]; 2] = unsafe {
    [
        [
            77 as std::ffi::c_char,
            95 as std::ffi::c_char,
            77 as std::ffi::c_char,
            83 as std::ffi::c_char,
            71 as std::ffi::c_char,
            79 as std::ffi::c_char,
            70 as std::ffi::c_char,
            70 as std::ffi::c_char,
            0,
        ],
        [
            77 as std::ffi::c_char,
            95 as std::ffi::c_char,
            77 as std::ffi::c_char,
            83 as std::ffi::c_char,
            71 as std::ffi::c_char,
            79 as std::ffi::c_char,
            78 as std::ffi::c_char,
            0,
            0,
        ],
    ]
};

pub unsafe extern "C" fn M_DrawOptions() {
    unsafe {
        V_DrawPatchDirect(
            108,
            15,
            0,
            W_CacheLumpName((c"M_OPTTTL").as_ptr(), PU_CACHE),
        );
        V_DrawPatchDirect(
            (OptionsDef.x + 175),
            (OptionsDef.y + (LINEHEIGHT * detail)),
            0,
            W_CacheLumpName(detailNames[(detailLevel) as usize], PU_CACHE),
        );
        V_DrawPatchDirect(
            (OptionsDef.x + 120),
            (OptionsDef.y + (LINEHEIGHT * messages)),
            0,
            W_CacheLumpName(msgNames[(showMessages) as usize], PU_CACHE),
        );
        M_DrawThermo(
            OptionsDef.x,
            (OptionsDef.y + (LINEHEIGHT * (mousesens + 1))),
            10,
            mouseSensitivity,
        );
        M_DrawThermo(
            OptionsDef.x,
            (OptionsDef.y + (LINEHEIGHT * (scrnsize + 1))),
            9,
            screenSize,
        );
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_Options(mut choice: std::ffi::c_int) {
    unsafe {
        M_SetupNextMenu((&(OptionsDef) as *const menu_t as *mut menu_t));
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_ChangeMessages(mut choice: std::ffi::c_int) {
    unsafe {
        choice = 0;
        showMessages = (1 - showMessages);
        // TODO: if statement not yet translated:
        //
        //
        //     if (!showMessages)
        // 	players[consoleplayer].message = MSGOFF;
        //     else
        // 	players[consoleplayer].message = MSGON ;
        todo!("if statement not yet translated");
        message_dontfuckwithme = true_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_EndGameResponse(mut ch: std::ffi::c_int) {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (ch != 'y')
        // 	return;
        todo!("if statement not yet translated");
        (*currentMenu).lastOn = itemOn;
        M_ClearMenus();
        D_StartTitle();
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_EndGame(mut choice: std::ffi::c_int) {
    unsafe {
        choice = 0;
        // TODO: if statement not yet translated:
        //
        //     if (!usergame)
        //     {
        // 	S_StartSound(NULL,sfx_oof);
        // 	return;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (netgame)
        //     {
        // 	M_StartMessage(NETEND,NULL,false);
        // 	return;
        //     }
        todo!("if statement not yet translated");
        M_StartMessage(ENDGAME, M_EndGameResponse, true_);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_ReadThis(mut choice: std::ffi::c_int) {
    unsafe {
        choice = 0;
        M_SetupNextMenu((&(ReadDef1) as *const menu_t as *mut menu_t));
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_ReadThis2(mut choice: std::ffi::c_int) {
    unsafe {
        choice = 0;
        M_SetupNextMenu((&(ReadDef2) as *const menu_t as *mut menu_t));
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_FinishReadThis(mut choice: std::ffi::c_int) {
    unsafe {
        choice = 0;
        M_SetupNextMenu((&(MainDef) as *const menu_t as *mut menu_t));
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub static mut quitsounds: [std::ffi::c_int; 8] = unsafe {
    [
        sfx_pldeth, sfx_dmpain, sfx_popain, sfx_slop, sfx_telept, sfx_posit1, sfx_posit3,
        sfx_sgtatk,
    ]
};

pub static mut quitsounds2: [std::ffi::c_int; 8] = unsafe {
    [
        sfx_vilact, sfx_getpow, sfx_boscub, sfx_slop, sfx_skeswg, sfx_kntdth, sfx_bspact,
        sfx_sgtatk,
    ]
};

pub unsafe extern "C" fn M_QuitResponse(mut ch: std::ffi::c_int) {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (ch != 'y')
        // 	return;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (!netgame)
        //     {
        // 	if (gamemode == commercial)
        // 	    S_StartSound(NULL,quitsounds2[(gametic>>2)&7]);
        // 	else
        // 	    S_StartSound(NULL,quitsounds[(gametic>>2)&7]);
        // 	I_WaitVBL(105);
        //     }
        todo!("if statement not yet translated");
        I_Quit();
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_QuitDOOM(mut choice: std::ffi::c_int) {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //   // We pick index 0 which is language sensitive,
        //   //  or one at random, between 1 and maximum number.
        //   if (language != english )
        //     sprintf(endstring,"%s\n\n"DOSY, endmsg[0] );
        //   else
        //     sprintf(endstring,"%s\n\n"DOSY, endmsg[ (gametic%(NUM_QUITMESSAGES-2))+1 ]);
        todo!("if statement not yet translated");
        M_StartMessage(endstring, M_QuitResponse, true_);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_ChangeSensitivity(mut choice: std::ffi::c_int) {
    unsafe {
        // TODO: switch statement not yet translated:
        //
        //     switch(choice)
        //     {
        //       case 0:
        // 	if (mouseSensitivity)
        // 	    mouseSensitivity--;
        // 	break;
        //       case 1:
        // 	if (mouseSensitivity < 9)
        // 	    mouseSensitivity++;
        // 	break;
        //     }
        todo!("switch statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_ChangeDetail(mut choice: std::ffi::c_int) {
    unsafe {
        choice = 0;
        detailLevel = (1 - detailLevel);
        fprintf(stderr, (c"M_ChangeDetail: low detail mode n.a.\n").as_ptr());
        return;
        // TODO: statement not yet translated:
        //
        //
        //     /*R_SetViewSize (screenblocks, detailLevel);
        //
        //     if (!detailLevel)
        // 	players[consoleplayer].message = DETAILHI;
        //     else
        // 	players[consoleplayer].message = DETAILLO;*/
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_SizeDisplay(mut choice: std::ffi::c_int) {
    unsafe {
        // TODO: switch statement not yet translated:
        //
        //     switch(choice)
        //     {
        //       case 0:
        // 	if (screenSize > 0)
        // 	{
        // 	    screenblocks--;
        // 	    screenSize--;
        // 	}
        // 	break;
        //       case 1:
        // 	if (screenSize < 8)
        // 	{
        // 	    screenblocks++;
        // 	    screenSize++;
        // 	}
        // 	break;
        //     }
        todo!("switch statement not yet translated");
        R_SetViewSize(screenblocks, detailLevel);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_DrawThermo(
    mut x: std::ffi::c_int,
    mut y: std::ffi::c_int,
    mut thermWidth: std::ffi::c_int,
    mut thermDot: std::ffi::c_int,
) {
    unsafe {
        let mut xx: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        xx = x;
        V_DrawPatchDirect(xx, y, 0, W_CacheLumpName((c"M_THERML").as_ptr(), PU_CACHE));
        xx += 8;
        // TODO: for statement not yet translated:
        //
        //     for (i=0;i<thermWidth;i++)
        //     {
        // 	V_DrawPatchDirect (xx,y,0,W_CacheLumpName("M_THERMM",PU_CACHE));
        // 	xx += 8;
        //     }
        todo!("for statement not yet translated");
        V_DrawPatchDirect(xx, y, 0, W_CacheLumpName((c"M_THERMR").as_ptr(), PU_CACHE));
        V_DrawPatchDirect(
            ((x + 8) + (thermDot * 8)),
            y,
            0,
            W_CacheLumpName((c"M_THERMO").as_ptr(), PU_CACHE),
        );
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_DrawEmptyCell(mut menu: *mut menu_t, mut item: std::ffi::c_int) {
    unsafe {
        V_DrawPatchDirect(
            ((*menu).x - 10),
            (((*menu).y + (item * LINEHEIGHT)) - 1),
            0,
            W_CacheLumpName((c"M_CELL1").as_ptr(), PU_CACHE),
        );
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_DrawSelCell(mut menu: *mut menu_t, mut item: std::ffi::c_int) {
    unsafe {
        V_DrawPatchDirect(
            ((*menu).x - 10),
            (((*menu).y + (item * LINEHEIGHT)) - 1),
            0,
            W_CacheLumpName((c"M_CELL2").as_ptr(), PU_CACHE),
        );
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_StartMessage(
    mut string: *mut std::ffi::c_char,
    mut routine: *mut std::ffi::c_void,
    mut input: boolean,
) {
    unsafe {
        messageLastMenuActive = menuactive;
        messageToPrint = 1;
        messageString = string;
        messageRoutine = routine;
        messageNeedsInput = input;
        menuactive = true_;
        return;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_StopMessage() {
    unsafe {
        menuactive = messageLastMenuActive;
        messageToPrint = 0;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_StringWidth(mut string: *mut std::ffi::c_char) -> std::ffi::c_int {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut w: std::ffi::c_int = unsafe { 0 };
        let mut c: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: for statement not yet translated:
        //
        //
        //     for (i = 0;i < strlen(string);i++)
        //     {
        // 	c = toupper(string[i]) - HU_FONTSTART;
        // 	if (c < 0 || c >= HU_FONTSIZE)
        // 	    w += 4;
        // 	else
        // 	    w += SHORT (hu_font[c]->width);
        //     }
        todo!("for statement not yet translated");
        return w;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn M_StringHeight(mut string: *mut std::ffi::c_char) -> std::ffi::c_int {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut h: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut height: std::ffi::c_int = unsafe { SHORT((*hu_font[(0) as usize]).height) };
        h = height;
        // TODO: for statement not yet translated:
        //
        //     for (i = 0;i < strlen(string);i++)
        // 	if (string[i] == '\n')
        // 	    h += height;
        todo!("for statement not yet translated");
        return h;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn M_WriteText(
    mut x: std::ffi::c_int,
    mut y: std::ffi::c_int,
    mut string: *mut std::ffi::c_char,
) {
    unsafe {
        let mut w: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut ch: *mut std::ffi::c_char = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut c: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut cx: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut cy: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        ch = string;
        cx = x;
        cy = y;
        // TODO: while statement not yet translated:
        //
        //
        //     while(1)
        //     {
        // 	c = *ch++;
        // 	if (!c)
        // 	    break;
        // 	if (c == '\n')
        // 	{
        // 	    cx = x;
        // 	    cy += 12;
        // 	    continue;
        // 	}
        //
        // 	c = toupper(c) - HU_FONTSTART;
        // 	if (c < 0 || c>= HU_FONTSIZE)
        // 	{
        // 	    cx += 4;
        // 	    continue;
        // 	}
        //
        // 	w = SHORT (hu_font[c]->width);
        // 	if (cx+w > SCREENWIDTH)
        // 	    break;
        // 	V_DrawPatchDirect(cx, cy, 0, hu_font[c]);
        // 	cx+=w;
        //     }
        todo!("while statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_Responder(mut ev: *mut event_t) -> boolean {
    unsafe {
        let mut ch: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        static mut joywait: std::ffi::c_int = unsafe { 0 };
        static mut mousewait: std::ffi::c_int = unsafe { 0 };
        static mut mousey: std::ffi::c_int = unsafe { 0 };
        static mut lasty: std::ffi::c_int = unsafe { 0 };
        static mut mousex: std::ffi::c_int = unsafe { 0 };
        static mut lastx: std::ffi::c_int = unsafe { 0 };
        ch = (-(1));
        // TODO: if statement not yet translated:
        //
        //
        //     if (ev->type == ev_joystick && joywait < I_GetTime())
        //     {
        // 	if (ev->data3 == -1)
        // 	{
        // 	    ch = KEY_UPARROW;
        // 	    joywait = I_GetTime() + 5;
        // 	}
        // 	else if (ev->data3 == 1)
        // 	{
        // 	    ch = KEY_DOWNARROW;
        // 	    joywait = I_GetTime() + 5;
        // 	}
        //
        // 	if (ev->data2 == -1)
        // 	{
        // 	    ch = KEY_LEFTARROW;
        // 	    joywait = I_GetTime() + 2;
        // 	}
        // 	else if (ev->data2 == 1)
        // 	{
        // 	    ch = KEY_RIGHTARROW;
        // 	    joywait = I_GetTime() + 2;
        // 	}
        //
        // 	if (ev->data1&1)
        // 	{
        // 	    ch = KEY_ENTER;
        // 	    joywait = I_GetTime() + 5;
        // 	}
        // 	if (ev->data1&2)
        // 	{
        // 	    ch = KEY_BACKSPACE;
        // 	    joywait = I_GetTime() + 5;
        // 	}
        //     }
        //     else
        //     {
        // 	if (ev->type == ev_mouse && mousewait < I_GetTime())
        // 	{
        // 	    mousey += ev->data3;
        // 	    if (mousey < lasty-30)
        // 	    {
        // 		ch = KEY_DOWNARROW;
        // 		mousewait = I_GetTime() + 5;
        // 		mousey = lasty -= 30;
        // 	    }
        // 	    else if (mousey > lasty+30)
        // 	    {
        // 		ch = KEY_UPARROW;
        // 		mousewait = I_GetTime() + 5;
        // 		mousey = lasty += 30;
        // 	    }
        //
        // 	    mousex += ev->data2;
        // 	    if (mousex < lastx-30)
        // 	    {
        // 		ch = KEY_LEFTARROW;
        // 		mousewait = I_GetTime() + 5;
        // 		mousex = lastx -= 30;
        // 	    }
        // 	    else if (mousex > lastx+30)
        // 	    {
        // 		ch = KEY_RIGHTARROW;
        // 		mousewait = I_GetTime() + 5;
        // 		mousex = lastx += 30;
        // 	    }
        //
        // 	    if (ev->data1&1)
        // 	    {
        // 		ch = KEY_ENTER;
        // 		mousewait = I_GetTime() + 15;
        // 	    }
        //
        // 	    if (ev->data1&2)
        // 	    {
        // 		ch = KEY_BACKSPACE;
        // 		mousewait = I_GetTime() + 15;
        // 	    }
        // 	}
        // 	else
        // 	    if (ev->type == ev_keydown)
        // 	    {
        // 		ch = ev->data1;
        // 	    }
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (ch == -1)
        // 	return false;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //
        //     // Save Game string input
        //     if (saveStringEnter)
        //     {
        // 	switch(ch)
        // 	{
        // 	  case KEY_BACKSPACE:
        // 	    if (saveCharIndex > 0)
        // 	    {
        // 		saveCharIndex--;
        // 		savegamestrings[saveSlot][saveCharIndex] = 0;
        // 	    }
        // 	    break;
        //
        // 	  case KEY_ESCAPE:
        // 	    saveStringEnter = 0;
        // 	    strcpy(&savegamestrings[saveSlot][0],saveOldString);
        // 	    break;
        //
        // 	  case KEY_ENTER:
        // 	    saveStringEnter = 0;
        // 	    if (savegamestrings[saveSlot][0])
        // 		M_DoSave(saveSlot);
        // 	    break;
        //
        // 	  default:
        // 	    ch = toupper(ch);
        // 	    if (ch != 32)
        // 		if (ch-HU_FONTSTART < 0 || ch-HU_FONTSTART >= HU_FONTSIZE)
        // 		    break;
        // 	    if (ch >= 32 && ch <= 127 &&
        // 		saveCharIndex < SAVESTRINGSIZE-1 &&
        // 		M_StringWidth(savegamestrings[saveSlot]) <
        // 		(SAVESTRINGSIZE-2)*8)
        // 	    {
        // 		savegamestrings[saveSlot][saveCharIndex++] = ch;
        // 		savegamestrings[saveSlot][saveCharIndex] = 0;
        // 	    }
        // 	    break;
        // 	}
        // 	return true;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // Take care of any messages that need input
        //     if (messageToPrint)
        //     {
        // 	if (messageNeedsInput == true &&
        // 	    !(ch == ' ' || ch == 'n' || ch == 'y' || ch == KEY_ESCAPE))
        // 	    return false;
        //
        // 	menuactive = messageLastMenuActive;
        // 	messageToPrint = 0;
        // 	if (messageRoutine)
        // 	    messageRoutine(ch);
        //
        // 	menuactive = false;
        // 	S_StartSound(NULL,sfx_swtchx);
        // 	return true;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (devparm && ch == KEY_F1)
        //     {
        // 	G_ScreenShot ();
        // 	return true;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //
        //     // F-Keys
        //     if (!menuactive)
        // 	switch(ch)
        // 	{
        // 	  case KEY_MINUS:         // Screen size down
        // 	    if (automapactive || chat_on)
        // 		return false;
        // 	    M_SizeDisplay(0);
        // 	    S_StartSound(NULL,sfx_stnmov);
        // 	    return true;
        //
        // 	  case KEY_EQUALS:        // Screen size up
        // 	    if (automapactive || chat_on)
        // 		return false;
        // 	    M_SizeDisplay(1);
        // 	    S_StartSound(NULL,sfx_stnmov);
        // 	    return true;
        //
        // 	  case KEY_F1:            // Help key
        // 	    M_StartControlPanel ();
        //
        // 	    if ( gamemode == retail )
        // 	      currentMenu = &ReadDef2;
        // 	    else
        // 	      currentMenu = &ReadDef1;
        //
        // 	    itemOn = 0;
        // 	    S_StartSound(NULL,sfx_swtchn);
        // 	    return true;
        //
        // 	  case KEY_F2:            // Save
        // 	    M_StartControlPanel();
        // 	    S_StartSound(NULL,sfx_swtchn);
        // 	    M_SaveGame(0);
        // 	    return true;
        //
        // 	  case KEY_F3:            // Load
        // 	    M_StartControlPanel();
        // 	    S_StartSound(NULL,sfx_swtchn);
        // 	    M_LoadGame(0);
        // 	    return true;
        //
        // 	  case KEY_F4:            // Sound Volume
        // 	    M_StartControlPanel ();
        // 	    currentMenu = &SoundDef;
        // 	    itemOn = sfx_vol;
        // 	    S_StartSound(NULL,sfx_swtchn);
        // 	    return true;
        //
        // 	  case KEY_F5:            // Detail toggle
        // 	    M_ChangeDetail(0);
        // 	    S_StartSound(NULL,sfx_swtchn);
        // 	    return true;
        //
        // 	  case KEY_F6:            // Quicksave
        // 	    S_StartSound(NULL,sfx_swtchn);
        // 	    M_QuickSave();
        // 	    return true;
        //
        // 	  case KEY_F7:            // End game
        // 	    S_StartSound(NULL,sfx_swtchn);
        // 	    M_EndGame(0);
        // 	    return true;
        //
        // 	  case KEY_F8:            // Toggle messages
        // 	    M_ChangeMessages(0);
        // 	    S_StartSound(NULL,sfx_swtchn);
        // 	    return true;
        //
        // 	  case KEY_F9:            // Quickload
        // 	    S_StartSound(NULL,sfx_swtchn);
        // 	    M_QuickLoad();
        // 	    return true;
        //
        // 	  case KEY_F10:           // Quit DOOM
        // 	    S_StartSound(NULL,sfx_swtchn);
        // 	    M_QuitDOOM(0);
        // 	    return true;
        //
        // 	  case KEY_F11:           // gamma toggle
        // 	    usegamma++;
        // 	    if (usegamma > 4)
        // 		usegamma = 0;
        // 	    players[consoleplayer].message = gammamsg[usegamma];
        // 	    I_SetPalette (W_CacheLumpName ("PLAYPAL",PU_CACHE));
        // 	    return true;
        //
        // 	}
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //
        //     // Pop-up menu?
        //     if (!menuactive)
        //     {
        // 	if (ch == KEY_ESCAPE)
        // 	{
        // 	    M_StartControlPanel ();
        // 	    S_StartSound(NULL,sfx_swtchn);
        // 	    return true;
        // 	}
        // 	return false;
        //     }
        todo!("if statement not yet translated");
        // TODO: switch statement not yet translated:
        //
        //
        //
        //     // Keys usable within menu
        //     switch (ch)
        //     {
        //       case KEY_DOWNARROW:
        // 	do
        // 	{
        // 	    if (itemOn+1 > currentMenu->numitems-1)
        // 		itemOn = 0;
        // 	    else itemOn++;
        // 	    S_StartSound(NULL,sfx_pstop);
        // 	} while(currentMenu->menuitems[itemOn].status==-1);
        // 	return true;
        //
        //       case KEY_UPARROW:
        // 	do
        // 	{
        // 	    if (!itemOn)
        // 		itemOn = currentMenu->numitems-1;
        // 	    else itemOn--;
        // 	    S_StartSound(NULL,sfx_pstop);
        // 	} while(currentMenu->menuitems[itemOn].status==-1);
        // 	return true;
        //
        //       case KEY_LEFTARROW:
        // 	if (currentMenu->menuitems[itemOn].routine &&
        // 	    currentMenu->menuitems[itemOn].status == 2)
        // 	{
        // 	    S_StartSound(NULL,sfx_stnmov);
        // 	    currentMenu->menuitems[itemOn].routine(0);
        // 	}
        // 	return true;
        //
        //       case KEY_RIGHTARROW:
        // 	if (currentMenu->menuitems[itemOn].routine &&
        // 	    currentMenu->menuitems[itemOn].status == 2)
        // 	{
        // 	    S_StartSound(NULL,sfx_stnmov);
        // 	    currentMenu->menuitems[itemOn].routine(1);
        // 	}
        // 	return true;
        //
        //       case KEY_ENTER:
        // 	if (currentMenu->menuitems[itemOn].routine &&
        // 	    currentMenu->menuitems[itemOn].status)
        // 	{
        // 	    currentMenu->lastOn = itemOn;
        // 	    if (currentMenu->menuitems[itemOn].status == 2)
        // 	    {
        // 		currentMenu->menuitems[itemOn].routine(1);      // right arrow
        // 		S_StartSound(NULL,sfx_stnmov);
        // 	    }
        // 	    else
        // 	    {
        // 		currentMenu->menuitems[itemOn].routine(itemOn);
        // 		S_StartSound(NULL,sfx_pistol);
        // 	    }
        // 	}
        // 	return true;
        //
        //       case KEY_ESCAPE:
        // 	currentMenu->lastOn = itemOn;
        // 	M_ClearMenus ();
        // 	S_StartSound(NULL,sfx_swtchx);
        // 	return true;
        //
        //       case KEY_BACKSPACE:
        // 	currentMenu->lastOn = itemOn;
        // 	if (currentMenu->prevMenu)
        // 	{
        // 	    currentMenu = currentMenu->prevMenu;
        // 	    itemOn = currentMenu->lastOn;
        // 	    S_StartSound(NULL,sfx_swtchn);
        // 	}
        // 	return true;
        //
        //       default:
        // 	for (i = itemOn+1;i < currentMenu->numitems;i++)
        // 	    if (currentMenu->menuitems[i].alphaKey == ch)
        // 	    {
        // 		itemOn = i;
        // 		S_StartSound(NULL,sfx_pstop);
        // 		return true;
        // 	    }
        // 	for (i = 0;i <= itemOn;i++)
        // 	    if (currentMenu->menuitems[i].alphaKey == ch)
        // 	    {
        // 		itemOn = i;
        // 		S_StartSound(NULL,sfx_pstop);
        // 		return true;
        // 	    }
        // 	break;
        //
        //     }
        todo!("switch statement not yet translated");
        return false_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn M_StartControlPanel() {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     // intro might call this repeatedly
        //     if (menuactive)
        // 	return;
        todo!("if statement not yet translated");
        menuactive = 1;
        currentMenu = (&(MainDef) as *const menu_t as *mut menu_t);
        itemOn = (*currentMenu).lastOn;
    }
}

pub unsafe extern "C" fn M_Drawer() {
    unsafe {
        static mut x: std::ffi::c_short = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        static mut y: std::ffi::c_short = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut i: std::ffi::c_short = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut max: std::ffi::c_short = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut string: [std::ffi::c_char; (40) as usize] = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut start: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        inhelpscreens = false_;
        // TODO: if statement not yet translated:
        //
        //
        //
        //     // Horiz. & Vertically center string and print it.
        //     if (messageToPrint)
        //     {
        // 	start = 0;
        // 	y = 100 - M_StringHeight(messageString)/2;
        // 	while(*(messageString+start))
        // 	{
        // 	    for (i = 0;i < strlen(messageString+start);i++)
        // 		if (*(messageString+start+i) == '\n')
        // 		{
        // 		    memset(string,0,40);
        // 		    strncpy(string,messageString+start,i);
        // 		    start += i+1;
        // 		    break;
        // 		}
        //
        // 	    if (i == strlen(messageString+start))
        // 	    {
        // 		strcpy(string,messageString+start);
        // 		start += i;
        // 	    }
        //
        // 	    x = 160 - M_StringWidth(string)/2;
        // 	    M_WriteText(x,y,string);
        // 	    y += SHORT(hu_font[0]->height);
        // 	}
        // 	return;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (!menuactive)
        // 	return;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (currentMenu->routine)
        // 	currentMenu->routine();         // call Draw routine
        todo!("if statement not yet translated");
        x = (*currentMenu).x;
        y = (*currentMenu).y;
        max = (*currentMenu).numitems;
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0;i<max;i++)
        //     {
        // 	if (currentMenu->menuitems[i].name[0])
        // 	    V_DrawPatchDirect (x,y,0,
        // 			       W_CacheLumpName(currentMenu->menuitems[i].name ,PU_CACHE));
        // 	y += LINEHEIGHT;
        //     }
        todo!("for statement not yet translated");
        V_DrawPatchDirect(
            (x + SKULLXOFF),
            (((*currentMenu).y - 5) + (itemOn * LINEHEIGHT)),
            0,
            W_CacheLumpName(skullName[(whichSkull) as usize], PU_CACHE),
        );
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_ClearMenus() {
    unsafe {
        menuactive = 0;
        // TODO: statement not yet translated:
        //
        //     // if (!netgame && usergame && paused)
        //     //       sendpause = true;
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_SetupNextMenu(mut menudef: *mut menu_t) {
    unsafe {
        currentMenu = menudef;
        itemOn = (*currentMenu).lastOn;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_Ticker() {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (--skullAnimCounter <= 0)
        //     {
        // 	whichSkull ^= 1;
        // 	skullAnimCounter = 8;
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_Init() {
    unsafe {
        currentMenu = (&(MainDef) as *const menu_t as *mut menu_t);
        menuactive = 0;
        itemOn = (*currentMenu).lastOn;
        whichSkull = 0;
        skullAnimCounter = 10;
        screenSize = (screenblocks - 3);
        messageToPrint = 0;
        messageString = NULL;
        messageLastMenuActive = menuactive;
        quickSaveSlot = (-(1));
        // TODO: switch statement not yet translated:
        //
        //
        //     // Here we could catch other version dependencies,
        //     //  like HELP1/2, and four episodes.
        //
        //
        //     switch ( gamemode )
        //     {
        //       case commercial:
        // 	// This is used because DOOM 2 had only one HELP
        //         //  page. I use CREDIT as second page now, but
        // 	//  kept this hack for educational purposes.
        // 	MainMenu[readthis] = MainMenu[quitdoom];
        // 	MainDef.numitems--;
        // 	MainDef.y += 8;
        // 	NewDef.prevMenu = &MainDef;
        // 	ReadDef1.routine = M_DrawReadThis1;
        // 	ReadDef1.x = 330;
        // 	ReadDef1.y = 165;
        // 	ReadMenu1[0].routine = M_FinishReadThis;
        // 	break;
        //       case shareware:
        // 	// Episode 2 and 3 are handled,
        // 	//  branching to an ad screen.
        //       case registered:
        // 	// We need to remove the fourth episode.
        // 	EpiDef.numitems--;
        // 	break;
        //       case retail:
        // 	// We are fine.
        //       default:
        // 	break;
        //     }
        todo!("switch statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

const ZEROED_menuitem_t: menuitem_t = unsafe { std::mem::zeroed() };
