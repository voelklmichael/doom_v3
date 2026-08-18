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
use crate::info::*;
use crate::m_fixed::*;
use crate::m_swap::*;
use crate::p_mobj::*;
use crate::p_pspr::*;
use crate::r_data::*;
use crate::r_defs::*;
use crate::r_state::*;
use crate::s_sound::*;
use crate::sounds::*;
use crate::tables::*;
use crate::v_video::*;
use crate::w_wad::*;
use crate::z_zone::*;

static mut rcsid: [std::ffi::c_char; 51] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        102 as std::ffi::c_char,
        95 as std::ffi::c_char,
        102 as std::ffi::c_char,
        105 as std::ffi::c_char,
        110 as std::ffi::c_char,
        97 as std::ffi::c_char,
        108 as std::ffi::c_char,
        101 as std::ffi::c_char,
        46 as std::ffi::c_char,
        99 as std::ffi::c_char,
        44 as std::ffi::c_char,
        118 as std::ffi::c_char,
        32 as std::ffi::c_char,
        49 as std::ffi::c_char,
        46 as std::ffi::c_char,
        53 as std::ffi::c_char,
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
        49 as std::ffi::c_char,
        58 as std::ffi::c_char,
        50 as std::ffi::c_char,
        54 as std::ffi::c_char,
        58 as std::ffi::c_char,
        51 as std::ffi::c_char,
        52 as std::ffi::c_char,
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

pub static mut finalestage: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut finalecount: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub const TEXTSPEED: std::ffi::c_int = 3;

pub const TEXTWAIT: std::ffi::c_int = 250;

pub static mut e1text: *mut std::ffi::c_char = unsafe { E1TEXT as *mut std::ffi::c_char };

pub static mut e2text: *mut std::ffi::c_char = unsafe { E2TEXT as *mut std::ffi::c_char };

pub static mut e3text: *mut std::ffi::c_char = unsafe { E3TEXT as *mut std::ffi::c_char };

pub static mut e4text: *mut std::ffi::c_char = unsafe { E4TEXT as *mut std::ffi::c_char };

pub static mut c1text: *mut std::ffi::c_char = unsafe { C1TEXT as *mut std::ffi::c_char };

pub static mut c2text: *mut std::ffi::c_char = unsafe { C2TEXT as *mut std::ffi::c_char };

pub static mut c3text: *mut std::ffi::c_char = unsafe { C3TEXT as *mut std::ffi::c_char };

pub static mut c4text: *mut std::ffi::c_char = unsafe { C4TEXT as *mut std::ffi::c_char };

pub static mut c5text: *mut std::ffi::c_char = unsafe { C5TEXT as *mut std::ffi::c_char };

pub static mut c6text: *mut std::ffi::c_char = unsafe { C6TEXT as *mut std::ffi::c_char };

pub static mut p1text: *mut std::ffi::c_char = unsafe { P1TEXT as *mut std::ffi::c_char };

pub static mut p2text: *mut std::ffi::c_char = unsafe { P2TEXT as *mut std::ffi::c_char };

pub static mut p3text: *mut std::ffi::c_char = unsafe { P3TEXT as *mut std::ffi::c_char };

pub static mut p4text: *mut std::ffi::c_char = unsafe { P4TEXT as *mut std::ffi::c_char };

pub static mut p5text: *mut std::ffi::c_char = unsafe { P5TEXT as *mut std::ffi::c_char };

pub static mut p6text: *mut std::ffi::c_char = unsafe { P6TEXT as *mut std::ffi::c_char };

pub static mut t1text: *mut std::ffi::c_char = unsafe { T1TEXT as *mut std::ffi::c_char };

pub static mut t2text: *mut std::ffi::c_char = unsafe { T2TEXT as *mut std::ffi::c_char };

pub static mut t3text: *mut std::ffi::c_char = unsafe { T3TEXT as *mut std::ffi::c_char };

pub static mut t4text: *mut std::ffi::c_char = unsafe { T4TEXT as *mut std::ffi::c_char };

pub static mut t5text: *mut std::ffi::c_char = unsafe { T5TEXT as *mut std::ffi::c_char };

pub static mut t6text: *mut std::ffi::c_char = unsafe { T6TEXT as *mut std::ffi::c_char };

pub static mut finaletext: *mut std::ffi::c_char = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut finaleflat: *mut std::ffi::c_char = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn F_StartFinale() {
    unsafe {
        gameaction = ga_nothing;
        gamestate = GS_FINALE;
        viewactive = false_;
        automapactive = false_;
        // TODO: switch statement not yet translated:
        //
        //
        //     // Okay - IWAD dependend stuff.
        //     // This has been changed severly, and
        //     //  some stuff might have changed in the process.
        //     switch ( gamemode )
        //     {
        //
        //       // DOOM 1 - E1, E3 or E4, but each nine missions
        //       case shareware:
        //       case registered:
        //       case retail:
        //       {
        // 	S_ChangeMusic(mus_victor, true);
        //
        // 	switch (gameepisode)
        // 	{
        // 	  case 1:
        // 	    finaleflat = "FLOOR4_8";
        // 	    finaletext = e1text;
        // 	    break;
        // 	  case 2:
        // 	    finaleflat = "SFLR6_1";
        // 	    finaletext = e2text;
        // 	    break;
        // 	  case 3:
        // 	    finaleflat = "MFLR8_4";
        // 	    finaletext = e3text;
        // 	    break;
        // 	  case 4:
        // 	    finaleflat = "MFLR8_3";
        // 	    finaletext = e4text;
        // 	    break;
        // 	  default:
        // 	    // Ouch.
        // 	    break;
        // 	}
        // 	break;
        //       }
        //
        //       // DOOM II and missions packs with E1, M34
        //       case commercial:
        //       {
        // 	  S_ChangeMusic(mus_read_m, true);
        //
        // 	  switch (gamemap)
        // 	  {
        // 	    case 6:
        // 	      finaleflat = "SLIME16";
        // 	      finaletext = c1text;
        // 	      break;
        // 	    case 11:
        // 	      finaleflat = "RROCK14";
        // 	      finaletext = c2text;
        // 	      break;
        // 	    case 20:
        // 	      finaleflat = "RROCK07";
        // 	      finaletext = c3text;
        // 	      break;
        // 	    case 30:
        // 	      finaleflat = "RROCK17";
        // 	      finaletext = c4text;
        // 	      break;
        // 	    case 15:
        // 	      finaleflat = "RROCK13";
        // 	      finaletext = c5text;
        // 	      break;
        // 	    case 31:
        // 	      finaleflat = "RROCK19";
        // 	      finaletext = c6text;
        // 	      break;
        // 	    default:
        // 	      // Ouch.
        // 	      break;
        // 	  }
        // 	  break;
        //       }
        //
        //
        //       // Indeterminate.
        //       default:
        // 	S_ChangeMusic(mus_read_m, true);
        // 	finaleflat = "F_SKY1"; // Not used anywhere else.
        // 	finaletext = c1text;  // FIXME - other text, music?
        // 	break;
        //     }
        todo!("switch statement not yet translated");
        finalestage = 0;
        finalecount = 0;
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn F_Responder(mut event: *mut event_t) -> boolean {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (finalestage == 2)
        // 	return F_CastResponder (event);
        todo!("if statement not yet translated");
        return false_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn F_Ticker() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     // check for skipping
        //     if ( (gamemode == commercial)
        //       && ( finalecount > 50) )
        //     {
        //       // go on to the next level
        //       for (i=0 ; i<MAXPLAYERS ; i++)
        // 	if (players[i].cmd.buttons)
        // 	  break;
        //
        //       if (i < MAXPLAYERS)
        //       {
        // 	if (gamemap == 30)
        // 	  F_StartCast ();
        // 	else
        // 	  gameaction = ga_worlddone;
        //       }
        //     }
        todo!("if statement not yet translated");
        {
            let __macro_tmp = finalecount;
            finalecount += 1;
            __macro_tmp
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (finalestage == 2)
        //     {
        // 	F_CastTicker ();
        // 	return;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if ( gamemode == commercial)
        // 	return;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (!finalestage && finalecount>strlen (finaletext)*TEXTSPEED + TEXTWAIT)
        //     {
        // 	finalecount = 0;
        // 	finalestage = 1;
        // 	wipegamestate = -1;		// force a wipe
        // 	if (gameepisode == 3)
        // 	    S_StartMusic (mus_bunny);
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

unsafe extern "C" {
    pub static mut hu_font: [*mut patch_t; (HU_FONTSIZE) as usize];
}

pub unsafe extern "C" fn F_TextWrite() {
    unsafe {
        let mut src: *mut byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut dest: *mut byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut x: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut y: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut w: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut count: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut ch: *mut std::ffi::c_char = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut c: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut cx: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut cy: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        src = W_CacheLumpName(finaleflat, PU_CACHE);
        dest = screens[(0) as usize];
        // TODO: for statement not yet translated:
        //
        //
        //     for (y=0 ; y<SCREENHEIGHT ; y++)
        //     {
        // 	for (x=0 ; x<SCREENWIDTH/64 ; x++)
        // 	{
        // 	    memcpy (dest, src+((y&63)<<6), 64);
        // 	    dest += 64;
        // 	}
        // 	if (SCREENWIDTH&63)
        // 	{
        // 	    memcpy (dest, src+((y&63)<<6), SCREENWIDTH&63);
        // 	    dest += (SCREENWIDTH&63);
        // 	}
        //     }
        todo!("for statement not yet translated");
        V_MarkRect(0, 0, SCREENWIDTH, SCREENHEIGHT);
        cx = 10;
        cy = 10;
        ch = finaletext;
        count = ((finalecount - 10) / TEXTSPEED);
        // TODO: if statement not yet translated:
        //
        //     if (count < 0)
        // 	count = 0;
        todo!("if statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //     for ( ; count ; count-- )
        //     {
        // 	c = *ch++;
        // 	if (!c)
        // 	    break;
        // 	if (c == '\n')
        // 	{
        // 	    cx = 10;
        // 	    cy += 11;
        // 	    continue;
        // 	}
        //
        // 	c = toupper(c) - HU_FONTSTART;
        // 	if (c < 0 || c> HU_FONTSIZE)
        // 	{
        // 	    cx += 4;
        // 	    continue;
        // 	}
        //
        // 	w = SHORT (hu_font[c]->width);
        // 	if (cx+w > SCREENWIDTH)
        // 	    break;
        // 	V_DrawPatch(cx, cy, 0, hu_font[c]);
        // 	cx+=w;
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct castinfo_t {
    pub name: *mut std::ffi::c_char,
    pub type_: mobjtype_t,
}

pub static mut castorder: [castinfo_t; 18] = unsafe {
    [
        castinfo_t {
            name: CC_ZOMBIE as *mut std::ffi::c_char,
            type_: MT_POSSESSED,
        },
        castinfo_t {
            name: CC_SHOTGUN as *mut std::ffi::c_char,
            type_: MT_SHOTGUY,
        },
        castinfo_t {
            name: CC_HEAVY as *mut std::ffi::c_char,
            type_: MT_CHAINGUY,
        },
        castinfo_t {
            name: CC_IMP as *mut std::ffi::c_char,
            type_: MT_TROOP,
        },
        castinfo_t {
            name: CC_DEMON as *mut std::ffi::c_char,
            type_: MT_SERGEANT,
        },
        castinfo_t {
            name: CC_LOST as *mut std::ffi::c_char,
            type_: MT_SKULL,
        },
        castinfo_t {
            name: CC_CACO as *mut std::ffi::c_char,
            type_: MT_HEAD,
        },
        castinfo_t {
            name: CC_HELL as *mut std::ffi::c_char,
            type_: MT_KNIGHT,
        },
        castinfo_t {
            name: CC_BARON as *mut std::ffi::c_char,
            type_: MT_BRUISER,
        },
        castinfo_t {
            name: CC_ARACH as *mut std::ffi::c_char,
            type_: MT_BABY,
        },
        castinfo_t {
            name: CC_PAIN as *mut std::ffi::c_char,
            type_: MT_PAIN,
        },
        castinfo_t {
            name: CC_REVEN as *mut std::ffi::c_char,
            type_: MT_UNDEAD,
        },
        castinfo_t {
            name: CC_MANCU as *mut std::ffi::c_char,
            type_: MT_FATSO,
        },
        castinfo_t {
            name: CC_ARCH as *mut std::ffi::c_char,
            type_: MT_VILE,
        },
        castinfo_t {
            name: CC_SPIDER as *mut std::ffi::c_char,
            type_: MT_SPIDER,
        },
        castinfo_t {
            name: CC_CYBER as *mut std::ffi::c_char,
            type_: MT_CYBORG,
        },
        castinfo_t {
            name: CC_HERO as *mut std::ffi::c_char,
            type_: MT_PLAYER,
        },
        castinfo_t {
            name: std::ptr::null_mut(),
            type_: 0,
        },
    ]
};

pub static mut castnum: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut casttics: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut caststate: *mut state_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut castdeath: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut castframes: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut castonmelee: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut castattacking: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

unsafe extern "C" {
    pub static mut wipegamestate: gamestate_t;
}

pub unsafe extern "C" fn F_StartCast() {
    unsafe {
        wipegamestate = (-(1));
        castnum = 0;
        caststate = (&(states
            [(mobjinfo[(castorder[(castnum) as usize].type_) as usize].seestate) as usize])
            as *const _ as *mut _);
        casttics = (*caststate).tics;
        castdeath = false_;
        finalestage = 2;
        castframes = 0;
        castonmelee = 0;
        castattacking = false_;
        S_ChangeMusic(mus_evil, true_);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn F_CastTicker() {
    unsafe {
        let mut st: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut sfx: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (--casttics > 0)
        // 	return;			// not time to change state yet
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (caststate->tics == -1 || caststate->nextstate == S_NULL)
        //     {
        // 	// switch from deathstate to next monster
        // 	castnum++;
        // 	castdeath = false;
        // 	if (castorder[castnum].name == NULL)
        // 	    castnum = 0;
        // 	if (mobjinfo[castorder[castnum].type].seesound)
        // 	    S_StartSound (NULL, mobjinfo[castorder[castnum].type].seesound);
        // 	caststate = &states[mobjinfo[castorder[castnum].type].seestate];
        // 	castframes = 0;
        //     }
        //     else
        //     {
        // 	// just advance to next state in animation
        // 	if (caststate == &states[S_PLAY_ATK1])
        // 	    goto stopattack;	// Oh, gross hack!
        // 	st = caststate->nextstate;
        // 	caststate = &states[st];
        // 	castframes++;
        //
        // 	// sound hacks....
        // 	switch (st)
        // 	{
        // 	  case S_PLAY_ATK1:	sfx = sfx_dshtgn; break;
        // 	  case S_POSS_ATK2:	sfx = sfx_pistol; break;
        // 	  case S_SPOS_ATK2:	sfx = sfx_shotgn; break;
        // 	  case S_VILE_ATK2:	sfx = sfx_vilatk; break;
        // 	  case S_SKEL_FIST2:	sfx = sfx_skeswg; break;
        // 	  case S_SKEL_FIST4:	sfx = sfx_skepch; break;
        // 	  case S_SKEL_MISS2:	sfx = sfx_skeatk; break;
        // 	  case S_FATT_ATK8:
        // 	  case S_FATT_ATK5:
        // 	  case S_FATT_ATK2:	sfx = sfx_firsht; break;
        // 	  case S_CPOS_ATK2:
        // 	  case S_CPOS_ATK3:
        // 	  case S_CPOS_ATK4:	sfx = sfx_shotgn; break;
        // 	  case S_TROO_ATK3:	sfx = sfx_claw; break;
        // 	  case S_SARG_ATK2:	sfx = sfx_sgtatk; break;
        // 	  case S_BOSS_ATK2:
        // 	  case S_BOS2_ATK2:
        // 	  case S_HEAD_ATK2:	sfx = sfx_firsht; break;
        // 	  case S_SKULL_ATK2:	sfx = sfx_sklatk; break;
        // 	  case S_SPID_ATK2:
        // 	  case S_SPID_ATK3:	sfx = sfx_shotgn; break;
        // 	  case S_BSPI_ATK2:	sfx = sfx_plasma; break;
        // 	  case S_CYBER_ATK2:
        // 	  case S_CYBER_ATK4:
        // 	  case S_CYBER_ATK6:	sfx = sfx_rlaunc; break;
        // 	  case S_PAIN_ATK3:	sfx = sfx_sklatk; break;
        // 	  default: sfx = 0; break;
        // 	}
        //
        // 	if (sfx)
        // 	    S_StartSound (NULL, sfx);
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (castframes == 12)
        //     {
        // 	// go into attack frame
        // 	castattacking = true;
        // 	if (castonmelee)
        // 	    caststate=&states[mobjinfo[castorder[castnum].type].meleestate];
        // 	else
        // 	    caststate=&states[mobjinfo[castorder[castnum].type].missilestate];
        // 	castonmelee ^= 1;
        // 	if (caststate == &states[S_NULL])
        // 	{
        // 	    if (castonmelee)
        // 		caststate=
        // 		    &states[mobjinfo[castorder[castnum].type].meleestate];
        // 	    else
        // 		caststate=
        // 		    &states[mobjinfo[castorder[castnum].type].missilestate];
        // 	}
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (castattacking)
        //     {
        // 	if (castframes == 24
        // 	    ||	caststate == &states[mobjinfo[castorder[castnum].type].seestate] )
        // 	{
        // 	  stopattack:
        // 	    castattacking = false;
        // 	    castframes = 0;
        // 	    caststate = &states[mobjinfo[castorder[castnum].type].seestate];
        // 	}
        //     }
        todo!("if statement not yet translated");
        casttics = (*caststate).tics;
        // TODO: if statement not yet translated:
        //
        //     if (casttics == -1)
        // 	casttics = 15;
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn F_CastResponder(mut ev: *mut event_t) -> boolean {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (ev->type != ev_keydown)
        // 	return false;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (castdeath)
        // 	return true;			// already in dying frames
        todo!("if statement not yet translated");
        castdeath = true_;
        caststate = (&(states
            [(mobjinfo[(castorder[(castnum) as usize].type_) as usize].deathstate) as usize])
            as *const _ as *mut _);
        casttics = (*caststate).tics;
        castframes = 0;
        castattacking = false_;
        // TODO: if statement not yet translated:
        //
        //     if (mobjinfo[castorder[castnum].type].deathsound)
        // 	S_StartSound (NULL, mobjinfo[castorder[castnum].type].deathsound);
        todo!("if statement not yet translated");
        return true_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn F_CastPrint(mut text: *mut std::ffi::c_char) {
    unsafe {
        let mut ch: *mut std::ffi::c_char = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut c: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut cx: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut w: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut width: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        ch = text;
        width = 0;
        // TODO: while statement not yet translated:
        //
        //
        //     while (ch)
        //     {
        // 	c = *ch++;
        // 	if (!c)
        // 	    break;
        // 	c = toupper(c) - HU_FONTSTART;
        // 	if (c < 0 || c> HU_FONTSIZE)
        // 	{
        // 	    width += 4;
        // 	    continue;
        // 	}
        //
        // 	w = SHORT (hu_font[c]->width);
        // 	width += w;
        //     }
        todo!("while statement not yet translated");
        cx = (160 - (width / 2));
        ch = text;
        // TODO: while statement not yet translated:
        //
        //     while (ch)
        //     {
        // 	c = *ch++;
        // 	if (!c)
        // 	    break;
        // 	c = toupper(c) - HU_FONTSTART;
        // 	if (c < 0 || c> HU_FONTSIZE)
        // 	{
        // 	    cx += 4;
        // 	    continue;
        // 	}
        //
        // 	w = SHORT (hu_font[c]->width);
        // 	V_DrawPatch(cx, 180, 0, hu_font[c]);
        // 	cx+=w;
        //     }
        todo!("while statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

unsafe extern "C" {
    pub fn V_DrawPatchFlipped(
        x: std::ffi::c_int,
        y: std::ffi::c_int,
        scrn: std::ffi::c_int,
        patch: *mut patch_t,
    );
}

pub unsafe extern "C" fn F_CastDrawer() {
    unsafe {
        let mut sprdef: *mut spritedef_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut sprframe: *mut spriteframe_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut lump: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut flip: boolean = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut patch: *mut patch_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        V_DrawPatch(0, 0, 0, W_CacheLumpName((c"BOSSBACK").as_ptr(), PU_CACHE));
        F_CastPrint(castorder[(castnum) as usize].name);
        sprdef = (&(sprites[((*caststate).sprite) as usize]) as *const _ as *mut _);
        sprframe = (&((*sprdef).spriteframes[((*caststate).frame & FF_FRAMEMASK) as usize])
            as *const _ as *mut _);
        lump = (*sprframe).lump[(0) as usize];
        flip = (((*sprframe).flip[(0) as usize]) as boolean);
        patch = W_CacheLumpNum((lump + firstspritelump), PU_CACHE);
        // TODO: if statement not yet translated:
        //
        //     if (flip)
        // 	V_DrawPatchFlipped (160,170,0,patch);
        //     else
        // 	V_DrawPatch (160,170,0,patch);
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn F_DrawPatchCol(
    mut x: std::ffi::c_int,
    mut patch: *mut patch_t,
    mut col: std::ffi::c_int,
) {
    unsafe {
        let mut column: *mut column_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut source: *mut byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut dest: *mut byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut desttop: *mut byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut count: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        column =
            ((((patch) as *mut byte) + LONG((*patch).columnofs[(col) as usize])) as *mut column_t);
        desttop = (screens[(0) as usize] + x);
        // TODO: while statement not yet translated:
        //
        //
        //     // step through the posts in a column
        //     while (column->topdelta != 0xff )
        //     {
        // 	source = (byte *)column + 3;
        // 	dest = desttop + column->topdelta*SCREENWIDTH;
        // 	count = column->length;
        //
        // 	while (count--)
        // 	{
        // 	    *dest = *source++;
        // 	    dest += SCREENWIDTH;
        // 	}
        // 	column = (column_t *)(  (byte *)column + column->length + 4 );
        //     }
        todo!("while statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn F_BunnyScroll() {
    unsafe {
        let mut scrolled: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut x: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut p1: *mut patch_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut p2: *mut patch_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut name: [std::ffi::c_char; (10) as usize] = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut stage: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        static mut laststage: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        p1 = W_CacheLumpName((c"PFUB2").as_ptr(), PU_LEVEL);
        p2 = W_CacheLumpName((c"PFUB1").as_ptr(), PU_LEVEL);
        V_MarkRect(0, 0, SCREENWIDTH, SCREENHEIGHT);
        scrolled = (320 - ((finalecount - 230) / 2));
        // TODO: if statement not yet translated:
        //
        //     if (scrolled > 320)
        // 	scrolled = 320;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (scrolled < 0)
        // 	scrolled = 0;
        todo!("if statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //
        //     for ( x=0 ; x<SCREENWIDTH ; x++)
        //     {
        // 	if (x+scrolled < 320)
        // 	    F_DrawPatchCol (x, p1, x+scrolled);
        // 	else
        // 	    F_DrawPatchCol (x, p2, x+scrolled - 320);
        //     }
        todo!("for statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (finalecount < 1130)
        // 	return;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (finalecount < 1180)
        //     {
        // 	V_DrawPatch ((SCREENWIDTH-13*8)/2,
        // 		     (SCREENHEIGHT-8*8)/2,0, W_CacheLumpName ("END0",PU_CACHE));
        // 	laststage = 0;
        // 	return;
        //     }
        todo!("if statement not yet translated");
        stage = ((finalecount - 1180) / 5);
        // TODO: if statement not yet translated:
        //
        //     if (stage > 6)
        // 	stage = 6;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (stage > laststage)
        //     {
        // 	S_StartSound (NULL, sfx_pistol);
        // 	laststage = stage;
        //     }
        todo!("if statement not yet translated");
        sprintf(name, (c"END%i").as_ptr(), stage);
        V_DrawPatch(
            ((SCREENWIDTH - (13 * 8)) / 2),
            ((SCREENHEIGHT - (8 * 8)) / 2),
            0,
            W_CacheLumpName(name, PU_CACHE),
        );
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn F_Drawer() {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (finalestage == 2)
        //     {
        // 	F_CastDrawer ();
        // 	return;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (!finalestage)
        // 	F_TextWrite ();
        //     else
        //     {
        // 	switch (gameepisode)
        // 	{
        // 	  case 1:
        // 	    if ( gamemode == retail )
        // 	      V_DrawPatch (0,0,0,
        // 			 W_CacheLumpName("CREDIT",PU_CACHE));
        // 	    else
        // 	      V_DrawPatch (0,0,0,
        // 			 W_CacheLumpName("HELP2",PU_CACHE));
        // 	    break;
        // 	  case 2:
        // 	    V_DrawPatch(0,0,0,
        // 			W_CacheLumpName("VICTORY2",PU_CACHE));
        // 	    break;
        // 	  case 3:
        // 	    F_BunnyScroll ();
        // 	    break;
        // 	  case 4:
        // 	    V_DrawPatch (0,0,0,
        // 			 W_CacheLumpName("ENDPIC",PU_CACHE));
        // 	    break;
        // 	}
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}
