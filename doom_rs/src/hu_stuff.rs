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
use crate::hu_lib::*;
use crate::info::*;
use crate::m_fixed::*;
use crate::m_swap::*;
use crate::p_mobj::*;
use crate::p_pspr::*;
use crate::r_defs::*;
use crate::s_sound::*;
use crate::sounds::*;
use crate::tables::*;
use crate::w_wad::*;
use crate::z_zone::*;

pub const HU_FONTSTART: std::ffi::c_int = (b'!' as std::ffi::c_int);

pub const HU_FONTEND: std::ffi::c_int = (b'_' as std::ffi::c_int);

pub const HU_FONTSIZE: std::ffi::c_int = ((HU_FONTEND - HU_FONTSTART) + 1);

pub const HU_BROADCAST: std::ffi::c_int = 5;

pub const HU_MSGREFRESH: std::ffi::c_int = KEY_ENTER;

pub const HU_MSGX: std::ffi::c_int = 0;

pub const HU_MSGY: std::ffi::c_int = 0;

pub const HU_MSGWIDTH: std::ffi::c_int = 64;

pub const HU_MSGHEIGHT: std::ffi::c_int = 1;

pub const HU_MSGTIMEOUT: std::ffi::c_int = (4 * TICRATE);

static mut rcsid: [std::ffi::c_char; 51] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        104 as std::ffi::c_char,
        117 as std::ffi::c_char,
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
        50 as std::ffi::c_char,
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

pub const HU_TITLE: std::ffi::c_int =
    (mapnames[((((gameepisode - 1) * 9) + gamemap) - 1) as usize]);

pub const HU_TITLE2: std::ffi::c_int = (mapnames2[(gamemap - 1) as usize]);

pub const HU_TITLEP: std::ffi::c_int = (mapnamesp[(gamemap - 1) as usize]);

pub const HU_TITLET: std::ffi::c_int = (mapnamest[(gamemap - 1) as usize]);

pub const HU_TITLEHEIGHT: std::ffi::c_int = 1;

pub const HU_TITLEX: std::ffi::c_int = 0;

pub const HU_TITLEY: std::ffi::c_int = (167 - SHORT((*hu_font[(0) as usize]).height));

pub const HU_INPUTTOGGLE: std::ffi::c_int = (b't' as std::ffi::c_int);

pub const HU_INPUTX: std::ffi::c_int = HU_MSGX;

pub const HU_INPUTY: std::ffi::c_int =
    (HU_MSGY + (HU_MSGHEIGHT * (SHORT((*hu_font[(0) as usize]).height) + 1)));

pub const HU_INPUTWIDTH: std::ffi::c_int = 64;

pub const HU_INPUTHEIGHT: std::ffi::c_int = 1;

pub static mut chat_macros: [*mut std::ffi::c_char; 10] = unsafe {
    [
        HUSTR_CHATMACRO0 as *mut std::ffi::c_char,
        HUSTR_CHATMACRO1 as *mut std::ffi::c_char,
        HUSTR_CHATMACRO2 as *mut std::ffi::c_char,
        HUSTR_CHATMACRO3 as *mut std::ffi::c_char,
        HUSTR_CHATMACRO4 as *mut std::ffi::c_char,
        HUSTR_CHATMACRO5 as *mut std::ffi::c_char,
        HUSTR_CHATMACRO6 as *mut std::ffi::c_char,
        HUSTR_CHATMACRO7 as *mut std::ffi::c_char,
        HUSTR_CHATMACRO8 as *mut std::ffi::c_char,
        HUSTR_CHATMACRO9 as *mut std::ffi::c_char,
    ]
};

pub static mut player_names: [*mut std::ffi::c_char; 4] = unsafe {
    [
        HUSTR_PLRGREEN as *mut std::ffi::c_char,
        HUSTR_PLRINDIGO as *mut std::ffi::c_char,
        HUSTR_PLRBROWN as *mut std::ffi::c_char,
        HUSTR_PLRRED as *mut std::ffi::c_char,
    ]
};

pub static mut chat_char: std::ffi::c_char = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut plr: *mut player_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut hu_font: [*mut patch_t; (HU_FONTSIZE) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut w_title: hu_textline_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut chat_on: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut w_chat: hu_itext_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut always_off: boolean = unsafe { false_ };

static mut chat_dest: [std::ffi::c_char; (MAXPLAYERS) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut w_inputbuffer: [hu_itext_t; (MAXPLAYERS) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut message_on: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut message_dontfuckwithme: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut message_nottobefuckedwith: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut w_message: hu_stext_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut message_counter: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

unsafe extern "C" {
    pub static mut showMessages: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut automapactive: boolean;
}

static mut headsupactive: boolean = unsafe { false_ };

pub static mut mapnames: [*mut std::ffi::c_char; 45] = unsafe {
    [
        HUSTR_E1M1 as *mut std::ffi::c_char,
        HUSTR_E1M2 as *mut std::ffi::c_char,
        HUSTR_E1M3 as *mut std::ffi::c_char,
        HUSTR_E1M4 as *mut std::ffi::c_char,
        HUSTR_E1M5 as *mut std::ffi::c_char,
        HUSTR_E1M6 as *mut std::ffi::c_char,
        HUSTR_E1M7 as *mut std::ffi::c_char,
        HUSTR_E1M8 as *mut std::ffi::c_char,
        HUSTR_E1M9 as *mut std::ffi::c_char,
        HUSTR_E2M1 as *mut std::ffi::c_char,
        HUSTR_E2M2 as *mut std::ffi::c_char,
        HUSTR_E2M3 as *mut std::ffi::c_char,
        HUSTR_E2M4 as *mut std::ffi::c_char,
        HUSTR_E2M5 as *mut std::ffi::c_char,
        HUSTR_E2M6 as *mut std::ffi::c_char,
        HUSTR_E2M7 as *mut std::ffi::c_char,
        HUSTR_E2M8 as *mut std::ffi::c_char,
        HUSTR_E2M9 as *mut std::ffi::c_char,
        HUSTR_E3M1 as *mut std::ffi::c_char,
        HUSTR_E3M2 as *mut std::ffi::c_char,
        HUSTR_E3M3 as *mut std::ffi::c_char,
        HUSTR_E3M4 as *mut std::ffi::c_char,
        HUSTR_E3M5 as *mut std::ffi::c_char,
        HUSTR_E3M6 as *mut std::ffi::c_char,
        HUSTR_E3M7 as *mut std::ffi::c_char,
        HUSTR_E3M8 as *mut std::ffi::c_char,
        HUSTR_E3M9 as *mut std::ffi::c_char,
        HUSTR_E4M1 as *mut std::ffi::c_char,
        HUSTR_E4M2 as *mut std::ffi::c_char,
        HUSTR_E4M3 as *mut std::ffi::c_char,
        HUSTR_E4M4 as *mut std::ffi::c_char,
        HUSTR_E4M5 as *mut std::ffi::c_char,
        HUSTR_E4M6 as *mut std::ffi::c_char,
        HUSTR_E4M7 as *mut std::ffi::c_char,
        HUSTR_E4M8 as *mut std::ffi::c_char,
        HUSTR_E4M9 as *mut std::ffi::c_char,
        (c"NEWLEVEL").as_ptr() as *mut std::ffi::c_char,
        (c"NEWLEVEL").as_ptr() as *mut std::ffi::c_char,
        (c"NEWLEVEL").as_ptr() as *mut std::ffi::c_char,
        (c"NEWLEVEL").as_ptr() as *mut std::ffi::c_char,
        (c"NEWLEVEL").as_ptr() as *mut std::ffi::c_char,
        (c"NEWLEVEL").as_ptr() as *mut std::ffi::c_char,
        (c"NEWLEVEL").as_ptr() as *mut std::ffi::c_char,
        (c"NEWLEVEL").as_ptr() as *mut std::ffi::c_char,
        (c"NEWLEVEL").as_ptr() as *mut std::ffi::c_char,
    ]
};

pub static mut mapnames2: [*mut std::ffi::c_char; 32] = unsafe {
    [
        HUSTR_1 as *mut std::ffi::c_char,
        HUSTR_2 as *mut std::ffi::c_char,
        HUSTR_3 as *mut std::ffi::c_char,
        HUSTR_4 as *mut std::ffi::c_char,
        HUSTR_5 as *mut std::ffi::c_char,
        HUSTR_6 as *mut std::ffi::c_char,
        HUSTR_7 as *mut std::ffi::c_char,
        HUSTR_8 as *mut std::ffi::c_char,
        HUSTR_9 as *mut std::ffi::c_char,
        HUSTR_10 as *mut std::ffi::c_char,
        HUSTR_11 as *mut std::ffi::c_char,
        HUSTR_12 as *mut std::ffi::c_char,
        HUSTR_13 as *mut std::ffi::c_char,
        HUSTR_14 as *mut std::ffi::c_char,
        HUSTR_15 as *mut std::ffi::c_char,
        HUSTR_16 as *mut std::ffi::c_char,
        HUSTR_17 as *mut std::ffi::c_char,
        HUSTR_18 as *mut std::ffi::c_char,
        HUSTR_19 as *mut std::ffi::c_char,
        HUSTR_20 as *mut std::ffi::c_char,
        HUSTR_21 as *mut std::ffi::c_char,
        HUSTR_22 as *mut std::ffi::c_char,
        HUSTR_23 as *mut std::ffi::c_char,
        HUSTR_24 as *mut std::ffi::c_char,
        HUSTR_25 as *mut std::ffi::c_char,
        HUSTR_26 as *mut std::ffi::c_char,
        HUSTR_27 as *mut std::ffi::c_char,
        HUSTR_28 as *mut std::ffi::c_char,
        HUSTR_29 as *mut std::ffi::c_char,
        HUSTR_30 as *mut std::ffi::c_char,
        HUSTR_31 as *mut std::ffi::c_char,
        HUSTR_32 as *mut std::ffi::c_char,
    ]
};

pub static mut mapnamesp: [*mut std::ffi::c_char; 32] = unsafe {
    [
        PHUSTR_1 as *mut std::ffi::c_char,
        PHUSTR_2 as *mut std::ffi::c_char,
        PHUSTR_3 as *mut std::ffi::c_char,
        PHUSTR_4 as *mut std::ffi::c_char,
        PHUSTR_5 as *mut std::ffi::c_char,
        PHUSTR_6 as *mut std::ffi::c_char,
        PHUSTR_7 as *mut std::ffi::c_char,
        PHUSTR_8 as *mut std::ffi::c_char,
        PHUSTR_9 as *mut std::ffi::c_char,
        PHUSTR_10 as *mut std::ffi::c_char,
        PHUSTR_11 as *mut std::ffi::c_char,
        PHUSTR_12 as *mut std::ffi::c_char,
        PHUSTR_13 as *mut std::ffi::c_char,
        PHUSTR_14 as *mut std::ffi::c_char,
        PHUSTR_15 as *mut std::ffi::c_char,
        PHUSTR_16 as *mut std::ffi::c_char,
        PHUSTR_17 as *mut std::ffi::c_char,
        PHUSTR_18 as *mut std::ffi::c_char,
        PHUSTR_19 as *mut std::ffi::c_char,
        PHUSTR_20 as *mut std::ffi::c_char,
        PHUSTR_21 as *mut std::ffi::c_char,
        PHUSTR_22 as *mut std::ffi::c_char,
        PHUSTR_23 as *mut std::ffi::c_char,
        PHUSTR_24 as *mut std::ffi::c_char,
        PHUSTR_25 as *mut std::ffi::c_char,
        PHUSTR_26 as *mut std::ffi::c_char,
        PHUSTR_27 as *mut std::ffi::c_char,
        PHUSTR_28 as *mut std::ffi::c_char,
        PHUSTR_29 as *mut std::ffi::c_char,
        PHUSTR_30 as *mut std::ffi::c_char,
        PHUSTR_31 as *mut std::ffi::c_char,
        PHUSTR_32 as *mut std::ffi::c_char,
    ]
};

pub static mut mapnamest: [*mut std::ffi::c_char; 32] = unsafe {
    [
        THUSTR_1 as *mut std::ffi::c_char,
        THUSTR_2 as *mut std::ffi::c_char,
        THUSTR_3 as *mut std::ffi::c_char,
        THUSTR_4 as *mut std::ffi::c_char,
        THUSTR_5 as *mut std::ffi::c_char,
        THUSTR_6 as *mut std::ffi::c_char,
        THUSTR_7 as *mut std::ffi::c_char,
        THUSTR_8 as *mut std::ffi::c_char,
        THUSTR_9 as *mut std::ffi::c_char,
        THUSTR_10 as *mut std::ffi::c_char,
        THUSTR_11 as *mut std::ffi::c_char,
        THUSTR_12 as *mut std::ffi::c_char,
        THUSTR_13 as *mut std::ffi::c_char,
        THUSTR_14 as *mut std::ffi::c_char,
        THUSTR_15 as *mut std::ffi::c_char,
        THUSTR_16 as *mut std::ffi::c_char,
        THUSTR_17 as *mut std::ffi::c_char,
        THUSTR_18 as *mut std::ffi::c_char,
        THUSTR_19 as *mut std::ffi::c_char,
        THUSTR_20 as *mut std::ffi::c_char,
        THUSTR_21 as *mut std::ffi::c_char,
        THUSTR_22 as *mut std::ffi::c_char,
        THUSTR_23 as *mut std::ffi::c_char,
        THUSTR_24 as *mut std::ffi::c_char,
        THUSTR_25 as *mut std::ffi::c_char,
        THUSTR_26 as *mut std::ffi::c_char,
        THUSTR_27 as *mut std::ffi::c_char,
        THUSTR_28 as *mut std::ffi::c_char,
        THUSTR_29 as *mut std::ffi::c_char,
        THUSTR_30 as *mut std::ffi::c_char,
        THUSTR_31 as *mut std::ffi::c_char,
        THUSTR_32 as *mut std::ffi::c_char,
    ]
};

pub static mut shiftxform: *mut std::ffi::c_char = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut french_shiftxform: [std::ffi::c_char; 128] = unsafe {
    [
        0,
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        11,
        12,
        13,
        14,
        15,
        16,
        17,
        18,
        19,
        20,
        21,
        22,
        23,
        24,
        25,
        26,
        27,
        28,
        29,
        30,
        31,
        (b' ' as std::ffi::c_int),
        (b'!' as std::ffi::c_int),
        (b'"' as std::ffi::c_int),
        (b'#' as std::ffi::c_int),
        (b'$' as std::ffi::c_int),
        (b'%' as std::ffi::c_int),
        (b'&' as std::ffi::c_int),
        (b'"' as std::ffi::c_int),
        (b'(' as std::ffi::c_int),
        (b')' as std::ffi::c_int),
        (b'*' as std::ffi::c_int),
        (b'+' as std::ffi::c_int),
        (b'?' as std::ffi::c_int),
        (b'_' as std::ffi::c_int),
        (b'>' as std::ffi::c_int),
        (b'?' as std::ffi::c_int),
        (b'0' as std::ffi::c_int),
        (b'1' as std::ffi::c_int),
        (b'2' as std::ffi::c_int),
        (b'3' as std::ffi::c_int),
        (b'4' as std::ffi::c_int),
        (b'5' as std::ffi::c_int),
        (b'6' as std::ffi::c_int),
        (b'7' as std::ffi::c_int),
        (b'8' as std::ffi::c_int),
        (b'9' as std::ffi::c_int),
        (b'/' as std::ffi::c_int),
        (b'.' as std::ffi::c_int),
        (b'<' as std::ffi::c_int),
        (b'+' as std::ffi::c_int),
        (b'>' as std::ffi::c_int),
        (b'?' as std::ffi::c_int),
        (b'@' as std::ffi::c_int),
        (b'A' as std::ffi::c_int),
        (b'B' as std::ffi::c_int),
        (b'C' as std::ffi::c_int),
        (b'D' as std::ffi::c_int),
        (b'E' as std::ffi::c_int),
        (b'F' as std::ffi::c_int),
        (b'G' as std::ffi::c_int),
        (b'H' as std::ffi::c_int),
        (b'I' as std::ffi::c_int),
        (b'J' as std::ffi::c_int),
        (b'K' as std::ffi::c_int),
        (b'L' as std::ffi::c_int),
        (b'M' as std::ffi::c_int),
        (b'N' as std::ffi::c_int),
        (b'O' as std::ffi::c_int),
        (b'P' as std::ffi::c_int),
        (b'Q' as std::ffi::c_int),
        (b'R' as std::ffi::c_int),
        (b'S' as std::ffi::c_int),
        (b'T' as std::ffi::c_int),
        (b'U' as std::ffi::c_int),
        (b'V' as std::ffi::c_int),
        (b'W' as std::ffi::c_int),
        (b'X' as std::ffi::c_int),
        (b'Y' as std::ffi::c_int),
        (b'Z' as std::ffi::c_int),
        (b'[' as std::ffi::c_int),
        (b'!' as std::ffi::c_int),
        (b']' as std::ffi::c_int),
        (b'"' as std::ffi::c_int),
        (b'_' as std::ffi::c_int),
        (b'\'' as std::ffi::c_int),
        (b'A' as std::ffi::c_int),
        (b'B' as std::ffi::c_int),
        (b'C' as std::ffi::c_int),
        (b'D' as std::ffi::c_int),
        (b'E' as std::ffi::c_int),
        (b'F' as std::ffi::c_int),
        (b'G' as std::ffi::c_int),
        (b'H' as std::ffi::c_int),
        (b'I' as std::ffi::c_int),
        (b'J' as std::ffi::c_int),
        (b'K' as std::ffi::c_int),
        (b'L' as std::ffi::c_int),
        (b'M' as std::ffi::c_int),
        (b'N' as std::ffi::c_int),
        (b'O' as std::ffi::c_int),
        (b'P' as std::ffi::c_int),
        (b'Q' as std::ffi::c_int),
        (b'R' as std::ffi::c_int),
        (b'S' as std::ffi::c_int),
        (b'T' as std::ffi::c_int),
        (b'U' as std::ffi::c_int),
        (b'V' as std::ffi::c_int),
        (b'W' as std::ffi::c_int),
        (b'X' as std::ffi::c_int),
        (b'Y' as std::ffi::c_int),
        (b'Z' as std::ffi::c_int),
        (b'{' as std::ffi::c_int),
        (b'|' as std::ffi::c_int),
        (b'}' as std::ffi::c_int),
        (b'~' as std::ffi::c_int),
        127,
    ]
};

pub static mut english_shiftxform: [std::ffi::c_char; 128] = unsafe {
    [
        0,
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        11,
        12,
        13,
        14,
        15,
        16,
        17,
        18,
        19,
        20,
        21,
        22,
        23,
        24,
        25,
        26,
        27,
        28,
        29,
        30,
        31,
        (b' ' as std::ffi::c_int),
        (b'!' as std::ffi::c_int),
        (b'"' as std::ffi::c_int),
        (b'#' as std::ffi::c_int),
        (b'$' as std::ffi::c_int),
        (b'%' as std::ffi::c_int),
        (b'&' as std::ffi::c_int),
        (b'"' as std::ffi::c_int),
        (b'(' as std::ffi::c_int),
        (b')' as std::ffi::c_int),
        (b'*' as std::ffi::c_int),
        (b'+' as std::ffi::c_int),
        (b'<' as std::ffi::c_int),
        (b'_' as std::ffi::c_int),
        (b'>' as std::ffi::c_int),
        (b'?' as std::ffi::c_int),
        (b')' as std::ffi::c_int),
        (b'!' as std::ffi::c_int),
        (b'@' as std::ffi::c_int),
        (b'#' as std::ffi::c_int),
        (b'$' as std::ffi::c_int),
        (b'%' as std::ffi::c_int),
        (b'^' as std::ffi::c_int),
        (b'&' as std::ffi::c_int),
        (b'*' as std::ffi::c_int),
        (b'(' as std::ffi::c_int),
        (b':' as std::ffi::c_int),
        (b':' as std::ffi::c_int),
        (b'<' as std::ffi::c_int),
        (b'+' as std::ffi::c_int),
        (b'>' as std::ffi::c_int),
        (b'?' as std::ffi::c_int),
        (b'@' as std::ffi::c_int),
        (b'A' as std::ffi::c_int),
        (b'B' as std::ffi::c_int),
        (b'C' as std::ffi::c_int),
        (b'D' as std::ffi::c_int),
        (b'E' as std::ffi::c_int),
        (b'F' as std::ffi::c_int),
        (b'G' as std::ffi::c_int),
        (b'H' as std::ffi::c_int),
        (b'I' as std::ffi::c_int),
        (b'J' as std::ffi::c_int),
        (b'K' as std::ffi::c_int),
        (b'L' as std::ffi::c_int),
        (b'M' as std::ffi::c_int),
        (b'N' as std::ffi::c_int),
        (b'O' as std::ffi::c_int),
        (b'P' as std::ffi::c_int),
        (b'Q' as std::ffi::c_int),
        (b'R' as std::ffi::c_int),
        (b'S' as std::ffi::c_int),
        (b'T' as std::ffi::c_int),
        (b'U' as std::ffi::c_int),
        (b'V' as std::ffi::c_int),
        (b'W' as std::ffi::c_int),
        (b'X' as std::ffi::c_int),
        (b'Y' as std::ffi::c_int),
        (b'Z' as std::ffi::c_int),
        (b'[' as std::ffi::c_int),
        (b'!' as std::ffi::c_int),
        (b']' as std::ffi::c_int),
        (b'"' as std::ffi::c_int),
        (b'_' as std::ffi::c_int),
        (b'\'' as std::ffi::c_int),
        (b'A' as std::ffi::c_int),
        (b'B' as std::ffi::c_int),
        (b'C' as std::ffi::c_int),
        (b'D' as std::ffi::c_int),
        (b'E' as std::ffi::c_int),
        (b'F' as std::ffi::c_int),
        (b'G' as std::ffi::c_int),
        (b'H' as std::ffi::c_int),
        (b'I' as std::ffi::c_int),
        (b'J' as std::ffi::c_int),
        (b'K' as std::ffi::c_int),
        (b'L' as std::ffi::c_int),
        (b'M' as std::ffi::c_int),
        (b'N' as std::ffi::c_int),
        (b'O' as std::ffi::c_int),
        (b'P' as std::ffi::c_int),
        (b'Q' as std::ffi::c_int),
        (b'R' as std::ffi::c_int),
        (b'S' as std::ffi::c_int),
        (b'T' as std::ffi::c_int),
        (b'U' as std::ffi::c_int),
        (b'V' as std::ffi::c_int),
        (b'W' as std::ffi::c_int),
        (b'X' as std::ffi::c_int),
        (b'Y' as std::ffi::c_int),
        (b'Z' as std::ffi::c_int),
        (b'{' as std::ffi::c_int),
        (b'|' as std::ffi::c_int),
        (b'}' as std::ffi::c_int),
        (b'~' as std::ffi::c_int),
        127,
    ]
};

pub static mut frenchKeyMap: [std::ffi::c_char; 128] = unsafe {
    [
        0,
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        11,
        12,
        13,
        14,
        15,
        16,
        17,
        18,
        19,
        20,
        21,
        22,
        23,
        24,
        25,
        26,
        27,
        28,
        29,
        30,
        31,
        (b' ' as std::ffi::c_int),
        (b'!' as std::ffi::c_int),
        (b'"' as std::ffi::c_int),
        (b'#' as std::ffi::c_int),
        (b'$' as std::ffi::c_int),
        (b'%' as std::ffi::c_int),
        (b'&' as std::ffi::c_int),
        (b'%' as std::ffi::c_int),
        (b'(' as std::ffi::c_int),
        (b')' as std::ffi::c_int),
        (b'*' as std::ffi::c_int),
        (b'+' as std::ffi::c_int),
        (b';' as std::ffi::c_int),
        (b'-' as std::ffi::c_int),
        (b':' as std::ffi::c_int),
        (b'!' as std::ffi::c_int),
        (b'0' as std::ffi::c_int),
        (b'1' as std::ffi::c_int),
        (b'2' as std::ffi::c_int),
        (b'3' as std::ffi::c_int),
        (b'4' as std::ffi::c_int),
        (b'5' as std::ffi::c_int),
        (b'6' as std::ffi::c_int),
        (b'7' as std::ffi::c_int),
        (b'8' as std::ffi::c_int),
        (b'9' as std::ffi::c_int),
        (b':' as std::ffi::c_int),
        (b'M' as std::ffi::c_int),
        (b'<' as std::ffi::c_int),
        (b'=' as std::ffi::c_int),
        (b'>' as std::ffi::c_int),
        (b'?' as std::ffi::c_int),
        (b'@' as std::ffi::c_int),
        (b'Q' as std::ffi::c_int),
        (b'B' as std::ffi::c_int),
        (b'C' as std::ffi::c_int),
        (b'D' as std::ffi::c_int),
        (b'E' as std::ffi::c_int),
        (b'F' as std::ffi::c_int),
        (b'G' as std::ffi::c_int),
        (b'H' as std::ffi::c_int),
        (b'I' as std::ffi::c_int),
        (b'J' as std::ffi::c_int),
        (b'K' as std::ffi::c_int),
        (b'L' as std::ffi::c_int),
        (b',' as std::ffi::c_int),
        (b'N' as std::ffi::c_int),
        (b'O' as std::ffi::c_int),
        (b'P' as std::ffi::c_int),
        (b'A' as std::ffi::c_int),
        (b'R' as std::ffi::c_int),
        (b'S' as std::ffi::c_int),
        (b'T' as std::ffi::c_int),
        (b'U' as std::ffi::c_int),
        (b'V' as std::ffi::c_int),
        (b'Z' as std::ffi::c_int),
        (b'X' as std::ffi::c_int),
        (b'Y' as std::ffi::c_int),
        (b'W' as std::ffi::c_int),
        (b'^' as std::ffi::c_int),
        (b'\\' as std::ffi::c_int),
        (b'$' as std::ffi::c_int),
        (b'^' as std::ffi::c_int),
        (b'_' as std::ffi::c_int),
        (b'@' as std::ffi::c_int),
        (b'Q' as std::ffi::c_int),
        (b'B' as std::ffi::c_int),
        (b'C' as std::ffi::c_int),
        (b'D' as std::ffi::c_int),
        (b'E' as std::ffi::c_int),
        (b'F' as std::ffi::c_int),
        (b'G' as std::ffi::c_int),
        (b'H' as std::ffi::c_int),
        (b'I' as std::ffi::c_int),
        (b'J' as std::ffi::c_int),
        (b'K' as std::ffi::c_int),
        (b'L' as std::ffi::c_int),
        (b',' as std::ffi::c_int),
        (b'N' as std::ffi::c_int),
        (b'O' as std::ffi::c_int),
        (b'P' as std::ffi::c_int),
        (b'A' as std::ffi::c_int),
        (b'R' as std::ffi::c_int),
        (b'S' as std::ffi::c_int),
        (b'T' as std::ffi::c_int),
        (b'U' as std::ffi::c_int),
        (b'V' as std::ffi::c_int),
        (b'Z' as std::ffi::c_int),
        (b'X' as std::ffi::c_int),
        (b'Y' as std::ffi::c_int),
        (b'W' as std::ffi::c_int),
        (b'^' as std::ffi::c_int),
        (b'\\' as std::ffi::c_int),
        (b'$' as std::ffi::c_int),
        (b'^' as std::ffi::c_int),
        127,
    ]
};

pub unsafe extern "C" fn ForeignTranslation(ch: std::ffi::c_uchar) -> std::ffi::c_char {
    unsafe { todo!("body not yet translated") }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn HU_Init() {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn HU_Stop() {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn HU_Start() {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn HU_Drawer() {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn HU_Erase() {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn HU_Ticker() {
    unsafe { todo!("body not yet translated") }
}

pub const QUEUESIZE: std::ffi::c_int = 128;

static mut chatchars: [std::ffi::c_char; (QUEUESIZE) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut head: std::ffi::c_int = unsafe { 0 };

static mut tail: std::ffi::c_int = unsafe { 0 };

pub unsafe extern "C" fn HU_queueChatChar(c: std::ffi::c_char) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn HU_dequeueChatChar() -> std::ffi::c_char {
    unsafe { todo!("body not yet translated") }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn HU_Responder(ev: *mut event_t) -> boolean {
    unsafe { todo!("body not yet translated") }
    todo!("fell off the end of a non-void C function")
}
