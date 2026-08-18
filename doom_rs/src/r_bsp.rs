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
use crate::p_mobj::*;
use crate::p_pspr::*;
use crate::r_data::*;
use crate::r_defs::*;
use crate::r_main::*;
use crate::r_plane::*;
use crate::r_state::*;
use crate::r_things::*;
use crate::tables::*;

unsafe extern "C" {
    pub static mut rw_x: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut rw_stopx: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut segtextured: boolean;
}

unsafe extern "C" {
    pub static mut markfloor: boolean;
}

unsafe extern "C" {
    pub static mut markceiling: boolean;
}

unsafe extern "C" {
    pub static mut skymap: boolean;
}

unsafe extern "C" {
    pub static mut hscalelight: *mut *mut lighttable_t;
}

unsafe extern "C" {
    pub static mut vscalelight: *mut *mut lighttable_t;
}

unsafe extern "C" {
    pub static mut dscalelight: *mut *mut lighttable_t;
}

pub type drawfunc_t = Option<unsafe extern "C" fn(std::ffi::c_int, std::ffi::c_int)>;

static mut rcsid: [std::ffi::c_char; 48] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        114 as std::ffi::c_char,
        95 as std::ffi::c_char,
        98 as std::ffi::c_char,
        115 as std::ffi::c_char,
        112 as std::ffi::c_char,
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

pub static mut curline: *mut seg_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut sidedef: *mut side_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut linedef: *mut line_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut frontsector: *mut sector_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut backsector: *mut sector_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut drawsegs: [drawseg_t; (MAXDRAWSEGS) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut ds_p: *mut drawseg_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

unsafe extern "C" {
    pub fn R_StoreWallRange(start: std::ffi::c_int, stop: std::ffi::c_int);
}

pub unsafe extern "C" fn R_ClearDrawSegs() {
    unsafe {
        ds_p = drawsegs;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cliprange_t {
    pub first: std::ffi::c_int,
    pub last: std::ffi::c_int,
}

pub const MAXSEGS: std::ffi::c_int = 32;

pub static mut newend: *mut cliprange_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut solidsegs: [cliprange_t; (MAXSEGS) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn R_ClipSolidWallSegment(
    mut first: std::ffi::c_int,
    mut last: std::ffi::c_int,
) {
    unsafe {
        let mut next: *mut cliprange_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut start: *mut cliprange_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        start = solidsegs;
        // TODO: while statement not yet translated:
        //
        //     while (start->last < first-1)
        // 	start++;
        todo!("while statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (first < start->first)
        //     {
        // 	if (last < start->first-1)
        // 	{
        // 	    // Post is entirely visible (above start),
        // 	    //  so insert a new clippost.
        // 	    R_StoreWallRange (first, last);
        // 	    next = newend;
        // 	    newend++;
        //
        // 	    while (next != start)
        // 	    {
        // 		*next = *(next-1);
        // 		next--;
        // 	    }
        // 	    next->first = first;
        // 	    next->last = last;
        // 	    return;
        // 	}
        //
        // 	// There is a fragment above *start.
        // 	R_StoreWallRange (first, start->first - 1);
        // 	// Now adjust the clip size.
        // 	start->first = first;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // Bottom contained in start?
        //     if (last <= start->last)
        // 	return;
        todo!("if statement not yet translated");
        next = start;
        // TODO: while statement not yet translated:
        //
        //     while (last >= (next+1)->first-1)
        //     {
        // 	// There is a fragment between two posts.
        // 	R_StoreWallRange (next->last + 1, (next+1)->first - 1);
        // 	next++;
        //
        // 	if (last <= next->last)
        // 	{
        // 	    // Bottom is contained in next.
        // 	    // Adjust the clip size.
        // 	    start->last = next->last;
        // 	    goto crunch;
        // 	}
        //     }
        todo!("while statement not yet translated");
        R_StoreWallRange(((*next).last + 1), last);
        (*start).last = last;
        // C label crunch: (goto targets are not translated)
        // TODO: if statement not yet translated:
        //
        //
        //     // Remove start+1 to next from the clip list,
        //     // because start now covers their area.
        //   crunch:
        //     if (next == start)
        //     {
        // 	// Post just extended past the bottom of one post.
        // 	return;
        //     }
        todo!("if statement not yet translated");
        // TODO: while statement not yet translated:
        //
        //
        //
        //     while (next++ != newend)
        //     {
        // 	// Remove a post.
        // 	*++start = *next;
        //     }
        todo!("while statement not yet translated");
        newend = (start + 1);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn R_ClipPassWallSegment(
    mut first: std::ffi::c_int,
    mut last: std::ffi::c_int,
) {
    unsafe {
        let mut start: *mut cliprange_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        start = solidsegs;
        // TODO: while statement not yet translated:
        //
        //     while (start->last < first-1)
        // 	start++;
        todo!("while statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (first < start->first)
        //     {
        // 	if (last < start->first-1)
        // 	{
        // 	    // Post is entirely visible (above start).
        // 	    R_StoreWallRange (first, last);
        // 	    return;
        // 	}
        //
        // 	// There is a fragment above *start.
        // 	R_StoreWallRange (first, start->first - 1);
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // Bottom contained in start?
        //     if (last <= start->last)
        // 	return;
        todo!("if statement not yet translated");
        // TODO: while statement not yet translated:
        //
        //
        //     while (last >= (start+1)->first-1)
        //     {
        // 	// There is a fragment between two posts.
        // 	R_StoreWallRange (start->last + 1, (start+1)->first - 1);
        // 	start++;
        //
        // 	if (last <= start->last)
        // 	    return;
        //     }
        todo!("while statement not yet translated");
        R_StoreWallRange(((*start).last + 1), last);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn R_ClearClipSegs() {
    unsafe {
        solidsegs[(0) as usize].first = (-(0x7fffffff));
        solidsegs[(0) as usize].last = (-(1));
        solidsegs[(1) as usize].first = viewwidth;
        solidsegs[(1) as usize].last = 0x7fffffff;
        newend = (solidsegs + 2);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn R_AddLine(mut line: *mut seg_t) {
    unsafe {
        let mut x1: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut x2: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut angle1: angle_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut angle2: angle_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut span: angle_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut tspan: angle_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        curline = line;
        angle1 = R_PointToAngle((*(*line).v1).x, (*(*line).v1).y);
        angle2 = R_PointToAngle((*(*line).v2).x, (*(*line).v2).y);
        span = (angle1 - angle2);
        // TODO: if statement not yet translated:
        //
        //
        //     // Back side? I.e. backface culling?
        //     if (span >= ANG180)
        // 	return;
        todo!("if statement not yet translated");
        rw_angle1 = angle1;
        angle1 -= viewangle;
        angle2 -= viewangle;
        tspan = (angle1 + clipangle);
        // TODO: if statement not yet translated:
        //
        //     if (tspan > 2*clipangle)
        //     {
        // 	tspan -= 2*clipangle;
        //
        // 	// Totally off the left edge?
        // 	if (tspan >= span)
        // 	    return;
        //
        // 	angle1 = clipangle;
        //     }
        todo!("if statement not yet translated");
        tspan = (clipangle - angle2);
        // TODO: if statement not yet translated:
        //
        //     if (tspan > 2*clipangle)
        //     {
        // 	tspan -= 2*clipangle;
        //
        // 	// Totally off the left edge?
        // 	if (tspan >= span)
        // 	    return;
        // 	angle2 = -clipangle;
        //     }
        todo!("if statement not yet translated");
        angle1 = ((angle1 + ANG90) >> ANGLETOFINESHIFT);
        angle2 = ((angle2 + ANG90) >> ANGLETOFINESHIFT);
        x1 = viewangletox[(angle1) as usize];
        x2 = viewangletox[(angle2) as usize];
        // TODO: if statement not yet translated:
        //
        //
        //     // Does not cross a pixel?
        //     if (x1 == x2)
        // 	return;
        todo!("if statement not yet translated");
        backsector = (*line).backsector;
        // TODO: if statement not yet translated:
        //
        //
        //     // Single sided line?
        //     if (!backsector)
        // 	goto clipsolid;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // Closed door.
        //     if (backsector->ceilingheight <= frontsector->floorheight
        // 	|| backsector->floorheight >= frontsector->ceilingheight)
        // 	goto clipsolid;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // Window.
        //     if (backsector->ceilingheight != frontsector->ceilingheight
        // 	|| backsector->floorheight != frontsector->floorheight)
        // 	goto clippass;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // Reject empty lines used for triggers
        //     //  and special events.
        //     // Identical floor and ceiling on both sides,
        //     // identical light levels on both sides,
        //     // and no middle texture.
        //     if (backsector->ceilingpic == frontsector->ceilingpic
        // 	&& backsector->floorpic == frontsector->floorpic
        // 	&& backsector->lightlevel == frontsector->lightlevel
        // 	&& curline->sidedef->midtexture == 0)
        //     {
        // 	return;
        //     }
        todo!("if statement not yet translated");
        // C label clippass: (goto targets are not translated)
        R_ClipPassWallSegment(x1, (x2 - 1));
        return;
        // C label clipsolid: (goto targets are not translated)
        R_ClipSolidWallSegment(x1, (x2 - 1));
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub static mut checkcoord: [[std::ffi::c_int; 4]; 12] = unsafe {
    [
        [3, 0, 2, 1],
        [3, 0, 2, 0],
        [3, 1, 2, 0],
        [
            0,
            std::mem::zeroed(),
            std::mem::zeroed(),
            std::mem::zeroed(),
        ],
        [2, 0, 2, 1],
        [0, 0, 0, 0],
        [3, 1, 3, 0],
        [
            0,
            std::mem::zeroed(),
            std::mem::zeroed(),
            std::mem::zeroed(),
        ],
        [2, 0, 3, 1],
        [2, 1, 3, 1],
        [2, 1, 3, 0],
        std::mem::zeroed(),
    ]
};

pub unsafe extern "C" fn R_CheckBBox(mut bspcoord: *mut fixed_t) -> boolean {
    unsafe {
        let mut boxx: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut boxy: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut boxpos: std::ffi::c_int = unsafe {
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
        let mut angle1: angle_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut angle2: angle_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut span: angle_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut tspan: angle_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut start: *mut cliprange_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut sx1: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut sx2: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     // Find the corners of the box
        //     // that define the edges from current viewpoint.
        //     if (viewx <= bspcoord[BOXLEFT])
        // 	boxx = 0;
        //     else if (viewx < bspcoord[BOXRIGHT])
        // 	boxx = 1;
        //     else
        // 	boxx = 2;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (viewy >= bspcoord[BOXTOP])
        // 	boxy = 0;
        //     else if (viewy > bspcoord[BOXBOTTOM])
        // 	boxy = 1;
        //     else
        // 	boxy = 2;
        todo!("if statement not yet translated");
        boxpos = ((boxy << 2) + boxx);
        // TODO: if statement not yet translated:
        //
        //     if (boxpos == 5)
        // 	return true;
        todo!("if statement not yet translated");
        x1 = bspcoord[(checkcoord[(boxpos) as usize][(0) as usize]) as usize];
        y1 = bspcoord[(checkcoord[(boxpos) as usize][(1) as usize]) as usize];
        x2 = bspcoord[(checkcoord[(boxpos) as usize][(2) as usize]) as usize];
        y2 = bspcoord[(checkcoord[(boxpos) as usize][(3) as usize]) as usize];
        angle1 = (R_PointToAngle(x1, y1) - viewangle);
        angle2 = (R_PointToAngle(x2, y2) - viewangle);
        span = (angle1 - angle2);
        // TODO: if statement not yet translated:
        //
        //
        //     // Sitting on a line?
        //     if (span >= ANG180)
        // 	return true;
        todo!("if statement not yet translated");
        tspan = (angle1 + clipangle);
        // TODO: if statement not yet translated:
        //
        //
        //     if (tspan > 2*clipangle)
        //     {
        // 	tspan -= 2*clipangle;
        //
        // 	// Totally off the left edge?
        // 	if (tspan >= span)
        // 	    return false;
        //
        // 	angle1 = clipangle;
        //     }
        todo!("if statement not yet translated");
        tspan = (clipangle - angle2);
        // TODO: if statement not yet translated:
        //
        //     if (tspan > 2*clipangle)
        //     {
        // 	tspan -= 2*clipangle;
        //
        // 	// Totally off the left edge?
        // 	if (tspan >= span)
        // 	    return false;
        //
        // 	angle2 = -clipangle;
        //     }
        todo!("if statement not yet translated");
        angle1 = ((angle1 + ANG90) >> ANGLETOFINESHIFT);
        angle2 = ((angle2 + ANG90) >> ANGLETOFINESHIFT);
        sx1 = viewangletox[(angle1) as usize];
        sx2 = viewangletox[(angle2) as usize];
        // TODO: if statement not yet translated:
        //
        //
        //     // Does not cross a pixel.
        //     if (sx1 == sx2)
        // 	return false;
        todo!("if statement not yet translated");
        {
            let __macro_tmp = sx2;
            sx2 -= 1;
            __macro_tmp
        };
        start = solidsegs;
        // TODO: while statement not yet translated:
        //
        //     while (start->last < sx2)
        // 	start++;
        todo!("while statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (sx1 >= start->first
        // 	&& sx2 <= start->last)
        //     {
        // 	// The clippost contains the new span.
        // 	return false;
        //     }
        todo!("if statement not yet translated");
        return true_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn R_Subsector(mut num: std::ffi::c_int) {
    unsafe {
        let mut count: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut line: *mut seg_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut sub: *mut subsector_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //     if (num>=numsubsectors)
        // 	I_Error ("R_Subsector: ss %i with numss = %i",
        // 		 num,
        // 		 numsubsectors);
        todo!("if statement not yet translated");
        {
            let __macro_tmp = sscount;
            sscount += 1;
            __macro_tmp
        };
        sub = (&(subsectors[(num) as usize]) as *const _ as *mut _);
        frontsector = (*sub).sector;
        count = (*sub).numlines;
        line = (&(segs[((*sub).firstline) as usize]) as *const _ as *mut _);
        // TODO: if statement not yet translated:
        //
        //
        //     if (frontsector->floorheight < viewz)
        //     {
        // 	floorplane = R_FindPlane (frontsector->floorheight,
        // 				  frontsector->floorpic,
        // 				  frontsector->lightlevel);
        //     }
        //     else
        // 	floorplane = NULL;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (frontsector->ceilingheight > viewz
        // 	|| frontsector->ceilingpic == skyflatnum)
        //     {
        // 	ceilingplane = R_FindPlane (frontsector->ceilingheight,
        // 				    frontsector->ceilingpic,
        // 				    frontsector->lightlevel);
        //     }
        //     else
        // 	ceilingplane = NULL;
        todo!("if statement not yet translated");
        R_AddSprites(frontsector);
        // TODO: while statement not yet translated:
        //
        //
        //     while (count--)
        //     {
        // 	R_AddLine (line);
        // 	line++;
        //     }
        todo!("while statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn R_RenderBSPNode(mut bspnum: std::ffi::c_int) {
    unsafe {
        let mut bsp: *mut node_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut side: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     // Found a subsector?
        //     if (bspnum & NF_SUBSECTOR)
        //     {
        // 	if (bspnum == -1)
        // 	    R_Subsector (0);
        // 	else
        // 	    R_Subsector (bspnum&(~NF_SUBSECTOR));
        // 	return;
        //     }
        todo!("if statement not yet translated");
        bsp = (&(nodes[(bspnum) as usize]) as *const _ as *mut _);
        side = R_PointOnSide(viewx, viewy, bsp);
        R_RenderBSPNode((*bsp).children[(side) as usize]);
        // TODO: if statement not yet translated:
        //
        //
        //     // Possibly divide back space.
        //     if (R_CheckBBox (bsp->bbox[side^1]))
        // 	R_RenderBSPNode (bsp->children[side^1]);
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}
