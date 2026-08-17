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

static mut rcsid: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

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

pub static mut chat_macros: *mut *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut player_names: *mut *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut chat_char: std::ffi::c_char = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut plr: *mut player_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut hu_font: [*mut patch_t; (HU_FONTSIZE) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut w_title: hu_textline_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut chat_on: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut w_chat: hu_itext_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut always_off: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

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

static mut headsupactive: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut mapnames: *mut *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut mapnames2: *mut *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut mapnamesp: *mut *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut mapnamest: *mut *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut shiftxform: *mut std::ffi::c_char = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut french_shiftxform: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut english_shiftxform: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut frenchKeyMap: [std::ffi::c_char; (128) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

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

static mut head: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut tail: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn HU_queueChatChar(c: std::ffi::c_char) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn HU_dequeueChatChar() -> std::ffi::c_char {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn HU_Responder(ev: *mut event_t) -> boolean {
    todo!("body not yet translated")
}
