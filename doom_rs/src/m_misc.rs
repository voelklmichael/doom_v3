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
use crate::hu_stuff::*;
use crate::i_system::*;
use crate::i_video::*;
use crate::info::*;
use crate::m_argv::*;
use crate::m_fixed::*;
use crate::m_swap::*;
use crate::p_mobj::*;
use crate::p_pspr::*;
use crate::r_data::*;
use crate::r_defs::*;
use crate::r_state::*;
use crate::tables::*;
use crate::v_video::*;
use crate::w_wad::*;
use crate::z_zone::*;

static mut rcsid: [std::ffi::c_char; 49] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        109 as std::ffi::c_char,
        95 as std::ffi::c_char,
        109 as std::ffi::c_char,
        105 as std::ffi::c_char,
        115 as std::ffi::c_char,
        99 as std::ffi::c_char,
        46 as std::ffi::c_char,
        99 as std::ffi::c_char,
        44 as std::ffi::c_char,
        118 as std::ffi::c_char,
        32 as std::ffi::c_char,
        49 as std::ffi::c_char,
        46 as std::ffi::c_char,
        54 as std::ffi::c_char,
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
        48 as std::ffi::c_char,
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
    pub static mut hu_font: [*mut patch_t; (HU_FONTSIZE) as usize];
}

pub unsafe extern "C" fn M_DrawText(
    x: std::ffi::c_int,
    y: std::ffi::c_int,
    direct: boolean,
    string: *mut std::ffi::c_char,
) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub const O_BINARY: std::ffi::c_int = 0;

pub unsafe extern "C" fn M_WriteFile(
    name: (), /* TODO: unparsed param type, needs manual translation */
    source: *mut std::ffi::c_void,
    length: std::ffi::c_int,
) -> boolean {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_ReadFile(
    name: (), /* TODO: unparsed param type, needs manual translation */
    buffer: *mut *mut byte,
) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub static mut usemouse: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut usejoystick: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

unsafe extern "C" {
    pub static mut key_right: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut key_left: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut key_up: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut key_down: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut key_strafeleft: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut key_straferight: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut key_fire: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut key_use: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut key_strafe: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut key_speed: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut mousebfire: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut mousebstrafe: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut mousebforward: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut joybfire: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut joybstrafe: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut joybuse: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut joybspeed: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut viewwidth: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut viewheight: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut mouseSensitivity: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut showMessages: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut detailLevel: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut screenblocks: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut numChannels: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut sndserver_filename: *mut std::ffi::c_char;
}

unsafe extern "C" {
    pub static mut mb_used: std::ffi::c_int;
}

pub static mut mousetype: *mut std::ffi::c_char = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut mousedev: *mut std::ffi::c_char = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

unsafe extern "C" {
    pub static mut chat_macros: *mut *mut std::ffi::c_char;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct default_t {
    pub name: *mut std::ffi::c_char,
    pub location: *mut std::ffi::c_int,
    pub defaultvalue: std::ffi::c_int,
    pub scantranslate: std::ffi::c_int,
    pub untranslated: std::ffi::c_int,
}

pub static mut defaults: [default_t; 41] = unsafe {
    [
        default_t {
            name: (c"mouse_sensitivity").as_ptr(),
            location: (&(mouseSensitivity) as *const _ as *mut _),
            defaultvalue: 5,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"sfx_volume").as_ptr(),
            location: (&(snd_SfxVolume) as *const _ as *mut _),
            defaultvalue: 8,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"music_volume").as_ptr(),
            location: (&(snd_MusicVolume) as *const _ as *mut _),
            defaultvalue: 8,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"show_messages").as_ptr(),
            location: (&(showMessages) as *const _ as *mut _),
            defaultvalue: 1,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"key_right").as_ptr(),
            location: (&(key_right) as *const _ as *mut _),
            defaultvalue: KEY_RIGHTARROW,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"key_left").as_ptr(),
            location: (&(key_left) as *const _ as *mut _),
            defaultvalue: KEY_LEFTARROW,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"key_up").as_ptr(),
            location: (&(key_up) as *const _ as *mut _),
            defaultvalue: KEY_UPARROW,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"key_down").as_ptr(),
            location: (&(key_down) as *const _ as *mut _),
            defaultvalue: KEY_DOWNARROW,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"key_strafeleft").as_ptr(),
            location: (&(key_strafeleft) as *const _ as *mut _),
            defaultvalue: (b',' as std::ffi::c_int),
            ..ZEROED_default_t
        },
        default_t {
            name: (c"key_straferight").as_ptr(),
            location: (&(key_straferight) as *const _ as *mut _),
            defaultvalue: (b'.' as std::ffi::c_int),
            ..ZEROED_default_t
        },
        default_t {
            name: (c"key_fire").as_ptr(),
            location: (&(key_fire) as *const _ as *mut _),
            defaultvalue: KEY_RCTRL,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"key_use").as_ptr(),
            location: (&(key_use) as *const _ as *mut _),
            defaultvalue: (b' ' as std::ffi::c_int),
            ..ZEROED_default_t
        },
        default_t {
            name: (c"key_strafe").as_ptr(),
            location: (&(key_strafe) as *const _ as *mut _),
            defaultvalue: KEY_RALT,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"key_speed").as_ptr(),
            location: (&(key_speed) as *const _ as *mut _),
            defaultvalue: KEY_RSHIFT,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"sndserver").as_ptr(),
            location: ((&(sndserver_filename) as *const _ as *mut _) as *mut std::ffi::c_int),
            defaultvalue: (((c"sndserver").as_ptr()) as std::ffi::c_int),
            ..ZEROED_default_t
        },
        default_t {
            name: (c"mb_used").as_ptr(),
            location: (&(mb_used) as *const _ as *mut _),
            defaultvalue: 2,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"mousedev").as_ptr(),
            location: ((&(mousedev) as *const _ as *mut _) as *mut std::ffi::c_int),
            defaultvalue: (((c"/dev/ttyS0").as_ptr()) as std::ffi::c_int),
            ..ZEROED_default_t
        },
        default_t {
            name: (c"mousetype").as_ptr(),
            location: ((&(mousetype) as *const _ as *mut _) as *mut std::ffi::c_int),
            defaultvalue: (((c"microsoft").as_ptr()) as std::ffi::c_int),
            ..ZEROED_default_t
        },
        default_t {
            name: (c"use_mouse").as_ptr(),
            location: (&(usemouse) as *const _ as *mut _),
            defaultvalue: 1,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"mouseb_fire").as_ptr(),
            location: (&(mousebfire) as *const _ as *mut _),
            defaultvalue: 0,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"mouseb_strafe").as_ptr(),
            location: (&(mousebstrafe) as *const _ as *mut _),
            defaultvalue: 1,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"mouseb_forward").as_ptr(),
            location: (&(mousebforward) as *const _ as *mut _),
            defaultvalue: 2,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"use_joystick").as_ptr(),
            location: (&(usejoystick) as *const _ as *mut _),
            defaultvalue: 0,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"joyb_fire").as_ptr(),
            location: (&(joybfire) as *const _ as *mut _),
            defaultvalue: 0,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"joyb_strafe").as_ptr(),
            location: (&(joybstrafe) as *const _ as *mut _),
            defaultvalue: 1,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"joyb_use").as_ptr(),
            location: (&(joybuse) as *const _ as *mut _),
            defaultvalue: 3,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"joyb_speed").as_ptr(),
            location: (&(joybspeed) as *const _ as *mut _),
            defaultvalue: 2,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"screenblocks").as_ptr(),
            location: (&(screenblocks) as *const _ as *mut _),
            defaultvalue: 9,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"detaillevel").as_ptr(),
            location: (&(detailLevel) as *const _ as *mut _),
            defaultvalue: 0,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"snd_channels").as_ptr(),
            location: (&(numChannels) as *const _ as *mut _),
            defaultvalue: 3,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"usegamma").as_ptr(),
            location: (&(usegamma) as *const _ as *mut _),
            defaultvalue: 0,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"chatmacro0").as_ptr(),
            location: ((&(chat_macros[(0) as usize]) as *const _ as *mut _)
                as *mut std::ffi::c_int),
            defaultvalue: ((HUSTR_CHATMACRO0) as std::ffi::c_int),
            ..ZEROED_default_t
        },
        default_t {
            name: (c"chatmacro1").as_ptr(),
            location: ((&(chat_macros[(1) as usize]) as *const _ as *mut _)
                as *mut std::ffi::c_int),
            defaultvalue: ((HUSTR_CHATMACRO1) as std::ffi::c_int),
            ..ZEROED_default_t
        },
        default_t {
            name: (c"chatmacro2").as_ptr(),
            location: ((&(chat_macros[(2) as usize]) as *const _ as *mut _)
                as *mut std::ffi::c_int),
            defaultvalue: ((HUSTR_CHATMACRO2) as std::ffi::c_int),
            ..ZEROED_default_t
        },
        default_t {
            name: (c"chatmacro3").as_ptr(),
            location: ((&(chat_macros[(3) as usize]) as *const _ as *mut _)
                as *mut std::ffi::c_int),
            defaultvalue: ((HUSTR_CHATMACRO3) as std::ffi::c_int),
            ..ZEROED_default_t
        },
        default_t {
            name: (c"chatmacro4").as_ptr(),
            location: ((&(chat_macros[(4) as usize]) as *const _ as *mut _)
                as *mut std::ffi::c_int),
            defaultvalue: ((HUSTR_CHATMACRO4) as std::ffi::c_int),
            ..ZEROED_default_t
        },
        default_t {
            name: (c"chatmacro5").as_ptr(),
            location: ((&(chat_macros[(5) as usize]) as *const _ as *mut _)
                as *mut std::ffi::c_int),
            defaultvalue: ((HUSTR_CHATMACRO5) as std::ffi::c_int),
            ..ZEROED_default_t
        },
        default_t {
            name: (c"chatmacro6").as_ptr(),
            location: ((&(chat_macros[(6) as usize]) as *const _ as *mut _)
                as *mut std::ffi::c_int),
            defaultvalue: ((HUSTR_CHATMACRO6) as std::ffi::c_int),
            ..ZEROED_default_t
        },
        default_t {
            name: (c"chatmacro7").as_ptr(),
            location: ((&(chat_macros[(7) as usize]) as *const _ as *mut _)
                as *mut std::ffi::c_int),
            defaultvalue: ((HUSTR_CHATMACRO7) as std::ffi::c_int),
            ..ZEROED_default_t
        },
        default_t {
            name: (c"chatmacro8").as_ptr(),
            location: ((&(chat_macros[(8) as usize]) as *const _ as *mut _)
                as *mut std::ffi::c_int),
            defaultvalue: ((HUSTR_CHATMACRO8) as std::ffi::c_int),
            ..ZEROED_default_t
        },
        default_t {
            name: (c"chatmacro9").as_ptr(),
            location: ((&(chat_macros[(9) as usize]) as *const _ as *mut _)
                as *mut std::ffi::c_int),
            defaultvalue: ((HUSTR_CHATMACRO9) as std::ffi::c_int),
            ..ZEROED_default_t
        },
    ]
};

pub static mut numdefaults: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut defaultfile: *mut std::ffi::c_char = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn M_SaveDefaults() {
    todo!("body not yet translated")
}

unsafe extern "C" {
    pub static mut scantokey: [byte; (128) as usize];
}

pub unsafe extern "C" fn M_LoadDefaults() {
    todo!("body not yet translated")
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcx_t {
    pub manufacturer: std::ffi::c_char,
    pub version: std::ffi::c_char,
    pub encoding: std::ffi::c_char,
    pub bits_per_pixel: std::ffi::c_char,
    pub xmin: std::ffi::c_ushort,
    pub ymin: std::ffi::c_ushort,
    pub xmax: std::ffi::c_ushort,
    pub ymax: std::ffi::c_ushort,
    pub hres: std::ffi::c_ushort,
    pub vres: std::ffi::c_ushort,
    pub palette: [std::ffi::c_uchar; (48) as usize],
    pub reserved: std::ffi::c_char,
    pub color_planes: std::ffi::c_char,
    pub bytes_per_line: std::ffi::c_ushort,
    pub palette_type: std::ffi::c_ushort,
    pub filler: [std::ffi::c_char; (58) as usize],
    pub data: std::ffi::c_uchar,
}

pub unsafe extern "C" fn WritePCXfile(
    filename: *mut std::ffi::c_char,
    data: *mut byte,
    width: std::ffi::c_int,
    height: std::ffi::c_int,
    palette: *mut byte,
) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_ScreenShot() {
    todo!("body not yet translated")
}

const ZEROED_default_t: default_t = unsafe { std::mem::zeroed() };
