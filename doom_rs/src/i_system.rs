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
use crate::i_sound::*;
use crate::i_video::*;
use crate::info::*;
use crate::m_fixed::*;
use crate::m_misc::*;
use crate::p_mobj::*;
use crate::p_pspr::*;
use crate::sounds::*;
use crate::tables::*;

unsafe extern "C" {
    pub fn I_StartFrame();
}

unsafe extern "C" {
    pub fn I_StartTic();
}

static mut rcsid: [std::ffi::c_char; 49] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        109 as std::ffi::c_char,
        95 as std::ffi::c_char,
        98 as std::ffi::c_char,
        98 as std::ffi::c_char,
        111 as std::ffi::c_char,
        120 as std::ffi::c_char,
        46 as std::ffi::c_char,
        99 as std::ffi::c_char,
        44 as std::ffi::c_char,
        118 as std::ffi::c_char,
        32 as std::ffi::c_char,
        49 as std::ffi::c_char,
        46 as std::ffi::c_char,
        49 as std::ffi::c_char,
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

pub static mut mb_used: std::ffi::c_int = unsafe { 6 };

pub unsafe extern "C" fn I_Tactile(
    mut on: std::ffi::c_int,
    mut off: std::ffi::c_int,
    mut total: std::ffi::c_int,
) {
    unsafe {
        on = off = total = 0;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub static mut emptycmd: ticcmd_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn I_BaseTiccmd() -> *mut ticcmd_t {
    unsafe {
        return (&(emptycmd) as *const ticcmd_t as *mut ticcmd_t);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn I_GetHeapSize() -> std::ffi::c_int {
    unsafe {
        return ((mb_used * 1024) * 1024);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn I_ZoneBase(mut size: *mut std::ffi::c_int) -> *mut byte {
    unsafe {
        (*(size)) = ((mb_used * 1024) * 1024);
        return ((malloc((*(size)))) as *mut byte);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn I_GetTime() -> std::ffi::c_int {
    unsafe {
        let mut tp: timeval = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut tzp: timezone = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut newtics: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        static mut basetime: std::ffi::c_int = unsafe { 0 };
        gettimeofday(
            (&(tp) as *const _ as *mut _),
            (&(tzp) as *const _ as *mut _),
        );
        // TODO: if statement not yet translated:
        //
        //     if (!basetime)
        // 	basetime = tp.tv_sec;
        todo!("if statement not yet translated");
        newtics = (((tp.tv_sec - basetime) * TICRATE) + ((tp.tv_usec * TICRATE) / 1000000));
        return newtics;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn I_Init() {
    unsafe {
        I_InitSound();
        // TODO: statement not yet translated:
        //
        //     //  I_InitGraphics();
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn I_Quit() {
    unsafe {
        D_QuitNetGame();
        I_ShutdownSound();
        I_ShutdownMusic();
        M_SaveDefaults();
        I_ShutdownGraphics();
        exit(0);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn I_WaitVBL(mut count: std::ffi::c_int) {
    unsafe {
        usleep((count * (1000000 / 70)));
    }
}

pub unsafe extern "C" fn I_BeginRead() {
    unsafe {
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn I_EndRead() {
    unsafe {
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn I_AllocLow(mut length: std::ffi::c_int) -> *mut byte {
    unsafe {
        let mut mem: *mut byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        mem = ((malloc(length)) as *mut byte);
        memset(mem, 0, length);
        return mem;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

unsafe extern "C" {
    pub static mut demorecording: boolean;
}

pub unsafe extern "C" fn I_Error(mut error: *mut std::ffi::c_char) {
    unsafe {
        // TODO: statement not yet translated:
        //
        //     va_list	argptr;
        todo!("statement not yet translated");
        va_start(argptr, error);
        fprintf(stderr, (c"Error: ").as_ptr());
        vfprintf(stderr, error, argptr);
        fprintf(stderr, (c"\n").as_ptr());
        va_end(argptr);
        fflush(stderr);
        // TODO: if statement not yet translated:
        //
        //
        //     // Shutdown. Here might be other errors.
        //     if (demorecording)
        // 	G_CheckDemoStatus();
        todo!("if statement not yet translated");
        D_QuitNetGame();
        I_ShutdownGraphics();
        exit((-(1)));
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
} // TODO: variadic definition not supported, C variadic marker dropped
