use crate::d_items::*;
use crate::d_player::*;
use crate::d_think::*;
use crate::d_ticcmd::*;
use crate::doomdata::*;
use crate::doomdef::*;
use crate::doomtype::*;
use crate::info::*;
use crate::m_fixed::*;
use crate::m_random::*;
use crate::p_local::*;
use crate::p_mobj::*;
use crate::p_pspr::*;
use crate::p_spec::*;
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
use crate::tables::*;
use crate::z_zone::*;

static mut rcsid: [std::ffi::c_char; 51] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        112 as std::ffi::c_char,
        95 as std::ffi::c_char,
        108 as std::ffi::c_char,
        105 as std::ffi::c_char,
        103 as std::ffi::c_char,
        104 as std::ffi::c_char,
        116 as std::ffi::c_char,
        115 as std::ffi::c_char,
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
        50 as std::ffi::c_char,
        58 as std::ffi::c_char,
        52 as std::ffi::c_char,
        53 as std::ffi::c_char,
        58 as std::ffi::c_char,
        49 as std::ffi::c_char,
        49 as std::ffi::c_char,
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

pub unsafe extern "C" fn T_FireFlicker(mut flick: *mut fireflicker_t) {
    unsafe {
        let mut amount: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (--flick->count)
        // 	return;
        todo!("if statement not yet translated");
        amount = ((P_Random() & 3) * 16);
        // TODO: if statement not yet translated:
        //
        //
        //     if (flick->sector->lightlevel - amount < flick->minlight)
        // 	flick->sector->lightlevel = flick->minlight;
        //     else
        // 	flick->sector->lightlevel = flick->maxlight - amount;
        todo!("if statement not yet translated");
        (*flick).count = 4;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_SpawnFireFlicker(mut sector: *mut sector_t) {
    unsafe {
        let mut flick: *mut fireflicker_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        (*sector).special = 0;
        flick = Z_Malloc(std::mem::size_of_val(&(*(flick))), PU_LEVSPEC, 0);
        P_AddThinker((&((*flick).thinker) as *const _ as *mut _));
        (*flick).thinker.function.acp1 = ((T_FireFlicker) as actionf_p1);
        (*flick).sector = sector;
        (*flick).maxlight = (*sector).lightlevel;
        (*flick).minlight = (P_FindMinSurroundingLight(sector, (*sector).lightlevel) + 16);
        (*flick).count = 4;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn T_LightFlash(mut flash: *mut lightflash_t) {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (--flash->count)
        // 	return;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (flash->sector->lightlevel == flash->maxlight)
        //     {
        // 	flash-> sector->lightlevel = flash->minlight;
        // 	flash->count = (P_Random()&flash->mintime)+1;
        //     }
        //     else
        //     {
        // 	flash-> sector->lightlevel = flash->maxlight;
        // 	flash->count = (P_Random()&flash->maxtime)+1;
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_SpawnLightFlash(mut sector: *mut sector_t) {
    unsafe {
        let mut flash: *mut lightflash_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        (*sector).special = 0;
        flash = Z_Malloc(std::mem::size_of_val(&(*(flash))), PU_LEVSPEC, 0);
        P_AddThinker((&((*flash).thinker) as *const _ as *mut _));
        (*flash).thinker.function.acp1 = ((T_LightFlash) as actionf_p1);
        (*flash).sector = sector;
        (*flash).maxlight = (*sector).lightlevel;
        (*flash).minlight = P_FindMinSurroundingLight(sector, (*sector).lightlevel);
        (*flash).maxtime = 64;
        (*flash).mintime = 7;
        (*flash).count = ((P_Random() & (*flash).maxtime) + 1);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn T_StrobeFlash(mut flash: *mut strobe_t) {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (--flash->count)
        // 	return;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (flash->sector->lightlevel == flash->minlight)
        //     {
        // 	flash-> sector->lightlevel = flash->maxlight;
        // 	flash->count = flash->brighttime;
        //     }
        //     else
        //     {
        // 	flash-> sector->lightlevel = flash->minlight;
        // 	flash->count =flash->darktime;
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_SpawnStrobeFlash(
    mut sector: *mut sector_t,
    mut fastOrSlow: std::ffi::c_int,
    mut inSync: std::ffi::c_int,
) {
    unsafe {
        let mut flash: *mut strobe_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        flash = Z_Malloc(std::mem::size_of_val(&(*(flash))), PU_LEVSPEC, 0);
        P_AddThinker((&((*flash).thinker) as *const _ as *mut _));
        (*flash).sector = sector;
        (*flash).darktime = fastOrSlow;
        (*flash).brighttime = STROBEBRIGHT;
        (*flash).thinker.function.acp1 = ((T_StrobeFlash) as actionf_p1);
        (*flash).maxlight = (*sector).lightlevel;
        (*flash).minlight = P_FindMinSurroundingLight(sector, (*sector).lightlevel);
        // TODO: if statement not yet translated:
        //
        //
        //     if (flash->minlight == flash->maxlight)
        // 	flash->minlight = 0;
        todo!("if statement not yet translated");
        (*sector).special = 0;
        // TODO: if statement not yet translated:
        //
        //
        //     if (!inSync)
        // 	flash->count = (P_Random()&7)+1;
        //     else
        // 	flash->count = 1;
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn EV_StartLightStrobing(mut line: *mut line_t) {
    unsafe {
        let mut secnum: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut sec: *mut sector_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        secnum = (-(1));
        // TODO: while statement not yet translated:
        //
        //     while ((secnum = P_FindSectorFromLineTag(line,secnum)) >= 0)
        //     {
        // 	sec = &sectors[secnum];
        // 	if (sec->specialdata)
        // 	    continue;
        //
        // 	P_SpawnStrobeFlash (sec,SLOWDARK, 0);
        //     }
        todo!("while statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn EV_TurnTagLightsOff(mut line: *mut line_t) {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut j: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut min: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut sector: *mut sector_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut tsec: *mut sector_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut templine: *mut line_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        sector = sectors;
        // TODO: for statement not yet translated:
        //
        //
        //     for (j = 0;j < numsectors; j++, sector++)
        //     {
        // 	if (sector->tag == line->tag)
        // 	{
        // 	    min = sector->lightlevel;
        // 	    for (i = 0;i < sector->linecount; i++)
        // 	    {
        // 		templine = sector->lines[i];
        // 		tsec = getNextSector(templine,sector);
        // 		if (!tsec)
        // 		    continue;
        // 		if (tsec->lightlevel < min)
        // 		    min = tsec->lightlevel;
        // 	    }
        // 	    sector->lightlevel = min;
        // 	}
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn EV_LightTurnOn(mut line: *mut line_t, mut bright: std::ffi::c_int) {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut j: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut sector: *mut sector_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut temp: *mut sector_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut templine: *mut line_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        sector = sectors;
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0;i<numsectors;i++, sector++)
        //     {
        // 	if (sector->tag == line->tag)
        // 	{
        // 	    // bright = 0 means to search
        // 	    // for highest light level
        // 	    // surrounding sector
        // 	    if (!bright)
        // 	    {
        // 		for (j = 0;j < sector->linecount; j++)
        // 		{
        // 		    templine = sector->lines[j];
        // 		    temp = getNextSector(templine,sector);
        //
        // 		    if (!temp)
        // 			continue;
        //
        // 		    if (temp->lightlevel > bright)
        // 			bright = temp->lightlevel;
        // 		}
        // 	    }
        // 	    sector-> lightlevel = bright;
        // 	}
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn T_Glow(mut g: *mut glow_t) {
    unsafe {
        // TODO: switch statement not yet translated:
        //
        //     switch(g->direction)
        //     {
        //       case -1:
        // 	// DOWN
        // 	g->sector->lightlevel -= GLOWSPEED;
        // 	if (g->sector->lightlevel <= g->minlight)
        // 	{
        // 	    g->sector->lightlevel += GLOWSPEED;
        // 	    g->direction = 1;
        // 	}
        // 	break;
        //
        //       case 1:
        // 	// UP
        // 	g->sector->lightlevel += GLOWSPEED;
        // 	if (g->sector->lightlevel >= g->maxlight)
        // 	{
        // 	    g->sector->lightlevel -= GLOWSPEED;
        // 	    g->direction = -1;
        // 	}
        // 	break;
        //     }
        todo!("switch statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_SpawnGlowingLight(mut sector: *mut sector_t) {
    unsafe {
        let mut g: *mut glow_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        g = Z_Malloc(std::mem::size_of_val(&(*(g))), PU_LEVSPEC, 0);
        P_AddThinker((&((*g).thinker) as *const _ as *mut _));
        (*g).sector = sector;
        (*g).minlight = P_FindMinSurroundingLight(sector, (*sector).lightlevel);
        (*g).maxlight = (*sector).lightlevel;
        (*g).thinker.function.acp1 = ((T_Glow) as actionf_p1);
        (*g).direction = (-(1));
        (*sector).special = 0;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}
