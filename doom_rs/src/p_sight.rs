use crate::d_event::*;
use crate::d_items::*;
use crate::d_player::*;
use crate::d_think::*;
use crate::d_ticcmd::*;
use crate::doomdata::*;
use crate::doomdef::*;
use crate::doomtype::*;
use crate::i_system::*;
use crate::info::*;
use crate::m_fixed::*;
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
use crate::tables::*;

static mut rcsid: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut sightzstart: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut topslope: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut bottomslope: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut strace: divline_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut t2x: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut t2y: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut sightcounts: [std::ffi::c_int; (2) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn P_DivlineSide(
    x: fixed_t,
    y: fixed_t,
    node: *mut divline_t,
) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_InterceptVector2(v2: *mut divline_t, v1: *mut divline_t) -> fixed_t {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_CrossSubsector(num: std::ffi::c_int) -> boolean {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_CrossBSPNode(bspnum: std::ffi::c_int) -> boolean {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_CheckSight(t1: *mut mobj_t, t2: *mut mobj_t) -> boolean {
    todo!("body not yet translated")
}
