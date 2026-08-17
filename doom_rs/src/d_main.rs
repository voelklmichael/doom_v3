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
use crate::f_finale::*;
use crate::f_wipe::*;
use crate::g_game::*;
use crate::hu_stuff::*;
use crate::i_sound::*;
use crate::i_system::*;
use crate::i_video::*;
use crate::info::*;
use crate::m_argv::*;
use crate::m_fixed::*;
use crate::m_menu::*;
use crate::m_misc::*;
use crate::p_mobj::*;
use crate::p_pspr::*;
use crate::p_setup::*;
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
use crate::st_stuff::*;
use crate::tables::*;
use crate::v_video::*;
use crate::w_wad::*;
use crate::wi_stuff::*;
use crate::z_zone::*;

static mut rcsid: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut wadfiles: [*mut std::ffi::c_char; (MAXWADFILES) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut devparm: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut nomonsters: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut respawnparm: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut fastparm: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut drone: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut singletics: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

unsafe extern "C" {
    pub static mut inhelpscreens: boolean;
}

pub static mut startskill: skill_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut startepisode: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut startmap: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut autostart: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut debugfile: *mut FILE = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut advancedemo: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut wadfile: [std::ffi::c_char; (1024) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut mapdir: [std::ffi::c_char; (1024) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut basedefault: [std::ffi::c_char; (1024) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

unsafe extern "C" {
    pub fn D_CheckNetGame();
}

unsafe extern "C" {
    pub fn G_BuildTiccmd(cmd: *mut ticcmd_t);
}

pub static mut events: [event_t; (MAXEVENTS) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut eventhead: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut eventtail: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn D_PostEvent(ev: *mut event_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn D_ProcessEvents() {
    todo!("body not yet translated")
}

pub static mut wipegamestate: gamestate_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

unsafe extern "C" {
    pub static mut setsizeneeded: boolean;
}

unsafe extern "C" {
    pub static mut showMessages: std::ffi::c_int;
}

unsafe extern "C" {
    pub fn R_ExecuteSetViewSize();
}

pub unsafe extern "C" fn D_Display() {
    todo!("body not yet translated")
}

unsafe extern "C" {
    pub static mut demorecording: boolean;
}

pub unsafe extern "C" fn D_DoomLoop() {
    todo!("body not yet translated")
}

pub static mut demosequence: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut pagetic: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut pagename: *mut std::ffi::c_char = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn D_PageTicker() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn D_PageDrawer() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn D_AdvanceDemo() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn D_DoAdvanceDemo() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn D_StartTitle() {
    todo!("body not yet translated")
}

pub static mut title: [std::ffi::c_char; (128) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn D_AddFile(file: *mut std::ffi::c_char) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn IdentifyVersion() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn FindResponseFile() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn D_DoomMain() {
    todo!("body not yet translated")
}
