use crate::d_event::*;
use crate::d_items::*;
use crate::d_player::*;
use crate::d_think::*;
use crate::d_ticcmd::*;
use crate::doomdata::*;
use crate::doomdef::*;
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

static mut rcsid: [std::ffi::c_char; 50] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        112 as std::ffi::c_char,
        95 as std::ffi::c_char,
        115 as std::ffi::c_char,
        105 as std::ffi::c_char,
        103 as std::ffi::c_char,
        104 as std::ffi::c_char,
        116 as std::ffi::c_char,
        46 as std::ffi::c_char,
        99 as std::ffi::c_char,
        44 as std::ffi::c_char,
        118 as std::ffi::c_char,
        32 as std::ffi::c_char,
        49 as std::ffi::c_char,
        46 as std::ffi::c_char,
        51 as std::ffi::c_char,
        32 as std::ffi::c_char,
        49 as std::ffi::c_char,
        57 as std::ffi::c_char,
        57 as std::ffi::c_char,
        55 as std::ffi::c_char,
        47 as std::ffi::c_char,
        48 as std::ffi::c_char,
        49 as std::ffi::c_char,
        47 as std::ffi::c_char,
        50 as std::ffi::c_char,
        56 as std::ffi::c_char,
        32 as std::ffi::c_char,
        50 as std::ffi::c_char,
        50 as std::ffi::c_char,
        58 as std::ffi::c_char,
        48 as std::ffi::c_char,
        56 as std::ffi::c_char,
        58 as std::ffi::c_char,
        50 as std::ffi::c_char,
        56 as std::ffi::c_char,
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

pub static mut sightzstart: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut topslope: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut bottomslope: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut strace: divline_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut t2x: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut t2y: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut sightcounts: [std::ffi::c_int; (2) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn P_DivlineSide(
    mut x: fixed_t,
    mut y: fixed_t,
    mut node: *mut divline_t,
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
        //     if (!node->dx)
        //     {
        // 	if (x==node->x)
        // 	    return 2;
        //
        // 	if (x <= node->x)
        // 	    return node->dy > 0;
        //
        // 	return node->dy < 0;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (!node->dy)
        //     {
        // 	if (x==node->y)
        // 	    return 2;
        //
        // 	if (y <= node->y)
        // 	    return node->dx < 0;
        //
        // 	return node->dx > 0;
        //     }
        todo!("if statement not yet translated");
        dx = (x - (*node).x);
        dy = (y - (*node).y);
        left = (((*node).dy >> FRACBITS) * (dx >> FRACBITS));
        right = ((dy >> FRACBITS) * ((*node).dx >> FRACBITS));
        // TODO: if statement not yet translated:
        //
        //
        //     if (right < left)
        // 	return 0;	// front side
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (left == right)
        // 	return 2;
        todo!("if statement not yet translated");
        return 1;
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_InterceptVector2(
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
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_CrossSubsector(mut num: std::ffi::c_int) -> boolean {
    unsafe {
        let mut seg: *mut seg_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut line: *mut line_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut s1: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut s2: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut count: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut sub: *mut subsector_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut front: *mut sector_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut back: *mut sector_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut opentop: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut openbottom: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut divl: divline_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut v1: *mut vertex_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut v2: *mut vertex_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut frac: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut slope: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //     if (num>=numsubsectors)
        // 	I_Error ("P_CrossSubsector: ss %i with numss = %i",
        // 		 num,
        // 		 numsubsectors);
        todo!("if statement not yet translated");
        sub = (&(subsectors[(num) as usize]) as *const _ as *mut _);
        count = (*sub).numlines;
        seg = (&(segs[((*sub).firstline) as usize]) as *const _ as *mut _);
        // TODO: for statement not yet translated:
        //
        //
        //     for ( ; count ; seg++, count--)
        //     {
        // 	line = seg->linedef;
        //
        // 	// allready checked other side?
        // 	if (line->validcount == validcount)
        // 	    continue;
        //
        // 	line->validcount = validcount;
        //
        // 	v1 = line->v1;
        // 	v2 = line->v2;
        // 	s1 = P_DivlineSide (v1->x,v1->y, &strace);
        // 	s2 = P_DivlineSide (v2->x, v2->y, &strace);
        //
        // 	// line isn't crossed?
        // 	if (s1 == s2)
        // 	    continue;
        //
        // 	divl.x = v1->x;
        // 	divl.y = v1->y;
        // 	divl.dx = v2->x - v1->x;
        // 	divl.dy = v2->y - v1->y;
        // 	s1 = P_DivlineSide (strace.x, strace.y, &divl);
        // 	s2 = P_DivlineSide (t2x, t2y, &divl);
        //
        // 	// line isn't crossed?
        // 	if (s1 == s2)
        // 	    continue;
        //
        // 	// stop because it is not two sided anyway
        // 	// might do this after updating validcount?
        // 	if ( !(line->flags & ML_TWOSIDED) )
        // 	    return false;
        //
        // 	// crosses a two sided line
        // 	front = seg->frontsector;
        // 	back = seg->backsector;
        //
        // 	// no wall to block sight with?
        // 	if (front->floorheight == back->floorheight
        // 	    && front->ceilingheight == back->ceilingheight)
        // 	    continue;
        //
        // 	// possible occluder
        // 	// because of ceiling height differences
        // 	if (front->ceilingheight < back->ceilingheight)
        // 	    opentop = front->ceilingheight;
        // 	else
        // 	    opentop = back->ceilingheight;
        //
        // 	// because of ceiling height differences
        // 	if (front->floorheight > back->floorheight)
        // 	    openbottom = front->floorheight;
        // 	else
        // 	    openbottom = back->floorheight;
        //
        // 	// quick test for totally closed doors
        // 	if (openbottom >= opentop)
        // 	    return false;		// stop
        //
        // 	frac = P_InterceptVector2 (&strace, &divl);
        //
        // 	if (front->floorheight != back->floorheight)
        // 	{
        // 	    slope = FixedDiv (openbottom - sightzstart , frac);
        // 	    if (slope > bottomslope)
        // 		bottomslope = slope;
        // 	}
        //
        // 	if (front->ceilingheight != back->ceilingheight)
        // 	{
        // 	    slope = FixedDiv (opentop - sightzstart , frac);
        // 	    if (slope < topslope)
        // 		topslope = slope;
        // 	}
        //
        // 	if (topslope <= bottomslope)
        // 	    return false;		// stop
        //     }
        todo!("for statement not yet translated");
        return true_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_CrossBSPNode(mut bspnum: std::ffi::c_int) -> boolean {
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
        //     if (bspnum & NF_SUBSECTOR)
        //     {
        // 	if (bspnum == -1)
        // 	    return P_CrossSubsector (0);
        // 	else
        // 	    return P_CrossSubsector (bspnum&(~NF_SUBSECTOR));
        //     }
        todo!("if statement not yet translated");
        bsp = (&(nodes[(bspnum) as usize]) as *const _ as *mut _);
        side = P_DivlineSide(strace.x, strace.y, ((bsp) as *mut divline_t));
        // TODO: if statement not yet translated:
        //
        //     if (side == 2)
        // 	side = 0;	// an "on" should cross both sides
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     // cross the starting side
        //     if (!P_CrossBSPNode (bsp->children[side]) )
        // 	return false;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // the partition plane is crossed here
        //     if (side == P_DivlineSide (t2x, t2y,(divline_t *)bsp))
        //     {
        // 	// the line doesn't touch the other side
        // 	return true;
        //     }
        todo!("if statement not yet translated");
        return P_CrossBSPNode((*bsp).children[(side ^ 1) as usize]);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_CheckSight(mut t1: *mut mobj_t, mut t2: *mut mobj_t) -> boolean {
    unsafe {
        let mut s1: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut s2: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut pnum: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut bytenum: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut bitnum: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        s1 = ((*(*t1).subsector).sector - sectors);
        s2 = ((*(*t2).subsector).sector - sectors);
        pnum = ((s1 * numsectors) + s2);
        bytenum = (pnum >> 3);
        bitnum = (1 << (pnum & 7));
        // TODO: if statement not yet translated:
        //
        //
        //     // Check in REJECT table.
        //     if (rejectmatrix[bytenum]&bitnum)
        //     {
        // 	sightcounts[0]++;
        //
        // 	// can't possibly be connected
        // 	return false;
        //     }
        todo!("if statement not yet translated");
        {
            let __macro_tmp = sightcounts[(1) as usize];
            sightcounts[(1) as usize] += 1;
            __macro_tmp
        };
        {
            let __macro_tmp = validcount;
            validcount += 1;
            __macro_tmp
        };
        sightzstart = (((*t1).z + (*t1).height) - ((*t1).height >> 2));
        topslope = (((*t2).z + (*t2).height) - sightzstart);
        bottomslope = (((*t2).z) - sightzstart);
        strace.x = (*t1).x;
        strace.y = (*t1).y;
        t2x = (*t2).x;
        t2y = (*t2).y;
        strace.dx = ((*t2).x - (*t1).x);
        strace.dy = ((*t2).y - (*t1).y);
        return P_CrossBSPNode((numnodes - 1));
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}
