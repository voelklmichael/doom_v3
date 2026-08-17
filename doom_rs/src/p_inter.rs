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
use crate::i_system::*;
use crate::info::*;
use crate::m_fixed::*;
use crate::m_random::*;
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
use crate::tables::*;

static mut rcsid: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub const BONUSADD: std::ffi::c_int = 6;

pub static mut maxammo: [std::ffi::c_int; (NUMAMMO) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut clipammo: [std::ffi::c_int; (NUMAMMO) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn P_GiveAmmo(
    player: *mut player_t,
    ammo: ammotype_t,
    num: std::ffi::c_int,
) -> boolean {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_GiveWeapon(
    player: *mut player_t,
    weapon: weapontype_t,
    dropped: boolean,
) -> boolean {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_GiveBody(player: *mut player_t, num: std::ffi::c_int) -> boolean {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_GiveArmor(player: *mut player_t, armortype: std::ffi::c_int) -> boolean {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_GiveCard(player: *mut player_t, card: card_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_GivePower(player: *mut player_t, power: std::ffi::c_int) -> boolean {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_TouchSpecialThing(special: *mut mobj_t, toucher: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_KillMobj(source: *mut mobj_t, target: *mut mobj_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn P_DamageMobj(
    target: *mut mobj_t,
    inflictor: *mut mobj_t,
    source: *mut mobj_t,
    damage: std::ffi::c_int,
) {
    todo!("body not yet translated")
}
