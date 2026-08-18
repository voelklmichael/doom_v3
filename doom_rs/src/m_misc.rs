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
    mut x: std::ffi::c_int,
    mut y: std::ffi::c_int,
    mut direct: boolean,
    mut string: *mut std::ffi::c_char,
) -> std::ffi::c_int {
    unsafe {
        let mut c: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut w: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: while statement not yet translated:
        //
        //
        //     while (*string)
        //     {
        // 	c = toupper(*string) - HU_FONTSTART;
        // 	string++;
        // 	if (c < 0 || c> HU_FONTSIZE)
        // 	{
        // 	    x += 4;
        // 	    continue;
        // 	}
        //
        // 	w = SHORT (hu_font[c]->width);
        // 	if (x+w > SCREENWIDTH)
        // 	    break;
        // 	if (direct)
        // 	    V_DrawPatchDirect(x, y, 0, hu_font[c]);
        // 	else
        // 	    V_DrawPatch(x, y, 0, hu_font[c]);
        // 	x+=w;
        //     }
        todo!("while statement not yet translated");
        return x;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub const O_BINARY: std::ffi::c_int = 0;

pub unsafe extern "C" fn M_WriteFile(
    mut name: (), /* TODO: unparsed param type, needs manual translation */
    mut source: *mut std::ffi::c_void,
    mut length: std::ffi::c_int,
) -> boolean {
    unsafe {
        let mut handle: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut count: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        handle = open(name, (((O_WRONLY | O_CREAT) | O_TRUNC) | O_BINARY), 0666);
        // TODO: if statement not yet translated:
        //
        //
        //     if (handle == -1)
        // 	return false;
        todo!("if statement not yet translated");
        count = write(handle, source, length);
        close(handle);
        // TODO: if statement not yet translated:
        //
        //
        //     if (count < length)
        // 	return false;
        todo!("if statement not yet translated");
        return true_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn M_ReadFile(
    mut name: (), /* TODO: unparsed param type, needs manual translation */
    mut buffer: *mut *mut byte,
) -> std::ffi::c_int {
    unsafe {
        let mut handle: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut count: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut length: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut fileinfo: stat = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut buf: *mut byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        handle = open(name, (O_RDONLY | O_BINARY), 0666);
        // TODO: if statement not yet translated:
        //
        //     if (handle == -1)
        // 	I_Error ("Couldn't read file %s", name);
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (fstat (handle,&fileinfo) == -1)
        // 	I_Error ("Couldn't read file %s", name);
        todo!("if statement not yet translated");
        length = fileinfo.st_size;
        buf = Z_Malloc(length, PU_STATIC, NULL);
        count = read(handle, buf, length);
        close(handle);
        // TODO: if statement not yet translated:
        //
        //
        //     if (count < length)
        // 	I_Error ("Couldn't read file %s", name);
        todo!("if statement not yet translated");
        (*(buffer)) = buf;
        return length;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
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
            name: (c"mouse_sensitivity").as_ptr() as *mut std::ffi::c_char,
            location: (&(mouseSensitivity) as *const std::ffi::c_int as *mut std::ffi::c_int),
            defaultvalue: 5,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"sfx_volume").as_ptr() as *mut std::ffi::c_char,
            location: (&(snd_SfxVolume) as *const std::ffi::c_int as *mut std::ffi::c_int),
            defaultvalue: 8,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"music_volume").as_ptr() as *mut std::ffi::c_char,
            location: (&(snd_MusicVolume) as *const std::ffi::c_int as *mut std::ffi::c_int),
            defaultvalue: 8,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"show_messages").as_ptr() as *mut std::ffi::c_char,
            location: (&(showMessages) as *const std::ffi::c_int as *mut std::ffi::c_int),
            defaultvalue: 1,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"key_right").as_ptr() as *mut std::ffi::c_char,
            location: (&(key_right) as *const std::ffi::c_int as *mut std::ffi::c_int),
            defaultvalue: KEY_RIGHTARROW,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"key_left").as_ptr() as *mut std::ffi::c_char,
            location: (&(key_left) as *const std::ffi::c_int as *mut std::ffi::c_int),
            defaultvalue: KEY_LEFTARROW,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"key_up").as_ptr() as *mut std::ffi::c_char,
            location: (&(key_up) as *const std::ffi::c_int as *mut std::ffi::c_int),
            defaultvalue: KEY_UPARROW,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"key_down").as_ptr() as *mut std::ffi::c_char,
            location: (&(key_down) as *const std::ffi::c_int as *mut std::ffi::c_int),
            defaultvalue: KEY_DOWNARROW,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"key_strafeleft").as_ptr() as *mut std::ffi::c_char,
            location: (&(key_strafeleft) as *const std::ffi::c_int as *mut std::ffi::c_int),
            defaultvalue: (b',' as std::ffi::c_int),
            ..ZEROED_default_t
        },
        default_t {
            name: (c"key_straferight").as_ptr() as *mut std::ffi::c_char,
            location: (&(key_straferight) as *const std::ffi::c_int as *mut std::ffi::c_int),
            defaultvalue: (b'.' as std::ffi::c_int),
            ..ZEROED_default_t
        },
        default_t {
            name: (c"key_fire").as_ptr() as *mut std::ffi::c_char,
            location: (&(key_fire) as *const std::ffi::c_int as *mut std::ffi::c_int),
            defaultvalue: KEY_RCTRL,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"key_use").as_ptr() as *mut std::ffi::c_char,
            location: (&(key_use) as *const std::ffi::c_int as *mut std::ffi::c_int),
            defaultvalue: (b' ' as std::ffi::c_int),
            ..ZEROED_default_t
        },
        default_t {
            name: (c"key_strafe").as_ptr() as *mut std::ffi::c_char,
            location: (&(key_strafe) as *const std::ffi::c_int as *mut std::ffi::c_int),
            defaultvalue: KEY_RALT,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"key_speed").as_ptr() as *mut std::ffi::c_char,
            location: (&(key_speed) as *const std::ffi::c_int as *mut std::ffi::c_int),
            defaultvalue: KEY_RSHIFT,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"sndserver").as_ptr() as *mut std::ffi::c_char,
            location: ((&(sndserver_filename) as *const *mut std::ffi::c_char
                as *mut *mut std::ffi::c_char) as *mut std::ffi::c_int),
            defaultvalue: (((c"sndserver").as_ptr()) as std::ffi::c_int),
            ..ZEROED_default_t
        },
        default_t {
            name: (c"mb_used").as_ptr() as *mut std::ffi::c_char,
            location: (&(mb_used) as *const std::ffi::c_int as *mut std::ffi::c_int),
            defaultvalue: 2,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"mousedev").as_ptr() as *mut std::ffi::c_char,
            location: ((&(mousedev) as *const *mut std::ffi::c_char as *mut *mut std::ffi::c_char)
                as *mut std::ffi::c_int),
            defaultvalue: (((c"/dev/ttyS0").as_ptr()) as std::ffi::c_int),
            ..ZEROED_default_t
        },
        default_t {
            name: (c"mousetype").as_ptr() as *mut std::ffi::c_char,
            location: ((&(mousetype) as *const *mut std::ffi::c_char as *mut *mut std::ffi::c_char)
                as *mut std::ffi::c_int),
            defaultvalue: (((c"microsoft").as_ptr()) as std::ffi::c_int),
            ..ZEROED_default_t
        },
        default_t {
            name: (c"use_mouse").as_ptr() as *mut std::ffi::c_char,
            location: (&(usemouse) as *const std::ffi::c_int as *mut std::ffi::c_int),
            defaultvalue: 1,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"mouseb_fire").as_ptr() as *mut std::ffi::c_char,
            location: (&(mousebfire) as *const std::ffi::c_int as *mut std::ffi::c_int),
            defaultvalue: 0,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"mouseb_strafe").as_ptr() as *mut std::ffi::c_char,
            location: (&(mousebstrafe) as *const std::ffi::c_int as *mut std::ffi::c_int),
            defaultvalue: 1,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"mouseb_forward").as_ptr() as *mut std::ffi::c_char,
            location: (&(mousebforward) as *const std::ffi::c_int as *mut std::ffi::c_int),
            defaultvalue: 2,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"use_joystick").as_ptr() as *mut std::ffi::c_char,
            location: (&(usejoystick) as *const std::ffi::c_int as *mut std::ffi::c_int),
            defaultvalue: 0,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"joyb_fire").as_ptr() as *mut std::ffi::c_char,
            location: (&(joybfire) as *const std::ffi::c_int as *mut std::ffi::c_int),
            defaultvalue: 0,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"joyb_strafe").as_ptr() as *mut std::ffi::c_char,
            location: (&(joybstrafe) as *const std::ffi::c_int as *mut std::ffi::c_int),
            defaultvalue: 1,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"joyb_use").as_ptr() as *mut std::ffi::c_char,
            location: (&(joybuse) as *const std::ffi::c_int as *mut std::ffi::c_int),
            defaultvalue: 3,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"joyb_speed").as_ptr() as *mut std::ffi::c_char,
            location: (&(joybspeed) as *const std::ffi::c_int as *mut std::ffi::c_int),
            defaultvalue: 2,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"screenblocks").as_ptr() as *mut std::ffi::c_char,
            location: (&(screenblocks) as *const std::ffi::c_int as *mut std::ffi::c_int),
            defaultvalue: 9,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"detaillevel").as_ptr() as *mut std::ffi::c_char,
            location: (&(detailLevel) as *const std::ffi::c_int as *mut std::ffi::c_int),
            defaultvalue: 0,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"snd_channels").as_ptr() as *mut std::ffi::c_char,
            location: (&(numChannels) as *const std::ffi::c_int as *mut std::ffi::c_int),
            defaultvalue: 3,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"usegamma").as_ptr() as *mut std::ffi::c_char,
            location: (&(usegamma) as *const std::ffi::c_int as *mut std::ffi::c_int),
            defaultvalue: 0,
            ..ZEROED_default_t
        },
        default_t {
            name: (c"chatmacro0").as_ptr() as *mut std::ffi::c_char,
            location: ((chat_macros.add((0) as usize)) as *mut std::ffi::c_int),
            defaultvalue: ((HUSTR_CHATMACRO0) as std::ffi::c_int),
            ..ZEROED_default_t
        },
        default_t {
            name: (c"chatmacro1").as_ptr() as *mut std::ffi::c_char,
            location: ((chat_macros.add((1) as usize)) as *mut std::ffi::c_int),
            defaultvalue: ((HUSTR_CHATMACRO1) as std::ffi::c_int),
            ..ZEROED_default_t
        },
        default_t {
            name: (c"chatmacro2").as_ptr() as *mut std::ffi::c_char,
            location: ((chat_macros.add((2) as usize)) as *mut std::ffi::c_int),
            defaultvalue: ((HUSTR_CHATMACRO2) as std::ffi::c_int),
            ..ZEROED_default_t
        },
        default_t {
            name: (c"chatmacro3").as_ptr() as *mut std::ffi::c_char,
            location: ((chat_macros.add((3) as usize)) as *mut std::ffi::c_int),
            defaultvalue: ((HUSTR_CHATMACRO3) as std::ffi::c_int),
            ..ZEROED_default_t
        },
        default_t {
            name: (c"chatmacro4").as_ptr() as *mut std::ffi::c_char,
            location: ((chat_macros.add((4) as usize)) as *mut std::ffi::c_int),
            defaultvalue: ((HUSTR_CHATMACRO4) as std::ffi::c_int),
            ..ZEROED_default_t
        },
        default_t {
            name: (c"chatmacro5").as_ptr() as *mut std::ffi::c_char,
            location: ((chat_macros.add((5) as usize)) as *mut std::ffi::c_int),
            defaultvalue: ((HUSTR_CHATMACRO5) as std::ffi::c_int),
            ..ZEROED_default_t
        },
        default_t {
            name: (c"chatmacro6").as_ptr() as *mut std::ffi::c_char,
            location: ((chat_macros.add((6) as usize)) as *mut std::ffi::c_int),
            defaultvalue: ((HUSTR_CHATMACRO6) as std::ffi::c_int),
            ..ZEROED_default_t
        },
        default_t {
            name: (c"chatmacro7").as_ptr() as *mut std::ffi::c_char,
            location: ((chat_macros.add((7) as usize)) as *mut std::ffi::c_int),
            defaultvalue: ((HUSTR_CHATMACRO7) as std::ffi::c_int),
            ..ZEROED_default_t
        },
        default_t {
            name: (c"chatmacro8").as_ptr() as *mut std::ffi::c_char,
            location: ((chat_macros.add((8) as usize)) as *mut std::ffi::c_int),
            defaultvalue: ((HUSTR_CHATMACRO8) as std::ffi::c_int),
            ..ZEROED_default_t
        },
        default_t {
            name: (c"chatmacro9").as_ptr() as *mut std::ffi::c_char,
            location: ((chat_macros.add((9) as usize)) as *mut std::ffi::c_int),
            defaultvalue: ((HUSTR_CHATMACRO9) as std::ffi::c_int),
            ..ZEROED_default_t
        },
    ]
};

pub static mut numdefaults: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut defaultfile: *mut std::ffi::c_char = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn M_SaveDefaults() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut v: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        (FILE * f);
        f = fopen(defaultfile, (c"w").as_ptr());
        // TODO: if statement not yet translated:
        //
        //     if (!f)
        // 	return; // can't write the file, but don't complain
        todo!("if statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //     for (i=0 ; i<numdefaults ; i++)
        //     {
        // 	if (defaults[i].defaultvalue > -0xfff
        // 	    && defaults[i].defaultvalue < 0xfff)
        // 	{
        // 	    v = *defaults[i].location;
        // 	    fprintf (f,"%s\t\t%i\n",defaults[i].name,v);
        // 	} else {
        // 	    fprintf (f,"%s\t\t\"%s\"\n",defaults[i].name,
        // 		     * (char **) (defaults[i].location));
        // 	}
        //     }
        todo!("for statement not yet translated");
        fclose(f);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

unsafe extern "C" {
    pub static mut scantokey: [byte; (128) as usize];
}

pub unsafe extern "C" fn M_LoadDefaults() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut len: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        (FILE * f);
        let mut def: [std::ffi::c_char; (80) as usize] = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut strparm: [std::ffi::c_char; (100) as usize] = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut newstring: *mut std::ffi::c_char = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut parm: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut isstring: boolean = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        numdefaults =
            (std::mem::size_of_val(&(defaults)) / std::mem::size_of_val(&(defaults[(0) as usize])));
        // TODO: for statement not yet translated:
        //
        //     for (i=0 ; i<numdefaults ; i++)
        // 	*defaults[i].location = defaults[i].defaultvalue;
        todo!("for statement not yet translated");
        i = M_CheckParm((c"-config").as_ptr());
        // TODO: if statement not yet translated:
        //
        //     if (i && i<myargc-1)
        //     {
        // 	defaultfile = myargv[i+1];
        // 	printf ("	default file: %s\n",defaultfile);
        //     }
        //     else
        // 	defaultfile = basedefault;
        todo!("if statement not yet translated");
        f = fopen(defaultfile, (c"r").as_ptr());
        // TODO: if statement not yet translated:
        //
        //     if (f)
        //     {
        // 	while (!feof(f))
        // 	{
        // 	    isstring = false;
        // 	    if (fscanf (f, "%79s %[^\n]\n", def, strparm) == 2)
        // 	    {
        // 		if (strparm[0] == '"')
        // 		{
        // 		    // get a string default
        // 		    isstring = true;
        // 		    len = strlen(strparm);
        // 		    newstring = (char *) malloc(len);
        // 		    strparm[len-1] = 0;
        // 		    strcpy(newstring, strparm+1);
        // 		}
        // 		else if (strparm[0] == '0' && strparm[1] == 'x')
        // 		    sscanf(strparm+2, "%x", &parm);
        // 		else
        // 		    sscanf(strparm, "%i", &parm);
        // 		for (i=0 ; i<numdefaults ; i++)
        // 		    if (!strcmp(def, defaults[i].name))
        // 		    {
        // 			if (!isstring)
        // 			    *defaults[i].location = parm;
        // 			else
        // 			    *defaults[i].location =
        // 				(int) newstring;
        // 			break;
        // 		    }
        // 	    }
        // 	}
        //
        // 	fclose (f);
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
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
    mut filename: *mut std::ffi::c_char,
    mut data: *mut byte,
    mut width: std::ffi::c_int,
    mut height: std::ffi::c_int,
    mut palette: *mut byte,
) {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut length: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut pcx: *mut pcx_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut pack: *mut byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        pcx = Z_Malloc((((width * height) * 2) + 1000), PU_STATIC, NULL);
        (*pcx).manufacturer = 0x0a;
        (*pcx).version = 5;
        (*pcx).encoding = 1;
        (*pcx).bits_per_pixel = 8;
        (*pcx).xmin = 0;
        (*pcx).ymin = 0;
        (*pcx).xmax = SHORT((width - 1));
        (*pcx).ymax = SHORT((height - 1));
        (*pcx).hres = SHORT(width);
        (*pcx).vres = SHORT(height);
        memset((*pcx).palette, 0, std::mem::size_of_val(&((*pcx).palette)));
        (*pcx).color_planes = 1;
        (*pcx).bytes_per_line = SHORT(width);
        (*pcx).palette_type = SHORT(2);
        memset((*pcx).filler, 0, std::mem::size_of_val(&((*pcx).filler)));
        pack = (&((*pcx).data) as *const _ as *mut _);
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<width*height ; i++)
        //     {
        // 	if ( (*data & 0xc0) != 0xc0)
        // 	    *pack++ = *data++;
        // 	else
        // 	{
        // 	    *pack++ = 0xc1;
        // 	    *pack++ = *data++;
        // 	}
        //     }
        todo!("for statement not yet translated");
        (*({
            let __macro_tmp = pack;
            pack += 1;
            __macro_tmp
        })) = 0x0c;
        // TODO: for statement not yet translated:
        //     for (i=0 ; i<768 ; i++)
        // 	*pack++ = *palette++;
        todo!("for statement not yet translated");
        length = (pack - ((pcx) as *mut byte));
        M_WriteFile(filename, pcx, length);
        Z_Free(pcx);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn M_ScreenShot() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut linear: *mut byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut lbmname: [std::ffi::c_char; (12) as usize] = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        linear = screens[(2) as usize];
        I_ReadScreen(linear);
        strcpy(lbmname, (c"DOOM00.pcx").as_ptr());
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<=99 ; i++)
        //     {
        // 	lbmname[4] = i/10 + '0';
        // 	lbmname[5] = i%10 + '0';
        // 	if (access(lbmname,0) == -1)
        // 	    break;	// file doesn't exist
        //     }
        todo!("for statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (i==100)
        // 	I_Error ("M_ScreenShot: Couldn't create a PCX");
        todo!("if statement not yet translated");
        WritePCXfile(
            lbmname,
            linear,
            SCREENWIDTH,
            SCREENHEIGHT,
            W_CacheLumpName((c"PLAYPAL").as_ptr(), PU_CACHE),
        );
        players[(consoleplayer) as usize].message = (c"screen shot").as_ptr();
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

const ZEROED_default_t: default_t = unsafe { std::mem::zeroed() };
