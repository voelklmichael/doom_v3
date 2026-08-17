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

static mut rcsid: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

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

pub const ST_MAPWIDTH: std::ffi::c_int =
    (strlen(mapnames[(((gameepisode - 1) * 9) + (gamemap - 1)) as usize]));

pub const ST_MAPTITLEX: std::ffi::c_int = (SCREENWIDTH - (ST_MAPWIDTH * ST_CHATFONTWIDTH));

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

static mut st_palette: std::ffi::c_int = unsafe { 0 };

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

static mut st_stopped: boolean = unsafe { true_ };

pub unsafe extern "C" fn ST_Start() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn ST_Stop() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn ST_Init() {
    todo!("body not yet translated")
}
