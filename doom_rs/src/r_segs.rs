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
use crate::r_plane::*;
use crate::r_sky::*;
use crate::r_state::*;
use crate::r_things::*;
use crate::tables::*;

static mut rcsid: [std::ffi::c_char; 49] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        114 as std::ffi::c_char,
        95 as std::ffi::c_char,
        115 as std::ffi::c_char,
        101 as std::ffi::c_char,
        103 as std::ffi::c_char,
        115 as std::ffi::c_char,
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
        57 as std::ffi::c_char,
        32 as std::ffi::c_char,
        50 as std::ffi::c_char,
        48 as std::ffi::c_char,
        58 as std::ffi::c_char,
        49 as std::ffi::c_char,
        48 as std::ffi::c_char,
        58 as std::ffi::c_char,
        49 as std::ffi::c_char,
        57 as std::ffi::c_char,
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

pub static mut segtextured: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut markfloor: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut markceiling: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut maskedtexture: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut toptexture: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut bottomtexture: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut midtexture: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut rw_normalangle: angle_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut rw_angle1: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut rw_x: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut rw_stopx: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut rw_centerangle: angle_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut rw_offset: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut rw_distance: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut rw_scale: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut rw_scalestep: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut rw_midtexturemid: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut rw_toptexturemid: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut rw_bottomtexturemid: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut worldtop: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut worldbottom: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut worldhigh: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut worldlow: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut pixhigh: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut pixlow: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut pixhighstep: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut pixlowstep: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut topfrac: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut topstep: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut bottomfrac: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut bottomstep: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut walllights: *mut *mut lighttable_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut maskedtexturecol: *mut std::ffi::c_short = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn R_RenderMaskedSegRange(
    mut ds: *mut drawseg_t,
    mut x1: std::ffi::c_int,
    mut x2: std::ffi::c_int,
) {
    unsafe {
        let mut index: std::ffi::c_uint = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut col: *mut column_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut lightnum: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut texnum: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        curline = (*ds).curline;
        frontsector = (*curline).frontsector;
        backsector = (*curline).backsector;
        texnum = texturetranslation[((*(*curline).sidedef).midtexture) as usize];
        lightnum = (((*frontsector).lightlevel >> LIGHTSEGSHIFT) + extralight);
        // TODO: if statement not yet translated:
        //
        //
        //     if (curline->v1->y == curline->v2->y)
        // 	lightnum--;
        //     else if (curline->v1->x == curline->v2->x)
        // 	lightnum++;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (lightnum < 0)
        // 	walllights = scalelight[0];
        //     else if (lightnum >= LIGHTLEVELS)
        // 	walllights = scalelight[LIGHTLEVELS-1];
        //     else
        // 	walllights = scalelight[lightnum];
        todo!("if statement not yet translated");
        maskedtexturecol = (*ds).maskedtexturecol;
        rw_scalestep = (*ds).scalestep;
        spryscale = ((*ds).scale1 + ((x1 - (*ds).x1) * rw_scalestep));
        mfloorclip = (*ds).sprbottomclip;
        mceilingclip = (*ds).sprtopclip;
        // TODO: if statement not yet translated:
        //
        //
        //     // find positioning
        //     if (curline->linedef->flags & ML_DONTPEGBOTTOM)
        //     {
        // 	dc_texturemid = frontsector->floorheight > backsector->floorheight
        // 	    ? frontsector->floorheight : backsector->floorheight;
        // 	dc_texturemid = dc_texturemid + textureheight[texnum] - viewz;
        //     }
        //     else
        //     {
        // 	dc_texturemid =frontsector->ceilingheight<backsector->ceilingheight
        // 	    ? frontsector->ceilingheight : backsector->ceilingheight;
        // 	dc_texturemid = dc_texturemid - viewz;
        //     }
        todo!("if statement not yet translated");
        dc_texturemid += (*(*curline).sidedef).rowoffset;
        // TODO: if statement not yet translated:
        //
        //
        //     if (fixedcolormap)
        // 	dc_colormap = fixedcolormap;
        todo!("if statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //
        //     // draw the columns
        //     for (dc_x = x1 ; dc_x <= x2 ; dc_x++)
        //     {
        // 	// calculate lighting
        // 	if (maskedtexturecol[dc_x] != MAXSHORT)
        // 	{
        // 	    if (!fixedcolormap)
        // 	    {
        // 		index = spryscale>>LIGHTSCALESHIFT;
        //
        // 		if (index >=  MAXLIGHTSCALE )
        // 		    index = MAXLIGHTSCALE-1;
        //
        // 		dc_colormap = walllights[index];
        // 	    }
        //
        // 	    sprtopscreen = centeryfrac - FixedMul(dc_texturemid, spryscale);
        // 	    dc_iscale = 0xffffffffu / (unsigned)spryscale;
        //
        // 	    // draw the texture
        // 	    col = (column_t *)(
        // 		(byte *)R_GetColumn(texnum,maskedtexturecol[dc_x]) -3);
        //
        // 	    R_DrawMaskedColumn (col);
        // 	    maskedtexturecol[dc_x] = MAXSHORT;
        // 	}
        // 	spryscale += rw_scalestep;
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub const HEIGHTBITS: std::ffi::c_int = 12;

pub const HEIGHTUNIT: std::ffi::c_int = (1 << HEIGHTBITS);

pub unsafe extern "C" fn R_RenderSegLoop() {
    unsafe {
        let mut angle: angle_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut index: std::ffi::c_uint = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut yl: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut yh: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut mid: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut texturecolumn: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut top: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut bottom: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: for statement not yet translated:
        //
        //
        //     //texturecolumn = 0;				// shut up compiler warning
        //
        //     for ( ; rw_x < rw_stopx ; rw_x++)
        //     {
        // 	// mark floor / ceiling areas
        // 	yl = (topfrac+HEIGHTUNIT-1)>>HEIGHTBITS;
        //
        // 	// no space above wall?
        // 	if (yl < ceilingclip[rw_x]+1)
        // 	    yl = ceilingclip[rw_x]+1;
        //
        // 	if (markceiling)
        // 	{
        // 	    top = ceilingclip[rw_x]+1;
        // 	    bottom = yl-1;
        //
        // 	    if (bottom >= floorclip[rw_x])
        // 		bottom = floorclip[rw_x]-1;
        //
        // 	    if (top <= bottom)
        // 	    {
        // 		ceilingplane->top[rw_x] = top;
        // 		ceilingplane->bottom[rw_x] = bottom;
        // 	    }
        // 	}
        //
        // 	yh = bottomfrac>>HEIGHTBITS;
        //
        // 	if (yh >= floorclip[rw_x])
        // 	    yh = floorclip[rw_x]-1;
        //
        // 	if (markfloor)
        // 	{
        // 	    top = yh+1;
        // 	    bottom = floorclip[rw_x]-1;
        // 	    if (top <= ceilingclip[rw_x])
        // 		top = ceilingclip[rw_x]+1;
        // 	    if (top <= bottom)
        // 	    {
        // 		floorplane->top[rw_x] = top;
        // 		floorplane->bottom[rw_x] = bottom;
        // 	    }
        // 	}
        //
        // 	// texturecolumn and lighting are independent of wall tiers
        // 	if (segtextured)
        // 	{
        // 	    // calculate texture offset
        // 	    angle = (rw_centerangle + xtoviewangle[rw_x])>>ANGLETOFINESHIFT;
        // 	    texturecolumn = rw_offset-FixedMul(finetangent[angle],rw_distance);
        // 	    texturecolumn >>= FRACBITS;
        // 	    // calculate lighting
        // 	    index = rw_scale>>LIGHTSCALESHIFT;
        //
        // 	    if (index >=  MAXLIGHTSCALE )
        // 		index = MAXLIGHTSCALE-1;
        //
        // 	    dc_colormap = walllights[index];
        // 	    dc_x = rw_x;
        // 	    dc_iscale = 0xffffffffu / (unsigned)rw_scale;
        // 	}
        //
        // 	// draw the wall tiers
        // 	if (midtexture)
        // 	{
        // 	    // single sided line
        // 	    dc_yl = yl;
        // 	    dc_yh = yh;
        // 	    dc_texturemid = rw_midtexturemid;
        // 	    dc_source = R_GetColumn(midtexture,texturecolumn);
        // 	    colfunc ();
        // 	    ceilingclip[rw_x] = viewheight;
        // 	    floorclip[rw_x] = -1;
        // 	}
        // 	else
        // 	{
        // 	    // two sided line
        // 	    if (toptexture)
        // 	    {
        // 		// top wall
        // 		mid = pixhigh>>HEIGHTBITS;
        // 		pixhigh += pixhighstep;
        //
        // 		if (mid >= floorclip[rw_x])
        // 		    mid = floorclip[rw_x]-1;
        //
        // 		if (mid >= yl)
        // 		{
        // 		    dc_yl = yl;
        // 		    dc_yh = mid;
        // 		    dc_texturemid = rw_toptexturemid;
        // 		    dc_source = R_GetColumn(toptexture,texturecolumn);
        // 		    colfunc ();
        // 		    ceilingclip[rw_x] = mid;
        // 		}
        // 		else
        // 		    ceilingclip[rw_x] = yl-1;
        // 	    }
        // 	    else
        // 	    {
        // 		// no top wall
        // 		if (markceiling)
        // 		    ceilingclip[rw_x] = yl-1;
        // 	    }
        //
        // 	    if (bottomtexture)
        // 	    {
        // 		// bottom wall
        // 		mid = (pixlow+HEIGHTUNIT-1)>>HEIGHTBITS;
        // 		pixlow += pixlowstep;
        //
        // 		// no space above wall?
        // 		if (mid <= ceilingclip[rw_x])
        // 		    mid = ceilingclip[rw_x]+1;
        //
        // 		if (mid <= yh)
        // 		{
        // 		    dc_yl = mid;
        // 		    dc_yh = yh;
        // 		    dc_texturemid = rw_bottomtexturemid;
        // 		    dc_source = R_GetColumn(bottomtexture,
        // 					    texturecolumn);
        // 		    colfunc ();
        // 		    floorclip[rw_x] = mid;
        // 		}
        // 		else
        // 		    floorclip[rw_x] = yh+1;
        // 	    }
        // 	    else
        // 	    {
        // 		// no bottom wall
        // 		if (markfloor)
        // 		    floorclip[rw_x] = yh+1;
        // 	    }
        //
        // 	    if (maskedtexture)
        // 	    {
        // 		// save texturecol
        // 		//  for backdrawing of masked mid texture
        // 		maskedtexturecol[rw_x] = texturecolumn;
        // 	    }
        // 	}
        //
        // 	rw_scale += rw_scalestep;
        // 	topfrac += topstep;
        // 	bottomfrac += bottomstep;
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn R_StoreWallRange(mut start: std::ffi::c_int, mut stop: std::ffi::c_int) {
    unsafe {
        let mut hyp: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut sineval: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut distangle: angle_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut offsetangle: angle_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut vtop: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut lightnum: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     // don't overflow and crash
        //     if (ds_p == &drawsegs[MAXDRAWSEGS])
        // 	return;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //     if (start >=viewwidth || start > stop)
        // 	I_Error ("Bad R_RenderWallRange: %i to %i", start , stop);
        todo!("if statement not yet translated");
        sidedef = (*curline).sidedef;
        linedef = (*curline).linedef;
        (*linedef).flags |= ML_MAPPED;
        rw_normalangle = ((*curline).angle + ANG90);
        offsetangle = abs((rw_normalangle - rw_angle1));
        // TODO: if statement not yet translated:
        //
        //
        //     if (offsetangle > ANG90)
        // 	offsetangle = ANG90;
        todo!("if statement not yet translated");
        distangle = (ANG90 - offsetangle);
        hyp = R_PointToDist((*(*curline).v1).x, (*(*curline).v1).y);
        sineval = finesine[(distangle >> ANGLETOFINESHIFT) as usize];
        rw_distance = FixedMul(hyp, sineval);
        (*ds_p).x1 = rw_x = start;
        (*ds_p).x2 = stop;
        (*ds_p).curline = curline;
        rw_stopx = (stop + 1);
        (*ds_p).scale1 =
            rw_scale = R_ScaleFromGlobalAngle((viewangle + xtoviewangle[(start) as usize]));
        // TODO: if statement not yet translated:
        //
        //
        //     if (stop > start )
        //     {
        // 	ds_p->scale2 = R_ScaleFromGlobalAngle (viewangle + xtoviewangle[stop]);
        // 	ds_p->scalestep = rw_scalestep =
        // 	    (ds_p->scale2 - rw_scale) / (stop-start);
        //     }
        //     else
        //     {
        // 	// UNUSED: try to fix the stretched line bug
        // #if 0
        // 	if (rw_distance < FRACUNIT/2)
        // 	{
        // 	    fixed_t		trx,try;
        // 	    fixed_t		gxt,gyt;
        //
        // 	    trx = curline->v1->x - viewx;
        // 	    try = curline->v1->y - viewy;
        //
        // 	    gxt = FixedMul(trx,viewcos);
        // 	    gyt = -FixedMul(try,viewsin);
        // 	    ds_p->scale1 = FixedDiv(projection, gxt-gyt)<<detailshift;
        // 	}
        // #endif
        // 	ds_p->scale2 = ds_p->scale1;
        //     }
        todo!("if statement not yet translated");
        worldtop = ((*frontsector).ceilingheight - viewz);
        worldbottom = ((*frontsector).floorheight - viewz);
        midtexture = toptexture = bottomtexture = maskedtexture = 0;
        (*ds_p).maskedtexturecol = NULL;
        // TODO: if statement not yet translated:
        //
        //
        //     if (!backsector)
        //     {
        // 	// single sided line
        // 	midtexture = texturetranslation[sidedef->midtexture];
        // 	// a single sided line is terminal, so it must mark ends
        // 	markfloor = markceiling = true;
        // 	if (linedef->flags & ML_DONTPEGBOTTOM)
        // 	{
        // 	    vtop = frontsector->floorheight +
        // 		textureheight[sidedef->midtexture];
        // 	    // bottom of texture at bottom
        // 	    rw_midtexturemid = vtop - viewz;
        // 	}
        // 	else
        // 	{
        // 	    // top of texture at top
        // 	    rw_midtexturemid = worldtop;
        // 	}
        // 	rw_midtexturemid += sidedef->rowoffset;
        //
        // 	ds_p->silhouette = SIL_BOTH;
        // 	ds_p->sprtopclip = screenheightarray;
        // 	ds_p->sprbottomclip = negonearray;
        // 	ds_p->bsilheight = MAXINT;
        // 	ds_p->tsilheight = MININT;
        //     }
        //     else
        //     {
        // 	// two sided line
        // 	ds_p->sprtopclip = ds_p->sprbottomclip = NULL;
        // 	ds_p->silhouette = 0;
        //
        // 	if (frontsector->floorheight > backsector->floorheight)
        // 	{
        // 	    ds_p->silhouette = SIL_BOTTOM;
        // 	    ds_p->bsilheight = frontsector->floorheight;
        // 	}
        // 	else if (backsector->floorheight > viewz)
        // 	{
        // 	    ds_p->silhouette = SIL_BOTTOM;
        // 	    ds_p->bsilheight = MAXINT;
        // 	    // ds_p->sprbottomclip = negonearray;
        // 	}
        //
        // 	if (frontsector->ceilingheight < backsector->ceilingheight)
        // 	{
        // 	    ds_p->silhouette |= SIL_TOP;
        // 	    ds_p->tsilheight = frontsector->ceilingheight;
        // 	}
        // 	else if (backsector->ceilingheight < viewz)
        // 	{
        // 	    ds_p->silhouette |= SIL_TOP;
        // 	    ds_p->tsilheight = MININT;
        // 	    // ds_p->sprtopclip = screenheightarray;
        // 	}
        //
        // 	if (backsector->ceilingheight <= frontsector->floorheight)
        // 	{
        // 	    ds_p->sprbottomclip = negonearray;
        // 	    ds_p->bsilheight = MAXINT;
        // 	    ds_p->silhouette |= SIL_BOTTOM;
        // 	}
        //
        // 	if (backsector->floorheight >= frontsector->ceilingheight)
        // 	{
        // 	    ds_p->sprtopclip = screenheightarray;
        // 	    ds_p->tsilheight = MININT;
        // 	    ds_p->silhouette |= SIL_TOP;
        // 	}
        //
        // 	worldhigh = backsector->ceilingheight - viewz;
        // 	worldlow = backsector->floorheight - viewz;
        //
        // 	// hack to allow height changes in outdoor areas
        // 	if (frontsector->ceilingpic == skyflatnum
        // 	    && backsector->ceilingpic == skyflatnum)
        // 	{
        // 	    worldtop = worldhigh;
        // 	}
        //
        //
        // 	if (worldlow != worldbottom
        // 	    || backsector->floorpic != frontsector->floorpic
        // 	    || backsector->lightlevel != frontsector->lightlevel)
        // 	{
        // 	    markfloor = true;
        // 	}
        // 	else
        // 	{
        // 	    // same plane on both sides
        // 	    markfloor = false;
        // 	}
        //
        //
        // 	if (worldhigh != worldtop
        // 	    || backsector->ceilingpic != frontsector->ceilingpic
        // 	    || backsector->lightlevel != frontsector->lightlevel)
        // 	{
        // 	    markceiling = true;
        // 	}
        // 	else
        // 	{
        // 	    // same plane on both sides
        // 	    markceiling = false;
        // 	}
        //
        // 	if (backsector->ceilingheight <= frontsector->floorheight
        // 	    || backsector->floorheight >= frontsector->ceilingheight)
        // 	{
        // 	    // closed door
        // 	    markceiling = markfloor = true;
        // 	}
        //
        //
        // 	if (worldhigh < worldtop)
        // 	{
        // 	    // top texture
        // 	    toptexture = texturetranslation[sidedef->toptexture];
        // 	    if (linedef->flags & ML_DONTPEGTOP)
        // 	    {
        // 		// top of texture at top
        // 		rw_toptexturemid = worldtop;
        // 	    }
        // 	    else
        // 	    {
        // 		vtop =
        // 		    backsector->ceilingheight
        // 		    + textureheight[sidedef->toptexture];
        //
        // 		// bottom of texture
        // 		rw_toptexturemid = vtop - viewz;
        // 	    }
        // 	}
        // 	if (worldlow > worldbottom)
        // 	{
        // 	    // bottom texture
        // 	    bottomtexture = texturetranslation[sidedef->bottomtexture];
        //
        // 	    if (linedef->flags & ML_DONTPEGBOTTOM )
        // 	    {
        // 		// bottom of texture at bottom
        // 		// top of texture at top
        // 		rw_bottomtexturemid = worldtop;
        // 	    }
        // 	    else	// top of texture at top
        // 		rw_bottomtexturemid = worldlow;
        // 	}
        // 	rw_toptexturemid += sidedef->rowoffset;
        // 	rw_bottomtexturemid += sidedef->rowoffset;
        //
        // 	// allocate space for masked texture tables
        // 	if (sidedef->midtexture)
        // 	{
        // 	    // masked midtexture
        // 	    maskedtexture = true;
        // 	    ds_p->maskedtexturecol = maskedtexturecol = lastopening - rw_x;
        // 	    lastopening += rw_stopx - rw_x;
        // 	}
        //     }
        todo!("if statement not yet translated");
        segtextured = (((midtexture | toptexture) | bottomtexture) | maskedtexture);
        // TODO: if statement not yet translated:
        //
        //
        //     if (segtextured)
        //     {
        // 	offsetangle = rw_normalangle-rw_angle1;
        //
        // 	if (offsetangle > ANG180)
        // 	    offsetangle = -offsetangle;
        //
        // 	if (offsetangle > ANG90)
        // 	    offsetangle = ANG90;
        //
        // 	sineval = finesine[offsetangle >>ANGLETOFINESHIFT];
        // 	rw_offset = FixedMul (hyp, sineval);
        //
        // 	if (rw_normalangle-rw_angle1 < ANG180)
        // 	    rw_offset = -rw_offset;
        //
        // 	rw_offset += sidedef->textureoffset + curline->offset;
        // 	rw_centerangle = ANG90 + viewangle - rw_normalangle;
        //
        // 	// calculate light table
        // 	//  use different light tables
        // 	//  for horizontal / vertical / diagonal
        // 	// OPTIMIZE: get rid of LIGHTSEGSHIFT globally
        // 	if (!fixedcolormap)
        // 	{
        // 	    lightnum = (frontsector->lightlevel >> LIGHTSEGSHIFT)+extralight;
        //
        // 	    if (curline->v1->y == curline->v2->y)
        // 		lightnum--;
        // 	    else if (curline->v1->x == curline->v2->x)
        // 		lightnum++;
        //
        // 	    if (lightnum < 0)
        // 		walllights = scalelight[0];
        // 	    else if (lightnum >= LIGHTLEVELS)
        // 		walllights = scalelight[LIGHTLEVELS-1];
        // 	    else
        // 		walllights = scalelight[lightnum];
        // 	}
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // if a floor / ceiling plane is on the wrong side
        //     //  of the view plane, it is definitely invisible
        //     //  and doesn't need to be marked.
        //
        //
        //     if (frontsector->floorheight >= viewz)
        //     {
        // 	// above view plane
        // 	markfloor = false;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (frontsector->ceilingheight <= viewz
        // 	&& frontsector->ceilingpic != skyflatnum)
        //     {
        // 	// below view plane
        // 	markceiling = false;
        //     }
        todo!("if statement not yet translated");
        worldtop >>= 4;
        worldbottom >>= 4;
        topstep = (-(FixedMul(rw_scalestep, worldtop)));
        topfrac = ((centeryfrac >> 4) - FixedMul(worldtop, rw_scale));
        bottomstep = (-(FixedMul(rw_scalestep, worldbottom)));
        bottomfrac = ((centeryfrac >> 4) - FixedMul(worldbottom, rw_scale));
        // TODO: if statement not yet translated:
        //
        //
        //     if (backsector)
        //     {
        // 	worldhigh >>= 4;
        // 	worldlow >>= 4;
        //
        // 	if (worldhigh < worldtop)
        // 	{
        // 	    pixhigh = (centeryfrac>>4) - FixedMul (worldhigh, rw_scale);
        // 	    pixhighstep = -FixedMul (rw_scalestep,worldhigh);
        // 	}
        //
        // 	if (worldlow > worldbottom)
        // 	{
        // 	    pixlow = (centeryfrac>>4) - FixedMul (worldlow, rw_scale);
        // 	    pixlowstep = -FixedMul (rw_scalestep,worldlow);
        // 	}
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // render it
        //     if (markceiling)
        // 	ceilingplane = R_CheckPlane (ceilingplane, rw_x, rw_stopx-1);
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (markfloor)
        // 	floorplane = R_CheckPlane (floorplane, rw_x, rw_stopx-1);
        todo!("if statement not yet translated");
        R_RenderSegLoop();
        // TODO: if statement not yet translated:
        //
        //
        //
        //     // save sprite clipping info
        //     if ( ((ds_p->silhouette & SIL_TOP) || maskedtexture)
        // 	 && !ds_p->sprtopclip)
        //     {
        // 	memcpy (lastopening, ceilingclip+start, 2*(rw_stopx-start));
        // 	ds_p->sprtopclip = lastopening - start;
        // 	lastopening += rw_stopx - start;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if ( ((ds_p->silhouette & SIL_BOTTOM) || maskedtexture)
        // 	 && !ds_p->sprbottomclip)
        //     {
        // 	memcpy (lastopening, floorclip+start, 2*(rw_stopx-start));
        // 	ds_p->sprbottomclip = lastopening - start;
        // 	lastopening += rw_stopx - start;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (maskedtexture && !(ds_p->silhouette&SIL_TOP))
        //     {
        // 	ds_p->silhouette |= SIL_TOP;
        // 	ds_p->tsilheight = MININT;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (maskedtexture && !(ds_p->silhouette&SIL_BOTTOM))
        //     {
        // 	ds_p->silhouette |= SIL_BOTTOM;
        // 	ds_p->bsilheight = MAXINT;
        //     }
        todo!("if statement not yet translated");
        {
            let __macro_tmp = ds_p;
            ds_p += 1;
            __macro_tmp
        };
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}
