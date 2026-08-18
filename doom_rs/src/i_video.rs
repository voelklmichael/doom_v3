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
    pub fn XShmGetEventBase(dpy: *mut x11::xlib::Display) -> std::ffi::c_int;
}

pub const POINTER_WARP_COUNTDOWN: std::ffi::c_int = 1;

pub static mut X_display: *mut x11::xlib::Display = unsafe { std::ptr::null_mut() };

pub static mut X_mainWindow: x11::xlib::Window = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut X_cmap: x11::xlib::Colormap = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut X_visual: *mut x11::xlib::Visual = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut X_gc: x11::xlib::GC = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut X_event: x11::xlib::XEvent = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut X_screen: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut X_visualinfo: x11::xlib::XVisualInfo = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut image: *mut x11::xlib::XImage = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut X_width: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut X_height: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut doShm: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut X_shminfo: x11::xshm::XShmSegmentInfo = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut X_shmeventtype: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut grabMouse: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut doPointerWarp: std::ffi::c_int = unsafe { POINTER_WARP_COUNTDOWN };

static mut multiply: std::ffi::c_int = unsafe { 1 };

pub unsafe extern "C" fn xlatekey() -> std::ffi::c_int {
    unsafe {
        let mut rc: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: switch statement not yet translated:
        //
        //
        //     switch(rc = XKeycodeToKeysym(X_display, X_event.xkey.keycode, 0))
        //     {
        //       case XK_Left:	rc = KEY_LEFTARROW;	break;
        //       case XK_Right:	rc = KEY_RIGHTARROW;	break;
        //       case XK_Down:	rc = KEY_DOWNARROW;	break;
        //       case XK_Up:	rc = KEY_UPARROW;	break;
        //       case XK_Escape:	rc = KEY_ESCAPE;	break;
        //       case XK_Return:	rc = KEY_ENTER;		break;
        //       case XK_Tab:	rc = KEY_TAB;		break;
        //       case XK_F1:	rc = KEY_F1;		break;
        //       case XK_F2:	rc = KEY_F2;		break;
        //       case XK_F3:	rc = KEY_F3;		break;
        //       case XK_F4:	rc = KEY_F4;		break;
        //       case XK_F5:	rc = KEY_F5;		break;
        //       case XK_F6:	rc = KEY_F6;		break;
        //       case XK_F7:	rc = KEY_F7;		break;
        //       case XK_F8:	rc = KEY_F8;		break;
        //       case XK_F9:	rc = KEY_F9;		break;
        //       case XK_F10:	rc = KEY_F10;		break;
        //       case XK_F11:	rc = KEY_F11;		break;
        //       case XK_F12:	rc = KEY_F12;		break;
        //
        //       case XK_BackSpace:
        //       case XK_Delete:	rc = KEY_BACKSPACE;	break;
        //
        //       case XK_Pause:	rc = KEY_PAUSE;		break;
        //
        //       case XK_KP_Equal:
        //       case XK_equal:	rc = KEY_EQUALS;	break;
        //
        //       case XK_KP_Subtract:
        //       case XK_minus:	rc = KEY_MINUS;		break;
        //
        //       case XK_Shift_L:
        //       case XK_Shift_R:
        // 	rc = KEY_RSHIFT;
        // 	break;
        //
        //       case XK_Control_L:
        //       case XK_Control_R:
        // 	rc = KEY_RCTRL;
        // 	break;
        //
        //       case XK_Alt_L:
        //       case XK_Meta_L:
        //       case XK_Alt_R:
        //       case XK_Meta_R:
        // 	rc = KEY_RALT;
        // 	break;
        //
        //       default:
        // 	if (rc >= XK_space && rc <= XK_asciitilde)
        // 	    rc = rc - XK_space + ' ';
        // 	if (rc >= 'A' && rc <= 'Z')
        // 	    rc = rc - 'A' + 'a';
        // 	break;
        //     }
        todo!("switch statement not yet translated");
        return rc;
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn I_ShutdownGraphics() {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //   // Detach from X server
        //   if (!XShmDetach(X_display, &X_shminfo))
        // 	    I_Error("XShmDetach() failed in I_ShutdownGraphics()");
        todo!("if statement not yet translated");
        shmdt(X_shminfo.shmaddr);
        shmctl(X_shminfo.shmid, IPC_RMID, 0);
        (*image).data = NULL;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn I_StartFrame() {
    unsafe {
        // TODO: statement not yet translated:
        //
        //     // er?
        //
        todo!("statement not yet translated");
    }
}

static mut lastmousex: std::ffi::c_int = unsafe { 0 };

static mut lastmousey: std::ffi::c_int = unsafe { 0 };

pub static mut mousemoved: boolean = unsafe { false_ };

pub static mut shmFinished: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn I_GetEvent() {
    unsafe {
        let mut event: event_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        XNextEvent(
            X_display,
            (&(X_event) as *const x11::xlib::XEvent as *mut x11::xlib::XEvent),
        );
        // TODO: switch statement not yet translated:
        //
        //     switch (X_event.type)
        //     {
        //       case KeyPress:
        // 	event.type = ev_keydown;
        // 	event.data1 = xlatekey();
        // 	D_PostEvent(&event);
        // 	// fprintf(stderr, "k");
        // 	break;
        //       case KeyRelease:
        // 	event.type = ev_keyup;
        // 	event.data1 = xlatekey();
        // 	D_PostEvent(&event);
        // 	// fprintf(stderr, "ku");
        // 	break;
        //       case ButtonPress:
        // 	event.type = ev_mouse;
        // 	event.data1 =
        // 	    (X_event.xbutton.state & Button1Mask)
        // 	    | (X_event.xbutton.state & Button2Mask ? 2 : 0)
        // 	    | (X_event.xbutton.state & Button3Mask ? 4 : 0)
        // 	    | (X_event.xbutton.button == Button1)
        // 	    | (X_event.xbutton.button == Button2 ? 2 : 0)
        // 	    | (X_event.xbutton.button == Button3 ? 4 : 0);
        // 	event.data2 = event.data3 = 0;
        // 	D_PostEvent(&event);
        // 	// fprintf(stderr, "b");
        // 	break;
        //       case ButtonRelease:
        // 	event.type = ev_mouse;
        // 	event.data1 =
        // 	    (X_event.xbutton.state & Button1Mask)
        // 	    | (X_event.xbutton.state & Button2Mask ? 2 : 0)
        // 	    | (X_event.xbutton.state & Button3Mask ? 4 : 0);
        // 	// suggest parentheses around arithmetic in operand of |
        // 	event.data1 =
        // 	    event.data1
        // 	    ^ (X_event.xbutton.button == Button1 ? 1 : 0)
        // 	    ^ (X_event.xbutton.button == Button2 ? 2 : 0)
        // 	    ^ (X_event.xbutton.button == Button3 ? 4 : 0);
        // 	event.data2 = event.data3 = 0;
        // 	D_PostEvent(&event);
        // 	// fprintf(stderr, "bu");
        // 	break;
        //       case MotionNotify:
        // 	event.type = ev_mouse;
        // 	event.data1 =
        // 	    (X_event.xmotion.state & Button1Mask)
        // 	    | (X_event.xmotion.state & Button2Mask ? 2 : 0)
        // 	    | (X_event.xmotion.state & Button3Mask ? 4 : 0);
        // 	event.data2 = (X_event.xmotion.x - lastmousex) << 2;
        // 	event.data3 = (lastmousey - X_event.xmotion.y) << 2;
        //
        // 	if (event.data2 || event.data3)
        // 	{
        // 	    lastmousex = X_event.xmotion.x;
        // 	    lastmousey = X_event.xmotion.y;
        // 	    if (X_event.xmotion.x != X_width/2 &&
        // 		X_event.xmotion.y != X_height/2)
        // 	    {
        // 		D_PostEvent(&event);
        // 		// fprintf(stderr, "m");
        // 		mousemoved = false;
        // 	    } else
        // 	    {
        // 		mousemoved = true;
        // 	    }
        // 	}
        // 	break;
        //
        //       case Expose:
        //       case ConfigureNotify:
        // 	break;
        //
        //       default:
        // 	if (doShm && X_event.type == X_shmeventtype) shmFinished = true;
        // 	break;
        //     }
        todo!("switch statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn createnullcursor(
    mut display: *mut x11::xlib::Display,
    mut root: x11::xlib::Window,
) -> x11::xlib::Cursor {
    unsafe {
        // TODO: statement not yet translated:
        //
        //     Pixmap cursormask;
        todo!("statement not yet translated");
        // TODO: statement not yet translated:
        //
        //     XGCValues xgc;
        todo!("statement not yet translated");
        // TODO: statement not yet translated:
        //
        //     GC gc;
        todo!("statement not yet translated");
        // TODO: statement not yet translated:
        //
        //     XColor dummycolour;
        todo!("statement not yet translated");
        // TODO: statement not yet translated:
        //
        //     Cursor cursor;
        todo!("statement not yet translated");
        cursormask = XCreatePixmap(display, root, 1, 1, 1);
        xgc.function = GXclear;
        gc = XCreateGC(
            display,
            cursormask,
            GCFunction,
            (&(xgc) as *const _ as *mut _),
        );
        XFillRectangle(display, cursormask, gc, 0, 0, 1, 1);
        dummycolour.pixel = 0;
        dummycolour.red = 0;
        dummycolour.flags = 04;
        cursor = XCreatePixmapCursor(
            display,
            cursormask,
            cursormask,
            (&(dummycolour) as *const _ as *mut _),
            (&(dummycolour) as *const _ as *mut _),
            0,
            0,
        );
        XFreePixmap(display, cursormask);
        XFreeGC(display, gc);
        return cursor;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn I_StartTic() {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //
        //     if (!X_display)
        // 	return;
        todo!("if statement not yet translated");
        // TODO: while statement not yet translated:
        //
        //
        //     while (XPending(X_display))
        // 	I_GetEvent();
        todo!("while statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // Warp the pointer back to the middle of the window
        //     //  or it will wander off - that is, the game will
        //     //  loose input focus within X11.
        //     if (grabMouse)
        //     {
        // 	if (!--doPointerWarp)
        // 	{
        // 	    XWarpPointer( X_display,
        // 			  None,
        // 			  X_mainWindow,
        // 			  0, 0,
        // 			  0, 0,
        // 			  X_width/2, X_height/2);
        //
        // 	    doPointerWarp = POINTER_WARP_COUNTDOWN;
        // 	}
        //     }
        todo!("if statement not yet translated");
        mousemoved = false_;
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn I_UpdateNoBlit() {
    unsafe {
        // TODO: statement not yet translated:
        //
        //     // what is this?
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn I_FinishUpdate() {
    unsafe {
        static mut lasttic: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut tics: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //     // UNUSED static unsigned char *bigscreen=0;
        //
        //     // draws little dots on the bottom of the screen
        //     if (devparm)
        //     {
        //
        // 	i = I_GetTime();
        // 	tics = i - lasttic;
        // 	lasttic = i;
        // 	if (tics > 20) tics = 20;
        //
        // 	for (i=0 ; i<tics*2 ; i+=2)
        // 	    screens[0][ (SCREENHEIGHT-1)*SCREENWIDTH + i] = 0xff;
        // 	for ( ; i<20*2 ; i+=2)
        // 	    screens[0][ (SCREENHEIGHT-1)*SCREENWIDTH + i] = 0x0;
        //
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // scales the screen size before blitting it
        //     if (multiply == 2)
        //     {
        // 	unsigned int *olineptrs[2];
        // 	unsigned int *ilineptr;
        // 	int x, y, i;
        // 	unsigned int twoopixels;
        // 	unsigned int twomoreopixels;
        // 	unsigned int fouripixels;
        //
        // 	ilineptr = (unsigned int *) (screens[0]);
        // 	for (i=0 ; i<2 ; i++)
        // 	    olineptrs[i] = (unsigned int *) &image->data[i*X_width];
        //
        // 	y = SCREENHEIGHT;
        // 	while (y--)
        // 	{
        // 	    x = SCREENWIDTH;
        // 	    do
        // 	    {
        // 		fouripixels = *ilineptr++;
        // 		twoopixels =	(fouripixels & 0xff000000)
        // 		    |	((fouripixels>>8) & 0xffff00)
        // 		    |	((fouripixels>>16) & 0xff);
        // 		twomoreopixels =	((fouripixels<<16) & 0xff000000)
        // 		    |	((fouripixels<<8) & 0xffff00)
        // 		    |	(fouripixels & 0xff);
        // #ifdef __BIG_ENDIAN__
        // 		*olineptrs[0]++ = twoopixels;
        // 		*olineptrs[1]++ = twoopixels;
        // 		*olineptrs[0]++ = twomoreopixels;
        // 		*olineptrs[1]++ = twomoreopixels;
        // #else
        // 		*olineptrs[0]++ = twomoreopixels;
        // 		*olineptrs[1]++ = twomoreopixels;
        // 		*olineptrs[0]++ = twoopixels;
        // 		*olineptrs[1]++ = twoopixels;
        // #endif
        // 	    } while (x-=4);
        // 	    olineptrs[0] += X_width/4;
        // 	    olineptrs[1] += X_width/4;
        // 	}
        //
        //     }
        //     else if (multiply == 3)
        //     {
        // 	unsigned int *olineptrs[3];
        // 	unsigned int *ilineptr;
        // 	int x, y, i;
        // 	unsigned int fouropixels[3];
        // 	unsigned int fouripixels;
        //
        // 	ilineptr = (unsigned int *) (screens[0]);
        // 	for (i=0 ; i<3 ; i++)
        // 	    olineptrs[i] = (unsigned int *) &image->data[i*X_width];
        //
        // 	y = SCREENHEIGHT;
        // 	while (y--)
        // 	{
        // 	    x = SCREENWIDTH;
        // 	    do
        // 	    {
        // 		fouripixels = *ilineptr++;
        // 		fouropixels[0] = (fouripixels & 0xff000000)
        // 		    |	((fouripixels>>8) & 0xff0000)
        // 		    |	((fouripixels>>16) & 0xffff);
        // 		fouropixels[1] = ((fouripixels<<8) & 0xff000000)
        // 		    |	(fouripixels & 0xffff00)
        // 		    |	((fouripixels>>8) & 0xff);
        // 		fouropixels[2] = ((fouripixels<<16) & 0xffff0000)
        // 		    |	((fouripixels<<8) & 0xff00)
        // 		    |	(fouripixels & 0xff);
        // #ifdef __BIG_ENDIAN__
        // 		*olineptrs[0]++ = fouropixels[0];
        // 		*olineptrs[1]++ = fouropixels[0];
        // 		*olineptrs[2]++ = fouropixels[0];
        // 		*olineptrs[0]++ = fouropixels[1];
        // 		*olineptrs[1]++ = fouropixels[1];
        // 		*olineptrs[2]++ = fouropixels[1];
        // 		*olineptrs[0]++ = fouropixels[2];
        // 		*olineptrs[1]++ = fouropixels[2];
        // 		*olineptrs[2]++ = fouropixels[2];
        // #else
        // 		*olineptrs[0]++ = fouropixels[2];
        // 		*olineptrs[1]++ = fouropixels[2];
        // 		*olineptrs[2]++ = fouropixels[2];
        // 		*olineptrs[0]++ = fouropixels[1];
        // 		*olineptrs[1]++ = fouropixels[1];
        // 		*olineptrs[2]++ = fouropixels[1];
        // 		*olineptrs[0]++ = fouropixels[0];
        // 		*olineptrs[1]++ = fouropixels[0];
        // 		*olineptrs[2]++ = fouropixels[0];
        // #endif
        // 	    } while (x-=4);
        // 	    olineptrs[0] += 2*X_width/4;
        // 	    olineptrs[1] += 2*X_width/4;
        // 	    olineptrs[2] += 2*X_width/4;
        // 	}
        //
        //     }
        //     else if (multiply == 4)
        //     {
        // 	// Broken. Gotta fix this some day.
        // 	void Expand4(unsigned *, double *);
        //   	Expand4 ((unsigned *)(screens[0]), (double *) (image->data));
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (doShm)
        //     {
        //
        // 	if (!XShmPutImage(	X_display,
        // 				X_mainWindow,
        // 				X_gc,
        // 				image,
        // 				0, 0,
        // 				0, 0,
        // 				X_width, X_height,
        // 				True ))
        // 	    I_Error("XShmPutImage() failed\n");
        //
        // 	// wait for it to finish and processes all input events
        // 	shmFinished = false;
        // 	do
        // 	{
        // 	    I_GetEvent();
        // 	} while (!shmFinished);
        //
        //     }
        //     else
        //     {
        //
        // 	// draw the image
        // 	XPutImage(	X_display,
        // 			X_mainWindow,
        // 			X_gc,
        // 			image,
        // 			0, 0,
        // 			0, 0,
        // 			X_width, X_height );
        //
        // 	// sync up with server
        // 	XSync(X_display, False);
        //
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn I_ReadScreen(mut scr: *mut byte) {
    unsafe {
        memcpy(scr, screens[(0) as usize], (SCREENWIDTH * SCREENHEIGHT));
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

static mut colors: [x11::xlib::XColor; (256) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn UploadNewPalette(mut cmap: x11::xlib::Colormap, mut palette: *mut byte) {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut c: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        static mut firstcall: boolean = unsafe { true_ };
        // C preprocessor directive at statement position (not executable, nothing lost):
        //
        //
        // #ifdef __cplusplus
        // TODO: if statement not yet translated:
        //     if (X_visualinfo.c_class == PseudoColor && X_visualinfo.depth == 8)
        // #else
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //     if (X_visualinfo.class == PseudoColor && X_visualinfo.depth == 8)
        // #endif
        todo!("if statement not yet translated");
        {
            // TODO: if statement not yet translated:
            //
            // 	    // initialize the colormap
            // 	    if (firstcall)
            // 	    {
            // 		firstcall = false;
            // 		for (i=0 ; i<256 ; i++)
            // 		{
            // 		    colors[i].pixel = i;
            // 		    colors[i].flags = DoRed|DoGreen|DoBlue;
            // 		}
            // 	    }
            todo!("if statement not yet translated");
            // TODO: for statement not yet translated:
            //
            //
            // 	    // set the X colormap entries
            // 	    for (i=0 ; i<256 ; i++)
            // 	    {
            // 		c = gammatable[usegamma][*palette++];
            // 		colors[i].red = (c<<8) + c;
            // 		c = gammatable[usegamma][*palette++];
            // 		colors[i].green = (c<<8) + c;
            // 		c = gammatable[usegamma][*palette++];
            // 		colors[i].blue = (c<<8) + c;
            // 	    }
            todo!("for statement not yet translated");
            XStoreColors(X_display, cmap, colors, 256);
            // TODO: statement not yet translated:
            //
            //
            //
            todo!("statement not yet translated");
        }
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn I_SetPalette(mut palette: *mut byte) {
    unsafe {
        UploadNewPalette(X_cmap, palette);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn grabsharedmemory(mut size: std::ffi::c_int) {
    unsafe {
        let mut key: std::ffi::c_int = unsafe {
            (((((b'd' as std::ffi::c_int) << 24) | ((b'o' as std::ffi::c_int) << 16))
                | ((b'o' as std::ffi::c_int) << 8))
                | (b'm' as std::ffi::c_int))
        };
        let mut shminfo: shmid_ds = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut minsize: std::ffi::c_int = unsafe { (320 * 200) };
        let mut id: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut rc: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut pollution: std::ffi::c_int = unsafe { 5 };
        // TODO: do-while statement not yet translated:
        //
        //
        //   // try to use what was here before
        //   do
        //   {
        //     id = shmget((key_t) key, minsize, 0777); // just get the id
        //     if (id != -1)
        //     {
        //       rc=shmctl(id, IPC_STAT, &shminfo); // get stats on it
        //       if (!rc)
        //       {
        // 	if (shminfo.shm_nattch)
        // 	{
        // 	  fprintf(stderr, "User %d appears to be running "
        // 		  "DOOM.  Is that wise?\n", shminfo.shm_cpid);
        // 	  key++;
        // 	}
        // 	else
        // 	{
        // 	  if (getuid() == shminfo.shm_perm.cuid)
        // 	  {
        // 	    rc = shmctl(id, IPC_RMID, 0);
        // 	    if (!rc)
        // 	      fprintf(stderr,
        // 		      "Was able to kill my old shared memory\n");
        // 	    else
        // 	      I_Error("Was NOT able to kill my old shared memory");
        //
        // 	    id = shmget((key_t)key, size, IPC_CREAT|0777);
        // 	    if (id==-1)
        // 	      I_Error("Could not get shared memory");
        //
        // 	    rc=shmctl(id, IPC_STAT, &shminfo);
        //
        // 	    break;
        //
        // 	  }
        // 	  if (size >= shminfo.shm_segsz)
        // 	  {
        // 	    fprintf(stderr,
        // 		    "will use %d's stale shared memory\n",
        // 		    shminfo.shm_cpid);
        // 	    break;
        // 	  }
        // 	  else
        // 	  {
        // 	    fprintf(stderr,
        // 		    "warning: can't use stale "
        // 		    "shared memory belonging to id %d, "
        // 		    "key=0x%x\n",
        // 		    shminfo.shm_cpid, key);
        // 	    key++;
        // 	  }
        // 	}
        //       }
        //       else
        //       {
        // 	I_Error("could not get stats on key=%d", key);
        //       }
        //     }
        //     else
        //     {
        //       id = shmget((key_t)key, size, IPC_CREAT|0777);
        //       if (id==-1)
        //       {
        // 	extern int errno;
        // 	fprintf(stderr, "errno=%d\n", errno);
        // 	I_Error("Could not get any shared memory");
        //       }
        //       break;
        //     }
        //   } while (--pollution);
        todo!("do-while statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //   if (!pollution)
        //   {
        //     I_Error("Sorry, system too polluted with stale "
        // 	    "shared memory segments.\n");
        //     }
        todo!("if statement not yet translated");
        X_shminfo.shmid = id;
        (*image).data = X_shminfo.shmaddr = shmat(id, 0, 0);
        fprintf(
            stderr,
            (c"shared memory id=%d, addr=0x%x\n").as_ptr(),
            id,
            (((*image).data) as std::ffi::c_int),
        );
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn I_InitGraphics() {
    unsafe {
        let mut displayname: *mut std::ffi::c_char = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut d: *mut std::ffi::c_char = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut n: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut pnum: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut x: std::ffi::c_int = unsafe { 0 };
        let mut y: std::ffi::c_int = unsafe { 0 };
        let mut xsign: std::ffi::c_char = unsafe { (b' ' as std::ffi::c_int) };
        let mut ysign: std::ffi::c_char = unsafe { (b' ' as std::ffi::c_int) };
        let mut oktodraw: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut attribmask: std::ffi::c_ulong = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: statement not yet translated:
        //
        //     XSetWindowAttributes attribs;
        todo!("statement not yet translated");
        // TODO: statement not yet translated:
        //
        //     XGCValues		xgcvalues;
        todo!("statement not yet translated");
        let mut valuemask: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        static mut firsttime: std::ffi::c_int = unsafe { 1 };
        // TODO: if statement not yet translated:
        //
        //
        //     if (!firsttime)
        // 	return;
        todo!("if statement not yet translated");
        firsttime = 0;
        signal(
            SIGINT,
            ((I_Quit) as Option<unsafe extern "C" fn(std::ffi::c_int)>),
        );
        // TODO: if statement not yet translated:
        //
        //
        //     if (M_CheckParm("-2"))
        // 	multiply = 2;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (M_CheckParm("-3"))
        // 	multiply = 3;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (M_CheckParm("-4"))
        // 	multiply = 4;
        todo!("if statement not yet translated");
        X_width = (SCREENWIDTH * multiply);
        X_height = (SCREENHEIGHT * multiply);
        // TODO: if statement not yet translated:
        //
        //
        //     // check for command-line display name
        //     if ( (pnum=M_CheckParm("-disp")) ) // suggest parentheses around assignment
        // 	displayname = myargv[pnum+1];
        //     else
        // 	displayname = 0;
        todo!("if statement not yet translated");
        grabMouse = (((((M_CheckParm((c"-grabmouse").as_ptr())) == 0) as std::ffi::c_int) == 0)
            as std::ffi::c_int);
        // TODO: if statement not yet translated:
        //
        //
        //     // check for command-line geometry
        //     if ( (pnum=M_CheckParm("-geom")) ) // suggest parentheses around assignment
        //     {
        // 	// warning: char format, different type arg 3,5
        // 	n = sscanf(myargv[pnum+1], "%c%d%c%d", &xsign, &x, &ysign, &y);
        //
        // 	if (n==2)
        // 	    x = y = 0;
        // 	else if (n==6)
        // 	{
        // 	    if (xsign == '-')
        // 		x = -x;
        // 	    if (ysign == '-')
        // 		y = -y;
        // 	}
        // 	else
        // 	    I_Error("bad -geom parameter");
        //     }
        todo!("if statement not yet translated");
        X_display = XOpenDisplay(displayname);
        // TODO: if statement not yet translated:
        //
        //     if (!X_display)
        //     {
        // 	if (displayname)
        // 	    I_Error("Could not open display [%s]", displayname);
        // 	else
        // 	    I_Error("Could not open display (DISPLAY=[%s])", getenv("DISPLAY"));
        //     }
        todo!("if statement not yet translated");
        X_screen = DefaultScreen(X_display);
        // TODO: if statement not yet translated:
        //
        //     if (!XMatchVisualInfo(X_display, X_screen, 8, PseudoColor, &X_visualinfo))
        // 	I_Error("xdoom currently only supports 256-color PseudoColor screens");
        todo!("if statement not yet translated");
        X_visual = X_visualinfo.visual;
        doShm = XShmQueryExtension(X_display);
        // TODO: if statement not yet translated:
        //
        //
        //     // even if it's available, make sure it's a local connection
        //     if (doShm)
        //     {
        // 	if (!displayname) displayname = (char *) getenv("DISPLAY");
        // 	if (displayname)
        // 	{
        // 	    d = displayname;
        // 	    while (*d && (*d != ':')) d++;
        // 	    if (*d) *d = 0;
        // 	    if (!(!strcasecmp(displayname, "unix") || !*displayname)) doShm = false;
        // 	}
        //     }
        todo!("if statement not yet translated");
        fprintf(stderr, (c"Using MITSHM extension\n").as_ptr());
        X_cmap = XCreateColormap(
            X_display,
            RootWindow(X_display, X_screen),
            X_visual,
            AllocAll,
        );
        attribmask = ((CWEventMask | CWColormap) | CWBorderPixel);
        attribs.event_mask = ((KeyPressMask | KeyReleaseMask) | ExposureMask);
        attribs.colormap = X_cmap;
        attribs.border_pixel = 0;
        X_mainWindow = XCreateWindow(
            X_display,
            RootWindow(X_display, X_screen),
            x,
            y,
            X_width,
            X_height,
            0,
            8,
            InputOutput,
            X_visual,
            attribmask,
            (&(attribs) as *const _ as *mut _),
        );
        XDefineCursor(
            X_display,
            X_mainWindow,
            createnullcursor(X_display, X_mainWindow),
        );
        valuemask = GCGraphicsExposures;
        xgcvalues.graphics_exposures = False;
        X_gc = XCreateGC(
            X_display,
            X_mainWindow,
            valuemask,
            (&(xgcvalues) as *const _ as *mut _),
        );
        XMapWindow(X_display, X_mainWindow);
        oktodraw = 0;
        // TODO: while statement not yet translated:
        //
        //     while (!oktodraw)
        //     {
        // 	XNextEvent(X_display, &X_event);
        // 	if (X_event.type == Expose
        // 	    && !X_event.xexpose.count)
        // 	{
        // 	    oktodraw = 1;
        // 	}
        //     }
        todo!("while statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // grabs the pointer so it is restricted to this window
        //     if (grabMouse)
        // 	XGrabPointer(X_display, X_mainWindow, True,
        // 		     ButtonPressMask|ButtonReleaseMask|PointerMotionMask,
        // 		     GrabModeAsync, GrabModeAsync,
        // 		     X_mainWindow, None, CurrentTime);
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (doShm)
        //     {
        //
        // 	X_shmeventtype = XShmGetEventBase(X_display) + ShmCompletion;
        //
        // 	// create the image
        // 	image = XShmCreateImage(	X_display,
        // 					X_visual,
        // 					8,
        // 					ZPixmap,
        // 					0,
        // 					&X_shminfo,
        // 					X_width,
        // 					X_height );
        //
        // 	grabsharedmemory(image->bytes_per_line * image->height);
        //
        //
        // 	// UNUSED
        // 	// create the shared memory segment
        // 	// X_shminfo.shmid = shmget (IPC_PRIVATE,
        // 	// image->bytes_per_line * image->height, IPC_CREAT | 0777);
        // 	// if (X_shminfo.shmid < 0)
        // 	// {
        // 	// perror("");
        // 	// I_Error("shmget() failed in InitGraphics()");
        // 	// }
        // 	// fprintf(stderr, "shared memory id=%d\n", X_shminfo.shmid);
        // 	// attach to the shared memory segment
        // 	// image->data = X_shminfo.shmaddr = shmat(X_shminfo.shmid, 0, 0);
        //
        //
        // 	if (!image->data)
        // 	{
        // 	    perror("");
        // 	    I_Error("shmat() failed in InitGraphics()");
        // 	}
        //
        // 	// get the X server to attach to it
        // 	if (!XShmAttach(X_display, &X_shminfo))
        // 	    I_Error("XShmAttach() failed in InitGraphics()");
        //
        //     }
        //     else
        //     {
        // 	image = XCreateImage(	X_display,
        //     				X_visual,
        //     				8,
        //     				ZPixmap,
        //     				0,
        //     				(char*)malloc(X_width * X_height),
        //     				X_width, X_height,
        //     				8,
        //     				X_width );
        //
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (multiply == 1)
        // 	screens[0] = (unsigned char *) (image->data);
        //     else
        // 	screens[0] = (unsigned char *) malloc (SCREENWIDTH * SCREENHEIGHT);
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub static mut exptable: [std::ffi::c_uint; (256) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn InitExpand() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<256 ; i++)
        // 	exptable[i] = i | (i<<8) | (i<<16) | (i<<24);
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub static mut exptable2: [std::ffi::c_double; (256 * 256) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn InitExpand2() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut j: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut exp: *mut std::ffi::c_double = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut pixel: () = (); // TODO: unparsed local type, needs manual translation
        printf((c"building exptable2...\n").as_ptr());
        exp = exptable2;
        // TODO: for statement not yet translated:
        //
        //     for (i=0 ; i<256 ; i++)
        //     {
        // 	pixel.u[0] = i | (i<<8) | (i<<16) | (i<<24);
        // 	for (j=0 ; j<256 ; j++)
        // 	{
        // 	    pixel.u[1] = j | (j<<8) | (j<<16) | (j<<24);
        // 	    *exp++ = pixel.d;
        // 	}
        //     }
        todo!("for statement not yet translated");
        printf((c"done.\n").as_ptr());
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub static mut inited: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn Expand4(
    mut lineptr: *mut std::ffi::c_uint,
    mut xline: *mut std::ffi::c_double,
) {
    unsafe {
        let mut dpixel: std::ffi::c_double = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut x: std::ffi::c_uint = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut y: std::ffi::c_uint = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut fourpixels: std::ffi::c_uint = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut step: std::ffi::c_uint = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut exp: *mut std::ffi::c_double = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        exp = exptable2;
        // TODO: if statement not yet translated:
        //
        //     if (!inited)
        //     {
        // 	inited = 1;
        // 	InitExpand2 ();
        //     }
        todo!("if statement not yet translated");
        step = ((3 * SCREENWIDTH) / 2);
        y = (SCREENHEIGHT - 1);
        // TODO: do-while statement not yet translated:
        //
        //     do
        //     {
        // 	x = SCREENWIDTH;
        //
        // 	do
        // 	{
        // 	    fourpixels = lineptr[0];
        //
        // 	    dpixel = *(double *)( (int)exp + ( (fourpixels&0xffff0000)>>13) );
        // 	    xline[0] = dpixel;
        // 	    xline[160] = dpixel;
        // 	    xline[320] = dpixel;
        // 	    xline[480] = dpixel;
        //
        // 	    dpixel = *(double *)( (int)exp + ( (fourpixels&0xffff)<<3 ) );
        // 	    xline[1] = dpixel;
        // 	    xline[161] = dpixel;
        // 	    xline[321] = dpixel;
        // 	    xline[481] = dpixel;
        //
        // 	    fourpixels = lineptr[1];
        //
        // 	    dpixel = *(double *)( (int)exp + ( (fourpixels&0xffff0000)>>13) );
        // 	    xline[2] = dpixel;
        // 	    xline[162] = dpixel;
        // 	    xline[322] = dpixel;
        // 	    xline[482] = dpixel;
        //
        // 	    dpixel = *(double *)( (int)exp + ( (fourpixels&0xffff)<<3 ) );
        // 	    xline[3] = dpixel;
        // 	    xline[163] = dpixel;
        // 	    xline[323] = dpixel;
        // 	    xline[483] = dpixel;
        //
        // 	    fourpixels = lineptr[2];
        //
        // 	    dpixel = *(double *)( (int)exp + ( (fourpixels&0xffff0000)>>13) );
        // 	    xline[4] = dpixel;
        // 	    xline[164] = dpixel;
        // 	    xline[324] = dpixel;
        // 	    xline[484] = dpixel;
        //
        // 	    dpixel = *(double *)( (int)exp + ( (fourpixels&0xffff)<<3 ) );
        // 	    xline[5] = dpixel;
        // 	    xline[165] = dpixel;
        // 	    xline[325] = dpixel;
        // 	    xline[485] = dpixel;
        //
        // 	    fourpixels = lineptr[3];
        //
        // 	    dpixel = *(double *)( (int)exp + ( (fourpixels&0xffff0000)>>13) );
        // 	    xline[6] = dpixel;
        // 	    xline[166] = dpixel;
        // 	    xline[326] = dpixel;
        // 	    xline[486] = dpixel;
        //
        // 	    dpixel = *(double *)( (int)exp + ( (fourpixels&0xffff)<<3 ) );
        // 	    xline[7] = dpixel;
        // 	    xline[167] = dpixel;
        // 	    xline[327] = dpixel;
        // 	    xline[487] = dpixel;
        //
        // 	    lineptr+=4;
        // 	    xline+=8;
        // 	} while (x-=16);
        // 	xline += step;
        //     } while (y--);
        todo!("do-while statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}
