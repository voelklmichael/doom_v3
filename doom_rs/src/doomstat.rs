use crate::d_items::*;
use crate::d_net::*;
use crate::d_player::*;
use crate::d_think::*;
use crate::d_ticcmd::*;
use crate::doomdata::*;
use crate::doomdef::*;
use crate::doomtype::*;
use crate::info::*;
use crate::m_fixed::*;
use crate::p_mobj::*;
use crate::p_pspr::*;
use crate::tables::*;

unsafe extern "C" {
    pub static mut nomonsters: boolean;
}

unsafe extern "C" {
    pub static mut respawnparm: boolean;
}

unsafe extern "C" {
    pub static mut fastparm: boolean;
}

unsafe extern "C" {
    pub static mut devparm: boolean;
}

unsafe extern "C" {
    pub static mut startskill: skill_t;
}

unsafe extern "C" {
    pub static mut startepisode: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut startmap: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut autostart: boolean;
}

unsafe extern "C" {
    pub static mut gameskill: skill_t;
}

unsafe extern "C" {
    pub static mut gameepisode: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut gamemap: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut respawnmonsters: boolean;
}

unsafe extern "C" {
    pub static mut netgame: boolean;
}

unsafe extern "C" {
    pub static mut deathmatch: boolean;
}

unsafe extern "C" {
    pub static mut snd_SfxVolume: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut snd_MusicVolume: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut snd_MusicDevice: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut snd_SfxDevice: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut snd_DesiredMusicDevice: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut snd_DesiredSfxDevice: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut statusbaractive: boolean;
}

unsafe extern "C" {
    pub static mut automapactive: boolean;
}

unsafe extern "C" {
    pub static mut menuactive: boolean;
}

unsafe extern "C" {
    pub static mut paused: boolean;
}

unsafe extern "C" {
    pub static mut viewactive: boolean;
}

unsafe extern "C" {
    pub static mut nodrawers: boolean;
}

unsafe extern "C" {
    pub static mut noblit: boolean;
}

unsafe extern "C" {
    pub static mut viewwindowx: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut viewwindowy: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut viewheight: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut viewwidth: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut scaledviewwidth: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut viewangleoffset: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut consoleplayer: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut displayplayer: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut totalkills: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut totalitems: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut totalsecret: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut levelstarttic: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut leveltime: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut usergame: boolean;
}

unsafe extern "C" {
    pub static mut demoplayback: boolean;
}

unsafe extern "C" {
    pub static mut demorecording: boolean;
}

unsafe extern "C" {
    pub static mut singledemo: boolean;
}

unsafe extern "C" {
    pub static mut gamestate: gamestate_t;
}

unsafe extern "C" {
    pub static mut gametic: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut players: [player_t; (MAXPLAYERS) as usize];
}

unsafe extern "C" {
    pub static mut playeringame: [boolean; (MAXPLAYERS) as usize];
}

pub const MAX_DM_STARTS: std::ffi::c_int = 10;

unsafe extern "C" {
    pub static mut deathmatchstarts: [mapthing_t; (MAX_DM_STARTS) as usize];
}

unsafe extern "C" {
    pub static mut deathmatch_p: *mut mapthing_t;
}

unsafe extern "C" {
    pub static mut playerstarts: [mapthing_t; (MAXPLAYERS) as usize];
}

unsafe extern "C" {
    pub static mut wminfo: wbstartstruct_t;
}

unsafe extern "C" {
    pub static mut maxammo: [std::ffi::c_int; (NUMAMMO) as usize];
}

unsafe extern "C" {
    pub static mut basedefault: [std::ffi::c_char; (1024) as usize];
}

unsafe extern "C" {
    pub static mut debugfile: *mut FILE;
}

unsafe extern "C" {
    pub static mut precache: boolean;
}

unsafe extern "C" {
    pub static mut wipegamestate: gamestate_t;
}

unsafe extern "C" {
    pub static mut mouseSensitivity: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut singletics: boolean;
}

unsafe extern "C" {
    pub static mut bodyqueslot: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut skyflatnum: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut doomcom: *mut doomcom_t;
}

unsafe extern "C" {
    pub static mut netbuffer: *mut doomdata_t;
}

unsafe extern "C" {
    pub static mut localcmds: [ticcmd_t; (BACKUPTICS) as usize];
}

unsafe extern "C" {
    pub static mut rndindex: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut maketic: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut nettics: [std::ffi::c_int; (MAXNETNODES) as usize];
}

unsafe extern "C" {
    pub static mut netcmds: [[ticcmd_t; (BACKUPTICS) as usize]; (MAXPLAYERS) as usize];
}

unsafe extern "C" {
    pub static mut ticdup: std::ffi::c_int;
}

static mut rcsid: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut gamemode: GameMode_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut gamemission: GameMission_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut language: Language_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut modifiedgame: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated
