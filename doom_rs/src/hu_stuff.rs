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

pub unsafe extern "C" fn ForeignTranslation(mut ch: std::ffi::c_uchar) -> std::ffi::c_char {
    unsafe {
        return (if (ch < 128) {
            frenchKeyMap[(ch) as usize]
        } else {
            ch
        });
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn HU_Init() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut j: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut buffer: [std::ffi::c_char; (9) as usize] = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (french)
        // 	shiftxform = french_shiftxform;
        //     else
        // 	shiftxform = english_shiftxform;
        todo!("if statement not yet translated");
        j = HU_FONTSTART;
        // TODO: for statement not yet translated:
        //
        //     for (i=0;i<HU_FONTSIZE;i++)
        //     {
        // 	sprintf(buffer, "STCFN%.3d", j++);
        // 	hu_font[i] = (patch_t *) W_CacheLumpName(buffer, PU_STATIC);
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn HU_Stop() {
    unsafe {
        headsupactive = false_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn HU_Start() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut s: *mut std::ffi::c_char = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (headsupactive)
        // 	HU_Stop();
        todo!("if statement not yet translated");
        plr = (&(players[(consoleplayer) as usize]) as *const _ as *mut _);
        message_on = false_;
        message_dontfuckwithme = false_;
        message_nottobefuckedwith = false_;
        chat_on = false_;
        HUlib_initSText(
            (&(w_message) as *const hu_stext_t as *mut hu_stext_t),
            HU_MSGX,
            HU_MSGY,
            HU_MSGHEIGHT,
            hu_font,
            HU_FONTSTART,
            (&(message_on) as *const boolean as *mut boolean),
        );
        HUlib_initTextLine(
            (&(w_title) as *const hu_textline_t as *mut hu_textline_t),
            HU_TITLEX,
            HU_TITLEY,
            hu_font,
            HU_FONTSTART,
        );
        // TODO: switch statement not yet translated:
        //
        //
        //     switch ( gamemode )
        //     {
        //       case shareware:
        //       case registered:
        //       case retail:
        // 	s = HU_TITLE;
        // 	break;
        //
        // /* FIXME
        //       case pack_plut:
        // 	s = HU_TITLEP;
        // 	break;
        //       case pack_tnt:
        // 	s = HU_TITLET;
        // 	break;
        // */
        //
        //       case commercial:
        //       default:
        // 	 s = HU_TITLE2;
        // 	 break;
        //     }
        todo!("switch statement not yet translated");
        // TODO: while statement not yet translated:
        //
        //
        //     while (*s)
        // 	HUlib_addCharToTextLine(&w_title, *(s++));
        todo!("while statement not yet translated");
        HUlib_initIText(
            (&(w_chat) as *const hu_itext_t as *mut hu_itext_t),
            HU_INPUTX,
            HU_INPUTY,
            hu_font,
            HU_FONTSTART,
            (&(chat_on) as *const boolean as *mut boolean),
        );
        // TODO: for statement not yet translated:
        //
        //
        //     // create the inputbuffer widgets
        //     for (i=0 ; i<MAXPLAYERS ; i++)
        // 	HUlib_initIText(&w_inputbuffer[i], 0, 0, 0, 0, &always_off);
        todo!("for statement not yet translated");
        headsupactive = true_;
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn HU_Drawer() {
    unsafe {
        HUlib_drawSText((&(w_message) as *const hu_stext_t as *mut hu_stext_t));
        HUlib_drawIText((&(w_chat) as *const hu_itext_t as *mut hu_itext_t));
        // TODO: if statement not yet translated:
        //
        //     if (automapactive)
        // 	HUlib_drawTextLine(&w_title, false);
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn HU_Erase() {
    unsafe {
        HUlib_eraseSText((&(w_message) as *const hu_stext_t as *mut hu_stext_t));
        HUlib_eraseIText((&(w_chat) as *const hu_itext_t as *mut hu_itext_t));
        HUlib_eraseTextLine((&(w_title) as *const hu_textline_t as *mut hu_textline_t));
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn HU_Ticker() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut rc: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut c: std::ffi::c_char = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     // tick down message counter if message is up
        //     if (message_counter && !--message_counter)
        //     {
        // 	message_on = false;
        // 	message_nottobefuckedwith = false;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (showMessages || message_dontfuckwithme)
        //     {
        //
        // 	// display message if necessary
        // 	if ((plr->message && !message_nottobefuckedwith)
        // 	    || (plr->message && message_dontfuckwithme))
        // 	{
        // 	    HUlib_addMessageToSText(&w_message, 0, plr->message);
        // 	    plr->message = 0;
        // 	    message_on = true;
        // 	    message_counter = HU_MSGTIMEOUT;
        // 	    message_nottobefuckedwith = message_dontfuckwithme;
        // 	    message_dontfuckwithme = 0;
        // 	}
        //
        //     } // else message_on = false;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     // check for incoming chat characters
        //     if (netgame)
        //     {
        // 	for (i=0 ; i<MAXPLAYERS; i++)
        // 	{
        // 	    if (!playeringame[i])
        // 		continue;
        // 	    if (i != consoleplayer
        // 		&& (c = players[i].cmd.chatchar))
        // 	    {
        // 		if (c <= HU_BROADCAST)
        // 		    chat_dest[i] = c;
        // 		else
        // 		{
        // 		    if (c >= 'a' && c <= 'z')
        // 			c = (char) shiftxform[(unsigned char) c];
        // 		    rc = HUlib_keyInIText(&w_inputbuffer[i], c);
        // 		    if (rc && c == KEY_ENTER)
        // 		    {
        // 			if (w_inputbuffer[i].l.len
        // 			    && (chat_dest[i] == consoleplayer+1
        // 				|| chat_dest[i] == HU_BROADCAST))
        // 			{
        // 			    HUlib_addMessageToSText(&w_message,
        // 						    player_names[i],
        // 						    w_inputbuffer[i].l.l);
        //
        // 			    message_nottobefuckedwith = true;
        // 			    message_on = true;
        // 			    message_counter = HU_MSGTIMEOUT;
        // 			    if ( gamemode == commercial )
        // 			      S_StartSound(0, sfx_radio);
        // 			    else
        // 			      S_StartSound(0, sfx_tink);
        // 			}
        // 			HUlib_resetIText(&w_inputbuffer[i]);
        // 		    }
        // 		}
        // 		players[i].cmd.chatchar = 0;
        // 	    }
        // 	}
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub const QUEUESIZE: std::ffi::c_int = 128;

static mut chatchars: [std::ffi::c_char; (QUEUESIZE) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut head: std::ffi::c_int = unsafe { 0 };

static mut tail: std::ffi::c_int = unsafe { 0 };

pub unsafe extern "C" fn HU_queueChatChar(mut c: std::ffi::c_char) {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (((head + 1) & (QUEUESIZE-1)) == tail)
        //     {
        // 	plr->message = HUSTR_MSGU;
        //     }
        //     else
        //     {
        // 	chatchars[head] = c;
        // 	head = (head + 1) & (QUEUESIZE-1);
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn HU_dequeueChatChar() -> std::ffi::c_char {
    unsafe {
        let mut c: std::ffi::c_char = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (head != tail)
        //     {
        // 	c = chatchars[tail];
        // 	tail = (tail + 1) & (QUEUESIZE-1);
        //     }
        //     else
        //     {
        // 	c = 0;
        //     }
        todo!("if statement not yet translated");
        return c;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn HU_Responder(mut ev: *mut event_t) -> boolean {
    unsafe {
        static mut lastmessage: [std::ffi::c_char; (HU_MAXLINELENGTH + 1) as usize] = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut macromessage: *mut std::ffi::c_char = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut eatkey: boolean = unsafe { false_ };
        static mut shiftdown: boolean = unsafe { false_ };
        static mut altdown: boolean = unsafe { false_ };
        let mut c: std::ffi::c_uchar = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut numplayers: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        static mut destination_keys: [std::ffi::c_char; (MAXPLAYERS) as usize] = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        static mut num_nobrainers: std::ffi::c_int = unsafe { 0 };
        numplayers = 0;
        // TODO: for statement not yet translated:
        //
        //     for (i=0 ; i<MAXPLAYERS ; i++)
        // 	numplayers += playeringame[i];
        todo!("for statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (ev->data1 == KEY_RSHIFT)
        //     {
        // 	shiftdown = ev->type == ev_keydown;
        // 	return false;
        //     }
        //     else if (ev->data1 == KEY_RALT || ev->data1 == KEY_LALT)
        //     {
        // 	altdown = ev->type == ev_keydown;
        // 	return false;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (ev->type != ev_keydown)
        // 	return false;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (!chat_on)
        //     {
        // 	if (ev->data1 == HU_MSGREFRESH)
        // 	{
        // 	    message_on = true;
        // 	    message_counter = HU_MSGTIMEOUT;
        // 	    eatkey = true;
        // 	}
        // 	else if (netgame && ev->data1 == HU_INPUTTOGGLE)
        // 	{
        // 	    eatkey = chat_on = true;
        // 	    HUlib_resetIText(&w_chat);
        // 	    HU_queueChatChar(HU_BROADCAST);
        // 	}
        // 	else if (netgame && numplayers > 2)
        // 	{
        // 	    for (i=0; i<MAXPLAYERS ; i++)
        // 	    {
        // 		if (ev->data1 == destination_keys[i])
        // 		{
        // 		    if (playeringame[i] && i!=consoleplayer)
        // 		    {
        // 			eatkey = chat_on = true;
        // 			HUlib_resetIText(&w_chat);
        // 			HU_queueChatChar(i+1);
        // 			break;
        // 		    }
        // 		    else if (i == consoleplayer)
        // 		    {
        // 			num_nobrainers++;
        // 			if (num_nobrainers < 3)
        // 			    plr->message = HUSTR_TALKTOSELF1;
        // 			else if (num_nobrainers < 6)
        // 			    plr->message = HUSTR_TALKTOSELF2;
        // 			else if (num_nobrainers < 9)
        // 			    plr->message = HUSTR_TALKTOSELF3;
        // 			else if (num_nobrainers < 32)
        // 			    plr->message = HUSTR_TALKTOSELF4;
        // 			else
        // 			    plr->message = HUSTR_TALKTOSELF5;
        // 		    }
        // 		}
        // 	    }
        // 	}
        //     }
        //     else
        //     {
        // 	c = ev->data1;
        // 	// send a macro
        // 	if (altdown)
        // 	{
        // 	    c = c - '0';
        // 	    if (c > 9)
        // 		return false;
        // 	    // fprintf(stderr, "got here\n");
        // 	    macromessage = chat_macros[c];
        //
        // 	    // kill last message with a '\n'
        // 	    HU_queueChatChar(KEY_ENTER); // DEBUG!!!
        //
        // 	    // send the macro message
        // 	    while (*macromessage)
        // 		HU_queueChatChar(*macromessage++);
        // 	    HU_queueChatChar(KEY_ENTER);
        //
        // 	    // leave chat mode and notify that it was sent
        // 	    chat_on = false;
        // 	    strcpy(lastmessage, chat_macros[c]);
        // 	    plr->message = lastmessage;
        // 	    eatkey = true;
        // 	}
        // 	else
        // 	{
        // 	    if (french)
        // 		c = ForeignTranslation(c);
        // 	    if (shiftdown || (c >= 'a' && c <= 'z'))
        // 		c = shiftxform[c];
        // 	    eatkey = HUlib_keyInIText(&w_chat, c);
        // 	    if (eatkey)
        // 	    {
        // 		// static unsigned char buf[20]; // DEBUG
        // 		HU_queueChatChar(c);
        //
        // 		// sprintf(buf, "KEY: %d => %d", ev->data1, c);
        // 		//      plr->message = buf;
        // 	    }
        // 	    if (c == KEY_ENTER)
        // 	    {
        // 		chat_on = false;
        // 		if (w_chat.l.len)
        // 		{
        // 		    strcpy(lastmessage, w_chat.l.l);
        // 		    plr->message = lastmessage;
        // 		}
        // 	    }
        // 	    else if (c == KEY_ESCAPE)
        // 		chat_on = false;
        // 	}
        //     }
        todo!("if statement not yet translated");
        return eatkey;
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}
