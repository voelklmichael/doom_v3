use crate::d_englsh::*;
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
use crate::s_sound::*;
use crate::sounds::*;
use crate::tables::*;
use crate::z_zone::*;

static mut rcsid: [std::ffi::c_char; 50] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        112 as std::ffi::c_char,
        95 as std::ffi::c_char,
        100 as std::ffi::c_char,
        111 as std::ffi::c_char,
        111 as std::ffi::c_char,
        114 as std::ffi::c_char,
        115 as std::ffi::c_char,
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
        51 as std::ffi::c_char,
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

pub unsafe extern "C" fn T_VerticalDoor(mut door: *mut vldoor_t) {
    unsafe {
        let mut res: result_e = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: switch statement not yet translated:
        //
        //
        //     switch(door->direction)
        //     {
        //       case 0:
        // 	// WAITING
        // 	if (!--door->topcountdown)
        // 	{
        // 	    switch(door->type)
        // 	    {
        // 	      case blazeRaise:
        // 		door->direction = -1; // time to go back down
        // 		S_StartSound((mobj_t *)&door->sector->soundorg,
        // 			     sfx_bdcls);
        // 		break;
        //
        // 	      case normal:
        // 		door->direction = -1; // time to go back down
        // 		S_StartSound((mobj_t *)&door->sector->soundorg,
        // 			     sfx_dorcls);
        // 		break;
        //
        // 	      case close30ThenOpen:
        // 		door->direction = 1;
        // 		S_StartSound((mobj_t *)&door->sector->soundorg,
        // 			     sfx_doropn);
        // 		break;
        //
        // 	      default:
        // 		break;
        // 	    }
        // 	}
        // 	break;
        //
        //       case 2:
        // 	//  INITIAL WAIT
        // 	if (!--door->topcountdown)
        // 	{
        // 	    switch(door->type)
        // 	    {
        // 	      case raiseIn5Mins:
        // 		door->direction = 1;
        // 		door->type = normal;
        // 		S_StartSound((mobj_t *)&door->sector->soundorg,
        // 			     sfx_doropn);
        // 		break;
        //
        // 	      default:
        // 		break;
        // 	    }
        // 	}
        // 	break;
        //
        //       case -1:
        // 	// DOWN
        // 	res = T_MovePlane(door->sector,
        // 			  door->speed,
        // 			  door->sector->floorheight,
        // 			  false,1,door->direction);
        // 	if (res == pastdest)
        // 	{
        // 	    switch(door->type)
        // 	    {
        // 	      case blazeRaise:
        // 	      case blazeClose:
        // 		door->sector->specialdata = NULL;
        // 		P_RemoveThinker (&door->thinker);  // unlink and free
        // 		S_StartSound((mobj_t *)&door->sector->soundorg,
        // 			     sfx_bdcls);
        // 		break;
        //
        // 	      case normal:
        // 	      case close:
        // 		door->sector->specialdata = NULL;
        // 		P_RemoveThinker (&door->thinker);  // unlink and free
        // 		break;
        //
        // 	      case close30ThenOpen:
        // 		door->direction = 0;
        // 		door->topcountdown = 35*30;
        // 		break;
        //
        // 	      default:
        // 		break;
        // 	    }
        // 	}
        // 	else if (res == crushed)
        // 	{
        // 	    switch(door->type)
        // 	    {
        // 	      case blazeClose:
        // 	      case close:		// DO NOT GO BACK UP!
        // 		break;
        //
        // 	      default:
        // 		door->direction = 1;
        // 		S_StartSound((mobj_t *)&door->sector->soundorg,
        // 			     sfx_doropn);
        // 		break;
        // 	    }
        // 	}
        // 	break;
        //
        //       case 1:
        // 	// UP
        // 	res = T_MovePlane(door->sector,
        // 			  door->speed,
        // 			  door->topheight,
        // 			  false,1,door->direction);
        //
        // 	if (res == pastdest)
        // 	{
        // 	    switch(door->type)
        // 	    {
        // 	      case blazeRaise:
        // 	      case normal:
        // 		door->direction = 0; // wait at top
        // 		door->topcountdown = door->topwait;
        // 		break;
        //
        // 	      case close30ThenOpen:
        // 	      case blazeOpen:
        // 	      case open:
        // 		door->sector->specialdata = NULL;
        // 		P_RemoveThinker (&door->thinker);  // unlink and free
        // 		break;
        //
        // 	      default:
        // 		break;
        // 	    }
        // 	}
        // 	break;
        //     }
        todo!("switch statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn EV_DoLockedDoor(
    mut line: *mut line_t,
    mut type_: vldoor_e,
    mut thing: *mut mobj_t,
) -> std::ffi::c_int {
    unsafe {
        let mut p: *mut player_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        p = (*thing).player;
        // TODO: if statement not yet translated:
        //
        //
        //     if (!p)
        // 	return 0;
        todo!("if statement not yet translated");
        // TODO: switch statement not yet translated:
        //
        //
        //     switch(line->special)
        //     {
        //       case 99:	// Blue Lock
        //       case 133:
        // 	if ( !p )
        // 	    return 0;
        // 	if (!p->cards[it_bluecard] && !p->cards[it_blueskull])
        // 	{
        // 	    p->message = PD_BLUEO;
        // 	    S_StartSound(NULL,sfx_oof);
        // 	    return 0;
        // 	}
        // 	break;
        //
        //       case 134: // Red Lock
        //       case 135:
        // 	if ( !p )
        // 	    return 0;
        // 	if (!p->cards[it_redcard] && !p->cards[it_redskull])
        // 	{
        // 	    p->message = PD_REDO;
        // 	    S_StartSound(NULL,sfx_oof);
        // 	    return 0;
        // 	}
        // 	break;
        //
        //       case 136:	// Yellow Lock
        //       case 137:
        // 	if ( !p )
        // 	    return 0;
        // 	if (!p->cards[it_yellowcard] &&
        // 	    !p->cards[it_yellowskull])
        // 	{
        // 	    p->message = PD_YELLOWO;
        // 	    S_StartSound(NULL,sfx_oof);
        // 	    return 0;
        // 	}
        // 	break;
        //     }
        todo!("switch statement not yet translated");
        return EV_DoDoor(line, type_);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn EV_DoDoor(mut line: *mut line_t, mut type_: vldoor_e) -> std::ffi::c_int {
    unsafe {
        let mut secnum: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut rtn: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut sec: *mut sector_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut door: *mut vldoor_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        secnum = (-(1));
        rtn = 0;
        // TODO: while statement not yet translated:
        //
        //
        //     while ((secnum = P_FindSectorFromLineTag(line,secnum)) >= 0)
        //     {
        // 	sec = &sectors[secnum];
        // 	if (sec->specialdata)
        // 	    continue;
        //
        //
        // 	// new door thinker
        // 	rtn = 1;
        // 	door = Z_Malloc (sizeof(*door), PU_LEVSPEC, 0);
        // 	P_AddThinker (&door->thinker);
        // 	sec->specialdata = door;
        //
        // 	door->thinker.function.acp1 = (actionf_p1) T_VerticalDoor;
        // 	door->sector = sec;
        // 	door->type = type;
        // 	door->topwait = VDOORWAIT;
        // 	door->speed = VDOORSPEED;
        //
        // 	switch(type)
        // 	{
        // 	  case blazeClose:
        // 	    door->topheight = P_FindLowestCeilingSurrounding(sec);
        // 	    door->topheight -= 4*FRACUNIT;
        // 	    door->direction = -1;
        // 	    door->speed = VDOORSPEED * 4;
        // 	    S_StartSound((mobj_t *)&door->sector->soundorg,
        // 			 sfx_bdcls);
        // 	    break;
        //
        // 	  case close:
        // 	    door->topheight = P_FindLowestCeilingSurrounding(sec);
        // 	    door->topheight -= 4*FRACUNIT;
        // 	    door->direction = -1;
        // 	    S_StartSound((mobj_t *)&door->sector->soundorg,
        // 			 sfx_dorcls);
        // 	    break;
        //
        // 	  case close30ThenOpen:
        // 	    door->topheight = sec->ceilingheight;
        // 	    door->direction = -1;
        // 	    S_StartSound((mobj_t *)&door->sector->soundorg,
        // 			 sfx_dorcls);
        // 	    break;
        //
        // 	  case blazeRaise:
        // 	  case blazeOpen:
        // 	    door->direction = 1;
        // 	    door->topheight = P_FindLowestCeilingSurrounding(sec);
        // 	    door->topheight -= 4*FRACUNIT;
        // 	    door->speed = VDOORSPEED * 4;
        // 	    if (door->topheight != sec->ceilingheight)
        // 		S_StartSound((mobj_t *)&door->sector->soundorg,
        // 			     sfx_bdopn);
        // 	    break;
        //
        // 	  case normal:
        // 	  case open:
        // 	    door->direction = 1;
        // 	    door->topheight = P_FindLowestCeilingSurrounding(sec);
        // 	    door->topheight -= 4*FRACUNIT;
        // 	    if (door->topheight != sec->ceilingheight)
        // 		S_StartSound((mobj_t *)&door->sector->soundorg,
        // 			     sfx_doropn);
        // 	    break;
        //
        // 	  default:
        // 	    break;
        // 	}
        //
        //     }
        todo!("while statement not yet translated");
        return rtn;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn EV_VerticalDoor(mut line: *mut line_t, mut thing: *mut mobj_t) {
    unsafe {
        let mut player: *mut player_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut secnum: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut sec: *mut sector_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut door: *mut vldoor_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut side: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        side = 0;
        player = (*thing).player;
        // TODO: switch statement not yet translated:
        //
        //
        //     switch(line->special)
        //     {
        //       case 26: // Blue Lock
        //       case 32:
        // 	if ( !player )
        // 	    return;
        //
        // 	if (!player->cards[it_bluecard] && !player->cards[it_blueskull])
        // 	{
        // 	    player->message = PD_BLUEK;
        // 	    S_StartSound(NULL,sfx_oof);
        // 	    return;
        // 	}
        // 	break;
        //
        //       case 27: // Yellow Lock
        //       case 34:
        // 	if ( !player )
        // 	    return;
        //
        // 	if (!player->cards[it_yellowcard] &&
        // 	    !player->cards[it_yellowskull])
        // 	{
        // 	    player->message = PD_YELLOWK;
        // 	    S_StartSound(NULL,sfx_oof);
        // 	    return;
        // 	}
        // 	break;
        //
        //       case 28: // Red Lock
        //       case 33:
        // 	if ( !player )
        // 	    return;
        //
        // 	if (!player->cards[it_redcard] && !player->cards[it_redskull])
        // 	{
        // 	    player->message = PD_REDK;
        // 	    S_StartSound(NULL,sfx_oof);
        // 	    return;
        // 	}
        // 	break;
        //     }
        todo!("switch statement not yet translated");
        sec = sides[((*line).sidenum[(side ^ 1) as usize]) as usize].sector;
        secnum = (sec - sectors);
        // TODO: if statement not yet translated:
        //
        //
        //     if (sec->specialdata)
        //     {
        // 	door = sec->specialdata;
        // 	switch(line->special)
        // 	{
        // 	  case	1: // ONLY FOR "RAISE" DOORS, NOT "OPEN"s
        // 	  case	26:
        // 	  case	27:
        // 	  case	28:
        // 	  case	117:
        // 	    if (door->direction == -1)
        // 		door->direction = 1;	// go back up
        // 	    else
        // 	    {
        // 		if (!thing->player)
        // 		    return;		// JDC: bad guys never close doors
        //
        // 		door->direction = -1;	// start going down immediately
        // 	    }
        // 	    return;
        // 	}
        //     }
        todo!("if statement not yet translated");
        // TODO: switch statement not yet translated:
        //
        //
        //     // for proper sound
        //     switch(line->special)
        //     {
        //       case 117:	// BLAZING DOOR RAISE
        //       case 118:	// BLAZING DOOR OPEN
        // 	S_StartSound((mobj_t *)&sec->soundorg,sfx_bdopn);
        // 	break;
        //
        //       case 1:	// NORMAL DOOR SOUND
        //       case 31:
        // 	S_StartSound((mobj_t *)&sec->soundorg,sfx_doropn);
        // 	break;
        //
        //       default:	// LOCKED DOOR SOUND
        // 	S_StartSound((mobj_t *)&sec->soundorg,sfx_doropn);
        // 	break;
        //     }
        todo!("switch statement not yet translated");
        door = Z_Malloc(std::mem::size_of_val(&(*(door))), PU_LEVSPEC, 0);
        P_AddThinker((&((*door).thinker) as *const _ as *mut _));
        (*sec).specialdata = door;
        (*door).thinker.function.acp1 = ((T_VerticalDoor) as actionf_p1);
        (*door).sector = sec;
        (*door).direction = 1;
        (*door).speed = VDOORSPEED;
        (*door).topwait = VDOORWAIT;
        // TODO: switch statement not yet translated:
        //
        //
        //     switch(line->special)
        //     {
        //       case 1:
        //       case 26:
        //       case 27:
        //       case 28:
        // 	door->type = normal;
        // 	break;
        //
        //       case 31:
        //       case 32:
        //       case 33:
        //       case 34:
        // 	door->type = open;
        // 	line->special = 0;
        // 	break;
        //
        //       case 117:	// blazing door raise
        // 	door->type = blazeRaise;
        // 	door->speed = VDOORSPEED*4;
        // 	break;
        //       case 118:	// blazing door open
        // 	door->type = blazeOpen;
        // 	line->special = 0;
        // 	door->speed = VDOORSPEED*4;
        // 	break;
        //     }
        todo!("switch statement not yet translated");
        (*door).topheight = P_FindLowestCeilingSurrounding(sec);
        (*door).topheight -= (4 * FRACUNIT);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_SpawnDoorCloseIn30(mut sec: *mut sector_t) {
    unsafe {
        let mut door: *mut vldoor_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        door = Z_Malloc(std::mem::size_of_val(&(*(door))), PU_LEVSPEC, 0);
        P_AddThinker((&((*door).thinker) as *const _ as *mut _));
        (*sec).specialdata = door;
        (*sec).special = 0;
        (*door).thinker.function.acp1 = ((T_VerticalDoor) as actionf_p1);
        (*door).sector = sec;
        (*door).direction = 0;
        (*door).type_ = normal;
        (*door).speed = VDOORSPEED;
        (*door).topcountdown = (30 * 35);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_SpawnDoorRaiseIn5Mins(
    mut sec: *mut sector_t,
    mut secnum: std::ffi::c_int,
) {
    unsafe {
        let mut door: *mut vldoor_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        door = Z_Malloc(std::mem::size_of_val(&(*(door))), PU_LEVSPEC, 0);
        P_AddThinker((&((*door).thinker) as *const _ as *mut _));
        (*sec).specialdata = door;
        (*sec).special = 0;
        (*door).thinker.function.acp1 = ((T_VerticalDoor) as actionf_p1);
        (*door).sector = sec;
        (*door).direction = 2;
        (*door).type_ = raiseIn5Mins;
        (*door).speed = VDOORSPEED;
        (*door).topheight = P_FindLowestCeilingSurrounding(sec);
        (*door).topheight -= (4 * FRACUNIT);
        (*door).topwait = VDOORWAIT;
        (*door).topcountdown = ((5 * 60) * 35);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}
