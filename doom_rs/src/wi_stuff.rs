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
use crate::g_game::*;
use crate::i_system::*;
use crate::info::*;
use crate::m_fixed::*;
use crate::m_random::*;
use crate::m_swap::*;
use crate::p_mobj::*;
use crate::p_pspr::*;
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
use crate::tables::*;
use crate::v_video::*;
use crate::w_wad::*;
use crate::z_zone::*;

pub const NoState: std::ffi::c_int = -1;
pub const StatCount: std::ffi::c_int = NoState + 1;
pub const ShowNextLoc: std::ffi::c_int = StatCount + 1;

pub type stateenum_t = std::ffi::c_int;

static mut rcsid: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub const ANIM_ALWAYS: std::ffi::c_int = 0;
pub const ANIM_RANDOM: std::ffi::c_int = ANIM_ALWAYS + 1;
pub const ANIM_LEVEL: std::ffi::c_int = ANIM_RANDOM + 1;

pub type animenum_t = std::ffi::c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct point_t {
    pub x: std::ffi::c_int,
    pub y: std::ffi::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct anim_t {
    pub type_: animenum_t,
    pub period: std::ffi::c_int,
    pub nanims: std::ffi::c_int,
    pub loc: point_t,
    pub data1: std::ffi::c_int,
    pub data2: std::ffi::c_int,
    pub p: [*mut patch_t; (3) as usize],
    pub nexttic: std::ffi::c_int,
    pub lastdrawn: std::ffi::c_int,
    pub ctr: std::ffi::c_int,
    pub state: std::ffi::c_int,
}

static mut lnodes: [[point_t; (NUMMAPS) as usize]; (NUMEPISODES) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut epsd0animinfo: *mut anim_t /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut epsd1animinfo: *mut anim_t /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut epsd2animinfo: *mut anim_t /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut NUMANIMS: [std::ffi::c_int; (NUMEPISODES) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut anims: [*mut anim_t; (NUMEPISODES) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut acceleratestage: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut me: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut state: stateenum_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut wbs: *mut wbstartstruct_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut plrs: *mut wbplayerstruct_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut cnt: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut bcnt: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut firstrefresh: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut cnt_kills: [std::ffi::c_int; (MAXPLAYERS) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut cnt_items: [std::ffi::c_int; (MAXPLAYERS) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut cnt_secret: [std::ffi::c_int; (MAXPLAYERS) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut cnt_time: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut cnt_par: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut cnt_pause: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut NUMCMAPS: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut bg: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut yah: [*mut patch_t; (2) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut splat: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut percent: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut colon: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut num: [*mut patch_t; (10) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut wiminus: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut finished: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut entering: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut sp_secret: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut kills: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut secret: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut items: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut frags: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut time: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut par: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut sucks: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut killers: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut victims: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut total: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut star: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut bstar: *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut p: [*mut patch_t; (MAXPLAYERS) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut bp: [*mut patch_t; (MAXPLAYERS) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut lnames: *mut *mut patch_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn WI_slamBackground() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn WI_Responder(ev: *mut event_t) -> boolean {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn WI_drawLF() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn WI_drawEL() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn WI_drawOnLnode(
    n: std::ffi::c_int,
    c: *mut *mut patch_t, /* TODO: was unsized array */
) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn WI_initAnimatedBack() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn WI_updateAnimatedBack() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn WI_drawAnimatedBack() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn WI_drawNum(
    x: std::ffi::c_int,
    y: std::ffi::c_int,
    n: std::ffi::c_int,
    digits: std::ffi::c_int,
) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn WI_drawPercent(
    x: std::ffi::c_int,
    y: std::ffi::c_int,
    p: std::ffi::c_int,
) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn WI_drawTime(x: std::ffi::c_int, y: std::ffi::c_int, t: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn WI_End() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn WI_initNoState() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn WI_updateNoState() {
    todo!("body not yet translated")
}

static mut snl_pointeron: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn WI_initShowNextLoc() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn WI_updateShowNextLoc() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn WI_drawShowNextLoc() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn WI_drawNoState() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn WI_fragSum(playernum: std::ffi::c_int) -> std::ffi::c_int {
    todo!("body not yet translated")
}

static mut dm_state: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut dm_frags: [[std::ffi::c_int; (MAXPLAYERS) as usize]; (MAXPLAYERS) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut dm_totals: [std::ffi::c_int; (MAXPLAYERS) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn WI_initDeathmatchStats() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn WI_updateDeathmatchStats() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn WI_drawDeathmatchStats() {
    todo!("body not yet translated")
}

static mut cnt_frags: [std::ffi::c_int; (MAXPLAYERS) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut dofrags: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut ng_state: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn WI_initNetgameStats() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn WI_updateNetgameStats() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn WI_drawNetgameStats() {
    todo!("body not yet translated")
}

static mut sp_state: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn WI_initStats() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn WI_updateStats() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn WI_drawStats() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn WI_checkForAccelerate() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn WI_Ticker() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn WI_loadData() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn WI_unloadData() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn WI_Drawer() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn WI_initVariables(wbstartstruct: *mut wbstartstruct_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn WI_Start(wbstartstruct: *mut wbstartstruct_t) {
    todo!("body not yet translated")
}
