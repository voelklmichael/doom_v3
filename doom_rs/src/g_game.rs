use crate::am_map::*;
use crate::d_englsh::*;
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
use crate::dstrings::*;
use crate::f_finale::*;
use crate::hu_stuff::*;
use crate::i_system::*;
use crate::info::*;
use crate::m_argv::*;
use crate::m_fixed::*;
use crate::m_menu::*;
use crate::m_misc::*;
use crate::m_random::*;
use crate::p_local::*;
use crate::p_mobj::*;
use crate::p_pspr::*;
use crate::p_saveg::*;
use crate::p_setup::*;
use crate::p_spec::*;
use crate::p_tick::*;
use crate::r_bsp::*;
use crate::r_data::*;
use crate::r_defs::*;
use crate::r_draw::*;
use crate::r_local::*;
use crate::r_main::*;
use crate::r_plane::*;
use crate::r_segs::*;
use crate::r_sky::*;
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

unsafe extern "C" {
    pub fn G_PlayDemo(name: *mut std::ffi::c_char);
}

static mut rcsid: [std::ffi::c_char; 49] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        103 as std::ffi::c_char,
        95 as std::ffi::c_char,
        103 as std::ffi::c_char,
        97 as std::ffi::c_char,
        109 as std::ffi::c_char,
        101 as std::ffi::c_char,
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

pub const SAVEGAMESIZE: std::ffi::c_int = 0x2c000;

pub const SAVESTRINGSIZE: std::ffi::c_int = 24;

unsafe extern "C" {
    pub fn G_DoVictory();
}

pub static mut gameaction: gameaction_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut gamestate: gamestate_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut gameskill: skill_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut respawnmonsters: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut gameepisode: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut gamemap: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut paused: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut sendpause: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut sendsave: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut usergame: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut timingdemo: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut nodrawers: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut noblit: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut starttime: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut viewactive: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut deathmatch: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut netgame: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut playeringame: [boolean; (MAXPLAYERS) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut players: [player_t; (MAXPLAYERS) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut consoleplayer: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut displayplayer: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut gametic: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut levelstarttic: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut totalkills: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut totalitems: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut totalsecret: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut demoname: [std::ffi::c_char; (32) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut demorecording: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut demoplayback: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut netdemo: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut demobuffer: *mut byte = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut demo_p: *mut byte = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut demoend: *mut byte = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut singledemo: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut precache: boolean = unsafe { true_ };

pub static mut wminfo: wbstartstruct_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut consistancy: [[std::ffi::c_short; (BACKUPTICS) as usize]; (MAXPLAYERS) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut savebuffer: *mut byte = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut key_right: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut key_left: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut key_up: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut key_down: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut key_strafeleft: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut key_straferight: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut key_fire: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut key_use: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut key_strafe: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut key_speed: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut mousebfire: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut mousebstrafe: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut mousebforward: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut joybfire: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut joybstrafe: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut joybuse: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut joybspeed: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub const MAXPLMOVE: std::ffi::c_int = (forwardmove[(1) as usize]);

pub const TURBOTHRESHOLD: std::ffi::c_int = 0x32;

pub static mut forwardmove: [fixed_t; 2] = unsafe { [0x19, 0x32] };

pub static mut sidemove: [fixed_t; 2] = unsafe { [0x18, 0x28] };

pub static mut angleturn: [fixed_t; 3] = unsafe { [640, 1280, 320] };

pub const SLOWTURNTICS: std::ffi::c_int = 6;

pub const NUMKEYS: std::ffi::c_int = 256;

pub static mut gamekeydown: [boolean; (NUMKEYS) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut turnheld: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut mousearray: [boolean; (4) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut mousebuttons: *mut boolean =
    unsafe { (&(mousearray[(1) as usize]) as *const _ as *mut _) };

pub static mut mousex: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut mousey: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut dclicktime: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut dclickstate: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut dclicks: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut dclicktime2: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut dclickstate2: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut dclicks2: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut joyxmove: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut joyymove: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut joyarray: [boolean; (5) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut joybuttons: *mut boolean =
    unsafe { (&(joyarray[(1) as usize]) as *const _ as *mut _) };

pub static mut savegameslot: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut savedescription: [std::ffi::c_char; (32) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub const BODYQUESIZE: std::ffi::c_int = 32;

pub static mut bodyque: [*mut mobj_t; (BODYQUESIZE) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut bodyqueslot: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut statcopy: *mut std::ffi::c_void = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn G_CmdChecksum(mut cmd: *mut ticcmd_t) -> std::ffi::c_int {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut sum: std::ffi::c_int = unsafe { 0 };
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i< sizeof(*cmd)/4 - 1 ; i++)
        // 	sum += ((int *)cmd)[i];
        todo!("for statement not yet translated");
        return sum;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn G_BuildTiccmd(mut cmd: *mut ticcmd_t) {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut strafe: boolean = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut bstrafe: boolean = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut speed: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut tspeed: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut forward: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut side: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut base: *mut ticcmd_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        base = I_BaseTiccmd();
        memcpy(cmd, base, std::mem::size_of_val(&(*(cmd))));
        (*cmd).consistancy = consistancy[(consoleplayer) as usize][(maketic % BACKUPTICS) as usize];
        strafe = ((gamekeydown[(key_strafe) as usize] || mousebuttons[(mousebstrafe) as usize])
            || joybuttons[(joybstrafe) as usize]);
        speed = (gamekeydown[(key_speed) as usize] || joybuttons[(joybspeed) as usize]);
        forward = side = 0;
        // TODO: if statement not yet translated:
        //
        //
        //     // use two stage accelerative turning
        //     // on the keyboard and joystick
        //     if (joyxmove < 0
        // 	|| joyxmove > 0
        // 	|| gamekeydown[key_right]
        // 	|| gamekeydown[key_left])
        // 	turnheld += ticdup;
        //     else
        // 	turnheld = 0;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (turnheld < SLOWTURNTICS)
        // 	tspeed = 2;             // slow turn
        //     else
        // 	tspeed = speed;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // let movement keys cancel each other out
        //     if (strafe)
        //     {
        // 	if (gamekeydown[key_right])
        // 	{
        // 	    // fprintf(stderr, "strafe right\n");
        // 	    side += sidemove[speed];
        // 	}
        // 	if (gamekeydown[key_left])
        // 	{
        // 	    //	fprintf(stderr, "strafe left\n");
        // 	    side -= sidemove[speed];
        // 	}
        // 	if (joyxmove > 0)
        // 	    side += sidemove[speed];
        // 	if (joyxmove < 0)
        // 	    side -= sidemove[speed];
        //
        //     }
        //     else
        //     {
        // 	if (gamekeydown[key_right])
        // 	    cmd->angleturn -= angleturn[tspeed];
        // 	if (gamekeydown[key_left])
        // 	    cmd->angleturn += angleturn[tspeed];
        // 	if (joyxmove > 0)
        // 	    cmd->angleturn -= angleturn[tspeed];
        // 	if (joyxmove < 0)
        // 	    cmd->angleturn += angleturn[tspeed];
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (gamekeydown[key_up])
        //     {
        // 	// fprintf(stderr, "up\n");
        // 	forward += forwardmove[speed];
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (gamekeydown[key_down])
        //     {
        // 	// fprintf(stderr, "down\n");
        // 	forward -= forwardmove[speed];
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (joyymove < 0)
        // 	forward += forwardmove[speed];
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (joyymove > 0)
        // 	forward -= forwardmove[speed];
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (gamekeydown[key_straferight])
        // 	side += sidemove[speed];
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (gamekeydown[key_strafeleft])
        // 	side -= sidemove[speed];
        todo!("if statement not yet translated");
        (*cmd).chatchar = HU_dequeueChatChar();
        // TODO: if statement not yet translated:
        //
        //
        //     if (gamekeydown[key_fire] || mousebuttons[mousebfire]
        // 	|| joybuttons[joybfire])
        // 	cmd->buttons |= BT_ATTACK;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (gamekeydown[key_use] || joybuttons[joybuse] )
        //     {
        // 	cmd->buttons |= BT_USE;
        // 	// clear double clicks if hit use button
        // 	dclicks = 0;
        //     }
        todo!("if statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //
        //     // chainsaw overrides
        //     for (i=0 ; i<NUMWEAPONS-1 ; i++)
        // 	if (gamekeydown['1'+i])
        // 	{
        // 	    cmd->buttons |= BT_CHANGE;
        // 	    cmd->buttons |= i<<BT_WEAPONSHIFT;
        // 	    break;
        // 	}
        todo!("for statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // mouse
        //     if (mousebuttons[mousebforward])
        // 	forward += forwardmove[speed];
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // forward double click
        //     if (mousebuttons[mousebforward] != dclickstate && dclicktime > 1 )
        //     {
        // 	dclickstate = mousebuttons[mousebforward];
        // 	if (dclickstate)
        // 	    dclicks++;
        // 	if (dclicks == 2)
        // 	{
        // 	    cmd->buttons |= BT_USE;
        // 	    dclicks = 0;
        // 	}
        // 	else
        // 	    dclicktime = 0;
        //     }
        //     else
        //     {
        // 	dclicktime += ticdup;
        // 	if (dclicktime > 20)
        // 	{
        // 	    dclicks = 0;
        // 	    dclickstate = 0;
        // 	}
        //     }
        todo!("if statement not yet translated");
        bstrafe = (mousebuttons[(mousebstrafe) as usize] || joybuttons[(joybstrafe) as usize]);
        // TODO: if statement not yet translated:
        //
        //     if (bstrafe != dclickstate2 && dclicktime2 > 1 )
        //     {
        // 	dclickstate2 = bstrafe;
        // 	if (dclickstate2)
        // 	    dclicks2++;
        // 	if (dclicks2 == 2)
        // 	{
        // 	    cmd->buttons |= BT_USE;
        // 	    dclicks2 = 0;
        // 	}
        // 	else
        // 	    dclicktime2 = 0;
        //     }
        //     else
        //     {
        // 	dclicktime2 += ticdup;
        // 	if (dclicktime2 > 20)
        // 	{
        // 	    dclicks2 = 0;
        // 	    dclickstate2 = 0;
        // 	}
        //     }
        todo!("if statement not yet translated");
        forward += mousey;
        // TODO: if statement not yet translated:
        //
        //     if (strafe)
        // 	side += mousex*2;
        //     else
        // 	cmd->angleturn -= mousex*0x8;
        todo!("if statement not yet translated");
        mousex = mousey = 0;
        // TODO: if statement not yet translated:
        //
        //
        //     if (forward > MAXPLMOVE)
        // 	forward = MAXPLMOVE;
        //     else if (forward < -MAXPLMOVE)
        // 	forward = -MAXPLMOVE;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (side > MAXPLMOVE)
        // 	side = MAXPLMOVE;
        //     else if (side < -MAXPLMOVE)
        // 	side = -MAXPLMOVE;
        todo!("if statement not yet translated");
        (*cmd).forwardmove += forward;
        (*cmd).sidemove += side;
        // TODO: if statement not yet translated:
        //
        //
        //     // special buttons
        //     if (sendpause)
        //     {
        // 	sendpause = false;
        // 	cmd->buttons = BT_SPECIAL | BTS_PAUSE;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (sendsave)
        //     {
        // 	sendsave = false;
        // 	cmd->buttons = BT_SPECIAL | BTS_SAVEGAME | (savegameslot<<BTS_SAVESHIFT);
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

unsafe extern "C" {
    pub static mut wipegamestate: gamestate_t;
}

pub unsafe extern "C" fn G_DoLoadLevel() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        skyflatnum = R_FlatNumForName(SKYFLATNAME);
        // TODO: if statement not yet translated:
        //
        //
        //     // DOOM determines the sky texture to be used
        //     // depending on the current episode, and the game version.
        //     if ( (gamemode == commercial)
        // 	 || ( gamemode == pack_tnt )
        // 	 || ( gamemode == pack_plut ) )
        //     {
        // 	skytexture = R_TextureNumForName ("SKY3");
        // 	if (gamemap < 12)
        // 	    skytexture = R_TextureNumForName ("SKY1");
        // 	else
        // 	    if (gamemap < 21)
        // 		skytexture = R_TextureNumForName ("SKY2");
        //     }
        todo!("if statement not yet translated");
        levelstarttic = gametic;
        // TODO: if statement not yet translated:
        //
        //     if (wipegamestate == GS_LEVEL)
        // 	wipegamestate = -1;             // force a wipe
        todo!("if statement not yet translated");
        gamestate = GS_LEVEL;
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<MAXPLAYERS ; i++)
        //     {
        // 	if (playeringame[i] && players[i].playerstate == PST_DEAD)
        // 	    players[i].playerstate = PST_REBORN;
        // 	memset (players[i].frags,0,sizeof(players[i].frags));
        //     }
        todo!("for statement not yet translated");
        P_SetupLevel(gameepisode, gamemap, 0, gameskill);
        displayplayer = consoleplayer;
        starttime = I_GetTime();
        gameaction = ga_nothing;
        Z_CheckHeap();
        memset(gamekeydown, 0, std::mem::size_of_val(&(gamekeydown)));
        joyxmove = joyymove = 0;
        mousex = mousey = 0;
        sendpause = sendsave = paused = false_;
        memset(mousebuttons, 0, std::mem::size_of_val(&(mousebuttons)));
        memset(joybuttons, 0, std::mem::size_of_val(&(joybuttons)));
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn G_Responder(mut ev: *mut event_t) -> boolean {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     // allow spy mode changes even during the demo
        //     if (gamestate == GS_LEVEL && ev->type == ev_keydown
        // 	&& ev->data1 == KEY_F12 && (singledemo || !deathmatch) )
        //     {
        // 	// spy mode
        // 	do
        // 	{
        // 	    displayplayer++;
        // 	    if (displayplayer == MAXPLAYERS)
        // 		displayplayer = 0;
        // 	} while (!playeringame[displayplayer] && displayplayer != consoleplayer);
        // 	return true;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // any other key pops up menu if in demos
        //     if (gameaction == ga_nothing && !singledemo &&
        // 	(demoplayback || gamestate == GS_DEMOSCREEN)
        // 	)
        //     {
        // 	if (ev->type == ev_keydown ||
        // 	    (ev->type == ev_mouse && ev->data1) ||
        // 	    (ev->type == ev_joystick && ev->data1) )
        // 	{
        // 	    M_StartControlPanel ();
        // 	    return true;
        // 	}
        // 	return false;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (gamestate == GS_LEVEL)
        //     {
        // #if 0
        // 	if (devparm && ev->type == ev_keydown && ev->data1 == ';')
        // 	{
        // 	    G_DeathMatchSpawnPlayer (0);
        // 	    return true;
        // 	}
        // #endif
        // 	if (HU_Responder (ev))
        // 	    return true;	// chat ate the event
        // 	if (ST_Responder (ev))
        // 	    return true;	// status window ate it
        // 	if (AM_Responder (ev))
        // 	    return true;	// automap ate it
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (gamestate == GS_FINALE)
        //     {
        // 	if (F_Responder (ev))
        // 	    return true;	// finale ate the event
        //     }
        todo!("if statement not yet translated");
        // TODO: switch statement not yet translated:
        //
        //
        //     switch (ev->type)
        //     {
        //       case ev_keydown:
        // 	if (ev->data1 == KEY_PAUSE)
        // 	{
        // 	    sendpause = true;
        // 	    return true;
        // 	}
        // 	if (ev->data1 <NUMKEYS)
        // 	    gamekeydown[ev->data1] = true;
        // 	return true;    // eat key down events
        //
        //       case ev_keyup:
        // 	if (ev->data1 <NUMKEYS)
        // 	    gamekeydown[ev->data1] = false;
        // 	return false;   // always let key up events filter down
        //
        //       case ev_mouse:
        // 	mousebuttons[0] = ev->data1 & 1;
        // 	mousebuttons[1] = ev->data1 & 2;
        // 	mousebuttons[2] = ev->data1 & 4;
        // 	mousex = ev->data2*(mouseSensitivity+5)/10;
        // 	mousey = ev->data3*(mouseSensitivity+5)/10;
        // 	return true;    // eat events
        //
        //       case ev_joystick:
        // 	joybuttons[0] = ev->data1 & 1;
        // 	joybuttons[1] = ev->data1 & 2;
        // 	joybuttons[2] = ev->data1 & 4;
        // 	joybuttons[3] = ev->data1 & 8;
        // 	joyxmove = ev->data2;
        // 	joyymove = ev->data3;
        // 	return true;    // eat events
        //
        //       default:
        // 	break;
        //     }
        todo!("switch statement not yet translated");
        return false_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn G_Ticker() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut buf: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut cmd: *mut ticcmd_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: for statement not yet translated:
        //
        //
        //     // do player reborns if needed
        //     for (i=0 ; i<MAXPLAYERS ; i++)
        // 	if (playeringame[i] && players[i].playerstate == PST_REBORN)
        // 	    G_DoReborn (i);
        todo!("for statement not yet translated");
        // TODO: while statement not yet translated:
        //
        //
        //     // do things to change the game state
        //     while (gameaction != ga_nothing)
        //     {
        // 	switch (gameaction)
        // 	{
        // 	  case ga_loadlevel:
        // 	    G_DoLoadLevel ();
        // 	    break;
        // 	  case ga_newgame:
        // 	    G_DoNewGame ();
        // 	    break;
        // 	  case ga_loadgame:
        // 	    G_DoLoadGame ();
        // 	    break;
        // 	  case ga_savegame:
        // 	    G_DoSaveGame ();
        // 	    break;
        // 	  case ga_playdemo:
        // 	    G_DoPlayDemo ();
        // 	    break;
        // 	  case ga_completed:
        // 	    G_DoCompleted ();
        // 	    break;
        // 	  case ga_victory:
        // 	    F_StartFinale ();
        // 	    break;
        // 	  case ga_worlddone:
        // 	    G_DoWorldDone ();
        // 	    break;
        // 	  case ga_screenshot:
        // 	    M_ScreenShot ();
        // 	    gameaction = ga_nothing;
        // 	    break;
        // 	  case ga_nothing:
        // 	    break;
        // 	}
        //     }
        todo!("while statement not yet translated");
        buf = ((gametic / ticdup) % BACKUPTICS);
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<MAXPLAYERS ; i++)
        //     {
        // 	if (playeringame[i])
        // 	{
        // 	    cmd = &players[i].cmd;
        //
        // 	    memcpy (cmd, &netcmds[i][buf], sizeof(ticcmd_t));
        //
        // 	    if (demoplayback)
        // 		G_ReadDemoTiccmd (cmd);
        // 	    if (demorecording)
        // 		G_WriteDemoTiccmd (cmd);
        //
        // 	    // check for turbo cheats
        // 	    if (cmd->forwardmove > TURBOTHRESHOLD
        // 		&& !(gametic&31) && ((gametic>>5)&3) == i )
        // 	    {
        // 		static char turbomessage[80];
        // 		extern char *player_names[4];
        // 		sprintf (turbomessage, "%s is turbo!",player_names[i]);
        // 		players[consoleplayer].message = turbomessage;
        // 	    }
        //
        // 	    if (netgame && !netdemo && !(gametic%ticdup) )
        // 	    {
        // 		if (gametic > BACKUPTICS
        // 		    && consistancy[i][buf] != cmd->consistancy)
        // 		{
        // 		    I_Error ("consistency failure (%i should be %i)",
        // 			     cmd->consistancy, consistancy[i][buf]);
        // 		}
        // 		if (players[i].mo)
        // 		    consistancy[i][buf] = players[i].mo->x;
        // 		else
        // 		    consistancy[i][buf] = rndindex;
        // 	    }
        // 	}
        //     }
        todo!("for statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //
        //     // check for special buttons
        //     for (i=0 ; i<MAXPLAYERS ; i++)
        //     {
        // 	if (playeringame[i])
        // 	{
        // 	    if (players[i].cmd.buttons & BT_SPECIAL)
        // 	    {
        // 		switch (players[i].cmd.buttons & BT_SPECIALMASK)
        // 		{
        // 		  case BTS_PAUSE:
        // 		    paused ^= 1;
        // 		    if (paused)
        // 			S_PauseSound ();
        // 		    else
        // 			S_ResumeSound ();
        // 		    break;
        //
        // 		  case BTS_SAVEGAME:
        // 		    if (!savedescription[0])
        // 			strcpy (savedescription, "NET GAME");
        // 		    savegameslot =
        // 			(players[i].cmd.buttons & BTS_SAVEMASK)>>BTS_SAVESHIFT;
        // 		    gameaction = ga_savegame;
        // 		    break;
        // 		}
        // 	    }
        // 	}
        //     }
        todo!("for statement not yet translated");
        // TODO: switch statement not yet translated:
        //
        //
        //     // do main actions
        //     switch (gamestate)
        //     {
        //       case GS_LEVEL:
        // 	P_Ticker ();
        // 	ST_Ticker ();
        // 	AM_Ticker ();
        // 	HU_Ticker ();
        // 	break;
        //
        //       case GS_INTERMISSION:
        // 	WI_Ticker ();
        // 	break;
        //
        //       case GS_FINALE:
        // 	F_Ticker ();
        // 	break;
        //
        //       case GS_DEMOSCREEN:
        // 	D_PageTicker ();
        // 	break;
        //     }
        todo!("switch statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn G_InitPlayer(mut player: std::ffi::c_int) {
    unsafe {
        let mut p: *mut player_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        p = (&(players[(player) as usize]) as *const _ as *mut _);
        G_PlayerReborn(player);
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn G_PlayerFinishLevel(mut player: std::ffi::c_int) {
    unsafe {
        let mut p: *mut player_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        p = (&(players[(player) as usize]) as *const _ as *mut _);
        memset((*p).powers, 0, std::mem::size_of_val(&((*p).powers)));
        memset((*p).cards, 0, std::mem::size_of_val(&((*p).cards)));
        (*(*p).mo).flags &= (!(MF_SHADOW));
        (*p).extralight = 0;
        (*p).fixedcolormap = 0;
        (*p).damagecount = 0;
        (*p).bonuscount = 0;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn G_PlayerReborn(mut player: std::ffi::c_int) {
    unsafe {
        let mut p: *mut player_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut frags: [std::ffi::c_int; (MAXPLAYERS) as usize] = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut killcount: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut itemcount: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut secretcount: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        memcpy(
            frags,
            players[(player) as usize].frags,
            std::mem::size_of_val(&(frags)),
        );
        killcount = players[(player) as usize].killcount;
        itemcount = players[(player) as usize].itemcount;
        secretcount = players[(player) as usize].secretcount;
        p = (&(players[(player) as usize]) as *const _ as *mut _);
        memset(p, 0, std::mem::size_of_val(&(*(p))));
        memcpy(
            players[(player) as usize].frags,
            frags,
            std::mem::size_of_val(&(players[(player) as usize].frags)),
        );
        players[(player) as usize].killcount = killcount;
        players[(player) as usize].itemcount = itemcount;
        players[(player) as usize].secretcount = secretcount;
        (*p).usedown = (*p).attackdown = true_;
        (*p).playerstate = PST_LIVE;
        (*p).health = MAXHEALTH;
        (*p).readyweapon = (*p).pendingweapon = wp_pistol;
        (*p).weaponowned[(wp_fist) as usize] = true_;
        (*p).weaponowned[(wp_pistol) as usize] = true_;
        (*p).ammo[(am_clip) as usize] = 50;
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<NUMAMMO ; i++)
        // 	p->maxammo[i] = maxammo[i];
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

unsafe extern "C" {
    pub fn P_SpawnPlayer(mthing: *mut mapthing_t);
}

pub unsafe extern "C" fn G_CheckSpot(
    mut playernum: std::ffi::c_int,
    mut mthing: *mut mapthing_t,
) -> boolean {
    unsafe {
        let mut x: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut y: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut ss: *mut subsector_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut an: std::ffi::c_uint = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut mo: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (!players[playernum].mo)
        //     {
        // 	// first spawn of level, before corpses
        // 	for (i=0 ; i<playernum ; i++)
        // 	    if (players[i].mo->x == mthing->x << FRACBITS
        // 		&& players[i].mo->y == mthing->y << FRACBITS)
        // 		return false;
        // 	return true;
        //     }
        todo!("if statement not yet translated");
        x = ((*mthing).x << FRACBITS);
        y = ((*mthing).y << FRACBITS);
        // TODO: if statement not yet translated:
        //
        //
        //     if (!P_CheckPosition (players[playernum].mo, x, y) )
        // 	return false;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // flush an old corpse if needed
        //     if (bodyqueslot >= BODYQUESIZE)
        // 	P_RemoveMobj (bodyque[bodyqueslot%BODYQUESIZE]);
        todo!("if statement not yet translated");
        bodyque[(bodyqueslot % BODYQUESIZE) as usize] = players[(playernum) as usize].mo;
        {
            let __macro_tmp = bodyqueslot;
            bodyqueslot += 1;
            __macro_tmp
        };
        ss = R_PointInSubsector(x, y);
        an = ((ANG45 * ((*mthing).angle / 45)) >> ANGLETOFINESHIFT);
        mo = P_SpawnMobj(
            (x + (20 * finecosine[(an) as usize])),
            (y + (20 * finesine[(an) as usize])),
            (*(*ss).sector).floorheight,
            MT_TFOG,
        );
        // TODO: if statement not yet translated:
        //
        //
        //     if (players[consoleplayer].viewz != 1)
        // 	S_StartSound (mo, sfx_telept);	// don't start sound on first frame
        todo!("if statement not yet translated");
        return true_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn G_DeathMatchSpawnPlayer(mut playernum: std::ffi::c_int) {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut j: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut selections: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        selections = (deathmatch_p - deathmatchstarts);
        // TODO: if statement not yet translated:
        //
        //     if (selections < 4)
        // 	I_Error ("Only %i deathmatch spots, 4 required", selections);
        todo!("if statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //
        //     for (j=0 ; j<20 ; j++)
        //     {
        // 	i = P_Random() % selections;
        // 	if (G_CheckSpot (playernum, &deathmatchstarts[i]) )
        // 	{
        // 	    deathmatchstarts[i].type = playernum+1;
        // 	    P_SpawnPlayer (&deathmatchstarts[i]);
        // 	    return;
        // 	}
        //     }
        todo!("for statement not yet translated");
        P_SpawnPlayer((&(playerstarts[(playernum) as usize]) as *const _ as *mut _));
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn G_DoReborn(mut playernum: std::ffi::c_int) {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (!netgame)
        //     {
        // 	// reload the level from scratch
        // 	gameaction = ga_loadlevel;
        //     }
        //     else
        //     {
        // 	// respawn at the start
        //
        // 	// first dissasociate the corpse
        // 	players[playernum].mo->player = NULL;
        //
        // 	// spawn at random spot if in death match
        // 	if (deathmatch)
        // 	{
        // 	    G_DeathMatchSpawnPlayer (playernum);
        // 	    return;
        // 	}
        //
        // 	if (G_CheckSpot (playernum, &playerstarts[playernum]) )
        // 	{
        // 	    P_SpawnPlayer (&playerstarts[playernum]);
        // 	    return;
        // 	}
        //
        // 	// try to spawn at one of the other players spots
        // 	for (i=0 ; i<MAXPLAYERS ; i++)
        // 	{
        // 	    if (G_CheckSpot (playernum, &playerstarts[i]) )
        // 	    {
        // 		playerstarts[i].type = playernum+1;	// fake as other player
        // 		P_SpawnPlayer (&playerstarts[i]);
        // 		playerstarts[i].type = i+1;		// restore
        // 		return;
        // 	    }
        // 	    // he's going to be inside something.  Too bad.
        // 	}
        // 	P_SpawnPlayer (&playerstarts[playernum]);
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn G_ScreenShot() {
    unsafe {
        gameaction = ga_screenshot;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub static mut pars: [[std::ffi::c_int; 10]; 4] = unsafe {
    [
        [
            0,
            std::mem::zeroed(),
            std::mem::zeroed(),
            std::mem::zeroed(),
            std::mem::zeroed(),
            std::mem::zeroed(),
            std::mem::zeroed(),
            std::mem::zeroed(),
            std::mem::zeroed(),
            std::mem::zeroed(),
        ],
        [0, 30, 75, 120, 90, 165, 180, 180, 30, 165],
        [0, 90, 90, 90, 120, 90, 360, 240, 30, 170],
        [0, 90, 45, 90, 150, 90, 90, 165, 30, 135],
    ]
};

pub static mut cpars: [std::ffi::c_int; 32] = unsafe {
    [
        30, 90, 120, 120, 90, 150, 120, 120, 270, 90, 210, 150, 150, 150, 210, 150, 420, 150, 210,
        150, 240, 150, 180, 150, 150, 300, 330, 420, 300, 180, 120, 30,
    ]
};

pub static mut secretexit: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

unsafe extern "C" {
    pub static mut pagename: *mut std::ffi::c_char;
}

pub unsafe extern "C" fn G_ExitLevel() {
    unsafe {
        secretexit = false_;
        gameaction = ga_completed;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn G_SecretExitLevel() {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     // IF NO WOLF3D LEVELS, NO SECRET EXIT!
        //     if ( (gamemode == commercial)
        //       && (W_CheckNumForName("map31")<0))
        // 	secretexit = false;
        //     else
        // 	secretexit = true;
        todo!("if statement not yet translated");
        gameaction = ga_completed;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn G_DoCompleted() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        gameaction = ga_nothing;
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<MAXPLAYERS ; i++)
        // 	if (playeringame[i])
        // 	    G_PlayerFinishLevel (i);        // take away cards and stuff
        todo!("for statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (automapactive)
        // 	AM_Stop ();
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if ( gamemode != commercial)
        // 	switch(gamemap)
        // 	{
        // 	  case 8:
        // 	    gameaction = ga_victory;
        // 	    return;
        // 	  case 9:
        // 	    for (i=0 ; i<MAXPLAYERS ; i++)
        // 		players[i].didsecret = true;
        // 	    break;
        // 	}
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        // //#if 0  Hmmm - why?
        //     if ( (gamemap == 8)
        // 	 && (gamemode != commercial) )
        //     {
        // 	// victory
        // 	gameaction = ga_victory;
        // 	return;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if ( (gamemap == 9)
        // 	 && (gamemode != commercial) )
        //     {
        // 	// exit secret level
        // 	for (i=0 ; i<MAXPLAYERS ; i++)
        // 	    players[i].didsecret = true;
        //     }
        todo!("if statement not yet translated");
        wminfo.didsecret = players[(consoleplayer) as usize].didsecret;
        wminfo.epsd = (gameepisode - 1);
        wminfo.last = (gamemap - 1);
        // TODO: if statement not yet translated:
        //
        //
        //     // wminfo.next is 0 biased, unlike gamemap
        //     if ( gamemode == commercial)
        //     {
        // 	if (secretexit)
        // 	    switch(gamemap)
        // 	    {
        // 	      case 15: wminfo.next = 30; break;
        // 	      case 31: wminfo.next = 31; break;
        // 	    }
        // 	else
        // 	    switch(gamemap)
        // 	    {
        // 	      case 31:
        // 	      case 32: wminfo.next = 15; break;
        // 	      default: wminfo.next = gamemap;
        // 	    }
        //     }
        //     else
        //     {
        // 	if (secretexit)
        // 	    wminfo.next = 8; 	// go to secret level
        // 	else if (gamemap == 9)
        // 	{
        // 	    // returning from secret level
        // 	    switch (gameepisode)
        // 	    {
        // 	      case 1:
        // 		wminfo.next = 3;
        // 		break;
        // 	      case 2:
        // 		wminfo.next = 5;
        // 		break;
        // 	      case 3:
        // 		wminfo.next = 6;
        // 		break;
        // 	      case 4:
        // 		wminfo.next = 2;
        // 		break;
        // 	    }
        // 	}
        // 	else
        // 	    wminfo.next = gamemap;          // go to next level
        //     }
        todo!("if statement not yet translated");
        wminfo.maxkills = totalkills;
        wminfo.maxitems = totalitems;
        wminfo.maxsecret = totalsecret;
        wminfo.maxfrags = 0;
        // TODO: if statement not yet translated:
        //
        //     if ( gamemode == commercial )
        // 	wminfo.partime = 35*cpars[gamemap-1];
        //     else
        // 	wminfo.partime = 35*pars[gameepisode][gamemap];
        todo!("if statement not yet translated");
        wminfo.pnum = consoleplayer;
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<MAXPLAYERS ; i++)
        //     {
        // 	wminfo.plyr[i].in = playeringame[i];
        // 	wminfo.plyr[i].skills = players[i].killcount;
        // 	wminfo.plyr[i].sitems = players[i].itemcount;
        // 	wminfo.plyr[i].ssecret = players[i].secretcount;
        // 	wminfo.plyr[i].stime = leveltime;
        // 	memcpy (wminfo.plyr[i].frags, players[i].frags
        // 		, sizeof(wminfo.plyr[i].frags));
        //     }
        todo!("for statement not yet translated");
        gamestate = GS_INTERMISSION;
        viewactive = false_;
        automapactive = false_;
        // TODO: if statement not yet translated:
        //
        //
        //     if (statcopy)
        // 	memcpy (statcopy, &wminfo, sizeof(wminfo));
        todo!("if statement not yet translated");
        WI_Start((&(wminfo) as *const wbstartstruct_t as *mut wbstartstruct_t));
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn G_WorldDone() {
    unsafe {
        gameaction = ga_worlddone;
        // TODO: if statement not yet translated:
        //
        //
        //     if (secretexit)
        // 	players[consoleplayer].didsecret = true;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if ( gamemode == commercial )
        //     {
        // 	switch (gamemap)
        // 	{
        // 	  case 15:
        // 	  case 31:
        // 	    if (!secretexit)
        // 		break;
        // 	  case 6:
        // 	  case 11:
        // 	  case 20:
        // 	  case 30:
        // 	    F_StartFinale ();
        // 	    break;
        // 	}
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn G_DoWorldDone() {
    unsafe {
        gamestate = GS_LEVEL;
        gamemap = (wminfo.next + 1);
        G_DoLoadLevel();
        gameaction = ga_nothing;
        viewactive = true_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

unsafe extern "C" {
    pub static mut setsizeneeded: boolean;
}

unsafe extern "C" {
    pub fn R_ExecuteSetViewSize();
}

pub static mut savename: [std::ffi::c_char; (256) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn G_LoadGame(mut name: *mut std::ffi::c_char) {
    unsafe {
        strcpy(savename, name);
        gameaction = ga_loadgame;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub const VERSIONSIZE: std::ffi::c_int = 16;

pub unsafe extern "C" fn G_DoLoadGame() {
    unsafe {
        let mut length: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut a: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut b: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut c: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut vcheck: [std::ffi::c_char; (VERSIONSIZE) as usize] = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        gameaction = ga_nothing;
        length = M_ReadFile(
            savename,
            (&(savebuffer) as *const *mut byte as *mut *mut byte),
        );
        save_p = (savebuffer + SAVESTRINGSIZE);
        memset(vcheck, 0, std::mem::size_of_val(&(vcheck)));
        sprintf(vcheck, (c"version %i").as_ptr(), VERSION);
        // TODO: if statement not yet translated:
        //
        //     if (strcmp (save_p, vcheck))
        // 	return;				// bad version
        todo!("if statement not yet translated");
        save_p += VERSIONSIZE;
        gameskill = (*({
            let __macro_tmp = save_p;
            save_p += 1;
            __macro_tmp
        }));
        gameepisode = (*({
            let __macro_tmp = save_p;
            save_p += 1;
            __macro_tmp
        }));
        gamemap = (*({
            let __macro_tmp = save_p;
            save_p += 1;
            __macro_tmp
        }));
        // TODO: for statement not yet translated:
        //
        //     for (i=0 ; i<MAXPLAYERS ; i++)
        // 	playeringame[i] = *save_p++;
        todo!("for statement not yet translated");
        G_InitNew(gameskill, gameepisode, gamemap);
        a = (*({
            let __macro_tmp = save_p;
            save_p += 1;
            __macro_tmp
        }));
        b = (*({
            let __macro_tmp = save_p;
            save_p += 1;
            __macro_tmp
        }));
        c = (*({
            let __macro_tmp = save_p;
            save_p += 1;
            __macro_tmp
        }));
        leveltime = (((a << 16) + (b << 8)) + c);
        P_UnArchivePlayers();
        P_UnArchiveWorld();
        P_UnArchiveThinkers();
        P_UnArchiveSpecials();
        // TODO: if statement not yet translated:
        //
        //
        //     if (*save_p != 0x1d)
        // 	I_Error ("Bad savegame");
        todo!("if statement not yet translated");
        Z_Free(savebuffer);
        // TODO: if statement not yet translated:
        //
        //
        //     if (setsizeneeded)
        // 	R_ExecuteSetViewSize ();
        todo!("if statement not yet translated");
        R_FillBackScreen();
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn G_SaveGame(
    mut slot: std::ffi::c_int,
    mut description: *mut std::ffi::c_char,
) {
    unsafe {
        savegameslot = slot;
        strcpy(savedescription, description);
        sendsave = true_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn G_DoSaveGame() {
    unsafe {
        let mut name: [std::ffi::c_char; (100) as usize] = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut name2: [std::ffi::c_char; (VERSIONSIZE) as usize] = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut description: *mut std::ffi::c_char = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut length: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (M_CheckParm("-cdrom"))
        // 	sprintf(name,"c:\\doomdata\\"SAVEGAMENAME"%d.dsg",savegameslot);
        //     else
        // 	sprintf (name,SAVEGAMENAME"%d.dsg",savegameslot);
        todo!("if statement not yet translated");
        description = savedescription;
        save_p = savebuffer = (screens[(1) as usize] + 0x4000);
        memcpy(save_p, description, SAVESTRINGSIZE);
        save_p += SAVESTRINGSIZE;
        memset(name2, 0, std::mem::size_of_val(&(name2)));
        sprintf(name2, (c"version %i").as_ptr(), VERSION);
        memcpy(save_p, name2, VERSIONSIZE);
        save_p += VERSIONSIZE;
        (*({
            let __macro_tmp = save_p;
            save_p += 1;
            __macro_tmp
        })) = gameskill;
        (*({
            let __macro_tmp = save_p;
            save_p += 1;
            __macro_tmp
        })) = gameepisode;
        (*({
            let __macro_tmp = save_p;
            save_p += 1;
            __macro_tmp
        })) = gamemap;
        // TODO: for statement not yet translated:
        //
        //     for (i=0 ; i<MAXPLAYERS ; i++)
        // 	*save_p++ = playeringame[i];
        todo!("for statement not yet translated");
        (*({
            let __macro_tmp = save_p;
            save_p += 1;
            __macro_tmp
        })) = (leveltime >> 16);
        (*({
            let __macro_tmp = save_p;
            save_p += 1;
            __macro_tmp
        })) = (leveltime >> 8);
        (*({
            let __macro_tmp = save_p;
            save_p += 1;
            __macro_tmp
        })) = leveltime;
        P_ArchivePlayers();
        P_ArchiveWorld();
        P_ArchiveThinkers();
        P_ArchiveSpecials();
        (*({
            let __macro_tmp = save_p;
            save_p += 1;
            __macro_tmp
        })) = 0x1d;
        length = (save_p - savebuffer);
        // TODO: if statement not yet translated:
        //
        //     if (length > SAVEGAMESIZE)
        // 	I_Error ("Savegame buffer overrun");
        todo!("if statement not yet translated");
        M_WriteFile(name, savebuffer, length);
        gameaction = ga_nothing;
        savedescription[(0) as usize] = 0;
        players[(consoleplayer) as usize].message = GGSAVED;
        R_FillBackScreen();
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub static mut d_skill: skill_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut d_episode: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut d_map: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn G_DeferedInitNew(
    mut skill: skill_t,
    mut episode: std::ffi::c_int,
    mut map: std::ffi::c_int,
) {
    unsafe {
        d_skill = skill;
        d_episode = episode;
        d_map = map;
        gameaction = ga_newgame;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn G_DoNewGame() {
    unsafe {
        demoplayback = false_;
        netdemo = false_;
        netgame = false_;
        deathmatch = false_;
        playeringame[(1) as usize] = playeringame[(2) as usize] = playeringame[(3) as usize] = 0;
        respawnparm = false_;
        fastparm = false_;
        nomonsters = false_;
        consoleplayer = 0;
        G_InitNew(d_skill, d_episode, d_map);
        gameaction = ga_nothing;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

unsafe extern "C" {
    pub static mut skytexture: std::ffi::c_int;
}

pub unsafe extern "C" fn G_InitNew(
    mut skill: skill_t,
    mut episode: std::ffi::c_int,
    mut map: std::ffi::c_int,
) {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (paused)
        //     {
        // 	paused = false;
        // 	S_ResumeSound ();
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //
        //     if (skill > sk_nightmare)
        // 	skill = sk_nightmare;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //
        //     // This was quite messy with SPECIAL and commented parts.
        //     // Supposedly hacks to make the latest edition work.
        //     // It might not work properly.
        //     if (episode < 1)
        //       episode = 1;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if ( gamemode == retail )
        //     {
        //       if (episode > 4)
        // 	episode = 4;
        //     }
        //     else if ( gamemode == shareware )
        //     {
        //       if (episode > 1)
        // 	   episode = 1;	// only start episode 1 on shareware
        //     }
        //     else
        //     {
        //       if (episode > 3)
        // 	episode = 3;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //
        //
        //     if (map < 1)
        // 	map = 1;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if ( (map > 9)
        // 	 && ( gamemode != commercial) )
        //       map = 9;
        todo!("if statement not yet translated");
        M_ClearRandom();
        // TODO: if statement not yet translated:
        //
        //
        //     if (skill == sk_nightmare || respawnparm )
        // 	respawnmonsters = true;
        //     else
        // 	respawnmonsters = false;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (fastparm || (skill == sk_nightmare && gameskill != sk_nightmare) )
        //     {
        // 	for (i=S_SARG_RUN1 ; i<=S_SARG_PAIN2 ; i++)
        // 	    states[i].tics >>= 1;
        // 	mobjinfo[MT_BRUISERSHOT].speed = 20*FRACUNIT;
        // 	mobjinfo[MT_HEADSHOT].speed = 20*FRACUNIT;
        // 	mobjinfo[MT_TROOPSHOT].speed = 20*FRACUNIT;
        //     }
        //     else if (skill != sk_nightmare && gameskill == sk_nightmare)
        //     {
        // 	for (i=S_SARG_RUN1 ; i<=S_SARG_PAIN2 ; i++)
        // 	    states[i].tics <<= 1;
        // 	mobjinfo[MT_BRUISERSHOT].speed = 15*FRACUNIT;
        // 	mobjinfo[MT_HEADSHOT].speed = 10*FRACUNIT;
        // 	mobjinfo[MT_TROOPSHOT].speed = 10*FRACUNIT;
        //     }
        todo!("if statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //
        //
        //     // force players to be initialized upon first level load
        //     for (i=0 ; i<MAXPLAYERS ; i++)
        // 	players[i].playerstate = PST_REBORN;
        todo!("for statement not yet translated");
        usergame = true_;
        paused = false_;
        demoplayback = false_;
        automapactive = false_;
        viewactive = true_;
        gameepisode = episode;
        gamemap = map;
        gameskill = skill;
        viewactive = true_;
        // TODO: if statement not yet translated:
        //
        //
        //     // set the sky map for the episode
        //     if ( gamemode == commercial)
        //     {
        // 	skytexture = R_TextureNumForName ("SKY3");
        // 	if (gamemap < 12)
        // 	    skytexture = R_TextureNumForName ("SKY1");
        // 	else
        // 	    if (gamemap < 21)
        // 		skytexture = R_TextureNumForName ("SKY2");
        //     }
        //     else
        // 	switch (episode)
        // 	{
        // 	  case 1:
        // 	    skytexture = R_TextureNumForName ("SKY1");
        // 	    break;
        // 	  case 2:
        // 	    skytexture = R_TextureNumForName ("SKY2");
        // 	    break;
        // 	  case 3:
        // 	    skytexture = R_TextureNumForName ("SKY3");
        // 	    break;
        // 	  case 4:	// Special Edition sky
        // 	    skytexture = R_TextureNumForName ("SKY4");
        // 	    break;
        // 	}
        todo!("if statement not yet translated");
        G_DoLoadLevel();
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub const DEMOMARKER: std::ffi::c_int = 0x80;

pub unsafe extern "C" fn G_ReadDemoTiccmd(mut cmd: *mut ticcmd_t) {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (*demo_p == DEMOMARKER)
        //     {
        // 	// end of demo data stream
        // 	G_CheckDemoStatus ();
        // 	return;
        //     }
        todo!("if statement not yet translated");
        (*cmd).forwardmove = ((*({
            let __macro_tmp = demo_p;
            demo_p += 1;
            __macro_tmp
        })) as std::ffi::c_schar);
        (*cmd).sidemove = ((*({
            let __macro_tmp = demo_p;
            demo_p += 1;
            __macro_tmp
        })) as std::ffi::c_schar);
        (*cmd).angleturn = (((*({
            let __macro_tmp = demo_p;
            demo_p += 1;
            __macro_tmp
        })) as std::ffi::c_uchar)
            << 8);
        (*cmd).buttons = ((*({
            let __macro_tmp = demo_p;
            demo_p += 1;
            __macro_tmp
        })) as std::ffi::c_uchar);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn G_WriteDemoTiccmd(mut cmd: *mut ticcmd_t) {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (gamekeydown['q'])           // press q to end demo recording
        // 	G_CheckDemoStatus ();
        todo!("if statement not yet translated");
        (*({
            let __macro_tmp = demo_p;
            demo_p += 1;
            __macro_tmp
        })) = (*cmd).forwardmove;
        (*({
            let __macro_tmp = demo_p;
            demo_p += 1;
            __macro_tmp
        })) = (*cmd).sidemove;
        (*({
            let __macro_tmp = demo_p;
            demo_p += 1;
            __macro_tmp
        })) = (((*cmd).angleturn + 128) >> 8);
        (*({
            let __macro_tmp = demo_p;
            demo_p += 1;
            __macro_tmp
        })) = (*cmd).buttons;
        demo_p -= 4;
        // TODO: if statement not yet translated:
        //
        //     if (demo_p > demoend - 16)
        //     {
        // 	// no more space
        // 	G_CheckDemoStatus ();
        // 	return;
        //     }
        todo!("if statement not yet translated");
        G_ReadDemoTiccmd(cmd);
    }
}

pub unsafe extern "C" fn G_RecordDemo(mut name: *mut std::ffi::c_char) {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut maxsize: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        usergame = false_;
        strcpy(demoname, name);
        strcat(demoname, (c".lmp").as_ptr());
        maxsize = 0x20000;
        i = M_CheckParm((c"-maxdemo").as_ptr());
        // TODO: if statement not yet translated:
        //
        //     if (i && i<myargc-1)
        // 	maxsize = atoi(myargv[i+1])*1024;
        todo!("if statement not yet translated");
        demobuffer = Z_Malloc(maxsize, PU_STATIC, NULL);
        demoend = (demobuffer + maxsize);
        demorecording = true_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn G_BeginRecording() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        demo_p = demobuffer;
        (*({
            let __macro_tmp = demo_p;
            demo_p += 1;
            __macro_tmp
        })) = VERSION;
        (*({
            let __macro_tmp = demo_p;
            demo_p += 1;
            __macro_tmp
        })) = gameskill;
        (*({
            let __macro_tmp = demo_p;
            demo_p += 1;
            __macro_tmp
        })) = gameepisode;
        (*({
            let __macro_tmp = demo_p;
            demo_p += 1;
            __macro_tmp
        })) = gamemap;
        (*({
            let __macro_tmp = demo_p;
            demo_p += 1;
            __macro_tmp
        })) = deathmatch;
        (*({
            let __macro_tmp = demo_p;
            demo_p += 1;
            __macro_tmp
        })) = respawnparm;
        (*({
            let __macro_tmp = demo_p;
            demo_p += 1;
            __macro_tmp
        })) = fastparm;
        (*({
            let __macro_tmp = demo_p;
            demo_p += 1;
            __macro_tmp
        })) = nomonsters;
        (*({
            let __macro_tmp = demo_p;
            demo_p += 1;
            __macro_tmp
        })) = consoleplayer;
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<MAXPLAYERS ; i++)
        // 	*demo_p++ = playeringame[i];
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub static mut defdemoname: *mut std::ffi::c_char = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn G_DeferedPlayDemo(mut name: *mut std::ffi::c_char) {
    unsafe {
        defdemoname = name;
        gameaction = ga_playdemo;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn G_DoPlayDemo() {
    unsafe {
        let mut skill: skill_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut episode: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut map: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        gameaction = ga_nothing;
        demobuffer = demo_p = W_CacheLumpName(defdemoname, PU_STATIC);
        // TODO: if statement not yet translated:
        //
        //     if ( *demo_p++ != VERSION)
        //     {
        //       fprintf( stderr, "Demo is from a different game version!\n");
        //       gameaction = ga_nothing;
        //       return;
        //     }
        todo!("if statement not yet translated");
        skill = (*({
            let __macro_tmp = demo_p;
            demo_p += 1;
            __macro_tmp
        }));
        episode = (*({
            let __macro_tmp = demo_p;
            demo_p += 1;
            __macro_tmp
        }));
        map = (*({
            let __macro_tmp = demo_p;
            demo_p += 1;
            __macro_tmp
        }));
        deathmatch = (*({
            let __macro_tmp = demo_p;
            demo_p += 1;
            __macro_tmp
        }));
        respawnparm = (*({
            let __macro_tmp = demo_p;
            demo_p += 1;
            __macro_tmp
        }));
        fastparm = (*({
            let __macro_tmp = demo_p;
            demo_p += 1;
            __macro_tmp
        }));
        nomonsters = (*({
            let __macro_tmp = demo_p;
            demo_p += 1;
            __macro_tmp
        }));
        consoleplayer = (*({
            let __macro_tmp = demo_p;
            demo_p += 1;
            __macro_tmp
        }));
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<MAXPLAYERS ; i++)
        // 	playeringame[i] = *demo_p++;
        todo!("for statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (playeringame[1])
        //     {
        // 	netgame = true;
        // 	netdemo = true;
        //     }
        todo!("if statement not yet translated");
        precache = false_;
        G_InitNew(skill, episode, map);
        precache = true_;
        usergame = false_;
        demoplayback = true_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn G_TimeDemo(mut name: *mut std::ffi::c_char) {
    unsafe {
        nodrawers = M_CheckParm((c"-nodraw").as_ptr());
        noblit = M_CheckParm((c"-noblit").as_ptr());
        timingdemo = true_;
        singletics = true_;
        defdemoname = name;
        gameaction = ga_playdemo;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn G_CheckDemoStatus() -> boolean {
    unsafe {
        let mut endtime: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (timingdemo)
        //     {
        // 	endtime = I_GetTime ();
        // 	I_Error ("timed %i gametics in %i realtics",gametic
        // 		 , endtime-starttime);
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (demoplayback)
        //     {
        // 	if (singledemo)
        // 	    I_Quit ();
        //
        // 	Z_ChangeTag (demobuffer, PU_CACHE);
        // 	demoplayback = false;
        // 	netdemo = false;
        // 	netgame = false;
        // 	deathmatch = false;
        // 	playeringame[1] = playeringame[2] = playeringame[3] = 0;
        // 	respawnparm = false;
        // 	fastparm = false;
        // 	nomonsters = false;
        // 	consoleplayer = 0;
        // 	D_AdvanceDemo ();
        // 	return true;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (demorecording)
        //     {
        // 	*demo_p++ = DEMOMARKER;
        // 	M_WriteFile (demoname, demobuffer, demo_p - demobuffer);
        // 	Z_Free (demobuffer);
        // 	demorecording = false;
        // 	I_Error ("Demo %s recorded",demoname);
        //     }
        todo!("if statement not yet translated");
        return false_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}
