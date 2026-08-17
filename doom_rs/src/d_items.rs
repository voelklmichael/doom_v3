use crate::d_think::*;
use crate::doomdef::*;
use crate::info::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct weaponinfo_t {
    pub ammo: ammotype_t,
    pub upstate: std::ffi::c_int,
    pub downstate: std::ffi::c_int,
    pub readystate: std::ffi::c_int,
    pub atkstate: std::ffi::c_int,
    pub flashstate: std::ffi::c_int,
}

static mut rcsid: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut weaponinfo: [weaponinfo_t; (NUMWEAPONS) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated
