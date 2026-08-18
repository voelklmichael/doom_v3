use crate::d_items::*;
use crate::d_net::*;
use crate::d_player::*;
use crate::d_think::*;
use crate::d_ticcmd::*;
use crate::doomdata::*;
use crate::doomdef::*;
use crate::doomstat::*;
use crate::doomtype::*;
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
        102 as std::ffi::c_char,
        108 as std::ffi::c_char,
        111 as std::ffi::c_char,
        111 as std::ffi::c_char,
        114 as std::ffi::c_char,
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

pub unsafe extern "C" fn T_MovePlane(
    mut sector: *mut sector_t,
    mut speed: fixed_t,
    mut dest: fixed_t,
    mut crush: boolean,
    mut floorOrCeiling: std::ffi::c_int,
    mut direction: std::ffi::c_int,
) -> result_e {
    unsafe {
        let mut flag: boolean = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut lastpos: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: switch statement not yet translated:
        //
        //
        //     switch(floorOrCeiling)
        //     {
        //       case 0:
        // 	// FLOOR
        // 	switch(direction)
        // 	{
        // 	  case -1:
        // 	    // DOWN
        // 	    if (sector->floorheight - speed < dest)
        // 	    {
        // 		lastpos = sector->floorheight;
        // 		sector->floorheight = dest;
        // 		flag = P_ChangeSector(sector,crush);
        // 		if (flag == true)
        // 		{
        // 		    sector->floorheight =lastpos;
        // 		    P_ChangeSector(sector,crush);
        // 		    //return crushed;
        // 		}
        // 		return pastdest;
        // 	    }
        // 	    else
        // 	    {
        // 		lastpos = sector->floorheight;
        // 		sector->floorheight -= speed;
        // 		flag = P_ChangeSector(sector,crush);
        // 		if (flag == true)
        // 		{
        // 		    sector->floorheight = lastpos;
        // 		    P_ChangeSector(sector,crush);
        // 		    return crushed;
        // 		}
        // 	    }
        // 	    break;
        //
        // 	  case 1:
        // 	    // UP
        // 	    if (sector->floorheight + speed > dest)
        // 	    {
        // 		lastpos = sector->floorheight;
        // 		sector->floorheight = dest;
        // 		flag = P_ChangeSector(sector,crush);
        // 		if (flag == true)
        // 		{
        // 		    sector->floorheight = lastpos;
        // 		    P_ChangeSector(sector,crush);
        // 		    //return crushed;
        // 		}
        // 		return pastdest;
        // 	    }
        // 	    else
        // 	    {
        // 		// COULD GET CRUSHED
        // 		lastpos = sector->floorheight;
        // 		sector->floorheight += speed;
        // 		flag = P_ChangeSector(sector,crush);
        // 		if (flag == true)
        // 		{
        // 		    if (crush == true)
        // 			return crushed;
        // 		    sector->floorheight = lastpos;
        // 		    P_ChangeSector(sector,crush);
        // 		    return crushed;
        // 		}
        // 	    }
        // 	    break;
        // 	}
        // 	break;
        //
        //       case 1:
        // 	// CEILING
        // 	switch(direction)
        // 	{
        // 	  case -1:
        // 	    // DOWN
        // 	    if (sector->ceilingheight - speed < dest)
        // 	    {
        // 		lastpos = sector->ceilingheight;
        // 		sector->ceilingheight = dest;
        // 		flag = P_ChangeSector(sector,crush);
        //
        // 		if (flag == true)
        // 		{
        // 		    sector->ceilingheight = lastpos;
        // 		    P_ChangeSector(sector,crush);
        // 		    //return crushed;
        // 		}
        // 		return pastdest;
        // 	    }
        // 	    else
        // 	    {
        // 		// COULD GET CRUSHED
        // 		lastpos = sector->ceilingheight;
        // 		sector->ceilingheight -= speed;
        // 		flag = P_ChangeSector(sector,crush);
        //
        // 		if (flag == true)
        // 		{
        // 		    if (crush == true)
        // 			return crushed;
        // 		    sector->ceilingheight = lastpos;
        // 		    P_ChangeSector(sector,crush);
        // 		    return crushed;
        // 		}
        // 	    }
        // 	    break;
        //
        // 	  case 1:
        // 	    // UP
        // 	    if (sector->ceilingheight + speed > dest)
        // 	    {
        // 		lastpos = sector->ceilingheight;
        // 		sector->ceilingheight = dest;
        // 		flag = P_ChangeSector(sector,crush);
        // 		if (flag == true)
        // 		{
        // 		    sector->ceilingheight = lastpos;
        // 		    P_ChangeSector(sector,crush);
        // 		    //return crushed;
        // 		}
        // 		return pastdest;
        // 	    }
        // 	    else
        // 	    {
        // 		lastpos = sector->ceilingheight;
        // 		sector->ceilingheight += speed;
        // 		flag = P_ChangeSector(sector,crush);
        // // UNUSED
        // #if 0
        // 		if (flag == true)
        // 		{
        // 		    sector->ceilingheight = lastpos;
        // 		    P_ChangeSector(sector,crush);
        // 		    return crushed;
        // 		}
        // #endif
        // 	    }
        // 	    break;
        // 	}
        // 	break;
        //
        //     }
        todo!("switch statement not yet translated");
        return ok;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn T_MoveFloor(mut floor: *mut floormove_t) {
    unsafe {
        let mut res: result_e = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        res = T_MovePlane(
            (*floor).sector,
            (*floor).speed,
            (*floor).floordestheight,
            (*floor).crush,
            0,
            (*floor).direction,
        );
        // TODO: if statement not yet translated:
        //
        //
        //     if (!(leveltime&7))
        // 	S_StartSound((mobj_t *)&floor->sector->soundorg,
        // 		     sfx_stnmov);
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (res == pastdest)
        //     {
        // 	floor->sector->specialdata = NULL;
        //
        // 	if (floor->direction == 1)
        // 	{
        // 	    switch(floor->type)
        // 	    {
        // 	      case donutRaise:
        // 		floor->sector->special = floor->newspecial;
        // 		floor->sector->floorpic = floor->texture;
        // 	      default:
        // 		break;
        // 	    }
        // 	}
        // 	else if (floor->direction == -1)
        // 	{
        // 	    switch(floor->type)
        // 	    {
        // 	      case lowerAndChange:
        // 		floor->sector->special = floor->newspecial;
        // 		floor->sector->floorpic = floor->texture;
        // 	      default:
        // 		break;
        // 	    }
        // 	}
        // 	P_RemoveThinker(&floor->thinker);
        //
        // 	S_StartSound((mobj_t *)&floor->sector->soundorg,
        // 		     sfx_pstop);
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn EV_DoFloor(
    mut line: *mut line_t,
    mut floortype: floor_e,
) -> std::ffi::c_int {
    unsafe {
        let mut secnum: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut rtn: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut sec: *mut sector_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut floor: *mut floormove_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        secnum = (-(1));
        rtn = 0;
        // TODO: while statement not yet translated:
        //
        //     while ((secnum = P_FindSectorFromLineTag(line,secnum)) >= 0)
        //     {
        // 	sec = &sectors[secnum];
        //
        // 	// ALREADY MOVING?  IF SO, KEEP GOING...
        // 	if (sec->specialdata)
        // 	    continue;
        //
        // 	// new floor thinker
        // 	rtn = 1;
        // 	floor = Z_Malloc (sizeof(*floor), PU_LEVSPEC, 0);
        // 	P_AddThinker (&floor->thinker);
        // 	sec->specialdata = floor;
        // 	floor->thinker.function.acp1 = (actionf_p1) T_MoveFloor;
        // 	floor->type = floortype;
        // 	floor->crush = false;
        //
        // 	switch(floortype)
        // 	{
        // 	  case lowerFloor:
        // 	    floor->direction = -1;
        // 	    floor->sector = sec;
        // 	    floor->speed = FLOORSPEED;
        // 	    floor->floordestheight =
        // 		P_FindHighestFloorSurrounding(sec);
        // 	    break;
        //
        // 	  case lowerFloorToLowest:
        // 	    floor->direction = -1;
        // 	    floor->sector = sec;
        // 	    floor->speed = FLOORSPEED;
        // 	    floor->floordestheight =
        // 		P_FindLowestFloorSurrounding(sec);
        // 	    break;
        //
        // 	  case turboLower:
        // 	    floor->direction = -1;
        // 	    floor->sector = sec;
        // 	    floor->speed = FLOORSPEED * 4;
        // 	    floor->floordestheight =
        // 		P_FindHighestFloorSurrounding(sec);
        // 	    if (floor->floordestheight != sec->floorheight)
        // 		floor->floordestheight += 8*FRACUNIT;
        // 	    break;
        //
        // 	  case raiseFloorCrush:
        // 	    floor->crush = true;
        // 	  case raiseFloor:
        // 	    floor->direction = 1;
        // 	    floor->sector = sec;
        // 	    floor->speed = FLOORSPEED;
        // 	    floor->floordestheight =
        // 		P_FindLowestCeilingSurrounding(sec);
        // 	    if (floor->floordestheight > sec->ceilingheight)
        // 		floor->floordestheight = sec->ceilingheight;
        // 	    floor->floordestheight -= (8*FRACUNIT)*
        // 		(floortype == raiseFloorCrush);
        // 	    break;
        //
        // 	  case raiseFloorTurbo:
        // 	    floor->direction = 1;
        // 	    floor->sector = sec;
        // 	    floor->speed = FLOORSPEED*4;
        // 	    floor->floordestheight =
        // 		P_FindNextHighestFloor(sec,sec->floorheight);
        // 	    break;
        //
        // 	  case raiseFloorToNearest:
        // 	    floor->direction = 1;
        // 	    floor->sector = sec;
        // 	    floor->speed = FLOORSPEED;
        // 	    floor->floordestheight =
        // 		P_FindNextHighestFloor(sec,sec->floorheight);
        // 	    break;
        //
        // 	  case raiseFloor24:
        // 	    floor->direction = 1;
        // 	    floor->sector = sec;
        // 	    floor->speed = FLOORSPEED;
        // 	    floor->floordestheight = floor->sector->floorheight +
        // 		24 * FRACUNIT;
        // 	    break;
        // 	  case raiseFloor512:
        // 	    floor->direction = 1;
        // 	    floor->sector = sec;
        // 	    floor->speed = FLOORSPEED;
        // 	    floor->floordestheight = floor->sector->floorheight +
        // 		512 * FRACUNIT;
        // 	    break;
        //
        // 	  case raiseFloor24AndChange:
        // 	    floor->direction = 1;
        // 	    floor->sector = sec;
        // 	    floor->speed = FLOORSPEED;
        // 	    floor->floordestheight = floor->sector->floorheight +
        // 		24 * FRACUNIT;
        // 	    sec->floorpic = line->frontsector->floorpic;
        // 	    sec->special = line->frontsector->special;
        // 	    break;
        //
        // 	  case raiseToTexture:
        // 	  {
        // 	      int	minsize = MAXINT;
        // 	      side_t*	side;
        //
        // 	      floor->direction = 1;
        // 	      floor->sector = sec;
        // 	      floor->speed = FLOORSPEED;
        // 	      for (i = 0; i < sec->linecount; i++)
        // 	      {
        // 		  if (twoSided (secnum, i) )
        // 		  {
        // 		      side = getSide(secnum,i,0);
        // 		      if (side->bottomtexture >= 0)
        // 			  if (textureheight[side->bottomtexture] <
        // 			      minsize)
        // 			      minsize =
        // 				  textureheight[side->bottomtexture];
        // 		      side = getSide(secnum,i,1);
        // 		      if (side->bottomtexture >= 0)
        // 			  if (textureheight[side->bottomtexture] <
        // 			      minsize)
        // 			      minsize =
        // 				  textureheight[side->bottomtexture];
        // 		  }
        // 	      }
        // 	      floor->floordestheight =
        // 		  floor->sector->floorheight + minsize;
        // 	  }
        // 	  break;
        //
        // 	  case lowerAndChange:
        // 	    floor->direction = -1;
        // 	    floor->sector = sec;
        // 	    floor->speed = FLOORSPEED;
        // 	    floor->floordestheight =
        // 		P_FindLowestFloorSurrounding(sec);
        // 	    floor->texture = sec->floorpic;
        //
        // 	    for (i = 0; i < sec->linecount; i++)
        // 	    {
        // 		if ( twoSided(secnum, i) )
        // 		{
        // 		    if (getSide(secnum,i,0)->sector-sectors == secnum)
        // 		    {
        // 			sec = getSector(secnum,i,1);
        //
        // 			if (sec->floorheight == floor->floordestheight)
        // 			{
        // 			    floor->texture = sec->floorpic;
        // 			    floor->newspecial = sec->special;
        // 			    break;
        // 			}
        // 		    }
        // 		    else
        // 		    {
        // 			sec = getSector(secnum,i,0);
        //
        // 			if (sec->floorheight == floor->floordestheight)
        // 			{
        // 			    floor->texture = sec->floorpic;
        // 			    floor->newspecial = sec->special;
        // 			    break;
        // 			}
        // 		    }
        // 		}
        // 	    }
        // 	  default:
        // 	    break;
        // 	}
        //     }
        todo!("while statement not yet translated");
        return rtn;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn EV_BuildStairs(
    mut line: *mut line_t,
    mut type_: stair_e,
) -> std::ffi::c_int {
    unsafe {
        let mut secnum: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut height: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut newsecnum: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut texture: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut ok: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut rtn: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut sec: *mut sector_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut tsec: *mut sector_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut floor: *mut floormove_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut stairsize: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut speed: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        secnum = (-(1));
        rtn = 0;
        // TODO: while statement not yet translated:
        //
        //     while ((secnum = P_FindSectorFromLineTag(line,secnum)) >= 0)
        //     {
        // 	sec = &sectors[secnum];
        //
        // 	// ALREADY MOVING?  IF SO, KEEP GOING...
        // 	if (sec->specialdata)
        // 	    continue;
        //
        // 	// new floor thinker
        // 	rtn = 1;
        // 	floor = Z_Malloc (sizeof(*floor), PU_LEVSPEC, 0);
        // 	P_AddThinker (&floor->thinker);
        // 	sec->specialdata = floor;
        // 	floor->thinker.function.acp1 = (actionf_p1) T_MoveFloor;
        // 	floor->direction = 1;
        // 	floor->sector = sec;
        // 	switch(type)
        // 	{
        // 	  case build8:
        // 	    speed = FLOORSPEED/4;
        // 	    stairsize = 8*FRACUNIT;
        // 	    break;
        // 	  case turbo16:
        // 	    speed = FLOORSPEED*4;
        // 	    stairsize = 16*FRACUNIT;
        // 	    break;
        // 	}
        // 	floor->speed = speed;
        // 	height = sec->floorheight + stairsize;
        // 	floor->floordestheight = height;
        //
        // 	texture = sec->floorpic;
        //
        // 	// Find next sector to raise
        // 	// 1.	Find 2-sided line with same sector side[0]
        // 	// 2.	Other side is the next sector to raise
        // 	do
        // 	{
        // 	    ok = 0;
        // 	    for (i = 0;i < sec->linecount;i++)
        // 	    {
        // 		if ( !((sec->lines[i])->flags & ML_TWOSIDED) )
        // 		    continue;
        //
        // 		tsec = (sec->lines[i])->frontsector;
        // 		newsecnum = tsec-sectors;
        //
        // 		if (secnum != newsecnum)
        // 		    continue;
        //
        // 		tsec = (sec->lines[i])->backsector;
        // 		newsecnum = tsec - sectors;
        //
        // 		if (tsec->floorpic != texture)
        // 		    continue;
        //
        // 		height += stairsize;
        //
        // 		if (tsec->specialdata)
        // 		    continue;
        //
        // 		sec = tsec;
        // 		secnum = newsecnum;
        // 		floor = Z_Malloc (sizeof(*floor), PU_LEVSPEC, 0);
        //
        // 		P_AddThinker (&floor->thinker);
        //
        // 		sec->specialdata = floor;
        // 		floor->thinker.function.acp1 = (actionf_p1) T_MoveFloor;
        // 		floor->direction = 1;
        // 		floor->sector = sec;
        // 		floor->speed = speed;
        // 		floor->floordestheight = height;
        // 		ok = 1;
        // 		break;
        // 	    }
        // 	} while(ok);
        //     }
        todo!("while statement not yet translated");
        return rtn;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}
