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
use crate::p_mobj::*;
use crate::p_pspr::*;
use crate::r_bsp::*;
use crate::r_data::*;
use crate::r_defs::*;
use crate::r_draw::*;
use crate::r_local::*;
use crate::r_main::*;
use crate::r_segs::*;
use crate::r_sky::*;
use crate::r_state::*;
use crate::r_things::*;
use crate::tables::*;
use crate::w_wad::*;
use crate::z_zone::*;

pub type planefunction_t = Option<unsafe extern "C" fn(std::ffi::c_int, std::ffi::c_int)>;

unsafe extern "C" {
    pub static mut ceilingfunc_t: planefunction_t;
}

static mut rcsid: [std::ffi::c_char; 50] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        114 as std::ffi::c_char,
        95 as std::ffi::c_char,
        112 as std::ffi::c_char,
        108 as std::ffi::c_char,
        97 as std::ffi::c_char,
        110 as std::ffi::c_char,
        101 as std::ffi::c_char,
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
        53 as std::ffi::c_char,
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

pub static mut floorfunc: planefunction_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut ceilingfunc: planefunction_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub const MAXVISPLANES: std::ffi::c_int = 128;

pub static mut visplanes: [visplane_t; (MAXVISPLANES) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut lastvisplane: *mut visplane_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut floorplane: *mut visplane_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut ceilingplane: *mut visplane_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub const MAXOPENINGS: std::ffi::c_int = (SCREENWIDTH * 64);

pub static mut openings: [std::ffi::c_short; (MAXOPENINGS) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut lastopening: *mut std::ffi::c_short = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut floorclip: [std::ffi::c_short; (SCREENWIDTH) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut ceilingclip: [std::ffi::c_short; (SCREENWIDTH) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut spanstart: [std::ffi::c_int; (SCREENHEIGHT) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut spanstop: [std::ffi::c_int; (SCREENHEIGHT) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut planezlight: *mut *mut lighttable_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut planeheight: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut yslope: [fixed_t; (SCREENHEIGHT) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut distscale: [fixed_t; (SCREENWIDTH) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut basexscale: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut baseyscale: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut cachedheight: [fixed_t; (SCREENHEIGHT) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut cacheddistance: [fixed_t; (SCREENHEIGHT) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut cachedxstep: [fixed_t; (SCREENHEIGHT) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut cachedystep: [fixed_t; (SCREENHEIGHT) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn R_InitPlanes() {
    unsafe {
        // TODO: statement not yet translated:
        //
        //   // Doh!
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn R_MapPlane(
    mut y: std::ffi::c_int,
    mut x1: std::ffi::c_int,
    mut x2: std::ffi::c_int,
) {
    unsafe {
        let mut angle: angle_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut distance: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut length: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut index: std::ffi::c_uint = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //     if (x2 < x1
        // 	|| x1<0
        // 	|| x2>=viewwidth
        // 	|| (unsigned)y>viewheight)
        //     {
        // 	I_Error ("R_MapPlane: %i, %i at %i",x1,x2,y);
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (planeheight != cachedheight[y])
        //     {
        // 	cachedheight[y] = planeheight;
        // 	distance = cacheddistance[y] = FixedMul (planeheight, yslope[y]);
        // 	ds_xstep = cachedxstep[y] = FixedMul (distance,basexscale);
        // 	ds_ystep = cachedystep[y] = FixedMul (distance,baseyscale);
        //     }
        //     else
        //     {
        // 	distance = cacheddistance[y];
        // 	ds_xstep = cachedxstep[y];
        // 	ds_ystep = cachedystep[y];
        //     }
        todo!("if statement not yet translated");
        length = FixedMul(distance, distscale[(x1) as usize]);
        angle = ((viewangle + xtoviewangle[(x1) as usize]) >> ANGLETOFINESHIFT);
        ds_xfrac = (viewx + FixedMul(finecosine[(angle) as usize], length));
        ds_yfrac = ((-(viewy)) - FixedMul(finesine[(angle) as usize], length));
        // TODO: if statement not yet translated:
        //
        //
        //     if (fixedcolormap)
        // 	ds_colormap = fixedcolormap;
        //     else
        //     {
        // 	index = distance >> LIGHTZSHIFT;
        //
        // 	if (index >= MAXLIGHTZ )
        // 	    index = MAXLIGHTZ-1;
        //
        // 	ds_colormap = planezlight[index];
        //     }
        todo!("if statement not yet translated");
        ds_y = y;
        ds_x1 = x1;
        ds_x2 = x2;
        spanfunc();
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn R_ClearPlanes() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut angle: angle_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: for statement not yet translated:
        //
        //
        //     // opening / clipping determination
        //     for (i=0 ; i<viewwidth ; i++)
        //     {
        // 	floorclip[i] = viewheight;
        // 	ceilingclip[i] = -1;
        //     }
        todo!("for statement not yet translated");
        lastvisplane = visplanes;
        lastopening = openings;
        memset(cachedheight, 0, std::mem::size_of_val(&(cachedheight)));
        angle = ((viewangle - ANG90) >> ANGLETOFINESHIFT);
        basexscale = FixedDiv(finecosine[(angle) as usize], centerxfrac);
        baseyscale = (-(FixedDiv(finesine[(angle) as usize], centerxfrac)));
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn R_FindPlane(
    mut height: fixed_t,
    mut picnum: std::ffi::c_int,
    mut lightlevel: std::ffi::c_int,
) -> *mut visplane_t {
    unsafe {
        let mut check: *mut visplane_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (picnum == skyflatnum)
        //     {
        // 	height = 0;			// all skys map together
        // 	lightlevel = 0;
        //     }
        todo!("if statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //
        //     for (check=visplanes; check<lastvisplane; check++)
        //     {
        // 	if (height == check->height
        // 	    && picnum == check->picnum
        // 	    && lightlevel == check->lightlevel)
        // 	{
        // 	    break;
        // 	}
        //     }
        todo!("for statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //
        //     if (check < lastvisplane)
        // 	return check;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (lastvisplane - visplanes == MAXVISPLANES)
        // 	I_Error ("R_FindPlane: no more visplanes");
        todo!("if statement not yet translated");
        {
            let __macro_tmp = lastvisplane;
            lastvisplane += 1;
            __macro_tmp
        };
        (*check).height = height;
        (*check).picnum = picnum;
        (*check).lightlevel = lightlevel;
        (*check).minx = SCREENWIDTH;
        (*check).maxx = (-(1));
        memset((*check).top, 0xff, std::mem::size_of_val(&((*check).top)));
        return check;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn R_CheckPlane(
    mut pl: *mut visplane_t,
    mut start: std::ffi::c_int,
    mut stop: std::ffi::c_int,
) -> *mut visplane_t {
    unsafe {
        let mut intrl: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut intrh: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut unionl: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut unionh: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut x: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (start < pl->minx)
        //     {
        // 	intrl = pl->minx;
        // 	unionl = start;
        //     }
        //     else
        //     {
        // 	unionl = pl->minx;
        // 	intrl = start;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (stop > pl->maxx)
        //     {
        // 	intrh = pl->maxx;
        // 	unionh = stop;
        //     }
        //     else
        //     {
        // 	unionh = pl->maxx;
        // 	intrh = stop;
        //     }
        todo!("if statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //
        //     for (x=intrl ; x<= intrh ; x++)
        // 	if (pl->top[x] != 0xff)
        // 	    break;
        todo!("for statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (x > intrh)
        //     {
        // 	pl->minx = unionl;
        // 	pl->maxx = unionh;
        //
        // 	// use the same one
        // 	return pl;
        //     }
        todo!("if statement not yet translated");
        (*lastvisplane).height = (*pl).height;
        (*lastvisplane).picnum = (*pl).picnum;
        (*lastvisplane).lightlevel = (*pl).lightlevel;
        pl = {
            let __macro_tmp = lastvisplane;
            lastvisplane += 1;
            __macro_tmp
        };
        (*pl).minx = start;
        (*pl).maxx = stop;
        memset((*pl).top, 0xff, std::mem::size_of_val(&((*pl).top)));
        return pl;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn R_MakeSpans(
    mut x: std::ffi::c_int,
    mut t1: std::ffi::c_int,
    mut b1: std::ffi::c_int,
    mut t2: std::ffi::c_int,
    mut b2: std::ffi::c_int,
) {
    unsafe {
        // TODO: while statement not yet translated:
        //
        //     while (t1 < t2 && t1<=b1)
        //     {
        // 	R_MapPlane (t1,spanstart[t1],x-1);
        // 	t1++;
        //     }
        todo!("while statement not yet translated");
        // TODO: while statement not yet translated:
        //
        //     while (b1 > b2 && b1>=t1)
        //     {
        // 	R_MapPlane (b1,spanstart[b1],x-1);
        // 	b1--;
        //     }
        todo!("while statement not yet translated");
        // TODO: while statement not yet translated:
        //
        //
        //     while (t2 < t1 && t2<=b2)
        //     {
        // 	spanstart[t2] = x;
        // 	t2++;
        //     }
        todo!("while statement not yet translated");
        // TODO: while statement not yet translated:
        //
        //     while (b2 > b1 && b2>=t2)
        //     {
        // 	spanstart[b2] = x;
        // 	b2--;
        //     }
        todo!("while statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn R_DrawPlanes() {
    unsafe {
        let mut pl: *mut visplane_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut light: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut x: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut stop: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut angle: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //     if (ds_p - drawsegs > MAXDRAWSEGS)
        // 	I_Error ("R_DrawPlanes: drawsegs overflow (%i)",
        // 		 ds_p - drawsegs);
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (lastvisplane - visplanes > MAXVISPLANES)
        // 	I_Error ("R_DrawPlanes: visplane overflow (%i)",
        // 		 lastvisplane - visplanes);
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (lastopening - openings > MAXOPENINGS)
        // 	I_Error ("R_DrawPlanes: opening overflow (%i)",
        // 		 lastopening - openings);
        todo!("if statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //     for (pl = visplanes ; pl < lastvisplane ; pl++)
        //     {
        // 	if (pl->minx > pl->maxx)
        // 	    continue;
        //
        //
        // 	// sky flat
        // 	if (pl->picnum == skyflatnum)
        // 	{
        // 	    dc_iscale = pspriteiscale>>detailshift;
        //
        // 	    // Sky is allways drawn full bright,
        // 	    //  i.e. colormaps[0] is used.
        // 	    // Because of this hack, sky is not affected
        // 	    //  by INVUL inverse mapping.
        // 	    dc_colormap = colormaps;
        // 	    dc_texturemid = skytexturemid;
        // 	    for (x=pl->minx ; x <= pl->maxx ; x++)
        // 	    {
        // 		dc_yl = pl->top[x];
        // 		dc_yh = pl->bottom[x];
        //
        // 		if (dc_yl <= dc_yh)
        // 		{
        // 		    angle = (viewangle + xtoviewangle[x])>>ANGLETOSKYSHIFT;
        // 		    dc_x = x;
        // 		    dc_source = R_GetColumn(skytexture, angle);
        // 		    colfunc ();
        // 		}
        // 	    }
        // 	    continue;
        // 	}
        //
        // 	// regular flat
        // 	ds_source = W_CacheLumpNum(firstflat +
        // 				   flattranslation[pl->picnum],
        // 				   PU_STATIC);
        //
        // 	planeheight = abs(pl->height-viewz);
        // 	light = (pl->lightlevel >> LIGHTSEGSHIFT)+extralight;
        //
        // 	if (light >= LIGHTLEVELS)
        // 	    light = LIGHTLEVELS-1;
        //
        // 	if (light < 0)
        // 	    light = 0;
        //
        // 	planezlight = zlight[light];
        //
        // 	pl->top[pl->maxx+1] = 0xff;
        // 	pl->top[pl->minx-1] = 0xff;
        //
        // 	stop = pl->maxx + 1;
        //
        // 	for (x=pl->minx ; x<= stop ; x++)
        // 	{
        // 	    R_MakeSpans(x,pl->top[x-1],
        // 			pl->bottom[x-1],
        // 			pl->top[x],
        // 			pl->bottom[x]);
        // 	}
        //
        // 	Z_ChangeTag (ds_source, PU_CACHE);
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}
