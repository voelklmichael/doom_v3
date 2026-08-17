use crate::am_map::*;
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
use crate::f_finale::*;
use crate::hu_stuff::*;
use crate::i_system::*;
use crate::info::*;
use crate::m_argv::*;
use crate::m_fixed::*;
use crate::m_menu::*;
use crate::m_misc::*;
use crate::m_random::*;
use crate::p_local::*;
use crate::p_mobj::*;
use crate::p_pspr::*;
use crate::p_saveg::*;
use crate::p_setup::*;
use crate::p_spec::*;
use crate::p_tick::*;
use crate::r_bsp::*;
use crate::r_data::*;
use crate::r_defs::*;
use crate::r_draw::*;
use crate::r_local::*;
use crate::r_main::*;
use crate::r_plane::*;
use crate::r_segs::*;
use crate::r_sky::*;
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

unsafe extern "C" {
    pub fn G_PlayDemo(name: *mut std::ffi::c_char);
}

static mut rcsid: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub const SAVEGAMESIZE: std::ffi::c_int = 0x2c000;

pub const SAVESTRINGSIZE: std::ffi::c_int = 24;

unsafe extern "C" {
    pub fn G_DoVictory();
}

pub static mut gameaction: gameaction_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut gamestate: gamestate_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut gameskill: skill_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut respawnmonsters: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut gameepisode: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut gamemap: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut paused: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut sendpause: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut sendsave: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut usergame: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut timingdemo: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut nodrawers: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut noblit: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut starttime: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut viewactive: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut deathmatch: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut netgame: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut playeringame: [boolean; (MAXPLAYERS) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut players: [player_t; (MAXPLAYERS) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut consoleplayer: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut displayplayer: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut gametic: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut levelstarttic: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

// TODO: unparsed multi-declarator variable, needs manual translation: totalsecret: /* unrecognized type: int totalkills, totalitems, */,

pub static mut demoname: [std::ffi::c_char; (32) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut demorecording: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut demoplayback: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut netdemo: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut demobuffer: *mut byte = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut demo_p: *mut byte = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut demoend: *mut byte = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut singledemo: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut precache: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut wminfo: wbstartstruct_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut consistancy: [[std::ffi::c_short; (BACKUPTICS) as usize]; (MAXPLAYERS) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut savebuffer: *mut byte = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut key_right: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut key_left: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut key_up: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut key_down: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut key_strafeleft: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut key_straferight: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut key_fire: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut key_use: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut key_strafe: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut key_speed: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut mousebfire: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut mousebstrafe: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut mousebforward: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut joybfire: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut joybstrafe: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut joybuse: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut joybspeed: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub const MAXPLMOVE: std::ffi::c_int = (forwardmove[(1) as usize]);

pub const TURBOTHRESHOLD: std::ffi::c_int = 0x32;

pub static mut forwardmove: [fixed_t; (2) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut sidemove: [fixed_t; (2) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut angleturn: [fixed_t; (3) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub const SLOWTURNTICS: std::ffi::c_int = 6;

pub const NUMKEYS: std::ffi::c_int = 256;

pub static mut gamekeydown: [boolean; (NUMKEYS) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut turnheld: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut mousearray: [boolean; (4) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut mousebuttons: *mut boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut mousex: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut mousey: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut dclicktime: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut dclickstate: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut dclicks: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut dclicktime2: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut dclickstate2: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut dclicks2: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut joyxmove: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut joyymove: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut joyarray: [boolean; (5) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut joybuttons: *mut boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut savegameslot: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut savedescription: [std::ffi::c_char; (32) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub const BODYQUESIZE: std::ffi::c_int = 32;

pub static mut bodyque: [*mut mobj_t; (BODYQUESIZE) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut bodyqueslot: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut statcopy: *mut std::ffi::c_void = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn G_CmdChecksum(cmd: *mut ticcmd_t) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn G_BuildTiccmd(cmd: *mut ticcmd_t) {
    todo!("body not yet translated")
}

unsafe extern "C" {
    pub static mut wipegamestate: gamestate_t;
}

pub unsafe extern "C" fn G_DoLoadLevel() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn G_Responder(ev: *mut event_t) -> boolean {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn G_Ticker() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn G_InitPlayer(player: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn G_PlayerFinishLevel(player: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn G_PlayerReborn(player: std::ffi::c_int) {
    todo!("body not yet translated")
}

unsafe extern "C" {
    pub fn P_SpawnPlayer(mthing: *mut mapthing_t);
}

pub unsafe extern "C" fn G_CheckSpot(
    playernum: std::ffi::c_int,
    mthing: *mut mapthing_t,
) -> boolean {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn G_DeathMatchSpawnPlayer(playernum: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn G_DoReborn(playernum: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn G_ScreenShot() {
    todo!("body not yet translated")
}

pub static mut pars: [[std::ffi::c_int; (10) as usize]; (4) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut cpars: [std::ffi::c_int; (32) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut secretexit: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

unsafe extern "C" {
    pub static mut pagename: *mut std::ffi::c_char;
}

pub unsafe extern "C" fn G_ExitLevel() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn G_SecretExitLevel() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn G_DoCompleted() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn G_WorldDone() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn G_DoWorldDone() {
    todo!("body not yet translated")
}

unsafe extern "C" {
    pub static mut setsizeneeded: boolean;
}

unsafe extern "C" {
    pub fn R_ExecuteSetViewSize();
}

pub static mut savename: [std::ffi::c_char; (256) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn G_LoadGame(name: *mut std::ffi::c_char) {
    todo!("body not yet translated")
}

pub const VERSIONSIZE: std::ffi::c_int = 16;

pub unsafe extern "C" fn G_DoLoadGame() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn G_SaveGame(slot: std::ffi::c_int, description: *mut std::ffi::c_char) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn G_DoSaveGame() {
    todo!("body not yet translated")
}

pub static mut d_skill: skill_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut d_episode: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut d_map: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn G_DeferedInitNew(
    skill: skill_t,
    episode: std::ffi::c_int,
    map: std::ffi::c_int,
) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn G_DoNewGame() {
    todo!("body not yet translated")
}

unsafe extern "C" {
    pub static mut skytexture: std::ffi::c_int;
}

pub unsafe extern "C" fn G_InitNew(skill: skill_t, episode: std::ffi::c_int, map: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub const DEMOMARKER: std::ffi::c_int = 0x80;

pub unsafe extern "C" fn G_ReadDemoTiccmd(cmd: *mut ticcmd_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn G_WriteDemoTiccmd(cmd: *mut ticcmd_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn G_RecordDemo(name: *mut std::ffi::c_char) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn G_BeginRecording() {
    todo!("body not yet translated")
}

pub static mut defdemoname: *mut std::ffi::c_char = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn G_DeferedPlayDemo(name: *mut std::ffi::c_char) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn G_DoPlayDemo() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn G_TimeDemo(name: *mut std::ffi::c_char) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn G_CheckDemoStatus() -> boolean {
    todo!("body not yet translated")
}
