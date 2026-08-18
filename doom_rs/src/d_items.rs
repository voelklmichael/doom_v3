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

static mut rcsid: [std::ffi::c_char; 6] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        36 as std::ffi::c_char,
        0,
    ]
};

pub static mut weaponinfo: [weaponinfo_t; (NUMWEAPONS) as usize] = unsafe {
    [
        weaponinfo_t {
            ammo: am_noammo,
            upstate: S_PUNCHUP,
            downstate: S_PUNCHDOWN,
            readystate: S_PUNCH,
            atkstate: S_PUNCH1,
            flashstate: S_NULL,
        },
        weaponinfo_t {
            ammo: am_clip,
            upstate: S_PISTOLUP,
            downstate: S_PISTOLDOWN,
            readystate: S_PISTOL,
            atkstate: S_PISTOL1,
            flashstate: S_PISTOLFLASH,
        },
        weaponinfo_t {
            ammo: am_shell,
            upstate: S_SGUNUP,
            downstate: S_SGUNDOWN,
            readystate: S_SGUN,
            atkstate: S_SGUN1,
            flashstate: S_SGUNFLASH1,
        },
        weaponinfo_t {
            ammo: am_clip,
            upstate: S_CHAINUP,
            downstate: S_CHAINDOWN,
            readystate: S_CHAIN,
            atkstate: S_CHAIN1,
            flashstate: S_CHAINFLASH1,
        },
        weaponinfo_t {
            ammo: am_misl,
            upstate: S_MISSILEUP,
            downstate: S_MISSILEDOWN,
            readystate: S_MISSILE,
            atkstate: S_MISSILE1,
            flashstate: S_MISSILEFLASH1,
        },
        weaponinfo_t {
            ammo: am_cell,
            upstate: S_PLASMAUP,
            downstate: S_PLASMADOWN,
            readystate: S_PLASMA,
            atkstate: S_PLASMA1,
            flashstate: S_PLASMAFLASH1,
        },
        weaponinfo_t {
            ammo: am_cell,
            upstate: S_BFGUP,
            downstate: S_BFGDOWN,
            readystate: S_BFG,
            atkstate: S_BFG1,
            flashstate: S_BFGFLASH1,
        },
        weaponinfo_t {
            ammo: am_noammo,
            upstate: S_SAWUP,
            downstate: S_SAWDOWN,
            readystate: S_SAW,
            atkstate: S_SAW1,
            flashstate: S_NULL,
        },
        weaponinfo_t {
            ammo: am_shell,
            upstate: S_DSGUNUP,
            downstate: S_DSGUNDOWN,
            readystate: S_DSGUN,
            atkstate: S_DSGUN1,
            flashstate: S_DSGUNFLASH1,
        },
    ]
};
