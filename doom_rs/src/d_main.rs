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
use crate::f_finale::*;
use crate::f_wipe::*;
use crate::g_game::*;
use crate::hu_stuff::*;
use crate::i_sound::*;
use crate::i_system::*;
use crate::i_video::*;
use crate::info::*;
use crate::m_argv::*;
use crate::m_fixed::*;
use crate::m_menu::*;
use crate::m_misc::*;
use crate::p_mobj::*;
use crate::p_pspr::*;
use crate::p_setup::*;
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
use crate::st_stuff::*;
use crate::tables::*;
use crate::v_video::*;
use crate::w_wad::*;
use crate::wi_stuff::*;
use crate::z_zone::*;

pub const MAXWADFILES: std::ffi::c_int = 20;

static mut rcsid: [std::ffi::c_char; 49] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        100 as std::ffi::c_char,
        95 as std::ffi::c_char,
        109 as std::ffi::c_char,
        97 as std::ffi::c_char,
        105 as std::ffi::c_char,
        110 as std::ffi::c_char,
        46 as std::ffi::c_char,
        99 as std::ffi::c_char,
        44 as std::ffi::c_char,
        118 as std::ffi::c_char,
        32 as std::ffi::c_char,
        49 as std::ffi::c_char,
        46 as std::ffi::c_char,
        56 as std::ffi::c_char,
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
        48 as std::ffi::c_char,
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

pub const BGCOLOR: std::ffi::c_int = 7;

pub const FGCOLOR: std::ffi::c_int = 8;

pub static mut wadfiles: [*mut std::ffi::c_char; (MAXWADFILES) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut devparm: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut nomonsters: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut respawnparm: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut fastparm: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut drone: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut singletics: boolean = unsafe { false_ };

unsafe extern "C" {
    pub static mut inhelpscreens: boolean;
}

pub static mut startskill: skill_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut startepisode: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut startmap: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut autostart: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut debugfile: *mut libc::FILE = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut advancedemo: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut wadfile: [std::ffi::c_char; (1024) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut mapdir: [std::ffi::c_char; (1024) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut basedefault: [std::ffi::c_char; (1024) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

unsafe extern "C" {
    pub fn D_CheckNetGame();
}

unsafe extern "C" {
    pub fn G_BuildTiccmd(cmd: *mut ticcmd_t);
}

pub static mut events: [event_t; (MAXEVENTS) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut eventhead: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut eventtail: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn D_PostEvent(mut ev: *mut event_t) {
    unsafe {
        events[(eventhead) as usize] = (*(ev));
        eventhead = (({
            eventhead += 1;
            eventhead
        }) & (MAXEVENTS - 1));
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn D_ProcessEvents() {
    unsafe {
        let mut ev: *mut event_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     // IF STORE DEMO, DO NOT ACCEPT INPUT
        //     if ( ( gamemode == commercial )
        // 	 && (W_CheckNumForName("map01")<0) )
        //       return;
        todo!("if statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //
        //     for ( ; eventtail != eventhead ; eventtail = (++eventtail)&(MAXEVENTS-1) )
        //     {
        // 	ev = &events[eventtail];
        // 	if (M_Responder (ev))
        // 	    continue;               // menu ate the event
        // 	G_Responder (ev);
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub static mut wipegamestate: gamestate_t = unsafe { GS_DEMOSCREEN };

unsafe extern "C" {
    pub static mut setsizeneeded: boolean;
}

unsafe extern "C" {
    pub static mut showMessages: std::ffi::c_int;
}

unsafe extern "C" {
    pub fn R_ExecuteSetViewSize();
}

pub unsafe extern "C" fn D_Display() {
    unsafe {
        static mut viewactivestate: boolean = unsafe { false_ };
        static mut menuactivestate: boolean = unsafe { false_ };
        static mut inhelpscreensstate: boolean = unsafe { false_ };
        static mut fullscreen: boolean = unsafe { false_ };
        static mut oldgamestate: gamestate_t = unsafe { (-(1)) };
        static mut borderdrawcount: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut nowtime: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut tics: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut wipestart: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut y: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut done: boolean = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut wipe: boolean = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut redrawsbar: boolean = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (nodrawers)
        // 	return;                    // for comparative timing / profiling
        todo!("if statement not yet translated");
        redrawsbar = false_;
        // TODO: if statement not yet translated:
        //
        //
        //     // change the view size if needed
        //     if (setsizeneeded)
        //     {
        // 	R_ExecuteSetViewSize ();
        // 	oldgamestate = -1;                      // force background redraw
        // 	borderdrawcount = 3;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // save the current screen if about to wipe
        //     if (gamestate != wipegamestate)
        //     {
        // 	wipe = true;
        // 	wipe_StartScreen(0, 0, SCREENWIDTH, SCREENHEIGHT);
        //     }
        //     else
        // 	wipe = false;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (gamestate == GS_LEVEL && gametic)
        // 	HU_Erase();
        todo!("if statement not yet translated");
        // TODO: switch statement not yet translated:
        //
        //
        //     // do buffered drawing
        //     switch (gamestate)
        //     {
        //       case GS_LEVEL:
        // 	if (!gametic)
        // 	    break;
        // 	if (automapactive)
        // 	    AM_Drawer ();
        // 	if (wipe || (viewheight != 200 && fullscreen) )
        // 	    redrawsbar = true;
        // 	if (inhelpscreensstate && !inhelpscreens)
        // 	    redrawsbar = true;              // just put away the help screen
        // 	ST_Drawer (viewheight == 200, redrawsbar );
        // 	fullscreen = viewheight == 200;
        // 	break;
        //
        //       case GS_INTERMISSION:
        // 	WI_Drawer ();
        // 	break;
        //
        //       case GS_FINALE:
        // 	F_Drawer ();
        // 	break;
        //
        //       case GS_DEMOSCREEN:
        // 	D_PageDrawer ();
        // 	break;
        //     }
        todo!("switch statement not yet translated");
        I_UpdateNoBlit();
        // TODO: if statement not yet translated:
        //
        //
        //     // draw the view directly
        //     if (gamestate == GS_LEVEL && !automapactive && gametic)
        // 	R_RenderPlayerView (&players[displayplayer]);
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (gamestate == GS_LEVEL && gametic)
        // 	HU_Drawer ();
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // clean up border stuff
        //     if (gamestate != oldgamestate && gamestate != GS_LEVEL)
        // 	I_SetPalette (W_CacheLumpName ("PLAYPAL",PU_CACHE));
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // see if the border needs to be initially drawn
        //     if (gamestate == GS_LEVEL && oldgamestate != GS_LEVEL)
        //     {
        // 	viewactivestate = false;        // view was not active
        // 	R_FillBackScreen ();    // draw the pattern into the back screen
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // see if the border needs to be updated to the screen
        //     if (gamestate == GS_LEVEL && !automapactive && scaledviewwidth != 320)
        //     {
        // 	if (menuactive || menuactivestate || !viewactivestate)
        // 	    borderdrawcount = 3;
        // 	if (borderdrawcount)
        // 	{
        // 	    R_DrawViewBorder ();    // erase old menu stuff
        // 	    borderdrawcount--;
        // 	}
        //
        //     }
        todo!("if statement not yet translated");
        menuactivestate = menuactive;
        viewactivestate = viewactive;
        inhelpscreensstate = inhelpscreens;
        oldgamestate = wipegamestate = gamestate;
        // TODO: if statement not yet translated:
        //
        //
        //     // draw pause pic
        //     if (paused)
        //     {
        // 	if (automapactive)
        // 	    y = 4;
        // 	else
        // 	    y = viewwindowy+4;
        // 	V_DrawPatchDirect(viewwindowx+(scaledviewwidth-68)/2,
        // 			  y,0,W_CacheLumpName ("M_PAUSE", PU_CACHE));
        //     }
        todo!("if statement not yet translated");
        M_Drawer();
        NetUpdate();
        // TODO: if statement not yet translated:
        //
        //
        //     // normal update
        //     if (!wipe)
        //     {
        // 	I_FinishUpdate ();              // page flip or blit buffer
        // 	return;
        //     }
        todo!("if statement not yet translated");
        wipe_EndScreen(0, 0, SCREENWIDTH, SCREENHEIGHT);
        wipestart = (I_GetTime() - 1);
        // TODO: do-while statement not yet translated:
        //
        //
        //     do
        //     {
        // 	do
        // 	{
        // 	    nowtime = I_GetTime ();
        // 	    tics = nowtime - wipestart;
        // 	} while (!tics);
        // 	wipestart = nowtime;
        // 	done = wipe_ScreenWipe(wipe_Melt
        // 			       , 0, 0, SCREENWIDTH, SCREENHEIGHT, tics);
        // 	I_UpdateNoBlit ();
        // 	M_Drawer ();                            // menu is drawn even on top of wipes
        // 	I_FinishUpdate ();                      // page flip or blit buffer
        //     } while (!done);
        todo!("do-while statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

unsafe extern "C" {
    pub static mut demorecording: boolean;
}

pub unsafe extern "C" fn D_DoomLoop() {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (demorecording)
        // 	G_BeginRecording ();
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (M_CheckParm ("-debugfile"))
        //     {
        // 	char    filename[20];
        // 	sprintf (filename,"debug%i.txt",consoleplayer);
        // 	printf ("debug output to: %s\n",filename);
        // 	debugfile = fopen (filename,"w");
        //     }
        todo!("if statement not yet translated");
        I_InitGraphics();
        // TODO: while statement not yet translated:
        //
        //
        //     while (1)
        //     {
        // 	// frame syncronous IO operations
        // 	I_StartFrame ();
        //
        // 	// process one or more tics
        // 	if (singletics)
        // 	{
        // 	    I_StartTic ();
        // 	    D_ProcessEvents ();
        // 	    G_BuildTiccmd (&netcmds[consoleplayer][maketic%BACKUPTICS]);
        // 	    if (advancedemo)
        // 		D_DoAdvanceDemo ();
        // 	    M_Ticker ();
        // 	    G_Ticker ();
        // 	    gametic++;
        // 	    maketic++;
        // 	}
        // 	else
        // 	{
        // 	    TryRunTics (); // will run at least one tic
        // 	}
        //
        // 	S_UpdateSounds (players[consoleplayer].mo);// move positional sounds
        //
        // 	// Update display, next frame, with current state.
        // 	D_Display ();
        //
        // #ifndef SNDSERV
        // 	// Sound mixing for the buffer is snychronous.
        // 	I_UpdateSound();
        // #endif
        // 	// Synchronous sound output is explicitly called.
        // #ifndef SNDINTR
        // 	// Update sound output.
        // 	I_SubmitSound();
        // #endif
        //     }
        todo!("while statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub static mut demosequence: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut pagetic: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut pagename: *mut std::ffi::c_char = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn D_PageTicker() {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (--pagetic < 0)
        // 	D_AdvanceDemo ();
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn D_PageDrawer() {
    unsafe {
        V_DrawPatch(0, 0, 0, W_CacheLumpName(pagename, PU_CACHE));
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn D_AdvanceDemo() {
    unsafe {
        advancedemo = true_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn D_DoAdvanceDemo() {
    unsafe {
        players[(consoleplayer) as usize].playerstate = PST_LIVE;
        advancedemo = false_;
        usergame = false_;
        paused = false_;
        gameaction = ga_nothing;
        // TODO: if statement not yet translated:
        //
        //
        //     if ( gamemode == retail )
        //       demosequence = (demosequence+1)%7;
        //     else
        //       demosequence = (demosequence+1)%6;
        todo!("if statement not yet translated");
        // TODO: switch statement not yet translated:
        //
        //
        //     switch (demosequence)
        //     {
        //       case 0:
        // 	if ( gamemode == commercial )
        // 	    pagetic = 35 * 11;
        // 	else
        // 	    pagetic = 170;
        // 	gamestate = GS_DEMOSCREEN;
        // 	pagename = "TITLEPIC";
        // 	if ( gamemode == commercial )
        // 	  S_StartMusic(mus_dm2ttl);
        // 	else
        // 	  S_StartMusic (mus_intro);
        // 	break;
        //       case 1:
        // 	G_DeferedPlayDemo ("demo1");
        // 	break;
        //       case 2:
        // 	pagetic = 200;
        // 	gamestate = GS_DEMOSCREEN;
        // 	pagename = "CREDIT";
        // 	break;
        //       case 3:
        // 	G_DeferedPlayDemo ("demo2");
        // 	break;
        //       case 4:
        // 	gamestate = GS_DEMOSCREEN;
        // 	if ( gamemode == commercial)
        // 	{
        // 	    pagetic = 35 * 11;
        // 	    pagename = "TITLEPIC";
        // 	    S_StartMusic(mus_dm2ttl);
        // 	}
        // 	else
        // 	{
        // 	    pagetic = 200;
        //
        // 	    if ( gamemode == retail )
        // 	      pagename = "CREDIT";
        // 	    else
        // 	      pagename = "HELP2";
        // 	}
        // 	break;
        //       case 5:
        // 	G_DeferedPlayDemo ("demo3");
        // 	break;
        //         // THE DEFINITIVE DOOM Special Edition demo
        //       case 6:
        // 	G_DeferedPlayDemo ("demo4");
        // 	break;
        //     }
        todo!("switch statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn D_StartTitle() {
    unsafe {
        gameaction = ga_nothing;
        demosequence = (-(1));
        D_AdvanceDemo();
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub static mut title: [std::ffi::c_char; (128) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn D_AddFile(mut file: *mut std::ffi::c_char) {
    unsafe {
        let mut numwadfiles: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut newfile: *mut std::ffi::c_char = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: for statement not yet translated:
        //
        //
        //     for (numwadfiles = 0 ; wadfiles[numwadfiles] ; numwadfiles++)
        // 	;
        todo!("for statement not yet translated");
        newfile = malloc((strlen(file) + 1));
        strcpy(newfile, file);
        wadfiles[(numwadfiles) as usize] = newfile;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn IdentifyVersion() {
    unsafe {
        let mut doom1wad: *mut std::ffi::c_char = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut doomwad: *mut std::ffi::c_char = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut doomuwad: *mut std::ffi::c_char = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut doom2wad: *mut std::ffi::c_char = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut doom2fwad: *mut std::ffi::c_char = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut plutoniawad: *mut std::ffi::c_char = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut tntwad: *mut std::ffi::c_char = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut home: *mut std::ffi::c_char = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut doomwaddir: *mut std::ffi::c_char = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        doomwaddir = getenv((c"DOOMWADDIR").as_ptr());
        // TODO: if statement not yet translated:
        //
        //     if (!doomwaddir)
        // 	doomwaddir = ".";
        todo!("if statement not yet translated");
        doom2wad = malloc((((strlen(doomwaddir) + 1) + 9) + 1));
        sprintf(doom2wad, (c"%s/doom2.wad").as_ptr(), doomwaddir);
        doomuwad = malloc((((strlen(doomwaddir) + 1) + 8) + 1));
        sprintf(doomuwad, (c"%s/doomu.wad").as_ptr(), doomwaddir);
        doomwad = malloc((((strlen(doomwaddir) + 1) + 8) + 1));
        sprintf(doomwad, (c"%s/doom.wad").as_ptr(), doomwaddir);
        doom1wad = malloc((((strlen(doomwaddir) + 1) + 9) + 1));
        sprintf(doom1wad, (c"%s/doom1.wad").as_ptr(), doomwaddir);
        plutoniawad = malloc((((strlen(doomwaddir) + 1) + 12) + 1));
        sprintf(plutoniawad, (c"%s/plutonia.wad").as_ptr(), doomwaddir);
        tntwad = malloc((((strlen(doomwaddir) + 1) + 9) + 1));
        sprintf(tntwad, (c"%s/tnt.wad").as_ptr(), doomwaddir);
        doom2fwad = malloc((((strlen(doomwaddir) + 1) + 10) + 1));
        sprintf(doom2fwad, (c"%s/doom2f.wad").as_ptr(), doomwaddir);
        home = getenv((c"HOME").as_ptr());
        // TODO: if statement not yet translated:
        //
        //     if (!home)
        //       I_Error("Please set $HOME to your home directory");
        todo!("if statement not yet translated");
        sprintf(basedefault, (c"%s/.doomrc").as_ptr(), home);
        // TODO: if statement not yet translated:
        //
        //     if (M_CheckParm ("-shdev"))
        //     {
        // 	gamemode = shareware;
        // 	devparm = true;
        // 	D_AddFile (DEVDATA"doom1.wad");
        // 	D_AddFile (DEVMAPS"data_se/texture1.lmp");
        // 	D_AddFile (DEVMAPS"data_se/pnames.lmp");
        // 	strcpy (basedefault,DEVDATA"default.cfg");
        // 	return;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (M_CheckParm ("-regdev"))
        //     {
        // 	gamemode = registered;
        // 	devparm = true;
        // 	D_AddFile (DEVDATA"doom.wad");
        // 	D_AddFile (DEVMAPS"data_se/texture1.lmp");
        // 	D_AddFile (DEVMAPS"data_se/texture2.lmp");
        // 	D_AddFile (DEVMAPS"data_se/pnames.lmp");
        // 	strcpy (basedefault,DEVDATA"default.cfg");
        // 	return;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (M_CheckParm ("-comdev"))
        //     {
        // 	gamemode = commercial;
        // 	devparm = true;
        // 	/* I don't bother
        // 	if(plutonia)
        // 	    D_AddFile (DEVDATA"plutonia.wad");
        // 	else if(tnt)
        // 	    D_AddFile (DEVDATA"tnt.wad");
        // 	else*/
        // 	    D_AddFile (DEVDATA"doom2.wad");
        //
        // 	D_AddFile (DEVMAPS"cdata/texture1.lmp");
        // 	D_AddFile (DEVMAPS"cdata/pnames.lmp");
        // 	strcpy (basedefault,DEVDATA"default.cfg");
        // 	return;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if ( !access (doom2fwad,R_OK) )
        //     {
        // 	gamemode = commercial;
        // 	// C'est ridicule!
        // 	// Let's handle languages in config files, okay?
        // 	language = french;
        // 	printf("French version\n");
        // 	D_AddFile (doom2fwad);
        // 	return;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if ( !access (doom2wad,R_OK) )
        //     {
        // 	gamemode = commercial;
        // 	D_AddFile (doom2wad);
        // 	return;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if ( !access (plutoniawad, R_OK ) )
        //     {
        //       gamemode = commercial;
        //       D_AddFile (plutoniawad);
        //       return;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if ( !access ( tntwad, R_OK ) )
        //     {
        //       gamemode = commercial;
        //       D_AddFile (tntwad);
        //       return;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if ( !access (doomuwad,R_OK) )
        //     {
        //       gamemode = retail;
        //       D_AddFile (doomuwad);
        //       return;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if ( !access (doomwad,R_OK) )
        //     {
        //       gamemode = registered;
        //       D_AddFile (doomwad);
        //       return;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if ( !access (doom1wad,R_OK) )
        //     {
        //       gamemode = shareware;
        //       D_AddFile (doom1wad);
        //       return;
        //     }
        todo!("if statement not yet translated");
        printf((c"Game mode indeterminate.\n").as_ptr());
        gamemode = indetermined;
        // TODO: statement not yet translated:
        //
        //
        //     // We don't abort. Let's see what the PWAD contains.
        //     //exit(1);
        //     //I_Error ("Game mode indeterminate\n");
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn FindResponseFile() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // C preprocessor directive at statement position (not executable, nothing lost):
        //
        // #define MAXARGVS        100
        // TODO: for statement not yet translated:
        //
        //     for (i = 1;i < myargc;i++)
        // 	if (myargv[i][0] == '@')
        // 	{
        // 	    FILE *          handle;
        // 	    int             size;
        // 	    int             k;
        // 	    int             index;
        // 	    int             indexinfile;
        // 	    char    *infile;
        // 	    char    *file;
        // 	    char    *moreargs[20];
        // 	    char    *firstargv;
        //
        // 	    // READ THE RESPONSE FILE INTO MEMORY
        // 	    handle = fopen (&myargv[i][1],"rb");
        // 	    if (!handle)
        // 	    {
        // 		printf ("\nNo such response file!");
        // 		exit(1);
        // 	    }
        // 	    printf("Found response file %s!\n",&myargv[i][1]);
        // 	    fseek (handle,0,SEEK_END);
        // 	    size = ftell(handle);
        // 	    fseek (handle,0,SEEK_SET);
        // 	    file = malloc (size);
        // 	    fread (file,size,1,handle);
        // 	    fclose (handle);
        //
        // 	    // KEEP ALL CMDLINE ARGS FOLLOWING @RESPONSEFILE ARG
        // 	    for (index = 0,k = i+1; k < myargc; k++)
        // 		moreargs[index++] = myargv[k];
        //
        // 	    firstargv = myargv[0];
        // 	    myargv = malloc(sizeof(char *)*MAXARGVS);
        // 	    memset(myargv,0,sizeof(char *)*MAXARGVS);
        // 	    myargv[0] = firstargv;
        //
        // 	    infile = file;
        // 	    indexinfile = k = 0;
        // 	    indexinfile++;  // SKIP PAST ARGV[0] (KEEP IT)
        // 	    do
        // 	    {
        // 		myargv[indexinfile++] = infile+k;
        // 		while(k < size &&
        // 		      ((*(infile+k)>= ' '+1) && (*(infile+k)<='z')))
        // 		    k++;
        // 		*(infile+k) = 0;
        // 		while(k < size &&
        // 		      ((*(infile+k)<= ' ') || (*(infile+k)>'z')))
        // 		    k++;
        // 	    } while(k < size);
        //
        // 	    for (k = 0;k < index;k++)
        // 		myargv[indexinfile++] = moreargs[k];
        // 	    myargc = indexinfile;
        //
        // 	    // DISPLAY ARGS
        // 	    printf("%d command-line args:\n",myargc);
        // 	    for (k=1;k<myargc;k++)
        // 		printf("%s\n",myargv[k]);
        //
        // 	    break;
        // 	}
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn D_DoomMain() {
    unsafe {
        let mut p: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut file: [std::ffi::c_char; (256) as usize] = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        FindResponseFile();
        IdentifyVersion();
        setbuf(stdout, NULL);
        modifiedgame = false_;
        nomonsters = M_CheckParm((c"-nomonsters").as_ptr());
        respawnparm = M_CheckParm((c"-respawn").as_ptr());
        fastparm = M_CheckParm((c"-fast").as_ptr());
        devparm = M_CheckParm((c"-devparm").as_ptr());
        // TODO: if statement not yet translated:
        //
        //     if (M_CheckParm ("-altdeath"))
        // 	deathmatch = 2;
        //     else if (M_CheckParm ("-deathmatch"))
        // 	deathmatch = 1;
        todo!("if statement not yet translated");
        // TODO: switch statement not yet translated:
        //
        //
        //     switch ( gamemode )
        //     {
        //       case retail:
        // 	sprintf (title,
        // 		 "                         "
        // 		 "The Ultimate DOOM Startup v%i.%i"
        // 		 "                           ",
        // 		 VERSION/100,VERSION%100);
        // 	break;
        //       case shareware:
        // 	sprintf (title,
        // 		 "                            "
        // 		 "DOOM Shareware Startup v%i.%i"
        // 		 "                           ",
        // 		 VERSION/100,VERSION%100);
        // 	break;
        //       case registered:
        // 	sprintf (title,
        // 		 "                            "
        // 		 "DOOM Registered Startup v%i.%i"
        // 		 "                           ",
        // 		 VERSION/100,VERSION%100);
        // 	break;
        //       case commercial:
        // 	sprintf (title,
        // 		 "                         "
        // 		 "DOOM 2: Hell on Earth v%i.%i"
        // 		 "                           ",
        // 		 VERSION/100,VERSION%100);
        // 	break;
        // /*FIXME
        //        case pack_plut:
        // 	sprintf (title,
        // 		 "                   "
        // 		 "DOOM 2: Plutonia Experiment v%i.%i"
        // 		 "                           ",
        // 		 VERSION/100,VERSION%100);
        // 	break;
        //       case pack_tnt:
        // 	sprintf (title,
        // 		 "                     "
        // 		 "DOOM 2: TNT - Evilution v%i.%i"
        // 		 "                           ",
        // 		 VERSION/100,VERSION%100);
        // 	break;
        // */
        //       default:
        // 	sprintf (title,
        // 		 "                     "
        // 		 "Public DOOM - v%i.%i"
        // 		 "                           ",
        // 		 VERSION/100,VERSION%100);
        // 	break;
        //     }
        todo!("switch statement not yet translated");
        printf((c"%s\n").as_ptr(), title);
        // TODO: if statement not yet translated:
        //
        //
        //     if (devparm)
        // 	printf(D_DEVSTR);
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (M_CheckParm("-cdrom"))
        //     {
        // 	printf(D_CDROM);
        // 	mkdir("c:\\doomdata",0);
        // 	strcpy (basedefault,"c:/doomdata/default.cfg");
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // turbo option
        //     if ( (p=M_CheckParm ("-turbo")) )
        //     {
        // 	int     scale = 200;
        // 	extern int forwardmove[2];
        // 	extern int sidemove[2];
        //
        // 	if (p<myargc-1)
        // 	    scale = atoi (myargv[p+1]);
        // 	if (scale < 10)
        // 	    scale = 10;
        // 	if (scale > 400)
        // 	    scale = 400;
        // 	printf ("turbo scale: %i%%\n",scale);
        // 	forwardmove[0] = forwardmove[0]*scale/100;
        // 	forwardmove[1] = forwardmove[1]*scale/100;
        // 	sidemove[0] = sidemove[0]*scale/100;
        // 	sidemove[1] = sidemove[1]*scale/100;
        //     }
        todo!("if statement not yet translated");
        p = M_CheckParm((c"-wart").as_ptr());
        // TODO: if statement not yet translated:
        //
        //     if (p)
        //     {
        // 	myargv[p][4] = 'p';     // big hack, change to -warp
        //
        // 	// Map name handling.
        // 	switch (gamemode )
        // 	{
        // 	  case shareware:
        // 	  case retail:
        // 	  case registered:
        // 	    sprintf (file,"~"DEVMAPS"E%cM%c.wad",
        // 		     myargv[p+1][0], myargv[p+2][0]);
        // 	    printf("Warping to Episode %s, Map %s.\n",
        // 		   myargv[p+1],myargv[p+2]);
        // 	    break;
        //
        // 	  case commercial:
        // 	  default:
        // 	    p = atoi (myargv[p+1]);
        // 	    if (p<10)
        // 	      sprintf (file,"~"DEVMAPS"cdata/map0%i.wad", p);
        // 	    else
        // 	      sprintf (file,"~"DEVMAPS"cdata/map%i.wad", p);
        // 	    break;
        // 	}
        // 	D_AddFile (file);
        //     }
        todo!("if statement not yet translated");
        p = M_CheckParm((c"-file").as_ptr());
        // TODO: if statement not yet translated:
        //
        //     if (p)
        //     {
        // 	// the parms after p are wadfile/lump names,
        // 	// until end of parms or another - preceded parm
        // 	modifiedgame = true;            // homebrew levels
        // 	while (++p != myargc && myargv[p][0] != '-')
        // 	    D_AddFile (myargv[p]);
        //     }
        todo!("if statement not yet translated");
        p = M_CheckParm((c"-playdemo").as_ptr());
        // TODO: if statement not yet translated:
        //
        //
        //     if (!p)
        // 	p = M_CheckParm ("-timedemo");
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (p && p < myargc-1)
        //     {
        // 	sprintf (file,"%s.lmp", myargv[p+1]);
        // 	D_AddFile (file);
        // 	printf("Playing demo %s.lmp.\n",myargv[p+1]);
        //     }
        todo!("if statement not yet translated");
        startskill = sk_medium;
        startepisode = 1;
        startmap = 1;
        autostart = false_;
        p = M_CheckParm((c"-skill").as_ptr());
        // TODO: if statement not yet translated:
        //
        //     if (p && p < myargc-1)
        //     {
        // 	startskill = myargv[p+1][0]-'1';
        // 	autostart = true;
        //     }
        todo!("if statement not yet translated");
        p = M_CheckParm((c"-episode").as_ptr());
        // TODO: if statement not yet translated:
        //
        //     if (p && p < myargc-1)
        //     {
        // 	startepisode = myargv[p+1][0]-'0';
        // 	startmap = 1;
        // 	autostart = true;
        //     }
        todo!("if statement not yet translated");
        p = M_CheckParm((c"-timer").as_ptr());
        // TODO: if statement not yet translated:
        //
        //     if (p && p < myargc-1 && deathmatch)
        //     {
        // 	int     time;
        // 	time = atoi(myargv[p+1]);
        // 	printf("Levels will end after %d minute",time);
        // 	if (time>1)
        // 	    printf("s");
        // 	printf(".\n");
        //     }
        todo!("if statement not yet translated");
        p = M_CheckParm((c"-avg").as_ptr());
        // TODO: if statement not yet translated:
        //
        //     if (p && p < myargc-1 && deathmatch)
        // 	printf("Austin Virtual Gaming: Levels will end after 20 minutes\n");
        todo!("if statement not yet translated");
        p = M_CheckParm((c"-warp").as_ptr());
        // TODO: if statement not yet translated:
        //
        //     if (p && p < myargc-1)
        //     {
        // 	if (gamemode == commercial)
        // 	    startmap = atoi (myargv[p+1]);
        // 	else
        // 	{
        // 	    startepisode = myargv[p+1][0]-'0';
        // 	    startmap = myargv[p+2][0]-'0';
        // 	}
        // 	autostart = true;
        //     }
        todo!("if statement not yet translated");
        printf((c"V_Init: allocate screens.\n").as_ptr());
        V_Init();
        printf((c"M_LoadDefaults: Load system defaults.\n").as_ptr());
        M_LoadDefaults();
        printf((c"Z_Init: Init zone memory allocation daemon. \n").as_ptr());
        Z_Init();
        printf((c"W_Init: Init WADfiles.\n").as_ptr());
        W_InitMultipleFiles(wadfiles);
        // TODO: if statement not yet translated:
        //
        //
        //
        //     // Check for -file in shareware
        //     if (modifiedgame)
        //     {
        // 	// These are the lumps that will be checked in IWAD,
        // 	// if any one is not present, execution will be aborted.
        // 	char name[23][8]=
        // 	{
        // 	    "e2m1","e2m2","e2m3","e2m4","e2m5","e2m6","e2m7","e2m8","e2m9",
        // 	    "e3m1","e3m3","e3m3","e3m4","e3m5","e3m6","e3m7","e3m8","e3m9",
        // 	    "dphoof","bfgga0","heada1","cybra1","spida1d1"
        // 	};
        // 	int i;
        //
        // 	if ( gamemode == shareware)
        // 	    I_Error("\nYou cannot -file with the shareware "
        // 		    "version. Register!");
        //
        // 	// Check for fake IWAD with right name,
        // 	// but w/o all the lumps of the registered version.
        // 	if (gamemode == registered)
        // 	    for (i = 0;i < 23; i++)
        // 		if (W_CheckNumForName(name[i])<0)
        // 		    I_Error("\nThis is not the registered version.");
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // Iff additonal PWAD files are used, print modified banner
        //     if (modifiedgame)
        //     {
        // 	/*m*/printf (
        // 	    "===========================================================================\n"
        // 	    "ATTENTION:  This version of DOOM has been modified.  If you would like to\n"
        // 	    "get a copy of the original game, call 1-800-IDGAMES or see the readme file.\n"
        // 	    "        You will not receive technical support for modified games.\n"
        // 	    "                      press enter to continue\n"
        // 	    "===========================================================================\n"
        // 	    );
        // 	getchar ();
        //     }
        todo!("if statement not yet translated");
        // TODO: switch statement not yet translated:
        //
        //
        //
        //     // Check and print which version is executed.
        //     switch ( gamemode )
        //     {
        //       case shareware:
        //       case indetermined:
        // 	printf (
        // 	    "===========================================================================\n"
        // 	    "                                Shareware!\n"
        // 	    "===========================================================================\n"
        // 	);
        // 	break;
        //       case registered:
        //       case retail:
        //       case commercial:
        // 	printf (
        // 	    "===========================================================================\n"
        // 	    "                 Commercial product - do not distribute!\n"
        // 	    "         Please report software piracy to the SPA: 1-800-388-PIR8\n"
        // 	    "===========================================================================\n"
        // 	);
        // 	break;
        //
        //       default:
        // 	// Ouch.
        // 	break;
        //     }
        todo!("switch statement not yet translated");
        printf((c"M_Init: Init miscellaneous info.\n").as_ptr());
        M_Init();
        printf((c"R_Init: Init DOOM refresh daemon - ").as_ptr());
        R_Init();
        printf((c"\nP_Init: Init Playloop state.\n").as_ptr());
        P_Init();
        printf((c"I_Init: Setting up machine state.\n").as_ptr());
        I_Init();
        printf((c"D_CheckNetGame: Checking network game status.\n").as_ptr());
        D_CheckNetGame();
        printf((c"S_Init: Setting up sound.\n").as_ptr());
        S_Init(snd_SfxVolume, snd_MusicVolume);
        printf((c"HU_Init: Setting up heads up display.\n").as_ptr());
        HU_Init();
        printf((c"ST_Init: Init status bar.\n").as_ptr());
        ST_Init();
        p = M_CheckParm((c"-statcopy").as_ptr());
        // TODO: if statement not yet translated:
        //
        //     if (p && p<myargc-1)
        //     {
        // 	// for statistics driver
        // 	extern  void*	statcopy;
        //
        // 	statcopy = (void*)atoi(myargv[p+1]);
        // 	printf ("External statistics registered.\n");
        //     }
        todo!("if statement not yet translated");
        p = M_CheckParm((c"-record").as_ptr());
        // TODO: if statement not yet translated:
        //
        //
        //     if (p && p < myargc-1)
        //     {
        // 	G_RecordDemo (myargv[p+1]);
        // 	autostart = true;
        //     }
        todo!("if statement not yet translated");
        p = M_CheckParm((c"-playdemo").as_ptr());
        // TODO: if statement not yet translated:
        //
        //     if (p && p < myargc-1)
        //     {
        // 	singledemo = true;              // quit after one demo
        // 	G_DeferedPlayDemo (myargv[p+1]);
        // 	D_DoomLoop ();  // never returns
        //     }
        todo!("if statement not yet translated");
        p = M_CheckParm((c"-timedemo").as_ptr());
        // TODO: if statement not yet translated:
        //
        //     if (p && p < myargc-1)
        //     {
        // 	G_TimeDemo (myargv[p+1]);
        // 	D_DoomLoop ();  // never returns
        //     }
        todo!("if statement not yet translated");
        p = M_CheckParm((c"-loadgame").as_ptr());
        // TODO: if statement not yet translated:
        //
        //     if (p && p < myargc-1)
        //     {
        // 	if (M_CheckParm("-cdrom"))
        // 	    sprintf(file, "c:\\doomdata\\"SAVEGAMENAME"%c.dsg",myargv[p+1][0]);
        // 	else
        // 	    sprintf(file, SAVEGAMENAME"%c.dsg",myargv[p+1][0]);
        // 	G_LoadGame (file);
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //
        //     if ( gameaction != ga_loadgame )
        //     {
        // 	if (autostart || netgame)
        // 	    G_InitNew (startskill, startepisode, startmap);
        // 	else
        // 	    D_StartTitle ();                // start up intro loop
        //
        //     }
        todo!("if statement not yet translated");
        D_DoomLoop();
    }
}
