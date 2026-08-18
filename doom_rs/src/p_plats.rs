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
        112 as std::ffi::c_char,
        108 as std::ffi::c_char,
        97 as std::ffi::c_char,
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
        50 as std::ffi::c_char,
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

pub static mut activeplats: [*mut plat_t; (MAXPLATS) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn T_PlatRaise(mut plat: *mut plat_t) {
    unsafe {
        let mut res: result_e = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: switch statement not yet translated:
        //
        //
        //     switch(plat->status)
        //     {
        //       case up:
        // 	res = T_MovePlane(plat->sector,
        // 			  plat->speed,
        // 			  plat->high,
        // 			  plat->crush,0,1);
        //
        // 	if (plat->type == raiseAndChange
        // 	    || plat->type == raiseToNearestAndChange)
        // 	{
        // 	    if (!(leveltime&7))
        // 		S_StartSound((mobj_t *)&plat->sector->soundorg,
        // 			     sfx_stnmov);
        // 	}
        //
        //
        // 	if (res == crushed && (!plat->crush))
        // 	{
        // 	    plat->count = plat->wait;
        // 	    plat->status = down;
        // 	    S_StartSound((mobj_t *)&plat->sector->soundorg,
        // 			 sfx_pstart);
        // 	}
        // 	else
        // 	{
        // 	    if (res == pastdest)
        // 	    {
        // 		plat->count = plat->wait;
        // 		plat->status = waiting;
        // 		S_StartSound((mobj_t *)&plat->sector->soundorg,
        // 			     sfx_pstop);
        //
        // 		switch(plat->type)
        // 		{
        // 		  case blazeDWUS:
        // 		  case downWaitUpStay:
        // 		    P_RemoveActivePlat(plat);
        // 		    break;
        //
        // 		  case raiseAndChange:
        // 		  case raiseToNearestAndChange:
        // 		    P_RemoveActivePlat(plat);
        // 		    break;
        //
        // 		  default:
        // 		    break;
        // 		}
        // 	    }
        // 	}
        // 	break;
        //
        //       case	down:
        // 	res = T_MovePlane(plat->sector,plat->speed,plat->low,false,0,-1);
        //
        // 	if (res == pastdest)
        // 	{
        // 	    plat->count = plat->wait;
        // 	    plat->status = waiting;
        // 	    S_StartSound((mobj_t *)&plat->sector->soundorg,sfx_pstop);
        // 	}
        // 	break;
        //
        //       case	waiting:
        // 	if (!--plat->count)
        // 	{
        // 	    if (plat->sector->floorheight == plat->low)
        // 		plat->status = up;
        // 	    else
        // 		plat->status = down;
        // 	    S_StartSound((mobj_t *)&plat->sector->soundorg,sfx_pstart);
        // 	}
        //       case	in_stasis:
        // 	break;
        //     }
        todo!("switch statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn EV_DoPlat(
    mut line: *mut line_t,
    mut type_: plattype_e,
    mut amount: std::ffi::c_int,
) -> std::ffi::c_int {
    unsafe {
        let mut plat: *mut plat_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut secnum: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut rtn: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut sec: *mut sector_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        secnum = (-(1));
        rtn = 0;
        // TODO: switch statement not yet translated:
        //
        //
        //
        //     //	Activate all <type> plats that are in_stasis
        //     switch(type)
        //     {
        //       case perpetualRaise:
        // 	P_ActivateInStasis(line->tag);
        // 	break;
        //
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
        //
        // 	if (sec->specialdata)
        // 	    continue;
        //
        // 	// Find lowest & highest floors around sector
        // 	rtn = 1;
        // 	plat = Z_Malloc( sizeof(*plat), PU_LEVSPEC, 0);
        // 	P_AddThinker(&plat->thinker);
        //
        // 	plat->type = type;
        // 	plat->sector = sec;
        // 	plat->sector->specialdata = plat;
        // 	plat->thinker.function.acp1 = (actionf_p1) T_PlatRaise;
        // 	plat->crush = false;
        // 	plat->tag = line->tag;
        //
        // 	switch(type)
        // 	{
        // 	  case raiseToNearestAndChange:
        // 	    plat->speed = PLATSPEED/2;
        // 	    sec->floorpic = sides[line->sidenum[0]].sector->floorpic;
        // 	    plat->high = P_FindNextHighestFloor(sec,sec->floorheight);
        // 	    plat->wait = 0;
        // 	    plat->status = up;
        // 	    // NO MORE DAMAGE, IF APPLICABLE
        // 	    sec->special = 0;
        //
        // 	    S_StartSound((mobj_t *)&sec->soundorg,sfx_stnmov);
        // 	    break;
        //
        // 	  case raiseAndChange:
        // 	    plat->speed = PLATSPEED/2;
        // 	    sec->floorpic = sides[line->sidenum[0]].sector->floorpic;
        // 	    plat->high = sec->floorheight + amount*FRACUNIT;
        // 	    plat->wait = 0;
        // 	    plat->status = up;
        //
        // 	    S_StartSound((mobj_t *)&sec->soundorg,sfx_stnmov);
        // 	    break;
        //
        // 	  case downWaitUpStay:
        // 	    plat->speed = PLATSPEED * 4;
        // 	    plat->low = P_FindLowestFloorSurrounding(sec);
        //
        // 	    if (plat->low > sec->floorheight)
        // 		plat->low = sec->floorheight;
        //
        // 	    plat->high = sec->floorheight;
        // 	    plat->wait = 35*PLATWAIT;
        // 	    plat->status = down;
        // 	    S_StartSound((mobj_t *)&sec->soundorg,sfx_pstart);
        // 	    break;
        //
        // 	  case blazeDWUS:
        // 	    plat->speed = PLATSPEED * 8;
        // 	    plat->low = P_FindLowestFloorSurrounding(sec);
        //
        // 	    if (plat->low > sec->floorheight)
        // 		plat->low = sec->floorheight;
        //
        // 	    plat->high = sec->floorheight;
        // 	    plat->wait = 35*PLATWAIT;
        // 	    plat->status = down;
        // 	    S_StartSound((mobj_t *)&sec->soundorg,sfx_pstart);
        // 	    break;
        //
        // 	  case perpetualRaise:
        // 	    plat->speed = PLATSPEED;
        // 	    plat->low = P_FindLowestFloorSurrounding(sec);
        //
        // 	    if (plat->low > sec->floorheight)
        // 		plat->low = sec->floorheight;
        //
        // 	    plat->high = P_FindHighestFloorSurrounding(sec);
        //
        // 	    if (plat->high < sec->floorheight)
        // 		plat->high = sec->floorheight;
        //
        // 	    plat->wait = 35*PLATWAIT;
        // 	    plat->status = P_Random()&1;
        //
        // 	    S_StartSound((mobj_t *)&sec->soundorg,sfx_pstart);
        // 	    break;
        // 	}
        // 	P_AddActivePlat(plat);
        //     }
        todo!("while statement not yet translated");
        return rtn;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_ActivateInStasis(mut tag: std::ffi::c_int) {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: for statement not yet translated:
        //
        //
        //     for (i = 0;i < MAXPLATS;i++)
        // 	if (activeplats[i]
        // 	    && (activeplats[i])->tag == tag
        // 	    && (activeplats[i])->status == in_stasis)
        // 	{
        // 	    (activeplats[i])->status = (activeplats[i])->oldstatus;
        // 	    (activeplats[i])->thinker.function.acp1
        // 	      = (actionf_p1) T_PlatRaise;
        // 	}
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn EV_StopPlat(mut line: *mut line_t) {
    unsafe {
        let mut j: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: for statement not yet translated:
        //
        //
        //     for (j = 0;j < MAXPLATS;j++)
        // 	if (activeplats[j]
        // 	    && ((activeplats[j])->status != in_stasis)
        // 	    && ((activeplats[j])->tag == line->tag))
        // 	{
        // 	    (activeplats[j])->oldstatus = (activeplats[j])->status;
        // 	    (activeplats[j])->status = in_stasis;
        // 	    (activeplats[j])->thinker.function.acv = (actionf_v)NULL;
        // 	}
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_AddActivePlat(mut plat: *mut plat_t) {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: for statement not yet translated:
        //
        //
        //     for (i = 0;i < MAXPLATS;i++)
        // 	if (activeplats[i] == NULL)
        // 	{
        // 	    activeplats[i] = plat;
        // 	    return;
        // 	}
        todo!("for statement not yet translated");
        I_Error((c"P_AddActivePlat: no more plats!").as_ptr());
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_RemoveActivePlat(mut plat: *mut plat_t) {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: for statement not yet translated:
        //
        //     for (i = 0;i < MAXPLATS;i++)
        // 	if (plat == activeplats[i])
        // 	{
        // 	    (activeplats[i])->sector->specialdata = NULL;
        // 	    P_RemoveThinker(&(activeplats[i])->thinker);
        // 	    activeplats[i] = NULL;
        //
        // 	    return;
        // 	}
        todo!("for statement not yet translated");
        I_Error((c"P_RemoveActivePlat: can't find plat!").as_ptr());
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}
