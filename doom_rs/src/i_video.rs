use crate::d_event::*;
use crate::d_items::*;
use crate::d_main::*;
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
use crate::m_argv::*;
use crate::m_fixed::*;
use crate::p_mobj::*;
use crate::p_pspr::*;
use crate::r_data::*;
use crate::r_defs::*;
use crate::r_state::*;
use crate::tables::*;
use crate::v_video::*;

unsafe extern "C" {
    pub fn I_WaitVBL(count: std::ffi::c_int);
}

unsafe extern "C" {
    pub fn I_BeginRead();
}

unsafe extern "C" {
    pub fn I_EndRead();
}

static mut rcsid: [std::ffi::c_char; 46] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        105 as std::ffi::c_char,
        95 as std::ffi::c_char,
        120 as std::ffi::c_char,
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
    pub fn XShmGetEventBase(dpy: *mut Display) -> std::ffi::c_int;
}

pub const POINTER_WARP_COUNTDOWN: std::ffi::c_int = 1;

pub static mut X_display: *mut Display = unsafe { std::ptr::null_mut() };

pub static mut X_mainWindow: Window = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut X_cmap: Colormap = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut X_visual: *mut Visual = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut X_gc: GC = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut X_event: XEvent = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut X_screen: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut X_visualinfo: XVisualInfo = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut image: *mut XImage = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut X_width: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut X_height: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut doShm: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut X_shminfo: XShmSegmentInfo = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut X_shmeventtype: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut grabMouse: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut doPointerWarp: std::ffi::c_int = unsafe { POINTER_WARP_COUNTDOWN };

static mut multiply: std::ffi::c_int = unsafe { 1 };

pub unsafe extern "C" fn xlatekey() -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn I_ShutdownGraphics() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn I_StartFrame() {
    todo!("body not yet translated")
}

static mut lastmousex: std::ffi::c_int = unsafe { 0 };

static mut lastmousey: std::ffi::c_int = unsafe { 0 };

pub static mut mousemoved: boolean = unsafe { false_ };

pub static mut shmFinished: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn I_GetEvent() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn createnullcursor(display: *mut Display, root: Window) -> Cursor {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn I_StartTic() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn I_UpdateNoBlit() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn I_FinishUpdate() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn I_ReadScreen(scr: *mut byte) {
    todo!("body not yet translated")
}

static mut colors: [XColor; (256) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn UploadNewPalette(cmap: Colormap, palette: *mut byte) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn I_SetPalette(palette: *mut byte) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn grabsharedmemory(size: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn I_InitGraphics() {
    todo!("body not yet translated")
}

pub static mut exptable: [std::ffi::c_uint; (256) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn InitExpand() {
    todo!("body not yet translated")
}

pub static mut exptable2: [std::ffi::c_double; (256 * 256) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn InitExpand2() {
    todo!("body not yet translated")
}

pub static mut inited: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn Expand4(lineptr: *mut std::ffi::c_uint, xline: *mut std::ffi::c_double) {
    todo!("body not yet translated")
}
