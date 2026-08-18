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
use crate::m_bbox::*;
use crate::m_fixed::*;
use crate::m_swap::*;
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
use crate::tables::*;
use crate::w_wad::*;
use crate::z_zone::*;

static mut rcsid: [std::ffi::c_char; 50] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        112 as std::ffi::c_char,
        95 as std::ffi::c_char,
        115 as std::ffi::c_char,
        101 as std::ffi::c_char,
        116 as std::ffi::c_char,
        117 as std::ffi::c_char,
        112 as std::ffi::c_char,
        46 as std::ffi::c_char,
        99 as std::ffi::c_char,
        44 as std::ffi::c_char,
        118 as std::ffi::c_char,
        32 as std::ffi::c_char,
        49 as std::ffi::c_char,
        46 as std::ffi::c_char,
        53 as std::ffi::c_char,
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
        50 as std::ffi::c_char,
        50 as std::ffi::c_char,
        58 as std::ffi::c_char,
        52 as std::ffi::c_char,
        53 as std::ffi::c_char,
        58 as std::ffi::c_char,
        49 as std::ffi::c_char,
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

unsafe extern "C" {
    pub fn P_SpawnMapThing(mthing: *mut mapthing_t);
}

pub static mut numvertexes: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut vertexes: *mut vertex_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut numsegs: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut segs: *mut seg_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut numsectors: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut sectors: *mut sector_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut numsubsectors: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut subsectors: *mut subsector_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut numnodes: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut nodes: *mut node_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut numlines: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut lines: *mut line_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut numsides: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut sides: *mut side_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut bmapwidth: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut bmapheight: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut blockmap: *mut std::ffi::c_short = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut blockmaplump: *mut std::ffi::c_short = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut bmaporgx: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut bmaporgy: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut blocklinks: *mut *mut mobj_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut rejectmatrix: *mut byte = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub const MAX_DEATHMATCH_STARTS: std::ffi::c_int = 10;

pub static mut deathmatchstarts: [mapthing_t; (MAX_DEATHMATCH_STARTS) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut deathmatch_p: *mut mapthing_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut playerstarts: [mapthing_t; (MAXPLAYERS) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn P_LoadVertexes(lump: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_LoadSegs(lump: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_LoadSubsectors(lump: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_LoadSectors(lump: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_LoadNodes(lump: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_LoadThings(lump: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_LoadLineDefs(lump: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_LoadSideDefs(lump: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_LoadBlockMap(lump: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_GroupLines() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_SetupLevel(
    episode: std::ffi::c_int,
    map: std::ffi::c_int,
    playermask: std::ffi::c_int,
    skill: skill_t,
) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_Init() {
    todo!("body not yet translated")
}
