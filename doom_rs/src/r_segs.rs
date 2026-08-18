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
use crate::r_plane::*;
use crate::r_sky::*;
use crate::r_state::*;
use crate::r_things::*;
use crate::tables::*;

static mut rcsid: [std::ffi::c_char; 49] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        114 as std::ffi::c_char,
        95 as std::ffi::c_char,
        115 as std::ffi::c_char,
        101 as std::ffi::c_char,
        103 as std::ffi::c_char,
        115 as std::ffi::c_char,
        46 as std::ffi::c_char,
        99 as std::ffi::c_char,
        44 as std::ffi::c_char,
        118 as std::ffi::c_char,
        32 as std::ffi::c_char,
        49 as std::ffi::c_char,
        46 as std::ffi::c_char,
        51 as std::ffi::c_char,
        32 as std::ffi::c_char,
        49 as std::ffi::c_char,
        57 as std::ffi::c_char,
        57 as std::ffi::c_char,
        55 as std::ffi::c_char,
        47 as std::ffi::c_char,
        48 as std::ffi::c_char,
        49 as std::ffi::c_char,
        47 as std::ffi::c_char,
        50 as std::ffi::c_char,
        57 as std::ffi::c_char,
        32 as std::ffi::c_char,
        50 as std::ffi::c_char,
        48 as std::ffi::c_char,
        58 as std::ffi::c_char,
        49 as std::ffi::c_char,
        48 as std::ffi::c_char,
        58 as std::ffi::c_char,
        49 as std::ffi::c_char,
        57 as std::ffi::c_char,
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

pub static mut segtextured: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut markfloor: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut markceiling: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut maskedtexture: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut toptexture: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut bottomtexture: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut midtexture: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut rw_normalangle: angle_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut rw_angle1: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut rw_x: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut rw_stopx: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut rw_centerangle: angle_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut rw_offset: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut rw_distance: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut rw_scale: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut rw_scalestep: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut rw_midtexturemid: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut rw_toptexturemid: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut rw_bottomtexturemid: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut worldtop: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut worldbottom: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut worldhigh: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut worldlow: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut pixhigh: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut pixlow: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut pixhighstep: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut pixlowstep: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut topfrac: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut topstep: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut bottomfrac: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut bottomstep: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut walllights: *mut *mut lighttable_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut maskedtexturecol: *mut std::ffi::c_short = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn R_RenderMaskedSegRange(
    ds: *mut drawseg_t,
    x1: std::ffi::c_int,
    x2: std::ffi::c_int,
) {
    todo!("body not yet translated")
}

pub const HEIGHTBITS: std::ffi::c_int = 12;

pub const HEIGHTUNIT: std::ffi::c_int = (1 << HEIGHTBITS);

pub unsafe extern "C" fn R_RenderSegLoop() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn R_StoreWallRange(start: std::ffi::c_int, stop: std::ffi::c_int) {
    todo!("body not yet translated")
}
