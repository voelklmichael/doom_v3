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

static mut rcsid: [std::ffi::c_char; 51] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        112 as std::ffi::c_char,
        95 as std::ffi::c_char,
        99 as std::ffi::c_char,
        101 as std::ffi::c_char,
        105 as std::ffi::c_char,
        108 as std::ffi::c_char,
        110 as std::ffi::c_char,
        103 as std::ffi::c_char,
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

pub static mut activeceilings: [*mut ceiling_t; (MAXCEILINGS) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn T_MoveCeiling(mut ceiling: *mut ceiling_t) {
    unsafe {
        let mut res: result_e = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: switch statement not yet translated:
        //
        //
        //     switch(ceiling->direction)
        //     {
        //       case 0:
        // 	// IN STASIS
        // 	break;
        //       case 1:
        // 	// UP
        // 	res = T_MovePlane(ceiling->sector,
        // 			  ceiling->speed,
        // 			  ceiling->topheight,
        // 			  false,1,ceiling->direction);
        //
        // 	if (!(leveltime&7))
        // 	{
        // 	    switch(ceiling->type)
        // 	    {
        // 	      case silentCrushAndRaise:
        // 		break;
        // 	      default:
        // 		S_StartSound((mobj_t *)&ceiling->sector->soundorg,
        // 			     sfx_stnmov);
        // 		// ?
        // 		break;
        // 	    }
        // 	}
        //
        // 	if (res == pastdest)
        // 	{
        // 	    switch(ceiling->type)
        // 	    {
        // 	      case raiseToHighest:
        // 		P_RemoveActiveCeiling(ceiling);
        // 		break;
        //
        // 	      case silentCrushAndRaise:
        // 		S_StartSound((mobj_t *)&ceiling->sector->soundorg,
        // 			     sfx_pstop);
        // 	      case fastCrushAndRaise:
        // 	      case crushAndRaise:
        // 		ceiling->direction = -1;
        // 		break;
        //
        // 	      default:
        // 		break;
        // 	    }
        //
        // 	}
        // 	break;
        //
        //       case -1:
        // 	// DOWN
        // 	res = T_MovePlane(ceiling->sector,
        // 			  ceiling->speed,
        // 			  ceiling->bottomheight,
        // 			  ceiling->crush,1,ceiling->direction);
        //
        // 	if (!(leveltime&7))
        // 	{
        // 	    switch(ceiling->type)
        // 	    {
        // 	      case silentCrushAndRaise: break;
        // 	      default:
        // 		S_StartSound((mobj_t *)&ceiling->sector->soundorg,
        // 			     sfx_stnmov);
        // 	    }
        // 	}
        //
        // 	if (res == pastdest)
        // 	{
        // 	    switch(ceiling->type)
        // 	    {
        // 	      case silentCrushAndRaise:
        // 		S_StartSound((mobj_t *)&ceiling->sector->soundorg,
        // 			     sfx_pstop);
        // 	      case crushAndRaise:
        // 		ceiling->speed = CEILSPEED;
        // 	      case fastCrushAndRaise:
        // 		ceiling->direction = 1;
        // 		break;
        //
        // 	      case lowerAndCrush:
        // 	      case lowerToFloor:
        // 		P_RemoveActiveCeiling(ceiling);
        // 		break;
        //
        // 	      default:
        // 		break;
        // 	    }
        // 	}
        // 	else // ( res != pastdest )
        // 	{
        // 	    if (res == crushed)
        // 	    {
        // 		switch(ceiling->type)
        // 		{
        // 		  case silentCrushAndRaise:
        // 		  case crushAndRaise:
        // 		  case lowerAndCrush:
        // 		    ceiling->speed = CEILSPEED / 8;
        // 		    break;
        //
        // 		  default:
        // 		    break;
        // 		}
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

pub unsafe extern "C" fn EV_DoCeiling(
    mut line: *mut line_t,
    mut type_: ceiling_e,
) -> std::ffi::c_int {
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
        let mut ceiling: *mut ceiling_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        secnum = (-(1));
        rtn = 0;
        // TODO: switch statement not yet translated:
        //
        //
        //     //	Reactivate in-stasis ceilings...for certain types.
        //     switch(type)
        //     {
        //       case fastCrushAndRaise:
        //       case silentCrushAndRaise:
        //       case crushAndRaise:
        // 	P_ActivateInStasisCeiling(line);
        //       default:
        // 	break;
        //     }
        todo!("switch statement not yet translated");
        // TODO: while statement not yet translated:
        //
        //
        //     while ((secnum = P_FindSectorFromLineTag(line,secnum)) >= 0)
        //     {
        // 	sec = &sectors[secnum];
        // 	if (sec->specialdata)
        // 	    continue;
        //
        // 	// new door thinker
        // 	rtn = 1;
        // 	ceiling = Z_Malloc (sizeof(*ceiling), PU_LEVSPEC, 0);
        // 	P_AddThinker (&ceiling->thinker);
        // 	sec->specialdata = ceiling;
        // 	ceiling->thinker.function.acp1 = (actionf_p1)T_MoveCeiling;
        // 	ceiling->sector = sec;
        // 	ceiling->crush = false;
        //
        // 	switch(type)
        // 	{
        // 	  case fastCrushAndRaise:
        // 	    ceiling->crush = true;
        // 	    ceiling->topheight = sec->ceilingheight;
        // 	    ceiling->bottomheight = sec->floorheight + (8*FRACUNIT);
        // 	    ceiling->direction = -1;
        // 	    ceiling->speed = CEILSPEED * 2;
        // 	    break;
        //
        // 	  case silentCrushAndRaise:
        // 	  case crushAndRaise:
        // 	    ceiling->crush = true;
        // 	    ceiling->topheight = sec->ceilingheight;
        // 	  case lowerAndCrush:
        // 	  case lowerToFloor:
        // 	    ceiling->bottomheight = sec->floorheight;
        // 	    if (type != lowerToFloor)
        // 		ceiling->bottomheight += 8*FRACUNIT;
        // 	    ceiling->direction = -1;
        // 	    ceiling->speed = CEILSPEED;
        // 	    break;
        //
        // 	  case raiseToHighest:
        // 	    ceiling->topheight = P_FindHighestCeilingSurrounding(sec);
        // 	    ceiling->direction = 1;
        // 	    ceiling->speed = CEILSPEED;
        // 	    break;
        // 	}
        //
        // 	ceiling->tag = sec->tag;
        // 	ceiling->type = type;
        // 	P_AddActiveCeiling(ceiling);
        //     }
        todo!("while statement not yet translated");
        return rtn;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_AddActiveCeiling(mut c: *mut ceiling_t) {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: for statement not yet translated:
        //
        //
        //     for (i = 0; i < MAXCEILINGS;i++)
        //     {
        // 	if (activeceilings[i] == NULL)
        // 	{
        // 	    activeceilings[i] = c;
        // 	    return;
        // 	}
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_RemoveActiveCeiling(mut c: *mut ceiling_t) {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: for statement not yet translated:
        //
        //
        //     for (i = 0;i < MAXCEILINGS;i++)
        //     {
        // 	if (activeceilings[i] == c)
        // 	{
        // 	    activeceilings[i]->sector->specialdata = NULL;
        // 	    P_RemoveThinker (&activeceilings[i]->thinker);
        // 	    activeceilings[i] = NULL;
        // 	    break;
        // 	}
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_ActivateInStasisCeiling(mut line: *mut line_t) {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: for statement not yet translated:
        //
        //
        //     for (i = 0;i < MAXCEILINGS;i++)
        //     {
        // 	if (activeceilings[i]
        // 	    && (activeceilings[i]->tag == line->tag)
        // 	    && (activeceilings[i]->direction == 0))
        // 	{
        // 	    activeceilings[i]->direction = activeceilings[i]->olddirection;
        // 	    activeceilings[i]->thinker.function.acp1
        // 	      = (actionf_p1)T_MoveCeiling;
        // 	}
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn EV_CeilingCrushStop(mut line: *mut line_t) -> std::ffi::c_int {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut rtn: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        rtn = 0;
        // TODO: for statement not yet translated:
        //
        //     for (i = 0;i < MAXCEILINGS;i++)
        //     {
        // 	if (activeceilings[i]
        // 	    && (activeceilings[i]->tag == line->tag)
        // 	    && (activeceilings[i]->direction != 0))
        // 	{
        // 	    activeceilings[i]->olddirection = activeceilings[i]->direction;
        // 	    activeceilings[i]->thinker.function.acv = (actionf_v)NULL;
        // 	    activeceilings[i]->direction = 0;		// in-stasis
        // 	    rtn = 1;
        // 	}
        //     }
        todo!("for statement not yet translated");
        return rtn;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}
