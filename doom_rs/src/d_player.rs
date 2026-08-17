use crate::d_items::*;
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

pub const PST_LIVE: std::ffi::c_int = 0;
pub const PST_DEAD: std::ffi::c_int = PST_LIVE + 1;
pub const PST_REBORN: std::ffi::c_int = PST_DEAD + 1;

pub type playerstate_t = std::ffi::c_int;

pub const CF_NOCLIP: std::ffi::c_int = 1;
pub const CF_GODMODE: std::ffi::c_int = 2;
pub const CF_NOMOMENTUM: std::ffi::c_int = 4;

pub type cheat_t = std::ffi::c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct player_t {
    pub mo: *mut mobj_t,
    pub playerstate: playerstate_t,
    pub cmd: ticcmd_t,
    pub viewz: fixed_t,
    pub viewheight: fixed_t,
    pub deltaviewheight: fixed_t,
    pub bob: fixed_t,
    pub health: std::ffi::c_int,
    pub armorpoints: std::ffi::c_int,
    pub armortype: std::ffi::c_int,
    pub powers: [std::ffi::c_int; (NUMPOWERS) as usize],
    pub cards: [boolean; (NUMCARDS) as usize],
    pub backpack: boolean,
    pub frags: [std::ffi::c_int; (MAXPLAYERS) as usize],
    pub readyweapon: weapontype_t,
    pub pendingweapon: weapontype_t,
    pub weaponowned: [boolean; (NUMWEAPONS) as usize],
    pub ammo: [std::ffi::c_int; (NUMAMMO) as usize],
    pub maxammo: [std::ffi::c_int; (NUMAMMO) as usize],
    pub attackdown: std::ffi::c_int,
    pub usedown: std::ffi::c_int,
    pub cheats: std::ffi::c_int,
    pub refire: std::ffi::c_int,
    pub killcount: std::ffi::c_int,
    pub itemcount: std::ffi::c_int,
    pub secretcount: std::ffi::c_int,
    pub message: *mut std::ffi::c_char,
    pub damagecount: std::ffi::c_int,
    pub bonuscount: std::ffi::c_int,
    pub attacker: *mut mobj_t,
    pub extralight: std::ffi::c_int,
    pub fixedcolormap: std::ffi::c_int,
    pub colormap: std::ffi::c_int,
    pub psprites: [pspdef_t; (NUMPSPRITES) as usize],
    pub didsecret: boolean,
}

pub type player_s = player_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wbplayerstruct_t {
    pub in_: boolean,
    pub skills: std::ffi::c_int,
    pub sitems: std::ffi::c_int,
    pub ssecret: std::ffi::c_int,
    pub stime: std::ffi::c_int,
    pub frags: [std::ffi::c_int; (4) as usize],
    pub score: std::ffi::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wbstartstruct_t {
    pub epsd: std::ffi::c_int,
    pub didsecret: boolean,
    pub last: std::ffi::c_int,
    pub next: std::ffi::c_int,
    pub maxkills: std::ffi::c_int,
    pub maxitems: std::ffi::c_int,
    pub maxsecret: std::ffi::c_int,
    pub maxfrags: std::ffi::c_int,
    pub partime: std::ffi::c_int,
    pub pnum: std::ffi::c_int,
    pub plyr: [wbplayerstruct_t; (MAXPLAYERS) as usize],
}
