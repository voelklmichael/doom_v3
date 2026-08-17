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

pub const AutomapState: std::ffi::c_int = 0;
pub const FirstPersonState: std::ffi::c_int = AutomapState + 1;

pub type st_stateenum_t = std::ffi::c_int;

pub const StartChatState: std::ffi::c_int = 0;
pub const WaitDestState: std::ffi::c_int = StartChatState + 1;
pub const GetChatState: std::ffi::c_int = WaitDestState + 1;

pub type st_chatstateenum_t = std::ffi::c_int;

static mut rcsid: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut plyr: *mut player_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut st_firsttime: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut veryfirsttime: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut lu_palette: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut st_clock: std::ffi::c_uint = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut st_msgcounter: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

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

static mut st_oldhealth: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut oldweaponsowned: [boolean; (NUMWEAPONS) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut st_facecount: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut st_faceindex: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut keyboxes: [std::ffi::c_int; (3) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut st_randomnumber: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut cheat_mus_seq: *mut std::ffi::c_uchar /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut cheat_choppers_seq: *mut std::ffi::c_uchar /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut cheat_god_seq: *mut std::ffi::c_uchar /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut cheat_ammo_seq: *mut std::ffi::c_uchar /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut cheat_ammonokey_seq: *mut std::ffi::c_uchar /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut cheat_noclip_seq: *mut std::ffi::c_uchar /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut cheat_commercial_noclip_seq: *mut std::ffi::c_uchar /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut cheat_powerup_seq: [[std::ffi::c_uchar; (10) as usize]; (7) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut cheat_clev_seq: *mut std::ffi::c_uchar /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut cheat_mypos_seq: *mut std::ffi::c_uchar /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut cheat_mus: cheatseq_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut cheat_god: cheatseq_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut cheat_ammo: cheatseq_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut cheat_ammonokey: cheatseq_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut cheat_noclip: cheatseq_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut cheat_commercial_noclip: cheatseq_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut cheat_powerup: [cheatseq_t; (7) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut cheat_choppers: cheatseq_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut cheat_clev: cheatseq_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut cheat_mypos: cheatseq_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

unsafe extern "C" {
    pub static mut mapnames: *mut *mut std::ffi::c_char;
}

pub unsafe extern "C" fn ST_refreshBackground() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn ST_Responder(ev: *mut event_t) -> boolean {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn ST_calcPainOffset() -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn ST_updateFaceWidget() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn ST_updateWidgets() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn ST_Ticker() {
    todo!("body not yet translated")
}

static mut st_palette: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn ST_doPaletteStuff() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn ST_drawWidgets(refresh: boolean) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn ST_doRefresh() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn ST_diffDraw() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn ST_Drawer(fullscreen: boolean, refresh: boolean) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn ST_loadGraphics() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn ST_loadData() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn ST_unloadGraphics() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn ST_unloadData() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn ST_initData() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn ST_createWidgets() {
    todo!("body not yet translated")
}

static mut st_stopped: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn ST_Start() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn ST_Stop() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn ST_Init() {
    todo!("body not yet translated")
}
