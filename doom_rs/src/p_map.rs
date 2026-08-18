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
use crate::m_bbox::*;
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

static mut rcsid: [std::ffi::c_char; 48] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        112 as std::ffi::c_char,
        95 as std::ffi::c_char,
        109 as std::ffi::c_char,
        97 as std::ffi::c_char,
        112 as std::ffi::c_char,
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

pub static mut tmbbox: [fixed_t; (4) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut tmthing: *mut mobj_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut tmflags: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut tmx: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut tmy: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut floatok: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut tmfloorz: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut tmceilingz: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut tmdropoffz: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut ceilingline: *mut line_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub const MAXSPECIALCROSS: std::ffi::c_int = 8;

pub static mut spechit: [*mut line_t; (MAXSPECIALCROSS) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut numspechit: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn PIT_StompThing(mut thing: *mut mobj_t) -> boolean {
    unsafe {
        let mut blockdist: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (!(thing->flags & MF_SHOOTABLE) )
        // 	return true;
        todo!("if statement not yet translated");
        blockdist = ((*thing).radius + (*tmthing).radius);
        // TODO: if statement not yet translated:
        //
        //
        //     if ( abs(thing->x - tmx) >= blockdist
        // 	 || abs(thing->y - tmy) >= blockdist )
        //     {
        // 	// didn't hit it
        // 	return true;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // don't clip against self
        //     if (thing == tmthing)
        // 	return true;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // monsters don't stomp things except on boss level
        //     if ( !tmthing->player && gamemap != 30)
        // 	return false;
        todo!("if statement not yet translated");
        P_DamageMobj(thing, tmthing, tmthing, 10000);
        return true_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_TeleportMove(
    mut thing: *mut mobj_t,
    mut x: fixed_t,
    mut y: fixed_t,
) -> boolean {
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
        let mut newsubsec: *mut subsector_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        tmthing = thing;
        tmflags = (*thing).flags;
        tmx = x;
        tmy = y;
        tmbbox[(BOXTOP) as usize] = (y + (*tmthing).radius);
        tmbbox[(BOXBOTTOM) as usize] = (y - (*tmthing).radius);
        tmbbox[(BOXRIGHT) as usize] = (x + (*tmthing).radius);
        tmbbox[(BOXLEFT) as usize] = (x - (*tmthing).radius);
        newsubsec = R_PointInSubsector(x, y);
        ceilingline = NULL;
        tmfloorz = tmdropoffz = (*(*newsubsec).sector).floorheight;
        tmceilingz = (*(*newsubsec).sector).ceilingheight;
        {
            let __macro_tmp = validcount;
            validcount += 1;
            __macro_tmp
        };
        numspechit = 0;
        xl = (((tmbbox[(BOXLEFT) as usize] - bmaporgx) - MAXRADIUS) >> MAPBLOCKSHIFT);
        xh = (((tmbbox[(BOXRIGHT) as usize] - bmaporgx) + MAXRADIUS) >> MAPBLOCKSHIFT);
        yl = (((tmbbox[(BOXBOTTOM) as usize] - bmaporgy) - MAXRADIUS) >> MAPBLOCKSHIFT);
        yh = (((tmbbox[(BOXTOP) as usize] - bmaporgy) + MAXRADIUS) >> MAPBLOCKSHIFT);
        // TODO: for statement not yet translated:
        //
        //
        //     for (bx=xl ; bx<=xh ; bx++)
        // 	for (by=yl ; by<=yh ; by++)
        // 	    if (!P_BlockThingsIterator(bx,by,PIT_StompThing))
        // 		return false;
        todo!("for statement not yet translated");
        P_UnsetThingPosition(thing);
        (*thing).floorz = tmfloorz;
        (*thing).ceilingz = tmceilingz;
        (*thing).x = x;
        (*thing).y = y;
        P_SetThingPosition(thing);
        return true_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn PIT_CheckLine(mut ld: *mut line_t) -> boolean {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (tmbbox[BOXRIGHT] <= ld->bbox[BOXLEFT]
        // 	|| tmbbox[BOXLEFT] >= ld->bbox[BOXRIGHT]
        // 	|| tmbbox[BOXTOP] <= ld->bbox[BOXBOTTOM]
        // 	|| tmbbox[BOXBOTTOM] >= ld->bbox[BOXTOP] )
        // 	return true;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (P_BoxOnLineSide (tmbbox, ld) != -1)
        // 	return true;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // A line has been hit
        //
        //     // The moving thing's destination position will cross
        //     // the given line.
        //     // If this should not be allowed, return false.
        //     // If the line is special, keep track of it
        //     // to process later if the move is proven ok.
        //     // NOTE: specials are NOT sorted by order,
        //     // so two special lines that are only 8 pixels apart
        //     // could be crossed in either order.
        //
        //     if (!ld->backsector)
        // 	return false;		// one sided line
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (!(tmthing->flags & MF_MISSILE) )
        //     {
        // 	if ( ld->flags & ML_BLOCKING )
        // 	    return false;	// explicitly blocking everything
        //
        // 	if ( !tmthing->player && ld->flags & ML_BLOCKMONSTERS )
        // 	    return false;	// block monsters only
        //     }
        todo!("if statement not yet translated");
        P_LineOpening(ld);
        // TODO: if statement not yet translated:
        //
        //
        //     // adjust floor / ceiling heights
        //     if (opentop < tmceilingz)
        //     {
        // 	tmceilingz = opentop;
        // 	ceilingline = ld;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (openbottom > tmfloorz)
        // 	tmfloorz = openbottom;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (lowfloor < tmdropoffz)
        // 	tmdropoffz = lowfloor;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // if contacted a special line, add it to the list
        //     if (ld->special)
        //     {
        // 	spechit[numspechit] = ld;
        // 	numspechit++;
        //     }
        todo!("if statement not yet translated");
        return true_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn PIT_CheckThing(mut thing: *mut mobj_t) -> boolean {
    unsafe {
        let mut blockdist: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut solid: boolean = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut damage: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (!(thing->flags & (MF_SOLID|MF_SPECIAL|MF_SHOOTABLE) ))
        // 	return true;
        todo!("if statement not yet translated");
        blockdist = ((*thing).radius + (*tmthing).radius);
        // TODO: if statement not yet translated:
        //
        //
        //     if ( abs(thing->x - tmx) >= blockdist
        // 	 || abs(thing->y - tmy) >= blockdist )
        //     {
        // 	// didn't hit it
        // 	return true;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // don't clip against self
        //     if (thing == tmthing)
        // 	return true;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // check for skulls slamming into things
        //     if (tmthing->flags & MF_SKULLFLY)
        //     {
        // 	damage = ((P_Random()%8)+1)*tmthing->info->damage;
        //
        // 	P_DamageMobj (thing, tmthing, tmthing, damage);
        //
        // 	tmthing->flags &= ~MF_SKULLFLY;
        // 	tmthing->momx = tmthing->momy = tmthing->momz = 0;
        //
        // 	P_SetMobjState (tmthing, tmthing->info->spawnstate);
        //
        // 	return false;		// stop moving
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //
        //     // missiles can hit other things
        //     if (tmthing->flags & MF_MISSILE)
        //     {
        // 	// see if it went over / under
        // 	if (tmthing->z > thing->z + thing->height)
        // 	    return true;		// overhead
        // 	if (tmthing->z+tmthing->height < thing->z)
        // 	    return true;		// underneath
        //
        // 	if (tmthing->target && (
        // 	    tmthing->target->type == thing->type ||
        // 	    (tmthing->target->type == MT_KNIGHT && thing->type == MT_BRUISER)||
        // 	    (tmthing->target->type == MT_BRUISER && thing->type == MT_KNIGHT) ) )
        // 	{
        // 	    // Don't hit same species as originator.
        // 	    if (thing == tmthing->target)
        // 		return true;
        //
        // 	    if (thing->type != MT_PLAYER)
        // 	    {
        // 		// Explode, but do no damage.
        // 		// Let players missile other players.
        // 		return false;
        // 	    }
        // 	}
        //
        // 	if (! (thing->flags & MF_SHOOTABLE) )
        // 	{
        // 	    // didn't do any damage
        // 	    return !(thing->flags & MF_SOLID);
        // 	}
        //
        // 	// damage / explode
        // 	damage = ((P_Random()%8)+1)*tmthing->info->damage;
        // 	P_DamageMobj (thing, tmthing, tmthing->target, damage);
        //
        // 	// don't traverse any more
        // 	return false;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // check for special pickup
        //     if (thing->flags & MF_SPECIAL)
        //     {
        // 	solid = thing->flags&MF_SOLID;
        // 	if (tmflags&MF_PICKUP)
        // 	{
        // 	    // can remove thing
        // 	    P_TouchSpecialThing (thing, tmthing);
        // 	}
        // 	return !solid;
        //     }
        todo!("if statement not yet translated");
        return ((((*thing).flags & MF_SOLID) == 0) as std::ffi::c_int);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_CheckPosition(
    mut thing: *mut mobj_t,
    mut x: fixed_t,
    mut y: fixed_t,
) -> boolean {
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
        let mut newsubsec: *mut subsector_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        tmthing = thing;
        tmflags = (*thing).flags;
        tmx = x;
        tmy = y;
        tmbbox[(BOXTOP) as usize] = (y + (*tmthing).radius);
        tmbbox[(BOXBOTTOM) as usize] = (y - (*tmthing).radius);
        tmbbox[(BOXRIGHT) as usize] = (x + (*tmthing).radius);
        tmbbox[(BOXLEFT) as usize] = (x - (*tmthing).radius);
        newsubsec = R_PointInSubsector(x, y);
        ceilingline = NULL;
        tmfloorz = tmdropoffz = (*(*newsubsec).sector).floorheight;
        tmceilingz = (*(*newsubsec).sector).ceilingheight;
        {
            let __macro_tmp = validcount;
            validcount += 1;
            __macro_tmp
        };
        numspechit = 0;
        // TODO: if statement not yet translated:
        //
        //
        //     if ( tmflags & MF_NOCLIP )
        // 	return true;
        todo!("if statement not yet translated");
        xl = (((tmbbox[(BOXLEFT) as usize] - bmaporgx) - MAXRADIUS) >> MAPBLOCKSHIFT);
        xh = (((tmbbox[(BOXRIGHT) as usize] - bmaporgx) + MAXRADIUS) >> MAPBLOCKSHIFT);
        yl = (((tmbbox[(BOXBOTTOM) as usize] - bmaporgy) - MAXRADIUS) >> MAPBLOCKSHIFT);
        yh = (((tmbbox[(BOXTOP) as usize] - bmaporgy) + MAXRADIUS) >> MAPBLOCKSHIFT);
        // TODO: for statement not yet translated:
        //
        //
        //     for (bx=xl ; bx<=xh ; bx++)
        // 	for (by=yl ; by<=yh ; by++)
        // 	    if (!P_BlockThingsIterator(bx,by,PIT_CheckThing))
        // 		return false;
        todo!("for statement not yet translated");
        xl = ((tmbbox[(BOXLEFT) as usize] - bmaporgx) >> MAPBLOCKSHIFT);
        xh = ((tmbbox[(BOXRIGHT) as usize] - bmaporgx) >> MAPBLOCKSHIFT);
        yl = ((tmbbox[(BOXBOTTOM) as usize] - bmaporgy) >> MAPBLOCKSHIFT);
        yh = ((tmbbox[(BOXTOP) as usize] - bmaporgy) >> MAPBLOCKSHIFT);
        // TODO: for statement not yet translated:
        //
        //
        //     for (bx=xl ; bx<=xh ; bx++)
        // 	for (by=yl ; by<=yh ; by++)
        // 	    if (!P_BlockLinesIterator (bx,by,PIT_CheckLine))
        // 		return false;
        todo!("for statement not yet translated");
        return true_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_TryMove(
    mut thing: *mut mobj_t,
    mut x: fixed_t,
    mut y: fixed_t,
) -> boolean {
    unsafe {
        let mut oldx: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut oldy: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut side: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut oldside: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut ld: *mut line_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        floatok = false_;
        // TODO: if statement not yet translated:
        //
        //     if (!P_CheckPosition (thing, x, y))
        // 	return false;		// solid wall or thing
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if ( !(thing->flags & MF_NOCLIP) )
        //     {
        // 	if (tmceilingz - tmfloorz < thing->height)
        // 	    return false;	// doesn't fit
        //
        // 	floatok = true;
        //
        // 	if ( !(thing->flags&MF_TELEPORT)
        // 	     &&tmceilingz - thing->z < thing->height)
        // 	    return false;	// mobj must lower itself to fit
        //
        // 	if ( !(thing->flags&MF_TELEPORT)
        // 	     && tmfloorz - thing->z > 24*FRACUNIT )
        // 	    return false;	// too big a step up
        //
        // 	if ( !(thing->flags&(MF_DROPOFF|MF_FLOAT))
        // 	     && tmfloorz - tmdropoffz > 24*FRACUNIT )
        // 	    return false;	// don't stand over a dropoff
        //     }
        todo!("if statement not yet translated");
        P_UnsetThingPosition(thing);
        oldx = (*thing).x;
        oldy = (*thing).y;
        (*thing).floorz = tmfloorz;
        (*thing).ceilingz = tmceilingz;
        (*thing).x = x;
        (*thing).y = y;
        P_SetThingPosition(thing);
        // TODO: if statement not yet translated:
        //
        //
        //     // if any special lines were hit, do the effect
        //     if (! (thing->flags&(MF_TELEPORT|MF_NOCLIP)) )
        //     {
        // 	while (numspechit--)
        // 	{
        // 	    // see if the line was crossed
        // 	    ld = spechit[numspechit];
        // 	    side = P_PointOnLineSide (thing->x, thing->y, ld);
        // 	    oldside = P_PointOnLineSide (oldx, oldy, ld);
        // 	    if (side != oldside)
        // 	    {
        // 		if (ld->special)
        // 		    P_CrossSpecialLine (ld-lines, oldside, thing);
        // 	    }
        // 	}
        //     }
        todo!("if statement not yet translated");
        return true_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_ThingHeightClip(mut thing: *mut mobj_t) -> boolean {
    unsafe {
        let mut onfloor: boolean = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        onfloor = ((*thing).z == (*thing).floorz);
        P_CheckPosition(thing, (*thing).x, (*thing).y);
        (*thing).floorz = tmfloorz;
        (*thing).ceilingz = tmceilingz;
        // TODO: if statement not yet translated:
        //
        //
        //     if (onfloor)
        //     {
        // 	// walking monsters rise and fall with the floor
        // 	thing->z = thing->floorz;
        //     }
        //     else
        //     {
        // 	// don't adjust a floating monster unless forced to
        // 	if (thing->z+thing->height > thing->ceilingz)
        // 	    thing->z = thing->ceilingz - thing->height;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (thing->ceilingz - thing->floorz < thing->height)
        // 	return false;
        todo!("if statement not yet translated");
        return true_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub static mut bestslidefrac: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut secondslidefrac: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut bestslideline: *mut line_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut secondslideline: *mut line_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut slidemo: *mut mobj_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut tmxmove: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut tmymove: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn P_HitSlideLine(mut ld: *mut line_t) {
    unsafe {
        let mut side: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut lineangle: angle_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut moveangle: angle_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut deltaangle: angle_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut movelen: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut newlen: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //
        //     if (ld->slopetype == ST_HORIZONTAL)
        //     {
        // 	tmymove = 0;
        // 	return;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (ld->slopetype == ST_VERTICAL)
        //     {
        // 	tmxmove = 0;
        // 	return;
        //     }
        todo!("if statement not yet translated");
        side = P_PointOnLineSide((*slidemo).x, (*slidemo).y, ld);
        lineangle = R_PointToAngle2(0, 0, (*ld).dx, (*ld).dy);
        // TODO: if statement not yet translated:
        //
        //
        //     if (side == 1)
        // 	lineangle += ANG180;
        todo!("if statement not yet translated");
        moveangle = R_PointToAngle2(0, 0, tmxmove, tmymove);
        deltaangle = (moveangle - lineangle);
        // TODO: if statement not yet translated:
        //
        //
        //     if (deltaangle > ANG180)
        // 	deltaangle += ANG180;
        todo!("if statement not yet translated");
        lineangle >>= ANGLETOFINESHIFT;
        deltaangle >>= ANGLETOFINESHIFT;
        movelen = P_AproxDistance(tmxmove, tmymove);
        newlen = FixedMul(movelen, finecosine[(deltaangle) as usize]);
        tmxmove = FixedMul(newlen, finecosine[(lineangle) as usize]);
        tmymove = FixedMul(newlen, finesine[(lineangle) as usize]);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn PTR_SlideTraverse(mut in_: *mut intercept_t) -> boolean {
    unsafe {
        let mut li: *mut line_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (!in->isaline)
        // 	I_Error ("PTR_SlideTraverse: not a line?");
        todo!("if statement not yet translated");
        li = (*in_).d.line;
        // TODO: if statement not yet translated:
        //
        //
        //     if ( ! (li->flags & ML_TWOSIDED) )
        //     {
        // 	if (P_PointOnLineSide (slidemo->x, slidemo->y, li))
        // 	{
        // 	    // don't hit the back side
        // 	    return true;
        // 	}
        // 	goto isblocking;
        //     }
        todo!("if statement not yet translated");
        P_LineOpening(li);
        // TODO: if statement not yet translated:
        //
        //
        //     if (openrange < slidemo->height)
        // 	goto isblocking;		// doesn't fit
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (opentop - slidemo->z < slidemo->height)
        // 	goto isblocking;		// mobj is too high
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (openbottom - slidemo->z > 24*FRACUNIT )
        // 	goto isblocking;		// too big a step up
        todo!("if statement not yet translated");
        return true_;
        // C label isblocking: (goto targets are not translated)
        // TODO: if statement not yet translated:
        //
        //
        //     // the line does block movement,
        //     // see if it is closer than best so far
        //   isblocking:
        //     if (in->frac < bestslidefrac)
        //     {
        // 	secondslidefrac = bestslidefrac;
        // 	secondslideline = bestslideline;
        // 	bestslidefrac = in->frac;
        // 	bestslideline = li;
        //     }
        todo!("if statement not yet translated");
        return false_;
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_SlideMove(mut mo: *mut mobj_t) {
    unsafe {
        let mut leadx: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut leady: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut trailx: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut traily: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut newx: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut newy: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut hitcount: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        slidemo = mo;
        hitcount = 0;
        // C label retry: (goto targets are not translated)
        // TODO: if statement not yet translated:
        //
        //
        //   retry:
        //     if (++hitcount == 3)
        // 	goto stairstep;		// don't loop forever
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // trace along the three leading corners
        //     if (mo->momx > 0)
        //     {
        // 	leadx = mo->x + mo->radius;
        // 	trailx = mo->x - mo->radius;
        //     }
        //     else
        //     {
        // 	leadx = mo->x - mo->radius;
        // 	trailx = mo->x + mo->radius;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (mo->momy > 0)
        //     {
        // 	leady = mo->y + mo->radius;
        // 	traily = mo->y - mo->radius;
        //     }
        //     else
        //     {
        // 	leady = mo->y - mo->radius;
        // 	traily = mo->y + mo->radius;
        //     }
        todo!("if statement not yet translated");
        bestslidefrac = (FRACUNIT + 1);
        P_PathTraverse(
            leadx,
            leady,
            (leadx + (*mo).momx),
            (leady + (*mo).momy),
            PT_ADDLINES,
            PTR_SlideTraverse,
        );
        P_PathTraverse(
            trailx,
            leady,
            (trailx + (*mo).momx),
            (leady + (*mo).momy),
            PT_ADDLINES,
            PTR_SlideTraverse,
        );
        P_PathTraverse(
            leadx,
            traily,
            (leadx + (*mo).momx),
            (traily + (*mo).momy),
            PT_ADDLINES,
            PTR_SlideTraverse,
        );
        // TODO: if statement not yet translated:
        //
        //
        //     // move up to the wall
        //     if (bestslidefrac == FRACUNIT+1)
        //     {
        // 	// the move most have hit the middle, so stairstep
        //       stairstep:
        // 	if (!P_TryMove (mo, mo->x, mo->y + mo->momy))
        // 	    P_TryMove (mo, mo->x + mo->momx, mo->y);
        // 	return;
        //     }
        todo!("if statement not yet translated");
        bestslidefrac -= 0x800;
        // TODO: if statement not yet translated:
        //
        //     if (bestslidefrac > 0)
        //     {
        // 	newx = FixedMul (mo->momx, bestslidefrac);
        // 	newy = FixedMul (mo->momy, bestslidefrac);
        //
        // 	if (!P_TryMove (mo, mo->x+newx, mo->y+newy))
        // 	    goto stairstep;
        //     }
        todo!("if statement not yet translated");
        bestslidefrac = (FRACUNIT - (bestslidefrac + 0x800));
        // TODO: if statement not yet translated:
        //
        //
        //     if (bestslidefrac > FRACUNIT)
        // 	bestslidefrac = FRACUNIT;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (bestslidefrac <= 0)
        // 	return;
        todo!("if statement not yet translated");
        tmxmove = FixedMul((*mo).momx, bestslidefrac);
        tmymove = FixedMul((*mo).momy, bestslidefrac);
        P_HitSlideLine(bestslideline);
        (*mo).momx = tmxmove;
        (*mo).momy = tmymove;
        // TODO: if statement not yet translated:
        //
        //
        //     if (!P_TryMove (mo, mo->x+tmxmove, mo->y+tmymove))
        //     {
        // 	goto retry;
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub static mut linetarget: *mut mobj_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut shootthing: *mut mobj_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut shootz: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut la_damage: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut attackrange: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut aimslope: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

unsafe extern "C" {
    pub static mut topslope: fixed_t;
}

unsafe extern "C" {
    pub static mut bottomslope: fixed_t;
}

pub unsafe extern "C" fn PTR_AimTraverse(mut in_: *mut intercept_t) -> boolean {
    unsafe {
        let mut li: *mut line_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut th: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut slope: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut thingtopslope: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut thingbottomslope: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut dist: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (in->isaline)
        //     {
        // 	li = in->d.line;
        //
        // 	if ( !(li->flags & ML_TWOSIDED) )
        // 	    return false;		// stop
        //
        // 	// Crosses a two sided line.
        // 	// A two sided line will restrict
        // 	// the possible target ranges.
        // 	P_LineOpening (li);
        //
        // 	if (openbottom >= opentop)
        // 	    return false;		// stop
        //
        // 	dist = FixedMul (attackrange, in->frac);
        //
        // 	if (li->frontsector->floorheight != li->backsector->floorheight)
        // 	{
        // 	    slope = FixedDiv (openbottom - shootz , dist);
        // 	    if (slope > bottomslope)
        // 		bottomslope = slope;
        // 	}
        //
        // 	if (li->frontsector->ceilingheight != li->backsector->ceilingheight)
        // 	{
        // 	    slope = FixedDiv (opentop - shootz , dist);
        // 	    if (slope < topslope)
        // 		topslope = slope;
        // 	}
        //
        // 	if (topslope <= bottomslope)
        // 	    return false;		// stop
        //
        // 	return true;			// shot continues
        //     }
        todo!("if statement not yet translated");
        th = (*in_).d.thing;
        // TODO: if statement not yet translated:
        //
        //     if (th == shootthing)
        // 	return true;			// can't shoot self
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (!(th->flags&MF_SHOOTABLE))
        // 	return true;			// corpse or something
        todo!("if statement not yet translated");
        dist = FixedMul(attackrange, (*in_).frac);
        thingtopslope = FixedDiv((((*th).z + (*th).height) - shootz), dist);
        // TODO: if statement not yet translated:
        //
        //
        //     if (thingtopslope < bottomslope)
        // 	return true;			// shot over the thing
        todo!("if statement not yet translated");
        thingbottomslope = FixedDiv(((*th).z - shootz), dist);
        // TODO: if statement not yet translated:
        //
        //
        //     if (thingbottomslope > topslope)
        // 	return true;			// shot under the thing
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     // this thing can be hit!
        //     if (thingtopslope > topslope)
        // 	thingtopslope = topslope;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (thingbottomslope < bottomslope)
        // 	thingbottomslope = bottomslope;
        todo!("if statement not yet translated");
        aimslope = ((thingtopslope + thingbottomslope) / 2);
        linetarget = th;
        return false_;
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn PTR_ShootTraverse(mut in_: *mut intercept_t) -> boolean {
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
        let mut frac: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut li: *mut line_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut th: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut slope: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut dist: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut thingtopslope: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut thingbottomslope: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (in->isaline)
        //     {
        // 	li = in->d.line;
        //
        // 	if (li->special)
        // 	    P_ShootSpecialLine (shootthing, li);
        //
        // 	if ( !(li->flags & ML_TWOSIDED) )
        // 	    goto hitline;
        //
        // 	// crosses a two sided line
        // 	P_LineOpening (li);
        //
        // 	dist = FixedMul (attackrange, in->frac);
        //
        // 	if (li->frontsector->floorheight != li->backsector->floorheight)
        // 	{
        // 	    slope = FixedDiv (openbottom - shootz , dist);
        // 	    if (slope > aimslope)
        // 		goto hitline;
        // 	}
        //
        // 	if (li->frontsector->ceilingheight != li->backsector->ceilingheight)
        // 	{
        // 	    slope = FixedDiv (opentop - shootz , dist);
        // 	    if (slope < aimslope)
        // 		goto hitline;
        // 	}
        //
        // 	// shot continues
        // 	return true;
        //
        //
        // 	// hit line
        //       hitline:
        // 	// position a bit closer
        // 	frac = in->frac - FixedDiv (4*FRACUNIT,attackrange);
        // 	x = trace.x + FixedMul (trace.dx, frac);
        // 	y = trace.y + FixedMul (trace.dy, frac);
        // 	z = shootz + FixedMul (aimslope, FixedMul(frac, attackrange));
        //
        // 	if (li->frontsector->ceilingpic == skyflatnum)
        // 	{
        // 	    // don't shoot the sky!
        // 	    if (z > li->frontsector->ceilingheight)
        // 		return false;
        //
        // 	    // it's a sky hack wall
        // 	    if	(li->backsector && li->backsector->ceilingpic == skyflatnum)
        // 		return false;
        // 	}
        //
        // 	// Spawn bullet puffs.
        // 	P_SpawnPuff (x,y,z);
        //
        // 	// don't go any farther
        // 	return false;
        //     }
        todo!("if statement not yet translated");
        th = (*in_).d.thing;
        // TODO: if statement not yet translated:
        //
        //     if (th == shootthing)
        // 	return true;		// can't shoot self
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (!(th->flags&MF_SHOOTABLE))
        // 	return true;		// corpse or something
        todo!("if statement not yet translated");
        dist = FixedMul(attackrange, (*in_).frac);
        thingtopslope = FixedDiv((((*th).z + (*th).height) - shootz), dist);
        // TODO: if statement not yet translated:
        //
        //
        //     if (thingtopslope < aimslope)
        // 	return true;		// shot over the thing
        todo!("if statement not yet translated");
        thingbottomslope = FixedDiv(((*th).z - shootz), dist);
        // TODO: if statement not yet translated:
        //
        //
        //     if (thingbottomslope > aimslope)
        // 	return true;		// shot under the thing
        todo!("if statement not yet translated");
        frac = ((*in_).frac - FixedDiv((10 * FRACUNIT), attackrange));
        x = (trace.x + FixedMul(trace.dx, frac));
        y = (trace.y + FixedMul(trace.dy, frac));
        z = (shootz + FixedMul(aimslope, FixedMul(frac, attackrange)));
        // TODO: if statement not yet translated:
        //
        //
        //     // Spawn bullet puffs or blod spots,
        //     // depending on target type.
        //     if (in->d.thing->flags & MF_NOBLOOD)
        // 	P_SpawnPuff (x,y,z);
        //     else
        // 	P_SpawnBlood (x,y,z, la_damage);
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (la_damage)
        // 	P_DamageMobj (th, shootthing, shootthing, la_damage);
        todo!("if statement not yet translated");
        return false_;
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_AimLineAttack(
    mut t1: *mut mobj_t,
    mut angle: angle_t,
    mut distance: fixed_t,
) -> fixed_t {
    unsafe {
        let mut x2: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut y2: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        angle >>= ANGLETOFINESHIFT;
        shootthing = t1;
        x2 = ((*t1).x + ((distance >> FRACBITS) * finecosine[(angle) as usize]));
        y2 = ((*t1).y + ((distance >> FRACBITS) * finesine[(angle) as usize]));
        shootz = (((*t1).z + ((*t1).height >> 1)) + (8 * FRACUNIT));
        topslope = ((100 * FRACUNIT) / 160);
        bottomslope = (((-(100)) * FRACUNIT) / 160);
        attackrange = distance;
        linetarget = NULL;
        P_PathTraverse(
            (*t1).x,
            (*t1).y,
            x2,
            y2,
            (PT_ADDLINES | PT_ADDTHINGS),
            PTR_AimTraverse,
        );
        // TODO: if statement not yet translated:
        //
        //
        //     if (linetarget)
        // 	return aimslope;
        todo!("if statement not yet translated");
        return 0;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_LineAttack(
    mut t1: *mut mobj_t,
    mut angle: angle_t,
    mut distance: fixed_t,
    mut slope: fixed_t,
    mut damage: std::ffi::c_int,
) {
    unsafe {
        let mut x2: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut y2: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        angle >>= ANGLETOFINESHIFT;
        shootthing = t1;
        la_damage = damage;
        x2 = ((*t1).x + ((distance >> FRACBITS) * finecosine[(angle) as usize]));
        y2 = ((*t1).y + ((distance >> FRACBITS) * finesine[(angle) as usize]));
        shootz = (((*t1).z + ((*t1).height >> 1)) + (8 * FRACUNIT));
        attackrange = distance;
        aimslope = slope;
        P_PathTraverse(
            (*t1).x,
            (*t1).y,
            x2,
            y2,
            (PT_ADDLINES | PT_ADDTHINGS),
            PTR_ShootTraverse,
        );
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub static mut usething: *mut mobj_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn PTR_UseTraverse(mut in_: *mut intercept_t) -> boolean {
    unsafe {
        let mut side: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (!in->d.line->special)
        //     {
        // 	P_LineOpening (in->d.line);
        // 	if (openrange <= 0)
        // 	{
        // 	    S_StartSound (usething, sfx_noway);
        //
        // 	    // can't use through a wall
        // 	    return false;
        // 	}
        // 	// not a special line, but keep checking
        // 	return true ;
        //     }
        todo!("if statement not yet translated");
        side = 0;
        // TODO: if statement not yet translated:
        //
        //     if (P_PointOnLineSide (usething->x, usething->y, in->d.line) == 1)
        // 	side = 1;
        todo!("if statement not yet translated");
        P_UseSpecialLine(usething, (*in_).d.line, side);
        return false_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_UseLines(mut player: *mut player_t) {
    unsafe {
        let mut angle: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut x1: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut y1: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut x2: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut y2: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        usething = (*player).mo;
        angle = ((*(*player).mo).angle >> ANGLETOFINESHIFT);
        x1 = (*(*player).mo).x;
        y1 = (*(*player).mo).y;
        x2 = (x1 + ((USERANGE >> FRACBITS) * finecosine[(angle) as usize]));
        y2 = (y1 + ((USERANGE >> FRACBITS) * finesine[(angle) as usize]));
        P_PathTraverse(x1, y1, x2, y2, PT_ADDLINES, PTR_UseTraverse);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub static mut bombsource: *mut mobj_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut bombspot: *mut mobj_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut bombdamage: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn PIT_RadiusAttack(mut thing: *mut mobj_t) -> boolean {
    unsafe {
        let mut dx: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut dy: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut dist: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (!(thing->flags & MF_SHOOTABLE) )
        // 	return true;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // Boss spider and cyborg
        //     // take no damage from concussion.
        //     if (thing->type == MT_CYBORG
        // 	|| thing->type == MT_SPIDER)
        // 	return true;
        todo!("if statement not yet translated");
        dx = abs(((*thing).x - (*bombspot).x));
        dy = abs(((*thing).y - (*bombspot).y));
        dist = (if (dx > dy) { dx } else { dy });
        dist = ((dist - (*thing).radius) >> FRACBITS);
        // TODO: if statement not yet translated:
        //
        //
        //     if (dist < 0)
        // 	dist = 0;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (dist >= bombdamage)
        // 	return true;	// out of range
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if ( P_CheckSight (thing, bombspot) )
        //     {
        // 	// must be in direct path
        // 	P_DamageMobj (thing, bombspot, bombsource, bombdamage - dist);
        //     }
        todo!("if statement not yet translated");
        return true_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_RadiusAttack(
    mut spot: *mut mobj_t,
    mut source: *mut mobj_t,
    mut damage: std::ffi::c_int,
) {
    unsafe {
        let mut x: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut y: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
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
        let mut dist: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        dist = ((damage + MAXRADIUS) << FRACBITS);
        yh = ((((*spot).y + dist) - bmaporgy) >> MAPBLOCKSHIFT);
        yl = ((((*spot).y - dist) - bmaporgy) >> MAPBLOCKSHIFT);
        xh = ((((*spot).x + dist) - bmaporgx) >> MAPBLOCKSHIFT);
        xl = ((((*spot).x - dist) - bmaporgx) >> MAPBLOCKSHIFT);
        bombspot = spot;
        bombsource = source;
        bombdamage = damage;
        // TODO: for statement not yet translated:
        //
        //
        //     for (y=yl ; y<=yh ; y++)
        // 	for (x=xl ; x<=xh ; x++)
        // 	    P_BlockThingsIterator (x, y, PIT_RadiusAttack );
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub static mut crushchange: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut nofit: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn PIT_ChangeSector(mut thing: *mut mobj_t) -> boolean {
    unsafe {
        let mut mo: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (P_ThingHeightClip (thing))
        //     {
        // 	// keep checking
        // 	return true;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //
        //     // crunch bodies to giblets
        //     if (thing->health <= 0)
        //     {
        // 	P_SetMobjState (thing, S_GIBS);
        //
        // 	thing->flags &= ~MF_SOLID;
        // 	thing->height = 0;
        // 	thing->radius = 0;
        //
        // 	// keep checking
        // 	return true;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // crunch dropped items
        //     if (thing->flags & MF_DROPPED)
        //     {
        // 	P_RemoveMobj (thing);
        //
        // 	// keep checking
        // 	return true;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (! (thing->flags & MF_SHOOTABLE) )
        //     {
        // 	// assume it is bloody gibs or something
        // 	return true;
        //     }
        todo!("if statement not yet translated");
        nofit = true_;
        // TODO: if statement not yet translated:
        //
        //
        //     if (crushchange && !(leveltime&3) )
        //     {
        // 	P_DamageMobj(thing,NULL,NULL,10);
        //
        // 	// spray blood in a random direction
        // 	mo = P_SpawnMobj (thing->x,
        // 			  thing->y,
        // 			  thing->z + thing->height/2, MT_BLOOD);
        //
        // 	mo->momx = (P_Random() - P_Random ())<<12;
        // 	mo->momy = (P_Random() - P_Random ())<<12;
        //     }
        todo!("if statement not yet translated");
        return true_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_ChangeSector(mut sector: *mut sector_t, mut crunch: boolean) -> boolean {
    unsafe {
        let mut x: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut y: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        nofit = false_;
        crushchange = crunch;
        // TODO: for statement not yet translated:
        //
        //
        //     // re-check heights for all things near the moving sector
        //     for (x=sector->blockbox[BOXLEFT] ; x<= sector->blockbox[BOXRIGHT] ; x++)
        // 	for (y=sector->blockbox[BOXBOTTOM];y<= sector->blockbox[BOXTOP] ; y++)
        // 	    P_BlockThingsIterator (x, y, PIT_ChangeSector);
        todo!("for statement not yet translated");
        return nofit;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}
