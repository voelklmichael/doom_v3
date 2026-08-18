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
        HUSTR_CHATMACRO0,
        HUSTR_CHATMACRO1,
        HUSTR_CHATMACRO2,
        HUSTR_CHATMACRO3,
        HUSTR_CHATMACRO4,
        HUSTR_CHATMACRO5,
        HUSTR_CHATMACRO6,
        HUSTR_CHATMACRO7,
        HUSTR_CHATMACRO8,
        HUSTR_CHATMACRO9,
    ]
};

pub static mut player_names: [*mut std::ffi::c_char; 4] = unsafe {
    [
        HUSTR_PLRGREEN,
        HUSTR_PLRINDIGO,
        HUSTR_PLRBROWN,
        HUSTR_PLRRED,
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
        HUSTR_E1M1,
        HUSTR_E1M2,
        HUSTR_E1M3,
        HUSTR_E1M4,
        HUSTR_E1M5,
        HUSTR_E1M6,
        HUSTR_E1M7,
        HUSTR_E1M8,
        HUSTR_E1M9,
        HUSTR_E2M1,
        HUSTR_E2M2,
        HUSTR_E2M3,
        HUSTR_E2M4,
        HUSTR_E2M5,
        HUSTR_E2M6,
        HUSTR_E2M7,
        HUSTR_E2M8,
        HUSTR_E2M9,
        HUSTR_E3M1,
        HUSTR_E3M2,
        HUSTR_E3M3,
        HUSTR_E3M4,
        HUSTR_E3M5,
        HUSTR_E3M6,
        HUSTR_E3M7,
        HUSTR_E3M8,
        HUSTR_E3M9,
        HUSTR_E4M1,
        HUSTR_E4M2,
        HUSTR_E4M3,
        HUSTR_E4M4,
        HUSTR_E4M5,
        HUSTR_E4M6,
        HUSTR_E4M7,
        HUSTR_E4M8,
        HUSTR_E4M9,
        (c"NEWLEVEL").as_ptr(),
        (c"NEWLEVEL").as_ptr(),
        (c"NEWLEVEL").as_ptr(),
        (c"NEWLEVEL").as_ptr(),
        (c"NEWLEVEL").as_ptr(),
        (c"NEWLEVEL").as_ptr(),
        (c"NEWLEVEL").as_ptr(),
        (c"NEWLEVEL").as_ptr(),
        (c"NEWLEVEL").as_ptr(),
    ]
};

pub static mut mapnames2: [*mut std::ffi::c_char; 32] = unsafe {
    [
        HUSTR_1, HUSTR_2, HUSTR_3, HUSTR_4, HUSTR_5, HUSTR_6, HUSTR_7, HUSTR_8, HUSTR_9, HUSTR_10,
        HUSTR_11, HUSTR_12, HUSTR_13, HUSTR_14, HUSTR_15, HUSTR_16, HUSTR_17, HUSTR_18, HUSTR_19,
        HUSTR_20, HUSTR_21, HUSTR_22, HUSTR_23, HUSTR_24, HUSTR_25, HUSTR_26, HUSTR_27, HUSTR_28,
        HUSTR_29, HUSTR_30, HUSTR_31, HUSTR_32,
    ]
};

pub static mut mapnamesp: [*mut std::ffi::c_char; 32] = unsafe {
    [
        PHUSTR_1, PHUSTR_2, PHUSTR_3, PHUSTR_4, PHUSTR_5, PHUSTR_6, PHUSTR_7, PHUSTR_8, PHUSTR_9,
        PHUSTR_10, PHUSTR_11, PHUSTR_12, PHUSTR_13, PHUSTR_14, PHUSTR_15, PHUSTR_16, PHUSTR_17,
        PHUSTR_18, PHUSTR_19, PHUSTR_20, PHUSTR_21, PHUSTR_22, PHUSTR_23, PHUSTR_24, PHUSTR_25,
        PHUSTR_26, PHUSTR_27, PHUSTR_28, PHUSTR_29, PHUSTR_30, PHUSTR_31, PHUSTR_32,
    ]
};

pub static mut mapnamest: [*mut std::ffi::c_char; 32] = unsafe {
    [
        THUSTR_1, THUSTR_2, THUSTR_3, THUSTR_4, THUSTR_5, THUSTR_6, THUSTR_7, THUSTR_8, THUSTR_9,
        THUSTR_10, THUSTR_11, THUSTR_12, THUSTR_13, THUSTR_14, THUSTR_15, THUSTR_16, THUSTR_17,
        THUSTR_18, THUSTR_19, THUSTR_20, THUSTR_21, THUSTR_22, THUSTR_23, THUSTR_24, THUSTR_25,
        THUSTR_26, THUSTR_27, THUSTR_28, THUSTR_29, THUSTR_30, THUSTR_31, THUSTR_32,
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

pub static mut frenchKeyMap: [std::ffi::c_char; (128) as usize] = unsafe {
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
    todo!("body not yet translated")
}

pub unsafe extern "C" fn HU_Init() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn HU_Stop() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn HU_Start() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn HU_Drawer() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn HU_Erase() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn HU_Ticker() {
    todo!("body not yet translated")
}

pub const QUEUESIZE: std::ffi::c_int = 128;

static mut chatchars: [std::ffi::c_char; (QUEUESIZE) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut head: std::ffi::c_int = unsafe { 0 };

static mut tail: std::ffi::c_int = unsafe { 0 };

pub unsafe extern "C" fn HU_queueChatChar(c: std::ffi::c_char) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn HU_dequeueChatChar() -> std::ffi::c_char {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn HU_Responder(ev: *mut event_t) -> boolean {
    todo!("body not yet translated")
}
