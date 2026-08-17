use crate::doomtype::*;

pub const ev_keydown: std::ffi::c_int = 0;
pub const ev_keyup: std::ffi::c_int = ev_keydown + 1;
pub const ev_mouse: std::ffi::c_int = ev_keyup + 1;
pub const ev_joystick: std::ffi::c_int = ev_mouse + 1;

pub type evtype_t = std::ffi::c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct event_t {
    pub type_: evtype_t,
    pub data1: std::ffi::c_int,
    pub data2: std::ffi::c_int,
    pub data3: std::ffi::c_int,
}

pub const ga_nothing: std::ffi::c_int = 0;
pub const ga_loadlevel: std::ffi::c_int = ga_nothing + 1;
pub const ga_newgame: std::ffi::c_int = ga_loadlevel + 1;
pub const ga_loadgame: std::ffi::c_int = ga_newgame + 1;
pub const ga_savegame: std::ffi::c_int = ga_loadgame + 1;
pub const ga_playdemo: std::ffi::c_int = ga_savegame + 1;
pub const ga_completed: std::ffi::c_int = ga_playdemo + 1;
pub const ga_victory: std::ffi::c_int = ga_completed + 1;
pub const ga_worlddone: std::ffi::c_int = ga_victory + 1;
pub const ga_screenshot: std::ffi::c_int = ga_worlddone + 1;

pub type gameaction_t = std::ffi::c_int;

pub const BT_ATTACK: std::ffi::c_int = 1;
pub const BT_USE: std::ffi::c_int = 2;
pub const BT_SPECIAL: std::ffi::c_int = 128;
pub const BT_SPECIALMASK: std::ffi::c_int = 3;
pub const BT_CHANGE: std::ffi::c_int = 4;
pub const BT_WEAPONMASK: std::ffi::c_int = (8 + 16 + 32);
pub const BT_WEAPONSHIFT: std::ffi::c_int = 3;
pub const BTS_PAUSE: std::ffi::c_int = 1;
pub const BTS_SAVEGAME: std::ffi::c_int = 2;
pub const BTS_SAVEMASK: std::ffi::c_int = (4 + 8 + 16);
pub const BTS_SAVESHIFT: std::ffi::c_int = 2;

pub type buttoncode_t = std::ffi::c_int;

unsafe extern "C" {
    pub static mut events: [event_t; (MAXEVENTS) as usize];
}

unsafe extern "C" {
    pub static mut eventhead: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut eventtail: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut gameaction: gameaction_t;
}
