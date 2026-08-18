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
use crate::g_game::*;
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

static mut rcsid: [std::ffi::c_char; 50] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        112 as std::ffi::c_char,
        95 as std::ffi::c_char,
        101 as std::ffi::c_char,
        110 as std::ffi::c_char,
        101 as std::ffi::c_char,
        109 as std::ffi::c_char,
        121 as std::ffi::c_char,
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

pub const DI_EAST: std::ffi::c_int = 0;
pub const DI_NORTHEAST: std::ffi::c_int = DI_EAST + 1;
pub const DI_NORTH: std::ffi::c_int = DI_NORTHEAST + 1;
pub const DI_NORTHWEST: std::ffi::c_int = DI_NORTH + 1;
pub const DI_WEST: std::ffi::c_int = DI_NORTHWEST + 1;
pub const DI_SOUTHWEST: std::ffi::c_int = DI_WEST + 1;
pub const DI_SOUTH: std::ffi::c_int = DI_SOUTHWEST + 1;
pub const DI_SOUTHEAST: std::ffi::c_int = DI_SOUTH + 1;
pub const DI_NODIR: std::ffi::c_int = DI_SOUTHEAST + 1;
pub const NUMDIRS: std::ffi::c_int = DI_NODIR + 1;

pub type dirtype_t = std::ffi::c_int;

pub static mut opposite: [dirtype_t; 9] = unsafe {
    [
        DI_WEST,
        DI_SOUTHWEST,
        DI_SOUTH,
        DI_SOUTHEAST,
        DI_EAST,
        DI_NORTHEAST,
        DI_NORTH,
        DI_NORTHWEST,
        DI_NODIR,
    ]
};

pub static mut diags: [dirtype_t; 4] =
    unsafe { [DI_NORTHWEST, DI_NORTHEAST, DI_SOUTHWEST, DI_SOUTHEAST] };

pub static mut soundtarget: *mut mobj_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn P_RecursiveSound(
    mut sec: *mut sector_t,
    mut soundblocks: std::ffi::c_int,
) {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut check: *mut line_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut other: *mut sector_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     // wake up all monsters in this sector
        //     if (sec->validcount == validcount
        // 	&& sec->soundtraversed <= soundblocks+1)
        //     {
        // 	return;		// already flooded
        //     }
        todo!("if statement not yet translated");
        (*sec).validcount = validcount;
        (*sec).soundtraversed = (soundblocks + 1);
        (*sec).soundtarget = soundtarget;
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ;i<sec->linecount ; i++)
        //     {
        // 	check = sec->lines[i];
        // 	if (! (check->flags & ML_TWOSIDED) )
        // 	    continue;
        //
        // 	P_LineOpening (check);
        //
        // 	if (openrange <= 0)
        // 	    continue;	// closed door
        //
        // 	if ( sides[ check->sidenum[0] ].sector == sec)
        // 	    other = sides[ check->sidenum[1] ] .sector;
        // 	else
        // 	    other = sides[ check->sidenum[0] ].sector;
        //
        // 	if (check->flags & ML_SOUNDBLOCK)
        // 	{
        // 	    if (!soundblocks)
        // 		P_RecursiveSound (other, 1);
        // 	}
        // 	else
        // 	    P_RecursiveSound (other, soundblocks);
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_NoiseAlert(mut target: *mut mobj_t, mut emmiter: *mut mobj_t) {
    unsafe {
        soundtarget = target;
        {
            let __macro_tmp = validcount;
            validcount += 1;
            __macro_tmp
        };
        P_RecursiveSound((*(*emmiter).subsector).sector, 0);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_CheckMeleeRange(mut actor: *mut mobj_t) -> boolean {
    unsafe {
        let mut pl: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut dist: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (!actor->target)
        // 	return false;
        todo!("if statement not yet translated");
        pl = (*actor).target;
        dist = P_AproxDistance(((*pl).x - (*actor).x), ((*pl).y - (*actor).y));
        // TODO: if statement not yet translated:
        //
        //
        //     if (dist >= MELEERANGE-20*FRACUNIT+pl->info->radius)
        // 	return false;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (! P_CheckSight (actor, actor->target) )
        // 	return false;
        todo!("if statement not yet translated");
        return true_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_CheckMissileRange(mut actor: *mut mobj_t) -> boolean {
    unsafe {
        let mut dist: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (! P_CheckSight (actor, actor->target) )
        // 	return false;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if ( actor->flags & MF_JUSTHIT )
        //     {
        // 	// the target just hit the enemy,
        // 	// so fight back!
        // 	actor->flags &= ~MF_JUSTHIT;
        // 	return true;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (actor->reactiontime)
        // 	return false;	// do not attack yet
        todo!("if statement not yet translated");
        dist = (P_AproxDistance(
            ((*actor).x - (*(*actor).target).x),
            ((*actor).y - (*(*actor).target).y),
        ) - (64 * FRACUNIT));
        // TODO: if statement not yet translated:
        //
        //
        //     if (!actor->info->meleestate)
        // 	dist -= 128*FRACUNIT;	// no melee attack, so fire more
        todo!("if statement not yet translated");
        dist >>= 16;
        // TODO: if statement not yet translated:
        //
        //
        //     if (actor->type == MT_VILE)
        //     {
        // 	if (dist > 14*64)
        // 	    return false;	// too far away
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //
        //     if (actor->type == MT_UNDEAD)
        //     {
        // 	if (dist < 196)
        // 	    return false;	// close for fist attack
        // 	dist >>= 1;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //
        //     if (actor->type == MT_CYBORG
        // 	|| actor->type == MT_SPIDER
        // 	|| actor->type == MT_SKULL)
        //     {
        // 	dist >>= 1;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (dist > 200)
        // 	dist = 200;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (actor->type == MT_CYBORG && dist > 160)
        // 	dist = 160;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (P_Random () < dist)
        // 	return false;
        todo!("if statement not yet translated");
        return true_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub static mut xspeed: [fixed_t; 8] = unsafe {
    [
        FRACUNIT,
        47000,
        0,
        (-(47000)),
        (-(FRACUNIT)),
        (-(47000)),
        0,
        47000,
    ]
};

pub static mut yspeed: [fixed_t; 8] = unsafe {
    [
        0,
        47000,
        FRACUNIT,
        47000,
        0,
        (-(47000)),
        (-(FRACUNIT)),
        (-(47000)),
    ]
};

pub const MAXSPECIALCROSS: std::ffi::c_int = 8;

unsafe extern "C" {
    pub static mut spechit: [*mut line_t; (MAXSPECIALCROSS) as usize];
}

unsafe extern "C" {
    pub static mut numspechit: std::ffi::c_int;
}

pub unsafe extern "C" fn P_Move(mut actor: *mut mobj_t) -> boolean {
    unsafe {
        let mut tryx: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut tryy: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut ld: *mut line_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut try_ok: boolean = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut good: boolean = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (actor->movedir == DI_NODIR)
        // 	return false;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if ((unsigned)actor->movedir >= 8)
        // 	I_Error ("Weird actor->movedir!");
        todo!("if statement not yet translated");
        tryx = ((*actor).x + ((*(*actor).info).speed * xspeed[((*actor).movedir) as usize]));
        tryy = ((*actor).y + ((*(*actor).info).speed * yspeed[((*actor).movedir) as usize]));
        try_ok = P_TryMove(actor, tryx, tryy);
        // TODO: if statement not yet translated:
        //
        //
        //     if (!try_ok)
        //     {
        // 	// open any specials
        // 	if (actor->flags & MF_FLOAT && floatok)
        // 	{
        // 	    // must adjust height
        // 	    if (actor->z < tmfloorz)
        // 		actor->z += FLOATSPEED;
        // 	    else
        // 		actor->z -= FLOATSPEED;
        //
        // 	    actor->flags |= MF_INFLOAT;
        // 	    return true;
        // 	}
        //
        // 	if (!numspechit)
        // 	    return false;
        //
        // 	actor->movedir = DI_NODIR;
        // 	good = false;
        // 	while (numspechit--)
        // 	{
        // 	    ld = spechit[numspechit];
        // 	    // if the special is not a door
        // 	    // that can be opened,
        // 	    // return false
        // 	    if (P_UseSpecialLine (actor, ld,0))
        // 		good = true;
        // 	}
        // 	return good;
        //     }
        //     else
        //     {
        // 	actor->flags &= ~MF_INFLOAT;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //
        //     if (! (actor->flags & MF_FLOAT) )
        // 	actor->z = actor->floorz;
        todo!("if statement not yet translated");
        return true_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_TryWalk(mut actor: *mut mobj_t) -> boolean {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (!P_Move (actor))
        //     {
        // 	return false;
        //     }
        todo!("if statement not yet translated");
        (*actor).movecount = (P_Random() & 15);
        return true_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_NewChaseDir(mut actor: *mut mobj_t) {
    unsafe {
        let mut deltax: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut deltay: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut d: [dirtype_t; (3) as usize] = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut tdir: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut olddir: dirtype_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut turnaround: dirtype_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (!actor->target)
        // 	I_Error ("P_NewChaseDir: called with no target");
        todo!("if statement not yet translated");
        olddir = (*actor).movedir;
        turnaround = opposite[(olddir) as usize];
        deltax = ((*(*actor).target).x - (*actor).x);
        deltay = ((*(*actor).target).y - (*actor).y);
        // TODO: if statement not yet translated:
        //
        //
        //     if (deltax>10*FRACUNIT)
        // 	d[1]= DI_EAST;
        //     else if (deltax<-10*FRACUNIT)
        // 	d[1]= DI_WEST;
        //     else
        // 	d[1]=DI_NODIR;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (deltay<-10*FRACUNIT)
        // 	d[2]= DI_SOUTH;
        //     else if (deltay>10*FRACUNIT)
        // 	d[2]= DI_NORTH;
        //     else
        // 	d[2]=DI_NODIR;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // try direct route
        //     if (d[1] != DI_NODIR
        // 	&& d[2] != DI_NODIR)
        //     {
        // 	actor->movedir = diags[((deltay<0)<<1)+(deltax>0)];
        // 	if (actor->movedir != turnaround && P_TryWalk(actor))
        // 	    return;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // try other directions
        //     if (P_Random() > 200
        // 	||  abs(deltay)>abs(deltax))
        //     {
        // 	tdir=d[1];
        // 	d[1]=d[2];
        // 	d[2]=tdir;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (d[1]==turnaround)
        // 	d[1]=DI_NODIR;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (d[2]==turnaround)
        // 	d[2]=DI_NODIR;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (d[1]!=DI_NODIR)
        //     {
        // 	actor->movedir = d[1];
        // 	if (P_TryWalk(actor))
        // 	{
        // 	    // either moved forward or attacked
        // 	    return;
        // 	}
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (d[2]!=DI_NODIR)
        //     {
        // 	actor->movedir =d[2];
        //
        // 	if (P_TryWalk(actor))
        // 	    return;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // there is no direct path to the player,
        //     // so pick another direction.
        //     if (olddir!=DI_NODIR)
        //     {
        // 	actor->movedir =olddir;
        //
        // 	if (P_TryWalk(actor))
        // 	    return;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // randomly determine direction of search
        //     if (P_Random()&1)
        //     {
        // 	for ( tdir=DI_EAST;
        // 	      tdir<=DI_SOUTHEAST;
        // 	      tdir++ )
        // 	{
        // 	    if (tdir!=turnaround)
        // 	    {
        // 		actor->movedir =tdir;
        //
        // 		if ( P_TryWalk(actor) )
        // 		    return;
        // 	    }
        // 	}
        //     }
        //     else
        //     {
        // 	for ( tdir=DI_SOUTHEAST;
        // 	      tdir != (DI_EAST-1);
        // 	      tdir-- )
        // 	{
        // 	    if (tdir!=turnaround)
        // 	    {
        // 		actor->movedir =tdir;
        //
        // 		if ( P_TryWalk(actor) )
        // 		    return;
        // 	    }
        // 	}
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (turnaround !=  DI_NODIR)
        //     {
        // 	actor->movedir =turnaround;
        // 	if ( P_TryWalk(actor) )
        // 	    return;
        //     }
        todo!("if statement not yet translated");
        (*actor).movedir = DI_NODIR;
    }
}

pub unsafe extern "C" fn P_LookForPlayers(
    mut actor: *mut mobj_t,
    mut allaround: boolean,
) -> boolean {
    unsafe {
        let mut c: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut stop: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut player: *mut player_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut sector: *mut sector_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut an: angle_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut dist: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        sector = (*(*actor).subsector).sector;
        c = 0;
        stop = (((*actor).lastlook - 1) & 3);
        // TODO: for statement not yet translated:
        //
        //
        //     for ( ; ; actor->lastlook = (actor->lastlook+1)&3 )
        //     {
        // 	if (!playeringame[actor->lastlook])
        // 	    continue;
        //
        // 	if (c++ == 2
        // 	    || actor->lastlook == stop)
        // 	{
        // 	    // done looking
        // 	    return false;
        // 	}
        //
        // 	player = &players[actor->lastlook];
        //
        // 	if (player->health <= 0)
        // 	    continue;		// dead
        //
        // 	if (!P_CheckSight (actor, player->mo))
        // 	    continue;		// out of sight
        //
        // 	if (!allaround)
        // 	{
        // 	    an = R_PointToAngle2 (actor->x,
        // 				  actor->y,
        // 				  player->mo->x,
        // 				  player->mo->y)
        // 		- actor->angle;
        //
        // 	    if (an > ANG90 && an < ANG270)
        // 	    {
        // 		dist = P_AproxDistance (player->mo->x - actor->x,
        // 					player->mo->y - actor->y);
        // 		// if real close, react anyway
        // 		if (dist > MELEERANGE)
        // 		    continue;	// behind back
        // 	    }
        // 	}
        //
        // 	actor->target = player->mo;
        // 	return true;
        //     }
        todo!("for statement not yet translated");
        return false_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn A_KeenDie(mut mo: *mut mobj_t) {
    unsafe {
        let mut th: *mut thinker_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut mo2: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut junk: line_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        A_Fall(mo);
        // TODO: for statement not yet translated:
        //
        //
        //     // scan the remaining thinkers
        //     // to see if all Keens are dead
        //     for (th = thinkercap.next ; th != &thinkercap ; th=th->next)
        //     {
        // 	if (th->function.acp1 != (actionf_p1)P_MobjThinker)
        // 	    continue;
        //
        // 	mo2 = (mobj_t *)th;
        // 	if (mo2 != mo
        // 	    && mo2->type == mo->type
        // 	    && mo2->health > 0)
        // 	{
        // 	    // other Keen not dead
        // 	    return;
        // 	}
        //     }
        todo!("for statement not yet translated");
        junk.tag = 666;
        EV_DoDoor((&(junk) as *const _ as *mut _), open);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_Look(mut actor: *mut mobj_t) {
    unsafe {
        let mut targ: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        (*actor).threshold = 0;
        targ = (*(*(*actor).subsector).sector).soundtarget;
        // TODO: if statement not yet translated:
        //
        //
        //     if (targ
        // 	&& (targ->flags & MF_SHOOTABLE) )
        //     {
        // 	actor->target = targ;
        //
        // 	if ( actor->flags & MF_AMBUSH )
        // 	{
        // 	    if (P_CheckSight (actor, actor->target))
        // 		goto seeyou;
        // 	}
        // 	else
        // 	    goto seeyou;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //
        //     if (!P_LookForPlayers (actor, false) )
        // 	return;
        todo!("if statement not yet translated");
        // C label seeyou: (goto targets are not translated)
        // TODO: if statement not yet translated:
        //
        //
        //     // go into chase state
        //   seeyou:
        //     if (actor->info->seesound)
        //     {
        // 	int		sound;
        //
        // 	switch (actor->info->seesound)
        // 	{
        // 	  case sfx_posit1:
        // 	  case sfx_posit2:
        // 	  case sfx_posit3:
        // 	    sound = sfx_posit1+P_Random()%3;
        // 	    break;
        //
        // 	  case sfx_bgsit1:
        // 	  case sfx_bgsit2:
        // 	    sound = sfx_bgsit1+P_Random()%2;
        // 	    break;
        //
        // 	  default:
        // 	    sound = actor->info->seesound;
        // 	    break;
        // 	}
        //
        // 	if (actor->type==MT_SPIDER
        // 	    || actor->type == MT_CYBORG)
        // 	{
        // 	    // full volume
        // 	    S_StartSound (NULL, sound);
        // 	}
        // 	else
        // 	    S_StartSound (actor, sound);
        //     }
        todo!("if statement not yet translated");
        P_SetMobjState(actor, (*(*actor).info).seestate);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_Chase(mut actor: *mut mobj_t) {
    unsafe {
        let mut delta: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (actor->reactiontime)
        // 	actor->reactiontime--;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //
        //     // modify target threshold
        //     if  (actor->threshold)
        //     {
        // 	if (!actor->target
        // 	    || actor->target->health <= 0)
        // 	{
        // 	    actor->threshold = 0;
        // 	}
        // 	else
        // 	    actor->threshold--;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // turn towards movement direction if not there yet
        //     if (actor->movedir < 8)
        //     {
        // 	actor->angle &= (7<<29);
        // 	delta = actor->angle - (actor->movedir << 29);
        //
        // 	if (delta > 0)
        // 	    actor->angle -= ANG90/2;
        // 	else if (delta < 0)
        // 	    actor->angle += ANG90/2;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (!actor->target
        // 	|| !(actor->target->flags&MF_SHOOTABLE))
        //     {
        // 	// look for a new target
        // 	if (P_LookForPlayers(actor,true))
        // 	    return; 	// got a new target
        //
        // 	P_SetMobjState (actor, actor->info->spawnstate);
        // 	return;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // do not attack twice in a row
        //     if (actor->flags & MF_JUSTATTACKED)
        //     {
        // 	actor->flags &= ~MF_JUSTATTACKED;
        // 	if (gameskill != sk_nightmare && !fastparm)
        // 	    P_NewChaseDir (actor);
        // 	return;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // check for melee attack
        //     if (actor->info->meleestate
        // 	&& P_CheckMeleeRange (actor))
        //     {
        // 	if (actor->info->attacksound)
        // 	    S_StartSound (actor, actor->info->attacksound);
        //
        // 	P_SetMobjState (actor, actor->info->meleestate);
        // 	return;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // check for missile attack
        //     if (actor->info->missilestate)
        //     {
        // 	if (gameskill < sk_nightmare
        // 	    && !fastparm && actor->movecount)
        // 	{
        // 	    goto nomissile;
        // 	}
        //
        // 	if (!P_CheckMissileRange (actor))
        // 	    goto nomissile;
        //
        // 	P_SetMobjState (actor, actor->info->missilestate);
        // 	actor->flags |= MF_JUSTATTACKED;
        // 	return;
        //     }
        todo!("if statement not yet translated");
        // C label nomissile: (goto targets are not translated)
        // TODO: if statement not yet translated:
        //
        //
        //     // ?
        //   nomissile:
        //     // possibly choose another target
        //     if (netgame
        // 	&& !actor->threshold
        // 	&& !P_CheckSight (actor, actor->target) )
        //     {
        // 	if (P_LookForPlayers(actor,true))
        // 	    return;	// got a new target
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // chase towards player
        //     if (--actor->movecount<0
        // 	|| !P_Move (actor))
        //     {
        // 	P_NewChaseDir (actor);
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // make active sound
        //     if (actor->info->activesound
        // 	&& P_Random () < 3)
        //     {
        // 	S_StartSound (actor, actor->info->activesound);
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_FaceTarget(mut actor: *mut mobj_t) {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (!actor->target)
        // 	return;
        todo!("if statement not yet translated");
        (*actor).flags &= (!(MF_AMBUSH));
        (*actor).angle = R_PointToAngle2(
            (*actor).x,
            (*actor).y,
            (*(*actor).target).x,
            (*(*actor).target).y,
        );
        // TODO: if statement not yet translated:
        //
        //
        //     if (actor->target->flags & MF_SHADOW)
        // 	actor->angle += (P_Random()-P_Random())<<21;
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_PosAttack(mut actor: *mut mobj_t) {
    unsafe {
        let mut angle: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut damage: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut slope: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (!actor->target)
        // 	return;
        todo!("if statement not yet translated");
        A_FaceTarget(actor);
        angle = (*actor).angle;
        slope = P_AimLineAttack(actor, angle, MISSILERANGE);
        S_StartSound(actor, sfx_pistol);
        angle += ((P_Random() - P_Random()) << 20);
        damage = (((P_Random() % 5) + 1) * 3);
        P_LineAttack(actor, angle, MISSILERANGE, slope, damage);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_SPosAttack(mut actor: *mut mobj_t) {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut angle: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut bangle: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut damage: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut slope: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (!actor->target)
        // 	return;
        todo!("if statement not yet translated");
        S_StartSound(actor, sfx_shotgn);
        A_FaceTarget(actor);
        bangle = (*actor).angle;
        slope = P_AimLineAttack(actor, bangle, MISSILERANGE);
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<3 ; i++)
        //     {
        // 	angle = bangle + ((P_Random()-P_Random())<<20);
        // 	damage = ((P_Random()%5)+1)*3;
        // 	P_LineAttack (actor, angle, MISSILERANGE, slope, damage);
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_CPosAttack(mut actor: *mut mobj_t) {
    unsafe {
        let mut angle: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut bangle: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut damage: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut slope: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (!actor->target)
        // 	return;
        todo!("if statement not yet translated");
        S_StartSound(actor, sfx_shotgn);
        A_FaceTarget(actor);
        bangle = (*actor).angle;
        slope = P_AimLineAttack(actor, bangle, MISSILERANGE);
        angle = (bangle + ((P_Random() - P_Random()) << 20));
        damage = (((P_Random() % 5) + 1) * 3);
        P_LineAttack(actor, angle, MISSILERANGE, slope, damage);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_CPosRefire(mut actor: *mut mobj_t) {
    unsafe {
        A_FaceTarget(actor);
        // TODO: if statement not yet translated:
        //
        //
        //     if (P_Random () < 40)
        // 	return;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (!actor->target
        // 	|| actor->target->health <= 0
        // 	|| !P_CheckSight (actor, actor->target) )
        //     {
        // 	P_SetMobjState (actor, actor->info->seestate);
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_SpidRefire(mut actor: *mut mobj_t) {
    unsafe {
        A_FaceTarget(actor);
        // TODO: if statement not yet translated:
        //
        //
        //     if (P_Random () < 10)
        // 	return;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (!actor->target
        // 	|| actor->target->health <= 0
        // 	|| !P_CheckSight (actor, actor->target) )
        //     {
        // 	P_SetMobjState (actor, actor->info->seestate);
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_BspiAttack(mut actor: *mut mobj_t) {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (!actor->target)
        // 	return;
        todo!("if statement not yet translated");
        A_FaceTarget(actor);
        P_SpawnMissile(actor, (*actor).target, MT_ARACHPLAZ);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_TroopAttack(mut actor: *mut mobj_t) {
    unsafe {
        let mut damage: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (!actor->target)
        // 	return;
        todo!("if statement not yet translated");
        A_FaceTarget(actor);
        // TODO: if statement not yet translated:
        //
        //     if (P_CheckMeleeRange (actor))
        //     {
        // 	S_StartSound (actor, sfx_claw);
        // 	damage = (P_Random()%8+1)*3;
        // 	P_DamageMobj (actor->target, actor, actor, damage);
        // 	return;
        //     }
        todo!("if statement not yet translated");
        P_SpawnMissile(actor, (*actor).target, MT_TROOPSHOT);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_SargAttack(mut actor: *mut mobj_t) {
    unsafe {
        let mut damage: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (!actor->target)
        // 	return;
        todo!("if statement not yet translated");
        A_FaceTarget(actor);
        // TODO: if statement not yet translated:
        //
        //     if (P_CheckMeleeRange (actor))
        //     {
        // 	damage = ((P_Random()%10)+1)*4;
        // 	P_DamageMobj (actor->target, actor, actor, damage);
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_HeadAttack(mut actor: *mut mobj_t) {
    unsafe {
        let mut damage: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (!actor->target)
        // 	return;
        todo!("if statement not yet translated");
        A_FaceTarget(actor);
        // TODO: if statement not yet translated:
        //
        //     if (P_CheckMeleeRange (actor))
        //     {
        // 	damage = (P_Random()%6+1)*10;
        // 	P_DamageMobj (actor->target, actor, actor, damage);
        // 	return;
        //     }
        todo!("if statement not yet translated");
        P_SpawnMissile(actor, (*actor).target, MT_HEADSHOT);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_CyberAttack(mut actor: *mut mobj_t) {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (!actor->target)
        // 	return;
        todo!("if statement not yet translated");
        A_FaceTarget(actor);
        P_SpawnMissile(actor, (*actor).target, MT_ROCKET);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_BruisAttack(mut actor: *mut mobj_t) {
    unsafe {
        let mut damage: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (!actor->target)
        // 	return;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (P_CheckMeleeRange (actor))
        //     {
        // 	S_StartSound (actor, sfx_claw);
        // 	damage = (P_Random()%8+1)*10;
        // 	P_DamageMobj (actor->target, actor, actor, damage);
        // 	return;
        //     }
        todo!("if statement not yet translated");
        P_SpawnMissile(actor, (*actor).target, MT_BRUISERSHOT);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_SkelMissile(mut actor: *mut mobj_t) {
    unsafe {
        let mut mo: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (!actor->target)
        // 	return;
        todo!("if statement not yet translated");
        A_FaceTarget(actor);
        (*actor).z += (16 * FRACUNIT);
        mo = P_SpawnMissile(actor, (*actor).target, MT_TRACER);
        (*actor).z -= (16 * FRACUNIT);
        (*mo).x += (*mo).momx;
        (*mo).y += (*mo).momy;
        (*mo).tracer = (*actor).target;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub static mut TRACEANGLE: std::ffi::c_int = unsafe { 0xc000000 };

pub unsafe extern "C" fn A_Tracer(mut actor: *mut mobj_t) {
    unsafe {
        let mut exact: angle_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut dist: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut slope: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut dest: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut th: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (gametic & 3)
        // 	return;
        todo!("if statement not yet translated");
        P_SpawnPuff((*actor).x, (*actor).y, (*actor).z);
        th = P_SpawnMobj(
            ((*actor).x - (*actor).momx),
            ((*actor).y - (*actor).momy),
            (*actor).z,
            MT_SMOKE,
        );
        (*th).momz = FRACUNIT;
        (*th).tics -= (P_Random() & 3);
        // TODO: if statement not yet translated:
        //
        //     if (th->tics < 1)
        // 	th->tics = 1;
        todo!("if statement not yet translated");
        dest = (*actor).tracer;
        // TODO: if statement not yet translated:
        //
        //
        //     if (!dest || dest->health <= 0)
        // 	return;
        todo!("if statement not yet translated");
        exact = R_PointToAngle2((*actor).x, (*actor).y, (*dest).x, (*dest).y);
        // TODO: if statement not yet translated:
        //
        //
        //     if (exact != actor->angle)
        //     {
        // 	if (exact - actor->angle > 0x80000000)
        // 	{
        // 	    actor->angle -= TRACEANGLE;
        // 	    if (exact - actor->angle < 0x80000000)
        // 		actor->angle = exact;
        // 	}
        // 	else
        // 	{
        // 	    actor->angle += TRACEANGLE;
        // 	    if (exact - actor->angle > 0x80000000)
        // 		actor->angle = exact;
        // 	}
        //     }
        todo!("if statement not yet translated");
        exact = ((*actor).angle >> ANGLETOFINESHIFT);
        (*actor).momx = FixedMul((*(*actor).info).speed, finecosine[(exact) as usize]);
        (*actor).momy = FixedMul((*(*actor).info).speed, finesine[(exact) as usize]);
        dist = P_AproxDistance(((*dest).x - (*actor).x), ((*dest).y - (*actor).y));
        dist = (dist / (*(*actor).info).speed);
        // TODO: if statement not yet translated:
        //
        //
        //     if (dist < 1)
        // 	dist = 1;
        todo!("if statement not yet translated");
        slope = ((((*dest).z + (40 * FRACUNIT)) - (*actor).z) / dist);
        // TODO: if statement not yet translated:
        //
        //
        //     if (slope < actor->momz)
        // 	actor->momz -= FRACUNIT/8;
        //     else
        // 	actor->momz += FRACUNIT/8;
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_SkelWhoosh(mut actor: *mut mobj_t) {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (!actor->target)
        // 	return;
        todo!("if statement not yet translated");
        A_FaceTarget(actor);
        S_StartSound(actor, sfx_skeswg);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_SkelFist(mut actor: *mut mobj_t) {
    unsafe {
        let mut damage: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (!actor->target)
        // 	return;
        todo!("if statement not yet translated");
        A_FaceTarget(actor);
        // TODO: if statement not yet translated:
        //
        //
        //     if (P_CheckMeleeRange (actor))
        //     {
        // 	damage = ((P_Random()%10)+1)*6;
        // 	S_StartSound (actor, sfx_skepch);
        // 	P_DamageMobj (actor->target, actor, actor, damage);
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub static mut corpsehit: *mut mobj_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut vileobj: *mut mobj_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut viletryx: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut viletryy: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn PIT_VileCheck(mut thing: *mut mobj_t) -> boolean {
    unsafe {
        let mut maxdist: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut check: boolean = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (!(thing->flags & MF_CORPSE) )
        // 	return true;	// not a monster
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (thing->tics != -1)
        // 	return true;	// not lying still yet
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (thing->info->raisestate == S_NULL)
        // 	return true;	// monster doesn't have a raise state
        todo!("if statement not yet translated");
        maxdist = ((*(*thing).info).radius + mobjinfo[(MT_VILE) as usize].radius);
        // TODO: if statement not yet translated:
        //
        //
        //     if ( abs(thing->x - viletryx) > maxdist
        // 	 || abs(thing->y - viletryy) > maxdist )
        // 	return true;		// not actually touching
        todo!("if statement not yet translated");
        corpsehit = thing;
        (*corpsehit).momx = (*corpsehit).momy = 0;
        (*corpsehit).height <<= 2;
        check = P_CheckPosition(corpsehit, (*corpsehit).x, (*corpsehit).y);
        (*corpsehit).height >>= 2;
        // TODO: if statement not yet translated:
        //
        //
        //     if (!check)
        // 	return true;		// doesn't fit here
        todo!("if statement not yet translated");
        return false_;
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn A_VileChase(mut actor: *mut mobj_t) {
    unsafe {
        let mut xl: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut xh: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut yl: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut yh: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut bx: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut by: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut info: *mut mobjinfo_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut temp: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (actor->movedir != DI_NODIR)
        //     {
        // 	// check for corpses to raise
        // 	viletryx =
        // 	    actor->x + actor->info->speed*xspeed[actor->movedir];
        // 	viletryy =
        // 	    actor->y + actor->info->speed*yspeed[actor->movedir];
        //
        // 	xl = (viletryx - bmaporgx - MAXRADIUS*2)>>MAPBLOCKSHIFT;
        // 	xh = (viletryx - bmaporgx + MAXRADIUS*2)>>MAPBLOCKSHIFT;
        // 	yl = (viletryy - bmaporgy - MAXRADIUS*2)>>MAPBLOCKSHIFT;
        // 	yh = (viletryy - bmaporgy + MAXRADIUS*2)>>MAPBLOCKSHIFT;
        //
        // 	vileobj = actor;
        // 	for (bx=xl ; bx<=xh ; bx++)
        // 	{
        // 	    for (by=yl ; by<=yh ; by++)
        // 	    {
        // 		// Call PIT_VileCheck to check
        // 		// whether object is a corpse
        // 		// that canbe raised.
        // 		if (!P_BlockThingsIterator(bx,by,PIT_VileCheck))
        // 		{
        // 		    // got one!
        // 		    temp = actor->target;
        // 		    actor->target = corpsehit;
        // 		    A_FaceTarget (actor);
        // 		    actor->target = temp;
        //
        // 		    P_SetMobjState (actor, S_VILE_HEAL1);
        // 		    S_StartSound (corpsehit, sfx_slop);
        // 		    info = corpsehit->info;
        //
        // 		    P_SetMobjState (corpsehit,info->raisestate);
        // 		    corpsehit->height <<= 2;
        // 		    corpsehit->flags = info->flags;
        // 		    corpsehit->health = info->spawnhealth;
        // 		    corpsehit->target = NULL;
        //
        // 		    return;
        // 		}
        // 	    }
        // 	}
        //     }
        todo!("if statement not yet translated");
        A_Chase(actor);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_VileStart(mut actor: *mut mobj_t) {
    unsafe {
        S_StartSound(actor, sfx_vilatk);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_StartFire(mut actor: *mut mobj_t) {
    unsafe {
        S_StartSound(actor, sfx_flamst);
        A_Fire(actor);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_FireCrackle(mut actor: *mut mobj_t) {
    unsafe {
        S_StartSound(actor, sfx_flame);
        A_Fire(actor);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_Fire(mut actor: *mut mobj_t) {
    unsafe {
        let mut dest: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut an: std::ffi::c_uint = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        dest = (*actor).tracer;
        // TODO: if statement not yet translated:
        //
        //     if (!dest)
        // 	return;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // don't move it if the vile lost sight
        //     if (!P_CheckSight (actor->target, dest) )
        // 	return;
        todo!("if statement not yet translated");
        an = ((*dest).angle >> ANGLETOFINESHIFT);
        P_UnsetThingPosition(actor);
        (*actor).x = ((*dest).x + FixedMul((24 * FRACUNIT), finecosine[(an) as usize]));
        (*actor).y = ((*dest).y + FixedMul((24 * FRACUNIT), finesine[(an) as usize]));
        (*actor).z = (*dest).z;
        P_SetThingPosition(actor);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_VileTarget(mut actor: *mut mobj_t) {
    unsafe {
        let mut fog: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (!actor->target)
        // 	return;
        todo!("if statement not yet translated");
        A_FaceTarget(actor);
        fog = P_SpawnMobj(
            (*(*actor).target).x,
            (*(*actor).target).x,
            (*(*actor).target).z,
            MT_FIRE,
        );
        (*actor).tracer = fog;
        (*fog).target = actor;
        (*fog).tracer = (*actor).target;
        A_Fire(fog);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_VileAttack(mut actor: *mut mobj_t) {
    unsafe {
        let mut fire: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut an: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (!actor->target)
        // 	return;
        todo!("if statement not yet translated");
        A_FaceTarget(actor);
        // TODO: if statement not yet translated:
        //
        //
        //     if (!P_CheckSight (actor, actor->target) )
        // 	return;
        todo!("if statement not yet translated");
        S_StartSound(actor, sfx_barexp);
        P_DamageMobj((*actor).target, actor, actor, 20);
        (*(*actor).target).momz = ((1000 * FRACUNIT) / (*(*(*actor).target).info).mass);
        an = ((*actor).angle >> ANGLETOFINESHIFT);
        fire = (*actor).tracer;
        // TODO: if statement not yet translated:
        //
        //
        //     if (!fire)
        // 	return;
        todo!("if statement not yet translated");
        (*fire).x = ((*(*actor).target).x - FixedMul((24 * FRACUNIT), finecosine[(an) as usize]));
        (*fire).y = ((*(*actor).target).y - FixedMul((24 * FRACUNIT), finesine[(an) as usize]));
        P_RadiusAttack(fire, actor, 70);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub const FATSPREAD: std::ffi::c_int = (ANG90 / 8);

pub unsafe extern "C" fn A_FatRaise(mut actor: *mut mobj_t) {
    unsafe {
        A_FaceTarget(actor);
        S_StartSound(actor, sfx_manatk);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_FatAttack1(mut actor: *mut mobj_t) {
    unsafe {
        let mut mo: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut an: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        A_FaceTarget(actor);
        (*actor).angle += FATSPREAD;
        P_SpawnMissile(actor, (*actor).target, MT_FATSHOT);
        mo = P_SpawnMissile(actor, (*actor).target, MT_FATSHOT);
        (*mo).angle += FATSPREAD;
        an = ((*mo).angle >> ANGLETOFINESHIFT);
        (*mo).momx = FixedMul((*(*mo).info).speed, finecosine[(an) as usize]);
        (*mo).momy = FixedMul((*(*mo).info).speed, finesine[(an) as usize]);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_FatAttack2(mut actor: *mut mobj_t) {
    unsafe {
        let mut mo: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut an: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        A_FaceTarget(actor);
        (*actor).angle -= FATSPREAD;
        P_SpawnMissile(actor, (*actor).target, MT_FATSHOT);
        mo = P_SpawnMissile(actor, (*actor).target, MT_FATSHOT);
        (*mo).angle -= (FATSPREAD * 2);
        an = ((*mo).angle >> ANGLETOFINESHIFT);
        (*mo).momx = FixedMul((*(*mo).info).speed, finecosine[(an) as usize]);
        (*mo).momy = FixedMul((*(*mo).info).speed, finesine[(an) as usize]);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_FatAttack3(mut actor: *mut mobj_t) {
    unsafe {
        let mut mo: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut an: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        A_FaceTarget(actor);
        mo = P_SpawnMissile(actor, (*actor).target, MT_FATSHOT);
        (*mo).angle -= (FATSPREAD / 2);
        an = ((*mo).angle >> ANGLETOFINESHIFT);
        (*mo).momx = FixedMul((*(*mo).info).speed, finecosine[(an) as usize]);
        (*mo).momy = FixedMul((*(*mo).info).speed, finesine[(an) as usize]);
        mo = P_SpawnMissile(actor, (*actor).target, MT_FATSHOT);
        (*mo).angle += (FATSPREAD / 2);
        an = ((*mo).angle >> ANGLETOFINESHIFT);
        (*mo).momx = FixedMul((*(*mo).info).speed, finecosine[(an) as usize]);
        (*mo).momy = FixedMul((*(*mo).info).speed, finesine[(an) as usize]);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub const SKULLSPEED: std::ffi::c_int = (20 * FRACUNIT);

pub unsafe extern "C" fn A_SkullAttack(mut actor: *mut mobj_t) {
    unsafe {
        let mut dest: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut an: angle_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut dist: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (!actor->target)
        // 	return;
        todo!("if statement not yet translated");
        dest = (*actor).target;
        (*actor).flags |= MF_SKULLFLY;
        S_StartSound(actor, (*(*actor).info).attacksound);
        A_FaceTarget(actor);
        an = ((*actor).angle >> ANGLETOFINESHIFT);
        (*actor).momx = FixedMul(SKULLSPEED, finecosine[(an) as usize]);
        (*actor).momy = FixedMul(SKULLSPEED, finesine[(an) as usize]);
        dist = P_AproxDistance(((*dest).x - (*actor).x), ((*dest).y - (*actor).y));
        dist = (dist / SKULLSPEED);
        // TODO: if statement not yet translated:
        //
        //
        //     if (dist < 1)
        // 	dist = 1;
        todo!("if statement not yet translated");
        (*actor).momz = ((((*dest).z + ((*dest).height >> 1)) - (*actor).z) / dist);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_PainShootSkull(mut actor: *mut mobj_t, mut angle: angle_t) {
    unsafe {
        let mut x: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut y: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut z: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut newmobj: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut an: angle_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut prestep: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut count: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut currentthinker: *mut thinker_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        count = 0;
        currentthinker = thinkercap.next;
        // TODO: while statement not yet translated:
        //
        //     while (currentthinker != &thinkercap)
        //     {
        // 	if (   (currentthinker->function.acp1 == (actionf_p1)P_MobjThinker)
        // 	    && ((mobj_t *)currentthinker)->type == MT_SKULL)
        // 	    count++;
        // 	currentthinker = currentthinker->next;
        //     }
        todo!("while statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // if there are allready 20 skulls on the level,
        //     // don't spit another one
        //     if (count > 20)
        // 	return;
        todo!("if statement not yet translated");
        an = (angle >> ANGLETOFINESHIFT);
        prestep = ((4 * FRACUNIT)
            + ((3 * ((*(*actor).info).radius + mobjinfo[(MT_SKULL) as usize].radius)) / 2));
        x = ((*actor).x + FixedMul(prestep, finecosine[(an) as usize]));
        y = ((*actor).y + FixedMul(prestep, finesine[(an) as usize]));
        z = ((*actor).z + (8 * FRACUNIT));
        newmobj = P_SpawnMobj(x, y, z, MT_SKULL);
        // TODO: if statement not yet translated:
        //
        //
        //     // Check for movements.
        //     if (!P_TryMove (newmobj, newmobj->x, newmobj->y))
        //     {
        // 	// kill it immediately
        // 	P_DamageMobj (newmobj,actor,actor,10000);
        // 	return;
        //     }
        todo!("if statement not yet translated");
        (*newmobj).target = (*actor).target;
        A_SkullAttack(newmobj);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_PainAttack(mut actor: *mut mobj_t) {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (!actor->target)
        // 	return;
        todo!("if statement not yet translated");
        A_FaceTarget(actor);
        A_PainShootSkull(actor, (*actor).angle);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_PainDie(mut actor: *mut mobj_t) {
    unsafe {
        A_Fall(actor);
        A_PainShootSkull(actor, ((*actor).angle + ANG90));
        A_PainShootSkull(actor, ((*actor).angle + ANG180));
        A_PainShootSkull(actor, ((*actor).angle + ANG270));
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_Scream(mut actor: *mut mobj_t) {
    unsafe {
        let mut sound: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: switch statement not yet translated:
        //
        //
        //     switch (actor->info->deathsound)
        //     {
        //       case 0:
        // 	return;
        //
        //       case sfx_podth1:
        //       case sfx_podth2:
        //       case sfx_podth3:
        // 	sound = sfx_podth1 + P_Random ()%3;
        // 	break;
        //
        //       case sfx_bgdth1:
        //       case sfx_bgdth2:
        // 	sound = sfx_bgdth1 + P_Random ()%2;
        // 	break;
        //
        //       default:
        // 	sound = actor->info->deathsound;
        // 	break;
        //     }
        todo!("switch statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // Check for bosses.
        //     if (actor->type==MT_SPIDER
        // 	|| actor->type == MT_CYBORG)
        //     {
        // 	// full volume
        // 	S_StartSound (NULL, sound);
        //     }
        //     else
        // 	S_StartSound (actor, sound);
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_XScream(mut actor: *mut mobj_t) {
    unsafe {
        S_StartSound(actor, sfx_slop);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_Pain(mut actor: *mut mobj_t) {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (actor->info->painsound)
        // 	S_StartSound (actor, actor->info->painsound);
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_Fall(mut actor: *mut mobj_t) {
    unsafe {
        (*actor).flags &= (!(MF_SOLID));
        // TODO: statement not yet translated:
        //
        //
        //     // So change this if corpse objects
        //     // are meant to be obstacles.
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_Explode(mut thingy: *mut mobj_t) {
    unsafe {
        P_RadiusAttack(thingy, (*thingy).target, 128);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_BossDeath(mut mo: *mut mobj_t) {
    unsafe {
        let mut th: *mut thinker_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut mo2: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut junk: line_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if ( gamemode == commercial)
        //     {
        // 	if (gamemap != 7)
        // 	    return;
        //
        // 	if ((mo->type != MT_FATSO)
        // 	    && (mo->type != MT_BABY))
        // 	    return;
        //     }
        //     else
        //     {
        // 	switch(gameepisode)
        // 	{
        // 	  case 1:
        // 	    if (gamemap != 8)
        // 		return;
        //
        // 	    if (mo->type != MT_BRUISER)
        // 		return;
        // 	    break;
        //
        // 	  case 2:
        // 	    if (gamemap != 8)
        // 		return;
        //
        // 	    if (mo->type != MT_CYBORG)
        // 		return;
        // 	    break;
        //
        // 	  case 3:
        // 	    if (gamemap != 8)
        // 		return;
        //
        // 	    if (mo->type != MT_SPIDER)
        // 		return;
        //
        // 	    break;
        //
        // 	  case 4:
        // 	    switch(gamemap)
        // 	    {
        // 	      case 6:
        // 		if (mo->type != MT_CYBORG)
        // 		    return;
        // 		break;
        //
        // 	      case 8:
        // 		if (mo->type != MT_SPIDER)
        // 		    return;
        // 		break;
        //
        // 	      default:
        // 		return;
        // 		break;
        // 	    }
        // 	    break;
        //
        // 	  default:
        // 	    if (gamemap != 8)
        // 		return;
        // 	    break;
        // 	}
        //
        //     }
        todo!("if statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //
        //
        //     // make sure there is a player alive for victory
        //     for (i=0 ; i<MAXPLAYERS ; i++)
        // 	if (playeringame[i] && players[i].health > 0)
        // 	    break;
        todo!("for statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (i==MAXPLAYERS)
        // 	return;	// no one left alive, so do not end game
        todo!("if statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //     // scan the remaining thinkers to see
        //     // if all bosses are dead
        //     for (th = thinkercap.next ; th != &thinkercap ; th=th->next)
        //     {
        // 	if (th->function.acp1 != (actionf_p1)P_MobjThinker)
        // 	    continue;
        //
        // 	mo2 = (mobj_t *)th;
        // 	if (mo2 != mo
        // 	    && mo2->type == mo->type
        // 	    && mo2->health > 0)
        // 	{
        // 	    // other boss not dead
        // 	    return;
        // 	}
        //     }
        todo!("for statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // victory!
        //     if ( gamemode == commercial)
        //     {
        // 	if (gamemap == 7)
        // 	{
        // 	    if (mo->type == MT_FATSO)
        // 	    {
        // 		junk.tag = 666;
        // 		EV_DoFloor(&junk,lowerFloorToLowest);
        // 		return;
        // 	    }
        //
        // 	    if (mo->type == MT_BABY)
        // 	    {
        // 		junk.tag = 667;
        // 		EV_DoFloor(&junk,raiseToTexture);
        // 		return;
        // 	    }
        // 	}
        //     }
        //     else
        //     {
        // 	switch(gameepisode)
        // 	{
        // 	  case 1:
        // 	    junk.tag = 666;
        // 	    EV_DoFloor (&junk, lowerFloorToLowest);
        // 	    return;
        // 	    break;
        //
        // 	  case 4:
        // 	    switch(gamemap)
        // 	    {
        // 	      case 6:
        // 		junk.tag = 666;
        // 		EV_DoDoor (&junk, blazeOpen);
        // 		return;
        // 		break;
        //
        // 	      case 8:
        // 		junk.tag = 666;
        // 		EV_DoFloor (&junk, lowerFloorToLowest);
        // 		return;
        // 		break;
        // 	    }
        // 	}
        //     }
        todo!("if statement not yet translated");
        G_ExitLevel();
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_Hoof(mut mo: *mut mobj_t) {
    unsafe {
        S_StartSound(mo, sfx_hoof);
        A_Chase(mo);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_Metal(mut mo: *mut mobj_t) {
    unsafe {
        S_StartSound(mo, sfx_metal);
        A_Chase(mo);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_BabyMetal(mut mo: *mut mobj_t) {
    unsafe {
        S_StartSound(mo, sfx_bspwlk);
        A_Chase(mo);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_OpenShotgun2(mut player: *mut player_t, mut psp: *mut pspdef_t) {
    unsafe {
        S_StartSound((*player).mo, sfx_dbopn);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_LoadShotgun2(mut player: *mut player_t, mut psp: *mut pspdef_t) {
    unsafe {
        S_StartSound((*player).mo, sfx_dbload);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

unsafe extern "C" {
    pub fn A_ReFire(player: *mut player_t, psp: *mut pspdef_t);
}

pub unsafe extern "C" fn A_CloseShotgun2(mut player: *mut player_t, mut psp: *mut pspdef_t) {
    unsafe {
        S_StartSound((*player).mo, sfx_dbcls);
        A_ReFire(player, psp);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub static mut braintargets: [*mut mobj_t; (32) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut numbraintargets: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut braintargeton: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn A_BrainAwake(mut mo: *mut mobj_t) {
    unsafe {
        let mut thinker: *mut thinker_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut m: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        numbraintargets = 0;
        braintargeton = 0;
        thinker = thinkercap.next;
        // TODO: for statement not yet translated:
        //
        //     for (thinker = thinkercap.next ;
        // 	 thinker != &thinkercap ;
        // 	 thinker = thinker->next)
        //     {
        // 	if (thinker->function.acp1 != (actionf_p1)P_MobjThinker)
        // 	    continue;	// not a mobj
        //
        // 	m = (mobj_t *)thinker;
        //
        // 	if (m->type == MT_BOSSTARGET )
        // 	{
        // 	    braintargets[numbraintargets] = m;
        // 	    numbraintargets++;
        // 	}
        //     }
        todo!("for statement not yet translated");
        S_StartSound(NULL, sfx_bossit);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_BrainPain(mut mo: *mut mobj_t) {
    unsafe {
        S_StartSound(NULL, sfx_bospn);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_BrainScream(mut mo: *mut mobj_t) {
    unsafe {
        let mut x: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut y: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut z: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut th: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: for statement not yet translated:
        //
        //
        //     for (x=mo->x - 196*FRACUNIT ; x< mo->x + 320*FRACUNIT ; x+= FRACUNIT*8)
        //     {
        // 	y = mo->y - 320*FRACUNIT;
        // 	z = 128 + P_Random()*2*FRACUNIT;
        // 	th = P_SpawnMobj (x,y,z, MT_ROCKET);
        // 	th->momz = P_Random()*512;
        //
        // 	P_SetMobjState (th, S_BRAINEXPLODE1);
        //
        // 	th->tics -= P_Random()&7;
        // 	if (th->tics < 1)
        // 	    th->tics = 1;
        //     }
        todo!("for statement not yet translated");
        S_StartSound(NULL, sfx_bosdth);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_BrainExplode(mut mo: *mut mobj_t) {
    unsafe {
        let mut x: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut y: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut z: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut th: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        x = ((*mo).x + ((P_Random() - P_Random()) * 2048));
        y = (*mo).y;
        z = (128 + ((P_Random() * 2) * FRACUNIT));
        th = P_SpawnMobj(x, y, z, MT_ROCKET);
        (*th).momz = (P_Random() * 512);
        P_SetMobjState(th, S_BRAINEXPLODE1);
        (*th).tics -= (P_Random() & 7);
        // TODO: if statement not yet translated:
        //
        //     if (th->tics < 1)
        // 	th->tics = 1;
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_BrainDie(mut mo: *mut mobj_t) {
    unsafe {
        G_ExitLevel();
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_BrainSpit(mut mo: *mut mobj_t) {
    unsafe {
        let mut targ: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut newmobj: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        static mut easy: std::ffi::c_int = unsafe { 0 };
        easy ^= 1;
        // TODO: if statement not yet translated:
        //
        //     if (gameskill <= sk_easy && (!easy))
        // 	return;
        todo!("if statement not yet translated");
        targ = braintargets[(braintargeton) as usize];
        braintargeton = ((braintargeton + 1) % numbraintargets);
        newmobj = P_SpawnMissile(mo, targ, MT_SPAWNSHOT);
        (*newmobj).target = targ;
        (*newmobj).reactiontime =
            ((((*targ).y - (*mo).y) / (*newmobj).momy) / (*(*newmobj).state).tics);
        S_StartSound(NULL, sfx_bospit);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_SpawnSound(mut mo: *mut mobj_t) {
    unsafe {
        S_StartSound(mo, sfx_boscub);
        A_SpawnFly(mo);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_SpawnFly(mut mo: *mut mobj_t) {
    unsafe {
        let mut newmobj: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut fog: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut targ: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut r: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut type_: mobjtype_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (--mo->reactiontime)
        // 	return;	// still flying
        todo!("if statement not yet translated");
        targ = (*mo).target;
        fog = P_SpawnMobj((*targ).x, (*targ).y, (*targ).z, MT_SPAWNFIRE);
        S_StartSound(fog, sfx_telept);
        r = P_Random();
        // TODO: if statement not yet translated:
        //
        //
        //     // Probability distribution (kind of :),
        //     // decreasing likelihood.
        //     if ( r<50 )
        // 	type = MT_TROOP;
        //     else if (r<90)
        // 	type = MT_SERGEANT;
        //     else if (r<120)
        // 	type = MT_SHADOWS;
        //     else if (r<130)
        // 	type = MT_PAIN;
        //     else if (r<160)
        // 	type = MT_HEAD;
        //     else if (r<162)
        // 	type = MT_VILE;
        //     else if (r<172)
        // 	type = MT_UNDEAD;
        //     else if (r<192)
        // 	type = MT_BABY;
        //     else if (r<222)
        // 	type = MT_FATSO;
        //     else if (r<246)
        // 	type = MT_KNIGHT;
        //     else
        // 	type = MT_BRUISER;
        todo!("if statement not yet translated");
        newmobj = P_SpawnMobj((*targ).x, (*targ).y, (*targ).z, type_);
        // TODO: if statement not yet translated:
        //
        //     if (P_LookForPlayers (newmobj, true) )
        // 	P_SetMobjState (newmobj, newmobj->info->seestate);
        todo!("if statement not yet translated");
        P_TeleportMove(newmobj, (*newmobj).x, (*newmobj).y);
        P_RemoveMobj(mo);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_PlayerScream(mut mo: *mut mobj_t) {
    unsafe {
        let mut sound: std::ffi::c_int = unsafe { sfx_pldeth };
        // TODO: if statement not yet translated:
        //
        //
        //     if ( (gamemode == commercial)
        // 	&& 	(mo->health < -50))
        //     {
        // 	// IF THE PLAYER DIES
        // 	// LESS THAN -50% WITHOUT GIBBING
        // 	sound = sfx_pdiehi;
        //     }
        todo!("if statement not yet translated");
        S_StartSound(mo, sound);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}
