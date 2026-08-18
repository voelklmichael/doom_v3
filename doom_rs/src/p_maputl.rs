use crate::d_items::*;
use crate::d_player::*;
use crate::d_think::*;
use crate::d_ticcmd::*;
use crate::doomdata::*;
use crate::doomdef::*;
use crate::doomtype::*;
use crate::info::*;
use crate::m_bbox::*;
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

static mut rcsid: [std::ffi::c_char; 51] = unsafe {
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
        117 as std::ffi::c_char,
        116 as std::ffi::c_char,
        108 as std::ffi::c_char,
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

pub unsafe extern "C" fn P_AproxDistance(mut dx: fixed_t, mut dy: fixed_t) -> fixed_t {
    unsafe {
        dx = abs(dx);
        dy = abs(dy);
        // TODO: if statement not yet translated:
        //
        //     if (dx < dy)
        // 	return dx+dy-(dx>>1);
        todo!("if statement not yet translated");
        return ((dx + dy) - (dy >> 1));
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_PointOnLineSide(
    mut x: fixed_t,
    mut y: fixed_t,
    mut line: *mut line_t,
) -> std::ffi::c_int {
    unsafe {
        let mut dx: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut dy: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut left: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut right: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (!line->dx)
        //     {
        // 	if (x <= line->v1->x)
        // 	    return line->dy > 0;
        //
        // 	return line->dy < 0;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (!line->dy)
        //     {
        // 	if (y <= line->v1->y)
        // 	    return line->dx < 0;
        //
        // 	return line->dx > 0;
        //     }
        todo!("if statement not yet translated");
        dx = (x - (*(*line).v1).x);
        dy = (y - (*(*line).v1).y);
        left = FixedMul(((*line).dy >> FRACBITS), dx);
        right = FixedMul(dy, ((*line).dx >> FRACBITS));
        // TODO: if statement not yet translated:
        //
        //
        //     if (right < left)
        // 	return 0;		// front side
        todo!("if statement not yet translated");
        return 1;
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_BoxOnLineSide(
    mut tmbox: *mut fixed_t,
    mut ld: *mut line_t,
) -> std::ffi::c_int {
    unsafe {
        let mut p1: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut p2: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: switch statement not yet translated:
        //
        //
        //     switch (ld->slopetype)
        //     {
        //       case ST_HORIZONTAL:
        // 	p1 = tmbox[BOXTOP] > ld->v1->y;
        // 	p2 = tmbox[BOXBOTTOM] > ld->v1->y;
        // 	if (ld->dx < 0)
        // 	{
        // 	    p1 ^= 1;
        // 	    p2 ^= 1;
        // 	}
        // 	break;
        //
        //       case ST_VERTICAL:
        // 	p1 = tmbox[BOXRIGHT] < ld->v1->x;
        // 	p2 = tmbox[BOXLEFT] < ld->v1->x;
        // 	if (ld->dy < 0)
        // 	{
        // 	    p1 ^= 1;
        // 	    p2 ^= 1;
        // 	}
        // 	break;
        //
        //       case ST_POSITIVE:
        // 	p1 = P_PointOnLineSide (tmbox[BOXLEFT], tmbox[BOXTOP], ld);
        // 	p2 = P_PointOnLineSide (tmbox[BOXRIGHT], tmbox[BOXBOTTOM], ld);
        // 	break;
        //
        //       case ST_NEGATIVE:
        // 	p1 = P_PointOnLineSide (tmbox[BOXRIGHT], tmbox[BOXTOP], ld);
        // 	p2 = P_PointOnLineSide (tmbox[BOXLEFT], tmbox[BOXBOTTOM], ld);
        // 	break;
        //     }
        todo!("switch statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (p1 == p2)
        // 	return p1;
        todo!("if statement not yet translated");
        return (-(1));
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_PointOnDivlineSide(
    mut x: fixed_t,
    mut y: fixed_t,
    mut line: *mut divline_t,
) -> std::ffi::c_int {
    unsafe {
        let mut dx: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut dy: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut left: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut right: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (!line->dx)
        //     {
        // 	if (x <= line->x)
        // 	    return line->dy > 0;
        //
        // 	return line->dy < 0;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (!line->dy)
        //     {
        // 	if (y <= line->y)
        // 	    return line->dx < 0;
        //
        // 	return line->dx > 0;
        //     }
        todo!("if statement not yet translated");
        dx = (x - (*line).x);
        dy = (y - (*line).y);
        // TODO: if statement not yet translated:
        //
        //
        //     // try to quickly decide by looking at sign bits
        //     if ( (line->dy ^ line->dx ^ dx ^ dy)&0x80000000 )
        //     {
        // 	if ( (line->dy ^ dx) & 0x80000000 )
        // 	    return 1;		// (left is negative)
        // 	return 0;
        //     }
        todo!("if statement not yet translated");
        left = FixedMul(((*line).dy >> 8), (dx >> 8));
        right = FixedMul((dy >> 8), ((*line).dx >> 8));
        // TODO: if statement not yet translated:
        //
        //
        //     if (right < left)
        // 	return 0;		// front side
        todo!("if statement not yet translated");
        return 1;
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_MakeDivline(mut li: *mut line_t, mut dl: *mut divline_t) {
    unsafe {
        (*dl).x = (*(*li).v1).x;
        (*dl).y = (*(*li).v1).y;
        (*dl).dx = (*li).dx;
        (*dl).dy = (*li).dy;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_InterceptVector(
    mut v2: *mut divline_t,
    mut v1: *mut divline_t,
) -> fixed_t {
    unsafe {
        let mut frac: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut num: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut den: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        den = (FixedMul(((*v1).dy >> 8), (*v2).dx) - FixedMul(((*v1).dx >> 8), (*v2).dy));
        // TODO: if statement not yet translated:
        //
        //
        //     if (den == 0)
        // 	return 0;
        todo!("if statement not yet translated");
        num = (FixedMul((((*v1).x - (*v2).x) >> 8), (*v1).dy)
            + FixedMul((((*v2).y - (*v1).y) >> 8), (*v1).dx));
        frac = FixedDiv(num, den);
        return frac;
    }
    todo!("fell off the end of a non-void C function")
}

pub static mut opentop: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut openbottom: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut openrange: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut lowfloor: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn P_LineOpening(mut linedef: *mut line_t) {
    unsafe {
        let mut front: *mut sector_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut back: *mut sector_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (linedef->sidenum[1] == -1)
        //     {
        // 	// single sided line
        // 	openrange = 0;
        // 	return;
        //     }
        todo!("if statement not yet translated");
        front = (*linedef).frontsector;
        back = (*linedef).backsector;
        // TODO: if statement not yet translated:
        //
        //
        //     if (front->ceilingheight < back->ceilingheight)
        // 	opentop = front->ceilingheight;
        //     else
        // 	opentop = back->ceilingheight;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (front->floorheight > back->floorheight)
        //     {
        // 	openbottom = front->floorheight;
        // 	lowfloor = back->floorheight;
        //     }
        //     else
        //     {
        // 	openbottom = back->floorheight;
        // 	lowfloor = front->floorheight;
        //     }
        todo!("if statement not yet translated");
        openrange = (opentop - openbottom);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_UnsetThingPosition(mut thing: *mut mobj_t) {
    unsafe {
        let mut blockx: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut blocky: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if ( ! (thing->flags & MF_NOSECTOR) )
        //     {
        // 	// inert things don't need to be in blockmap?
        // 	// unlink from subsector
        // 	if (thing->snext)
        // 	    thing->snext->sprev = thing->sprev;
        //
        // 	if (thing->sprev)
        // 	    thing->sprev->snext = thing->snext;
        // 	else
        // 	    thing->subsector->sector->thinglist = thing->snext;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if ( ! (thing->flags & MF_NOBLOCKMAP) )
        //     {
        // 	// inert things don't need to be in blockmap
        // 	// unlink from block map
        // 	if (thing->bnext)
        // 	    thing->bnext->bprev = thing->bprev;
        //
        // 	if (thing->bprev)
        // 	    thing->bprev->bnext = thing->bnext;
        // 	else
        // 	{
        // 	    blockx = (thing->x - bmaporgx)>>MAPBLOCKSHIFT;
        // 	    blocky = (thing->y - bmaporgy)>>MAPBLOCKSHIFT;
        //
        // 	    if (blockx>=0 && blockx < bmapwidth
        // 		&& blocky>=0 && blocky <bmapheight)
        // 	    {
        // 		blocklinks[blocky*bmapwidth+blockx] = thing->bnext;
        // 	    }
        // 	}
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_SetThingPosition(mut thing: *mut mobj_t) {
    unsafe {
        let mut ss: *mut subsector_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut sec: *mut sector_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut blockx: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut blocky: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut link: *mut *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        ss = R_PointInSubsector((*thing).x, (*thing).y);
        (*thing).subsector = ss;
        // TODO: if statement not yet translated:
        //
        //
        //     if ( ! (thing->flags & MF_NOSECTOR) )
        //     {
        // 	// invisible things don't go into the sector links
        // 	sec = ss->sector;
        //
        // 	thing->sprev = NULL;
        // 	thing->snext = sec->thinglist;
        //
        // 	if (sec->thinglist)
        // 	    sec->thinglist->sprev = thing;
        //
        // 	sec->thinglist = thing;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //
        //     // link into blockmap
        //     if ( ! (thing->flags & MF_NOBLOCKMAP) )
        //     {
        // 	// inert things don't need to be in blockmap
        // 	blockx = (thing->x - bmaporgx)>>MAPBLOCKSHIFT;
        // 	blocky = (thing->y - bmaporgy)>>MAPBLOCKSHIFT;
        //
        // 	if (blockx>=0
        // 	    && blockx < bmapwidth
        // 	    && blocky>=0
        // 	    && blocky < bmapheight)
        // 	{
        // 	    link = &blocklinks[blocky*bmapwidth+blockx];
        // 	    thing->bprev = NULL;
        // 	    thing->bnext = *link;
        // 	    if (*link)
        // 		(*link)->bprev = thing;
        //
        // 	    *link = thing;
        // 	}
        // 	else
        // 	{
        // 	    // thing is off the map
        // 	    thing->bnext = thing->bprev = NULL;
        // 	}
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_BlockLinesIterator(
    mut x: std::ffi::c_int,
    mut y: std::ffi::c_int,
    mut func: Option<unsafe extern "C" fn(*mut line_t) -> boolean>,
) -> boolean {
    unsafe {
        let mut offset: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut list: *mut std::ffi::c_short = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut ld: *mut line_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (x<0
        // 	|| y<0
        // 	|| x>=bmapwidth
        // 	|| y>=bmapheight)
        //     {
        // 	return true;
        //     }
        todo!("if statement not yet translated");
        offset = ((y * bmapwidth) + x);
        offset = (*(blockmap + offset));
        // TODO: for statement not yet translated:
        //
        //
        //     for ( list = blockmaplump+offset ; *list != -1 ; list++)
        //     {
        // 	ld = &lines[*list];
        //
        // 	if (ld->validcount == validcount)
        // 	    continue; 	// line has already been checked
        //
        // 	ld->validcount = validcount;
        //
        // 	if ( !func(ld) )
        // 	    return false;
        //     }
        todo!("for statement not yet translated");
        return true_;
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_BlockThingsIterator(
    mut x: std::ffi::c_int,
    mut y: std::ffi::c_int,
    mut func: Option<unsafe extern "C" fn(*mut mobj_t) -> boolean>,
) -> boolean {
    unsafe {
        let mut mobj: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if ( x<0
        // 	 || y<0
        // 	 || x>=bmapwidth
        // 	 || y>=bmapheight)
        //     {
        // 	return true;
        //     }
        todo!("if statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //
        //
        //     for (mobj = blocklinks[y*bmapwidth+x] ;
        // 	 mobj ;
        // 	 mobj = mobj->bnext)
        //     {
        // 	if (!func( mobj ) )
        // 	    return false;
        //     }
        todo!("for statement not yet translated");
        return true_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub static mut intercepts: [intercept_t; (MAXINTERCEPTS) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut intercept_p: *mut intercept_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut trace: divline_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut earlyout: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut ptflags: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn PIT_AddLineIntercepts(mut ld: *mut line_t) -> boolean {
    unsafe {
        let mut s1: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut s2: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut frac: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut dl: divline_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     // avoid precision problems with two routines
        //     if ( trace.dx > FRACUNIT*16
        // 	 || trace.dy > FRACUNIT*16
        // 	 || trace.dx < -FRACUNIT*16
        // 	 || trace.dy < -FRACUNIT*16)
        //     {
        // 	s1 = P_PointOnDivlineSide (ld->v1->x, ld->v1->y, &trace);
        // 	s2 = P_PointOnDivlineSide (ld->v2->x, ld->v2->y, &trace);
        //     }
        //     else
        //     {
        // 	s1 = P_PointOnLineSide (trace.x, trace.y, ld);
        // 	s2 = P_PointOnLineSide (trace.x+trace.dx, trace.y+trace.dy, ld);
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (s1 == s2)
        // 	return true;	// line isn't crossed
        todo!("if statement not yet translated");
        P_MakeDivline(ld, (&(dl) as *const _ as *mut _));
        frac = P_InterceptVector(
            (&(trace) as *const divline_t as *mut divline_t),
            (&(dl) as *const _ as *mut _),
        );
        // TODO: if statement not yet translated:
        //
        //
        //     if (frac < 0)
        // 	return true;	// behind source
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     // try to early out the check
        //     if (earlyout
        // 	&& frac < FRACUNIT
        // 	&& !ld->backsector)
        //     {
        // 	return false;	// stop checking
        //     }
        todo!("if statement not yet translated");
        (*intercept_p).frac = frac;
        (*intercept_p).isaline = true_;
        (*intercept_p).d.line = ld;
        {
            let __macro_tmp = intercept_p;
            intercept_p += 1;
            __macro_tmp
        };
        return true_;
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn PIT_AddThingIntercepts(mut thing: *mut mobj_t) -> boolean {
    unsafe {
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
        let mut s1: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut s2: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut tracepositive: boolean = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut dl: divline_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut frac: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        tracepositive = ((trace.dx ^ trace.dy) > 0);
        // TODO: if statement not yet translated:
        //
        //
        //     // check a corner to corner crossection for hit
        //     if (tracepositive)
        //     {
        // 	x1 = thing->x - thing->radius;
        // 	y1 = thing->y + thing->radius;
        //
        // 	x2 = thing->x + thing->radius;
        // 	y2 = thing->y - thing->radius;
        //     }
        //     else
        //     {
        // 	x1 = thing->x - thing->radius;
        // 	y1 = thing->y - thing->radius;
        //
        // 	x2 = thing->x + thing->radius;
        // 	y2 = thing->y + thing->radius;
        //     }
        todo!("if statement not yet translated");
        s1 = P_PointOnDivlineSide(x1, y1, (&(trace) as *const divline_t as *mut divline_t));
        s2 = P_PointOnDivlineSide(x2, y2, (&(trace) as *const divline_t as *mut divline_t));
        // TODO: if statement not yet translated:
        //
        //
        //     if (s1 == s2)
        // 	return true;		// line isn't crossed
        todo!("if statement not yet translated");
        dl.x = x1;
        dl.y = y1;
        dl.dx = (x2 - x1);
        dl.dy = (y2 - y1);
        frac = P_InterceptVector(
            (&(trace) as *const divline_t as *mut divline_t),
            (&(dl) as *const _ as *mut _),
        );
        // TODO: if statement not yet translated:
        //
        //
        //     if (frac < 0)
        // 	return true;		// behind source
        todo!("if statement not yet translated");
        (*intercept_p).frac = frac;
        (*intercept_p).isaline = false_;
        (*intercept_p).d.thing = thing;
        {
            let __macro_tmp = intercept_p;
            intercept_p += 1;
            __macro_tmp
        };
        return true_;
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_TraverseIntercepts(
    mut func: traverser_t,
    mut maxfrac: fixed_t,
) -> boolean {
    unsafe {
        let mut count: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut dist: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut scan: *mut intercept_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut in_: *mut intercept_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        count = (intercept_p - intercepts);
        in_ = 0;
        // TODO: while statement not yet translated:
        //
        //     while (count--)
        //     {
        // 	dist = MAXINT;
        // 	for (scan = intercepts ; scan<intercept_p ; scan++)
        // 	{
        // 	    if (scan->frac < dist)
        // 	    {
        // 		dist = scan->frac;
        // 		in = scan;
        // 	    }
        // 	}
        //
        // 	if (dist > maxfrac)
        // 	    return true;	// checked everything in range
        //
        // #if 0  // UNUSED
        //     {
        // 	// don't check these yet, there may be others inserted
        // 	in = scan = intercepts;
        // 	for ( scan = intercepts ; scan<intercept_p ; scan++)
        // 	    if (scan->frac > maxfrac)
        // 		*in++ = *scan;
        // 	intercept_p = in;
        // 	return false;
        //     }
        // #endif
        //
        //         if ( !func (in) )
        // 	    return false;	// don't bother going farther
        //
        // 	in->frac = MAXINT;
        //     }
        todo!("while statement not yet translated");
        return true_;
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_PathTraverse(
    mut x1: fixed_t,
    mut y1: fixed_t,
    mut x2: fixed_t,
    mut y2: fixed_t,
    mut flags: std::ffi::c_int,
    mut trav: Option<unsafe extern "C" fn(*mut intercept_t) -> boolean>,
) -> boolean {
    unsafe {
        let mut xt1: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut yt1: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut xt2: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut yt2: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut xstep: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut ystep: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut partial: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut xintercept: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut yintercept: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut mapx: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut mapy: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut mapxstep: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut mapystep: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut count: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        earlyout = (flags & PT_EARLYOUT);
        {
            let __macro_tmp = validcount;
            validcount += 1;
            __macro_tmp
        };
        intercept_p = intercepts;
        // TODO: if statement not yet translated:
        //
        //
        //     if ( ((x1-bmaporgx)&(MAPBLOCKSIZE-1)) == 0)
        // 	x1 += FRACUNIT;	// don't side exactly on a line
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if ( ((y1-bmaporgy)&(MAPBLOCKSIZE-1)) == 0)
        // 	y1 += FRACUNIT;	// don't side exactly on a line
        todo!("if statement not yet translated");
        trace.x = x1;
        trace.y = y1;
        trace.dx = (x2 - x1);
        trace.dy = (y2 - y1);
        x1 -= bmaporgx;
        y1 -= bmaporgy;
        xt1 = (x1 >> MAPBLOCKSHIFT);
        yt1 = (y1 >> MAPBLOCKSHIFT);
        x2 -= bmaporgx;
        y2 -= bmaporgy;
        xt2 = (x2 >> MAPBLOCKSHIFT);
        yt2 = (y2 >> MAPBLOCKSHIFT);
        // TODO: if statement not yet translated:
        //
        //
        //     if (xt2 > xt1)
        //     {
        // 	mapxstep = 1;
        // 	partial = FRACUNIT - ((x1>>MAPBTOFRAC)&(FRACUNIT-1));
        // 	ystep = FixedDiv (y2-y1,abs(x2-x1));
        //     }
        //     else if (xt2 < xt1)
        //     {
        // 	mapxstep = -1;
        // 	partial = (x1>>MAPBTOFRAC)&(FRACUNIT-1);
        // 	ystep = FixedDiv (y2-y1,abs(x2-x1));
        //     }
        //     else
        //     {
        // 	mapxstep = 0;
        // 	partial = FRACUNIT;
        // 	ystep = 256*FRACUNIT;
        //     }
        todo!("if statement not yet translated");
        yintercept = ((y1 >> MAPBTOFRAC) + FixedMul(partial, ystep));
        // TODO: if statement not yet translated:
        //
        //
        //
        //     if (yt2 > yt1)
        //     {
        // 	mapystep = 1;
        // 	partial = FRACUNIT - ((y1>>MAPBTOFRAC)&(FRACUNIT-1));
        // 	xstep = FixedDiv (x2-x1,abs(y2-y1));
        //     }
        //     else if (yt2 < yt1)
        //     {
        // 	mapystep = -1;
        // 	partial = (y1>>MAPBTOFRAC)&(FRACUNIT-1);
        // 	xstep = FixedDiv (x2-x1,abs(y2-y1));
        //     }
        //     else
        //     {
        // 	mapystep = 0;
        // 	partial = FRACUNIT;
        // 	xstep = 256*FRACUNIT;
        //     }
        todo!("if statement not yet translated");
        xintercept = ((x1 >> MAPBTOFRAC) + FixedMul(partial, xstep));
        mapx = xt1;
        mapy = yt1;
        // TODO: for statement not yet translated:
        //
        //
        //     for (count = 0 ; count < 64 ; count++)
        //     {
        // 	if (flags & PT_ADDLINES)
        // 	{
        // 	    if (!P_BlockLinesIterator (mapx, mapy,PIT_AddLineIntercepts))
        // 		return false;	// early out
        // 	}
        //
        // 	if (flags & PT_ADDTHINGS)
        // 	{
        // 	    if (!P_BlockThingsIterator (mapx, mapy,PIT_AddThingIntercepts))
        // 		return false;	// early out
        // 	}
        //
        // 	if (mapx == xt2
        // 	    && mapy == yt2)
        // 	{
        // 	    break;
        // 	}
        //
        // 	if ( (yintercept >> FRACBITS) == mapy)
        // 	{
        // 	    yintercept += ystep;
        // 	    mapx += mapxstep;
        // 	}
        // 	else if ( (xintercept >> FRACBITS) == mapx)
        // 	{
        // 	    xintercept += xstep;
        // 	    mapy += mapystep;
        // 	}
        //
        //     }
        todo!("for statement not yet translated");
        return P_TraverseIntercepts(trav, FRACUNIT);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}
