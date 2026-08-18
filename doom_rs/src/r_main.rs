use crate::d_items::*;
use crate::d_net::*;
use crate::d_player::*;
use crate::d_think::*;
use crate::d_ticcmd::*;
use crate::doomdata::*;
use crate::doomdef::*;
use crate::doomtype::*;
use crate::info::*;
use crate::m_bbox::*;
use crate::m_fixed::*;
use crate::p_mobj::*;
use crate::p_pspr::*;
use crate::r_bsp::*;
use crate::r_data::*;
use crate::r_defs::*;
use crate::r_draw::*;
use crate::r_local::*;
use crate::r_plane::*;
use crate::r_segs::*;
use crate::r_sky::*;
use crate::r_state::*;
use crate::r_things::*;
use crate::tables::*;

unsafe extern "C" {
    pub static mut viewwidth: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut viewheight: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut viewwindowx: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut viewwindowy: std::ffi::c_int;
}

pub const LIGHTLEVELS: std::ffi::c_int = 16;

pub const LIGHTSEGSHIFT: std::ffi::c_int = 4;

pub const MAXLIGHTSCALE: std::ffi::c_int = 48;

pub const LIGHTSCALESHIFT: std::ffi::c_int = 12;

pub const MAXLIGHTZ: std::ffi::c_int = 128;

pub const LIGHTZSHIFT: std::ffi::c_int = 20;

pub const NUMCOLORMAPS: std::ffi::c_int = 32;

static mut rcsid: [std::ffi::c_char; 49] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        114 as std::ffi::c_char,
        95 as std::ffi::c_char,
        109 as std::ffi::c_char,
        97 as std::ffi::c_char,
        105 as std::ffi::c_char,
        110 as std::ffi::c_char,
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

pub const FIELDOFVIEW: std::ffi::c_int = 2048;

pub static mut viewangleoffset: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut validcount: std::ffi::c_int = unsafe { 1 };

pub static mut fixedcolormap: *mut lighttable_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

unsafe extern "C" {
    pub static mut walllights: *mut *mut lighttable_t;
}

pub static mut centerx: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut centery: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut centerxfrac: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut centeryfrac: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut projection: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut framecount: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut sscount: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut linecount: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut loopcount: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut viewx: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut viewy: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut viewz: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut viewangle: angle_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut viewcos: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut viewsin: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut viewplayer: *mut player_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut detailshift: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut clipangle: angle_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut viewangletox: [std::ffi::c_int; (FINEANGLES / 2) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut xtoviewangle: [angle_t; (SCREENWIDTH + 1) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut finecosine: *mut fixed_t =
    unsafe { (&(finesine[(FINEANGLES / 4) as usize]) as *const _ as *mut _) };

pub static mut scalelight: [[*mut lighttable_t; (MAXLIGHTSCALE) as usize]; (LIGHTLEVELS) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut scalelightfixed: [*mut lighttable_t; (MAXLIGHTSCALE) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut zlight: [[*mut lighttable_t; (MAXLIGHTZ) as usize]; (LIGHTLEVELS) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut extralight: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut colfunc: Option<unsafe extern "C" fn()> = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut basecolfunc: Option<unsafe extern "C" fn()> = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut fuzzcolfunc: Option<unsafe extern "C" fn()> = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut transcolfunc: Option<unsafe extern "C" fn()> = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut spanfunc: Option<unsafe extern "C" fn()> = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn R_AddPointToBox(
    mut x: std::ffi::c_int,
    mut y: std::ffi::c_int,
    mut box_: *mut fixed_t,
) {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (x< box[BOXLEFT])
        // 	box[BOXLEFT] = x;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (x> box[BOXRIGHT])
        // 	box[BOXRIGHT] = x;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (y< box[BOXBOTTOM])
        // 	box[BOXBOTTOM] = y;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (y> box[BOXTOP])
        // 	box[BOXTOP] = y;
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn R_PointOnSide(
    mut x: fixed_t,
    mut y: fixed_t,
    mut node: *mut node_t,
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
        // 	if (x <= node->x)
        // 	    return node->dy > 0;
        //
        // 	return node->dy < 0;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (!node->dy)
        //     {
        // 	if (y <= node->y)
        // 	    return node->dx < 0;
        //
        // 	return node->dx > 0;
        //     }
        todo!("if statement not yet translated");
        dx = (x - (*node).x);
        dy = (y - (*node).y);
        // TODO: if statement not yet translated:
        //
        //
        //     // Try to quickly decide by looking at sign bits.
        //     if ( (node->dy ^ node->dx ^ dx ^ dy)&0x80000000 )
        //     {
        // 	if  ( (node->dy ^ dx) & 0x80000000 )
        // 	{
        // 	    // (left is negative)
        // 	    return 1;
        // 	}
        // 	return 0;
        //     }
        todo!("if statement not yet translated");
        left = FixedMul(((*node).dy >> FRACBITS), dx);
        right = FixedMul(dy, ((*node).dx >> FRACBITS));
        // TODO: if statement not yet translated:
        //
        //
        //     if (right < left)
        //     {
        // 	// front side
        // 	return 0;
        //     }
        todo!("if statement not yet translated");
        return 1;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn R_PointOnSegSide(
    mut x: fixed_t,
    mut y: fixed_t,
    mut line: *mut seg_t,
) -> std::ffi::c_int {
    unsafe {
        let mut lx: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut ly: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut ldx: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut ldy: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
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
        lx = (*(*line).v1).x;
        ly = (*(*line).v1).y;
        ldx = ((*(*line).v2).x - lx);
        ldy = ((*(*line).v2).y - ly);
        // TODO: if statement not yet translated:
        //
        //
        //     if (!ldx)
        //     {
        // 	if (x <= lx)
        // 	    return ldy > 0;
        //
        // 	return ldy < 0;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (!ldy)
        //     {
        // 	if (y <= ly)
        // 	    return ldx < 0;
        //
        // 	return ldx > 0;
        //     }
        todo!("if statement not yet translated");
        dx = (x - lx);
        dy = (y - ly);
        // TODO: if statement not yet translated:
        //
        //
        //     // Try to quickly decide by looking at sign bits.
        //     if ( (ldy ^ ldx ^ dx ^ dy)&0x80000000 )
        //     {
        // 	if  ( (ldy ^ dx) & 0x80000000 )
        // 	{
        // 	    // (left is negative)
        // 	    return 1;
        // 	}
        // 	return 0;
        //     }
        todo!("if statement not yet translated");
        left = FixedMul((ldy >> FRACBITS), dx);
        right = FixedMul(dy, (ldx >> FRACBITS));
        // TODO: if statement not yet translated:
        //
        //
        //     if (right < left)
        //     {
        // 	// front side
        // 	return 0;
        //     }
        todo!("if statement not yet translated");
        return 1;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn R_PointToAngle(mut x: fixed_t, mut y: fixed_t) -> angle_t {
    unsafe {
        x -= viewx;
        y -= viewy;
        // TODO: if statement not yet translated:
        //
        //
        //     if ( (!x) && (!y) )
        // 	return 0;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (x>= 0)
        //     {
        // 	// x >=0
        // 	if (y>= 0)
        // 	{
        // 	    // y>= 0
        //
        // 	    if (x>y)
        // 	    {
        // 		// octant 0
        // 		return tantoangle[ SlopeDiv(y,x)];
        // 	    }
        // 	    else
        // 	    {
        // 		// octant 1
        // 		return ANG90-1-tantoangle[ SlopeDiv(x,y)];
        // 	    }
        // 	}
        // 	else
        // 	{
        // 	    // y<0
        // 	    y = -y;
        //
        // 	    if (x>y)
        // 	    {
        // 		// octant 8
        // 		return -tantoangle[SlopeDiv(y,x)];
        // 	    }
        // 	    else
        // 	    {
        // 		// octant 7
        // 		return ANG270+tantoangle[ SlopeDiv(x,y)];
        // 	    }
        // 	}
        //     }
        //     else
        //     {
        // 	// x<0
        // 	x = -x;
        //
        // 	if (y>= 0)
        // 	{
        // 	    // y>= 0
        // 	    if (x>y)
        // 	    {
        // 		// octant 3
        // 		return ANG180-1-tantoangle[ SlopeDiv(y,x)];
        // 	    }
        // 	    else
        // 	    {
        // 		// octant 2
        // 		return ANG90+ tantoangle[ SlopeDiv(x,y)];
        // 	    }
        // 	}
        // 	else
        // 	{
        // 	    // y<0
        // 	    y = -y;
        //
        // 	    if (x>y)
        // 	    {
        // 		// octant 4
        // 		return ANG180+tantoangle[ SlopeDiv(y,x)];
        // 	    }
        // 	    else
        // 	    {
        // 		 // octant 5
        // 		return ANG270-1-tantoangle[ SlopeDiv(x,y)];
        // 	    }
        // 	}
        //     }
        todo!("if statement not yet translated");
        return 0;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn R_PointToAngle2(
    mut x1: fixed_t,
    mut y1: fixed_t,
    mut x2: fixed_t,
    mut y2: fixed_t,
) -> angle_t {
    unsafe {
        viewx = x1;
        viewy = y1;
        return R_PointToAngle(x2, y2);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn R_PointToDist(mut x: fixed_t, mut y: fixed_t) -> fixed_t {
    unsafe {
        let mut angle: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut dx: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut dy: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut temp: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut dist: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        dx = abs((x - viewx));
        dy = abs((y - viewy));
        // TODO: if statement not yet translated:
        //
        //
        //     if (dy>dx)
        //     {
        // 	temp = dx;
        // 	dx = dy;
        // 	dy = temp;
        //     }
        todo!("if statement not yet translated");
        angle = ((tantoangle[(FixedDiv(dy, dx) >> DBITS) as usize] + ANG90) >> ANGLETOFINESHIFT);
        dist = FixedDiv(dx, finesine[(angle) as usize]);
        return dist;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn R_InitPointToAngle() {
    unsafe {}
}

pub unsafe extern "C" fn R_ScaleFromGlobalAngle(mut visangle: angle_t) -> fixed_t {
    unsafe {
        let mut scale: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut anglea: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut angleb: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut sinea: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut sineb: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut num: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut den: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        anglea = (ANG90 + (visangle - viewangle));
        angleb = (ANG90 + (visangle - rw_normalangle));
        sinea = finesine[(anglea >> ANGLETOFINESHIFT) as usize];
        sineb = finesine[(angleb >> ANGLETOFINESHIFT) as usize];
        num = (FixedMul(projection, sineb) << detailshift);
        den = FixedMul(rw_distance, sinea);
        // TODO: if statement not yet translated:
        //
        //
        //     if (den > num>>16)
        //     {
        // 	scale = FixedDiv (num, den);
        //
        // 	if (scale > 64*FRACUNIT)
        // 	    scale = 64*FRACUNIT;
        // 	else if (scale < 256)
        // 	    scale = 256;
        //     }
        //     else
        // 	scale = 64*FRACUNIT;
        todo!("if statement not yet translated");
        return scale;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn R_InitTables() {
    unsafe {
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn R_InitTextureMapping() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut x: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut t: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut focallength: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        focallength = FixedDiv(
            centerxfrac,
            finetangent[((FINEANGLES / 4) + (FIELDOFVIEW / 2)) as usize],
        );
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<FINEANGLES/2 ; i++)
        //     {
        // 	if (finetangent[i] > FRACUNIT*2)
        // 	    t = -1;
        // 	else if (finetangent[i] < -FRACUNIT*2)
        // 	    t = viewwidth+1;
        // 	else
        // 	{
        // 	    t = FixedMul (finetangent[i], focallength);
        // 	    t = (centerxfrac - t+FRACUNIT-1)>>FRACBITS;
        //
        // 	    if (t < -1)
        // 		t = -1;
        // 	    else if (t>viewwidth+1)
        // 		t = viewwidth+1;
        // 	}
        // 	viewangletox[i] = t;
        //     }
        todo!("for statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //
        //     // Scan viewangletox[] to generate xtoviewangle[]:
        //     //  xtoviewangle will give the smallest view angle
        //     //  that maps to x.
        //     for (x=0;x<=viewwidth;x++)
        //     {
        // 	i = 0;
        // 	while (viewangletox[i]>x)
        // 	    i++;
        // 	xtoviewangle[x] = (i<<ANGLETOFINESHIFT)-ANG90;
        //     }
        todo!("for statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //
        //     // Take out the fencepost cases from viewangletox.
        //     for (i=0 ; i<FINEANGLES/2 ; i++)
        //     {
        // 	t = FixedMul (finetangent[i], focallength);
        // 	t = centerx - t;
        //
        // 	if (viewangletox[i] == -1)
        // 	    viewangletox[i] = 0;
        // 	else if (viewangletox[i] == viewwidth+1)
        // 	    viewangletox[i]  = viewwidth;
        //     }
        todo!("for statement not yet translated");
        clipangle = xtoviewangle[(0) as usize];
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub const DISTMAP: std::ffi::c_int = 2;

pub unsafe extern "C" fn R_InitLightTables() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut j: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut level: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut startmap: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut scale: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: for statement not yet translated:
        //
        //
        //     // Calculate the light levels to use
        //     //  for each level / distance combination.
        //     for (i=0 ; i< LIGHTLEVELS ; i++)
        //     {
        // 	startmap = ((LIGHTLEVELS-1-i)*2)*NUMCOLORMAPS/LIGHTLEVELS;
        // 	for (j=0 ; j<MAXLIGHTZ ; j++)
        // 	{
        // 	    scale = FixedDiv ((SCREENWIDTH/2*FRACUNIT), (j+1)<<LIGHTZSHIFT);
        // 	    scale >>= LIGHTSCALESHIFT;
        // 	    level = startmap - scale/DISTMAP;
        //
        // 	    if (level < 0)
        // 		level = 0;
        //
        // 	    if (level >= NUMCOLORMAPS)
        // 		level = NUMCOLORMAPS-1;
        //
        // 	    zlight[i][j] = colormaps + level*256;
        // 	}
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub static mut setsizeneeded: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut setblocks: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut setdetail: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn R_SetViewSize(mut blocks: std::ffi::c_int, mut detail: std::ffi::c_int) {
    unsafe {
        setsizeneeded = true_;
        setblocks = blocks;
        setdetail = detail;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn R_ExecuteSetViewSize() {
    unsafe {
        let mut cosadj: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut dy: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut j: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut level: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut startmap: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        setsizeneeded = false_;
        // TODO: if statement not yet translated:
        //
        //
        //     if (setblocks == 11)
        //     {
        // 	scaledviewwidth = SCREENWIDTH;
        // 	viewheight = SCREENHEIGHT;
        //     }
        //     else
        //     {
        // 	scaledviewwidth = setblocks*32;
        // 	viewheight = (setblocks*168/10)&~7;
        //     }
        todo!("if statement not yet translated");
        detailshift = setdetail;
        viewwidth = (scaledviewwidth >> detailshift);
        centery = (viewheight / 2);
        centerx = (viewwidth / 2);
        centerxfrac = (centerx << FRACBITS);
        centeryfrac = (centery << FRACBITS);
        projection = centerxfrac;
        // TODO: if statement not yet translated:
        //
        //
        //     if (!detailshift)
        //     {
        // 	colfunc = basecolfunc = R_DrawColumn;
        // 	fuzzcolfunc = R_DrawFuzzColumn;
        // 	transcolfunc = R_DrawTranslatedColumn;
        // 	spanfunc = R_DrawSpan;
        //     }
        //     else
        //     {
        // 	colfunc = basecolfunc = R_DrawColumnLow;
        // 	fuzzcolfunc = R_DrawFuzzColumn;
        // 	transcolfunc = R_DrawTranslatedColumn;
        // 	spanfunc = R_DrawSpanLow;
        //     }
        todo!("if statement not yet translated");
        R_InitBuffer(scaledviewwidth, viewheight);
        R_InitTextureMapping();
        pspritescale = ((FRACUNIT * viewwidth) / SCREENWIDTH);
        pspriteiscale = ((FRACUNIT * SCREENWIDTH) / viewwidth);
        // TODO: for statement not yet translated:
        //
        //
        //     // thing clipping
        //     for (i=0 ; i<viewwidth ; i++)
        // 	screenheightarray[i] = viewheight;
        todo!("for statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //
        //     // planes
        //     for (i=0 ; i<viewheight ; i++)
        //     {
        // 	dy = ((i-viewheight/2)<<FRACBITS)+FRACUNIT/2;
        // 	dy = abs(dy);
        // 	yslope[i] = FixedDiv ( (viewwidth<<detailshift)/2*FRACUNIT, dy);
        //     }
        todo!("for statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<viewwidth ; i++)
        //     {
        // 	cosadj = abs(finecosine[xtoviewangle[i]>>ANGLETOFINESHIFT]);
        // 	distscale[i] = FixedDiv (FRACUNIT,cosadj);
        //     }
        todo!("for statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //
        //     // Calculate the light levels to use
        //     //  for each level / scale combination.
        //     for (i=0 ; i< LIGHTLEVELS ; i++)
        //     {
        // 	startmap = ((LIGHTLEVELS-1-i)*2)*NUMCOLORMAPS/LIGHTLEVELS;
        // 	for (j=0 ; j<MAXLIGHTSCALE ; j++)
        // 	{
        // 	    level = startmap - j*SCREENWIDTH/(viewwidth<<detailshift)/DISTMAP;
        //
        // 	    if (level < 0)
        // 		level = 0;
        //
        // 	    if (level >= NUMCOLORMAPS)
        // 		level = NUMCOLORMAPS-1;
        //
        // 	    scalelight[i][j] = colormaps + level*256;
        // 	}
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

unsafe extern "C" {
    pub static mut detailLevel: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut screenblocks: std::ffi::c_int;
}

pub unsafe extern "C" fn R_Init() {
    unsafe {
        R_InitData();
        printf((c"\nR_InitData").as_ptr());
        R_InitPointToAngle();
        printf((c"\nR_InitPointToAngle").as_ptr());
        R_InitTables();
        printf((c"\nR_InitTables").as_ptr());
        R_SetViewSize(screenblocks, detailLevel);
        R_InitPlanes();
        printf((c"\nR_InitPlanes").as_ptr());
        R_InitLightTables();
        printf((c"\nR_InitLightTables").as_ptr());
        R_InitSkyMap();
        printf((c"\nR_InitSkyMap").as_ptr());
        R_InitTranslationTables();
        printf((c"\nR_InitTranslationsTables").as_ptr());
        framecount = 0;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn R_PointInSubsector(mut x: fixed_t, mut y: fixed_t) -> *mut subsector_t {
    unsafe {
        let mut node: *mut node_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut side: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut nodenum: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     // single subsector is a special case
        //     if (!numnodes)
        // 	return subsectors;
        todo!("if statement not yet translated");
        nodenum = (numnodes - 1);
        // TODO: while statement not yet translated:
        //
        //
        //     while (! (nodenum & NF_SUBSECTOR) )
        //     {
        // 	node = &nodes[nodenum];
        // 	side = R_PointOnSide (x, y, node);
        // 	nodenum = node->children[side];
        //     }
        todo!("while statement not yet translated");
        return (&(subsectors[(nodenum & (!(NF_SUBSECTOR))) as usize]) as *const _ as *mut _);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn R_SetupFrame(mut player: *mut player_t) {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        viewplayer = player;
        viewx = (*(*player).mo).x;
        viewy = (*(*player).mo).y;
        viewangle = ((*(*player).mo).angle + viewangleoffset);
        extralight = (*player).extralight;
        viewz = (*player).viewz;
        viewsin = finesine[(viewangle >> ANGLETOFINESHIFT) as usize];
        viewcos = finecosine[(viewangle >> ANGLETOFINESHIFT) as usize];
        sscount = 0;
        // TODO: if statement not yet translated:
        //
        //
        //     if (player->fixedcolormap)
        //     {
        // 	fixedcolormap =
        // 	    colormaps
        // 	    + player->fixedcolormap*256*sizeof(lighttable_t);
        //
        // 	walllights = scalelightfixed;
        //
        // 	for (i=0 ; i<MAXLIGHTSCALE ; i++)
        // 	    scalelightfixed[i] = fixedcolormap;
        //     }
        //     else
        // 	fixedcolormap = 0;
        todo!("if statement not yet translated");
        {
            let __macro_tmp = framecount;
            framecount += 1;
            __macro_tmp
        };
        {
            let __macro_tmp = validcount;
            validcount += 1;
            __macro_tmp
        };
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn R_RenderPlayerView(mut player: *mut player_t) {
    unsafe {
        R_SetupFrame(player);
        R_ClearClipSegs();
        R_ClearDrawSegs();
        R_ClearPlanes();
        R_ClearSprites();
        NetUpdate();
        R_RenderBSPNode((numnodes - 1));
        NetUpdate();
        R_DrawPlanes();
        NetUpdate();
        R_DrawMasked();
        NetUpdate();
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}
