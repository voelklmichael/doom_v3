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
use crate::i_system::*;
use crate::info::*;
use crate::m_fixed::*;
use crate::p_mobj::*;
use crate::p_pspr::*;
use crate::r_bsp::*;
use crate::r_data::*;
use crate::r_defs::*;
use crate::r_draw::*;
use crate::r_local::*;
use crate::r_main::*;
use crate::r_segs::*;
use crate::r_sky::*;
use crate::r_state::*;
use crate::r_things::*;
use crate::tables::*;
use crate::w_wad::*;
use crate::z_zone::*;

pub type planefunction_t = Option<unsafe extern "C" fn(std::ffi::c_int, std::ffi::c_int)>;

unsafe extern "C" {
    pub static mut ceilingfunc_t: planefunction_t;
}

static mut rcsid: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut floorfunc: planefunction_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut ceilingfunc: planefunction_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub const MAXVISPLANES: std::ffi::c_int = 128;

pub static mut visplanes: [visplane_t; (MAXVISPLANES) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut lastvisplane: *mut visplane_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut floorplane: *mut visplane_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut ceilingplane: *mut visplane_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub const MAXOPENINGS: std::ffi::c_int = (SCREENWIDTH * 64);

pub static mut openings: [std::ffi::c_short; (MAXOPENINGS) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut lastopening: *mut std::ffi::c_short = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut floorclip: [std::ffi::c_short; (SCREENWIDTH) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut ceilingclip: [std::ffi::c_short; (SCREENWIDTH) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut spanstart: [std::ffi::c_int; (SCREENHEIGHT) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut spanstop: [std::ffi::c_int; (SCREENHEIGHT) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut planezlight: *mut *mut lighttable_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut planeheight: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut yslope: [fixed_t; (SCREENHEIGHT) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut distscale: [fixed_t; (SCREENWIDTH) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut basexscale: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut baseyscale: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut cachedheight: [fixed_t; (SCREENHEIGHT) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut cacheddistance: [fixed_t; (SCREENHEIGHT) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut cachedxstep: [fixed_t; (SCREENHEIGHT) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut cachedystep: [fixed_t; (SCREENHEIGHT) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn R_InitPlanes() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_MapPlane(y: std::ffi::c_int, x1: std::ffi::c_int, x2: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_ClearPlanes() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_FindPlane(
    height: fixed_t,
    picnum: std::ffi::c_int,
    lightlevel: std::ffi::c_int,
) -> *mut visplane_t {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_CheckPlane(
    pl: *mut visplane_t,
    start: std::ffi::c_int,
    stop: std::ffi::c_int,
) -> *mut visplane_t {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_MakeSpans(
    x: std::ffi::c_int,
    t1: std::ffi::c_int,
    b1: std::ffi::c_int,
    t2: std::ffi::c_int,
    b2: std::ffi::c_int,
) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_DrawPlanes() {
    todo!("body not yet translated")
}
