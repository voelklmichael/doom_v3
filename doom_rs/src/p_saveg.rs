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
use crate::i_system::*;
use crate::info::*;
use crate::m_fixed::*;
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

static mut rcsid: [std::ffi::c_char; 49] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        112 as std::ffi::c_char,
        95 as std::ffi::c_char,
        116 as std::ffi::c_char,
        105 as std::ffi::c_char,
        99 as std::ffi::c_char,
        107 as std::ffi::c_char,
        46 as std::ffi::c_char,
        99 as std::ffi::c_char,
        44 as std::ffi::c_char,
        118 as std::ffi::c_char,
        32 as std::ffi::c_char,
        49 as std::ffi::c_char,
        46 as std::ffi::c_char,
        52 as std::ffi::c_char,
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
        49 as std::ffi::c_char,
        54 as std::ffi::c_char,
        58 as std::ffi::c_char,
        52 as std::ffi::c_char,
        55 as std::ffi::c_char,
        58 as std::ffi::c_char,
        53 as std::ffi::c_char,
        53 as std::ffi::c_char,
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

pub static mut save_p: *mut byte = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn PADSAVEP() -> std::ffi::c_int {
    save_p += ((4 - (((save_p) as std::ffi::c_int) & 3)) & 3)
}

pub unsafe extern "C" fn P_ArchivePlayers() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut j: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut dest: *mut player_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<MAXPLAYERS ; i++)
        //     {
        // 	if (!playeringame[i])
        // 	    continue;
        //
        // 	PADSAVEP();
        //
        // 	dest = (player_t *)save_p;
        // 	memcpy (dest,&players[i],sizeof(player_t));
        // 	save_p += sizeof(player_t);
        // 	for (j=0 ; j<NUMPSPRITES ; j++)
        // 	{
        // 	    if (dest->psprites[j].state)
        // 	    {
        // 		dest->psprites[j].state
        // 		    = (state_t *)(dest->psprites[j].state-states);
        // 	    }
        // 	}
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_UnArchivePlayers() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut j: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<MAXPLAYERS ; i++)
        //     {
        // 	if (!playeringame[i])
        // 	    continue;
        //
        // 	PADSAVEP();
        //
        // 	memcpy (&players[i],save_p, sizeof(player_t));
        // 	save_p += sizeof(player_t);
        //
        // 	// will be set when unarc thinker
        // 	players[i].mo = NULL;
        // 	players[i].message = NULL;
        // 	players[i].attacker = NULL;
        //
        // 	for (j=0 ; j<NUMPSPRITES ; j++)
        // 	{
        // 	    if (players[i]. psprites[j].state)
        // 	    {
        // 		players[i]. psprites[j].state
        // 		    = &states[ (int)players[i].psprites[j].state ];
        // 	    }
        // 	}
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_ArchiveWorld() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut j: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut sec: *mut sector_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut li: *mut line_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut si: *mut side_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut put: *mut std::ffi::c_short = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        put = ((save_p) as *mut std::ffi::c_short);
        // TODO: for statement not yet translated:
        //
        //
        //     // do sectors
        //     for (i=0, sec = sectors ; i<numsectors ; i++,sec++)
        //     {
        // 	*put++ = sec->floorheight >> FRACBITS;
        // 	*put++ = sec->ceilingheight >> FRACBITS;
        // 	*put++ = sec->floorpic;
        // 	*put++ = sec->ceilingpic;
        // 	*put++ = sec->lightlevel;
        // 	*put++ = sec->special;		// needed?
        // 	*put++ = sec->tag;		// needed?
        //     }
        todo!("for statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //
        //
        //     // do lines
        //     for (i=0, li = lines ; i<numlines ; i++,li++)
        //     {
        // 	*put++ = li->flags;
        // 	*put++ = li->special;
        // 	*put++ = li->tag;
        // 	for (j=0 ; j<2 ; j++)
        // 	{
        // 	    if (li->sidenum[j] == -1)
        // 		continue;
        //
        // 	    si = &sides[li->sidenum[j]];
        //
        // 	    *put++ = si->textureoffset >> FRACBITS;
        // 	    *put++ = si->rowoffset >> FRACBITS;
        // 	    *put++ = si->toptexture;
        // 	    *put++ = si->bottomtexture;
        // 	    *put++ = si->midtexture;
        // 	}
        //     }
        todo!("for statement not yet translated");
        save_p = ((put) as *mut byte);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_UnArchiveWorld() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut j: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut sec: *mut sector_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut li: *mut line_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut si: *mut side_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut get: *mut std::ffi::c_short = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        get = ((save_p) as *mut std::ffi::c_short);
        // TODO: for statement not yet translated:
        //
        //
        //     // do sectors
        //     for (i=0, sec = sectors ; i<numsectors ; i++,sec++)
        //     {
        // 	sec->floorheight = *get++ << FRACBITS;
        // 	sec->ceilingheight = *get++ << FRACBITS;
        // 	sec->floorpic = *get++;
        // 	sec->ceilingpic = *get++;
        // 	sec->lightlevel = *get++;
        // 	sec->special = *get++;		// needed?
        // 	sec->tag = *get++;		// needed?
        // 	sec->specialdata = 0;
        // 	sec->soundtarget = 0;
        //     }
        todo!("for statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //
        //     // do lines
        //     for (i=0, li = lines ; i<numlines ; i++,li++)
        //     {
        // 	li->flags = *get++;
        // 	li->special = *get++;
        // 	li->tag = *get++;
        // 	for (j=0 ; j<2 ; j++)
        // 	{
        // 	    if (li->sidenum[j] == -1)
        // 		continue;
        // 	    si = &sides[li->sidenum[j]];
        // 	    si->textureoffset = *get++ << FRACBITS;
        // 	    si->rowoffset = *get++ << FRACBITS;
        // 	    si->toptexture = *get++;
        // 	    si->bottomtexture = *get++;
        // 	    si->midtexture = *get++;
        // 	}
        //     }
        todo!("for statement not yet translated");
        save_p = ((get) as *mut byte);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub const tc_end: std::ffi::c_int = 0;
pub const tc_mobj: std::ffi::c_int = tc_end + 1;

pub type thinkerclass_t = std::ffi::c_int;

pub unsafe extern "C" fn P_ArchiveThinkers() {
    unsafe {
        let mut th: *mut thinker_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut mobj: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: for statement not yet translated:
        //
        //
        //     // save off the current thinkers
        //     for (th = thinkercap.next ; th != &thinkercap ; th=th->next)
        //     {
        // 	if (th->function.acp1 == (actionf_p1)P_MobjThinker)
        // 	{
        // 	    *save_p++ = tc_mobj;
        // 	    PADSAVEP();
        // 	    mobj = (mobj_t *)save_p;
        // 	    memcpy (mobj, th, sizeof(*mobj));
        // 	    save_p += sizeof(*mobj);
        // 	    mobj->state = (state_t *)(mobj->state - states);
        //
        // 	    if (mobj->player)
        // 		mobj->player = (player_t *)((mobj->player-players) + 1);
        // 	    continue;
        // 	}
        //
        // 	// I_Error ("P_ArchiveThinkers: Unknown thinker function");
        //     }
        todo!("for statement not yet translated");
        (*({
            let __macro_tmp = save_p;
            save_p += 1;
            __macro_tmp
        })) = tc_end;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_UnArchiveThinkers() {
    unsafe {
        let mut tclass: byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut currentthinker: *mut thinker_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut next: *mut thinker_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut mobj: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        currentthinker = thinkercap.next;
        // TODO: while statement not yet translated:
        //
        //     while (currentthinker != &thinkercap)
        //     {
        // 	next = currentthinker->next;
        //
        // 	if (currentthinker->function.acp1 == (actionf_p1)P_MobjThinker)
        // 	    P_RemoveMobj ((mobj_t *)currentthinker);
        // 	else
        // 	    Z_Free (currentthinker);
        //
        // 	currentthinker = next;
        //     }
        todo!("while statement not yet translated");
        P_InitThinkers();
        // TODO: while statement not yet translated:
        //
        //
        //     // read in saved thinkers
        //     while (1)
        //     {
        // 	tclass = *save_p++;
        // 	switch (tclass)
        // 	{
        // 	  case tc_end:
        // 	    return; 	// end of list
        //
        // 	  case tc_mobj:
        // 	    PADSAVEP();
        // 	    mobj = Z_Malloc (sizeof(*mobj), PU_LEVEL, NULL);
        // 	    memcpy (mobj, save_p, sizeof(*mobj));
        // 	    save_p += sizeof(*mobj);
        // 	    mobj->state = &states[(int)mobj->state];
        // 	    mobj->target = NULL;
        // 	    if (mobj->player)
        // 	    {
        // 		mobj->player = &players[(int)mobj->player-1];
        // 		mobj->player->mo = mobj;
        // 	    }
        // 	    P_SetThingPosition (mobj);
        // 	    mobj->info = &mobjinfo[mobj->type];
        // 	    mobj->floorz = mobj->subsector->sector->floorheight;
        // 	    mobj->ceilingz = mobj->subsector->sector->ceilingheight;
        // 	    mobj->thinker.function.acp1 = (actionf_p1)P_MobjThinker;
        // 	    P_AddThinker (&mobj->thinker);
        // 	    break;
        //
        // 	  default:
        // 	    I_Error ("Unknown tclass %i in savegame",tclass);
        // 	}
        //
        //     }
        todo!("while statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub const tc_ceiling: std::ffi::c_int = 0;
pub const tc_door: std::ffi::c_int = tc_ceiling + 1;
pub const tc_floor: std::ffi::c_int = tc_door + 1;
pub const tc_plat: std::ffi::c_int = tc_floor + 1;
pub const tc_flash: std::ffi::c_int = tc_plat + 1;
pub const tc_strobe: std::ffi::c_int = tc_flash + 1;
pub const tc_glow: std::ffi::c_int = tc_strobe + 1;
pub const tc_endspecials: std::ffi::c_int = tc_glow + 1;

pub type specials_e = std::ffi::c_int;

pub unsafe extern "C" fn P_ArchiveSpecials() {
    unsafe {
        let mut th: *mut thinker_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut ceiling: *mut ceiling_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut door: *mut vldoor_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut floor: *mut floormove_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut plat: *mut plat_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut flash: *mut lightflash_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut strobe: *mut strobe_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut glow: *mut glow_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: for statement not yet translated:
        //
        //
        //     // save off the current thinkers
        //     for (th = thinkercap.next ; th != &thinkercap ; th=th->next)
        //     {
        // 	if (th->function.acv == (actionf_v)NULL)
        // 	{
        // 	    for (i = 0; i < MAXCEILINGS;i++)
        // 		if (activeceilings[i] == (ceiling_t *)th)
        // 		    break;
        //
        // 	    if (i<MAXCEILINGS)
        // 	    {
        // 		*save_p++ = tc_ceiling;
        // 		PADSAVEP();
        // 		ceiling = (ceiling_t *)save_p;
        // 		memcpy (ceiling, th, sizeof(*ceiling));
        // 		save_p += sizeof(*ceiling);
        // 		ceiling->sector = (sector_t *)(ceiling->sector - sectors);
        // 	    }
        // 	    continue;
        // 	}
        //
        // 	if (th->function.acp1 == (actionf_p1)T_MoveCeiling)
        // 	{
        // 	    *save_p++ = tc_ceiling;
        // 	    PADSAVEP();
        // 	    ceiling = (ceiling_t *)save_p;
        // 	    memcpy (ceiling, th, sizeof(*ceiling));
        // 	    save_p += sizeof(*ceiling);
        // 	    ceiling->sector = (sector_t *)(ceiling->sector - sectors);
        // 	    continue;
        // 	}
        //
        // 	if (th->function.acp1 == (actionf_p1)T_VerticalDoor)
        // 	{
        // 	    *save_p++ = tc_door;
        // 	    PADSAVEP();
        // 	    door = (vldoor_t *)save_p;
        // 	    memcpy (door, th, sizeof(*door));
        // 	    save_p += sizeof(*door);
        // 	    door->sector = (sector_t *)(door->sector - sectors);
        // 	    continue;
        // 	}
        //
        // 	if (th->function.acp1 == (actionf_p1)T_MoveFloor)
        // 	{
        // 	    *save_p++ = tc_floor;
        // 	    PADSAVEP();
        // 	    floor = (floormove_t *)save_p;
        // 	    memcpy (floor, th, sizeof(*floor));
        // 	    save_p += sizeof(*floor);
        // 	    floor->sector = (sector_t *)(floor->sector - sectors);
        // 	    continue;
        // 	}
        //
        // 	if (th->function.acp1 == (actionf_p1)T_PlatRaise)
        // 	{
        // 	    *save_p++ = tc_plat;
        // 	    PADSAVEP();
        // 	    plat = (plat_t *)save_p;
        // 	    memcpy (plat, th, sizeof(*plat));
        // 	    save_p += sizeof(*plat);
        // 	    plat->sector = (sector_t *)(plat->sector - sectors);
        // 	    continue;
        // 	}
        //
        // 	if (th->function.acp1 == (actionf_p1)T_LightFlash)
        // 	{
        // 	    *save_p++ = tc_flash;
        // 	    PADSAVEP();
        // 	    flash = (lightflash_t *)save_p;
        // 	    memcpy (flash, th, sizeof(*flash));
        // 	    save_p += sizeof(*flash);
        // 	    flash->sector = (sector_t *)(flash->sector - sectors);
        // 	    continue;
        // 	}
        //
        // 	if (th->function.acp1 == (actionf_p1)T_StrobeFlash)
        // 	{
        // 	    *save_p++ = tc_strobe;
        // 	    PADSAVEP();
        // 	    strobe = (strobe_t *)save_p;
        // 	    memcpy (strobe, th, sizeof(*strobe));
        // 	    save_p += sizeof(*strobe);
        // 	    strobe->sector = (sector_t *)(strobe->sector - sectors);
        // 	    continue;
        // 	}
        //
        // 	if (th->function.acp1 == (actionf_p1)T_Glow)
        // 	{
        // 	    *save_p++ = tc_glow;
        // 	    PADSAVEP();
        // 	    glow = (glow_t *)save_p;
        // 	    memcpy (glow, th, sizeof(*glow));
        // 	    save_p += sizeof(*glow);
        // 	    glow->sector = (sector_t *)(glow->sector - sectors);
        // 	    continue;
        // 	}
        //     }
        todo!("for statement not yet translated");
        (*({
            let __macro_tmp = save_p;
            save_p += 1;
            __macro_tmp
        })) = tc_endspecials;
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_UnArchiveSpecials() {
    unsafe {
        let mut tclass: byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut ceiling: *mut ceiling_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut door: *mut vldoor_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut floor: *mut floormove_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut plat: *mut plat_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut flash: *mut lightflash_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut strobe: *mut strobe_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut glow: *mut glow_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: while statement not yet translated:
        //
        //
        //
        //     // read in saved thinkers
        //     while (1)
        //     {
        // 	tclass = *save_p++;
        // 	switch (tclass)
        // 	{
        // 	  case tc_endspecials:
        // 	    return;	// end of list
        //
        // 	  case tc_ceiling:
        // 	    PADSAVEP();
        // 	    ceiling = Z_Malloc (sizeof(*ceiling), PU_LEVEL, NULL);
        // 	    memcpy (ceiling, save_p, sizeof(*ceiling));
        // 	    save_p += sizeof(*ceiling);
        // 	    ceiling->sector = &sectors[(int)ceiling->sector];
        // 	    ceiling->sector->specialdata = ceiling;
        //
        // 	    if (ceiling->thinker.function.acp1)
        // 		ceiling->thinker.function.acp1 = (actionf_p1)T_MoveCeiling;
        //
        // 	    P_AddThinker (&ceiling->thinker);
        // 	    P_AddActiveCeiling(ceiling);
        // 	    break;
        //
        // 	  case tc_door:
        // 	    PADSAVEP();
        // 	    door = Z_Malloc (sizeof(*door), PU_LEVEL, NULL);
        // 	    memcpy (door, save_p, sizeof(*door));
        // 	    save_p += sizeof(*door);
        // 	    door->sector = &sectors[(int)door->sector];
        // 	    door->sector->specialdata = door;
        // 	    door->thinker.function.acp1 = (actionf_p1)T_VerticalDoor;
        // 	    P_AddThinker (&door->thinker);
        // 	    break;
        //
        // 	  case tc_floor:
        // 	    PADSAVEP();
        // 	    floor = Z_Malloc (sizeof(*floor), PU_LEVEL, NULL);
        // 	    memcpy (floor, save_p, sizeof(*floor));
        // 	    save_p += sizeof(*floor);
        // 	    floor->sector = &sectors[(int)floor->sector];
        // 	    floor->sector->specialdata = floor;
        // 	    floor->thinker.function.acp1 = (actionf_p1)T_MoveFloor;
        // 	    P_AddThinker (&floor->thinker);
        // 	    break;
        //
        // 	  case tc_plat:
        // 	    PADSAVEP();
        // 	    plat = Z_Malloc (sizeof(*plat), PU_LEVEL, NULL);
        // 	    memcpy (plat, save_p, sizeof(*plat));
        // 	    save_p += sizeof(*plat);
        // 	    plat->sector = &sectors[(int)plat->sector];
        // 	    plat->sector->specialdata = plat;
        //
        // 	    if (plat->thinker.function.acp1)
        // 		plat->thinker.function.acp1 = (actionf_p1)T_PlatRaise;
        //
        // 	    P_AddThinker (&plat->thinker);
        // 	    P_AddActivePlat(plat);
        // 	    break;
        //
        // 	  case tc_flash:
        // 	    PADSAVEP();
        // 	    flash = Z_Malloc (sizeof(*flash), PU_LEVEL, NULL);
        // 	    memcpy (flash, save_p, sizeof(*flash));
        // 	    save_p += sizeof(*flash);
        // 	    flash->sector = &sectors[(int)flash->sector];
        // 	    flash->thinker.function.acp1 = (actionf_p1)T_LightFlash;
        // 	    P_AddThinker (&flash->thinker);
        // 	    break;
        //
        // 	  case tc_strobe:
        // 	    PADSAVEP();
        // 	    strobe = Z_Malloc (sizeof(*strobe), PU_LEVEL, NULL);
        // 	    memcpy (strobe, save_p, sizeof(*strobe));
        // 	    save_p += sizeof(*strobe);
        // 	    strobe->sector = &sectors[(int)strobe->sector];
        // 	    strobe->thinker.function.acp1 = (actionf_p1)T_StrobeFlash;
        // 	    P_AddThinker (&strobe->thinker);
        // 	    break;
        //
        // 	  case tc_glow:
        // 	    PADSAVEP();
        // 	    glow = Z_Malloc (sizeof(*glow), PU_LEVEL, NULL);
        // 	    memcpy (glow, save_p, sizeof(*glow));
        // 	    save_p += sizeof(*glow);
        // 	    glow->sector = &sectors[(int)glow->sector];
        // 	    glow->thinker.function.acp1 = (actionf_p1)T_Glow;
        // 	    P_AddThinker (&glow->thinker);
        // 	    break;
        //
        // 	  default:
        // 	    I_Error ("P_UnarchiveSpecials:Unknown tclass %i "
        // 		     "in savegame",tclass);
        // 	}
        //
        //     }
        todo!("while statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}
