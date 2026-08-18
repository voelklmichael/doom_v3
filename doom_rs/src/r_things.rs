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
use crate::m_swap::*;
use crate::p_mobj::*;
use crate::p_pspr::*;
use crate::r_bsp::*;
use crate::r_data::*;
use crate::r_defs::*;
use crate::r_draw::*;
use crate::r_local::*;
use crate::r_main::*;
use crate::r_plane::*;
use crate::r_segs::*;
use crate::r_state::*;
use crate::tables::*;
use crate::w_wad::*;
use crate::z_zone::*;

pub const MAXVISSPRITES: std::ffi::c_int = 128;

unsafe extern "C" {
    pub fn R_AddPSprites();
}

unsafe extern "C" {
    pub fn R_DrawSprites();
}

unsafe extern "C" {
    pub fn R_ClipVisSprite(vis: *mut vissprite_t, xl: std::ffi::c_int, xh: std::ffi::c_int);
}

static mut rcsid: [std::ffi::c_char; 51] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        114 as std::ffi::c_char,
        95 as std::ffi::c_char,
        116 as std::ffi::c_char,
        104 as std::ffi::c_char,
        105 as std::ffi::c_char,
        110 as std::ffi::c_char,
        103 as std::ffi::c_char,
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
        49 as std::ffi::c_char,
        54 as std::ffi::c_char,
        58 as std::ffi::c_char,
        52 as std::ffi::c_char,
        55 as std::ffi::c_char,
        58 as std::ffi::c_char,
        53 as std::ffi::c_char,
        54 as std::ffi::c_char,
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

pub const MINZ: std::ffi::c_int = (FRACUNIT * 4);

pub const BASEYCENTER: std::ffi::c_int = 100;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct maskdraw_t {
    pub x1: std::ffi::c_int,
    pub x2: std::ffi::c_int,
    pub column: std::ffi::c_int,
    pub topclip: std::ffi::c_int,
    pub bottomclip: std::ffi::c_int,
}

pub static mut pspritescale: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut pspriteiscale: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut spritelights: *mut *mut lighttable_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut negonearray: [std::ffi::c_short; (SCREENWIDTH) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut screenheightarray: [std::ffi::c_short; (SCREENWIDTH) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut sprites: *mut spritedef_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut numsprites: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut sprtemp: [spriteframe_t; (29) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut maxframe: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut spritename: *mut std::ffi::c_char = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn R_InstallSpriteLump(
    mut lump: std::ffi::c_int,
    mut frame: std::ffi::c_uint,
    mut rotation: std::ffi::c_uint,
    mut flipped: boolean,
) {
    unsafe {
        let mut r: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (frame >= 29 || rotation > 8)
        // 	I_Error("R_InstallSpriteLump: "
        // 		"Bad frame characters in lump %i", lump);
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if ((int)frame > maxframe)
        // 	maxframe = frame;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (rotation == 0)
        //     {
        // 	// the lump should be used for all rotations
        // 	if (sprtemp[frame].rotate == false)
        // 	    I_Error ("R_InitSprites: Sprite %s frame %c has "
        // 		     "multip rot=0 lump", spritename, 'A'+frame);
        //
        // 	if (sprtemp[frame].rotate == true)
        // 	    I_Error ("R_InitSprites: Sprite %s frame %c has rotations "
        // 		     "and a rot=0 lump", spritename, 'A'+frame);
        //
        // 	sprtemp[frame].rotate = false;
        // 	for (r=0 ; r<8 ; r++)
        // 	{
        // 	    sprtemp[frame].lump[r] = lump - firstspritelump;
        // 	    sprtemp[frame].flip[r] = (byte)flipped;
        // 	}
        // 	return;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // the lump is only used for one rotation
        //     if (sprtemp[frame].rotate == false)
        // 	I_Error ("R_InitSprites: Sprite %s frame %c has rotations "
        // 		 "and a rot=0 lump", spritename, 'A'+frame);
        todo!("if statement not yet translated");
        sprtemp[(frame) as usize].rotate = true_;
        {
            let __macro_tmp = rotation;
            rotation -= 1;
            __macro_tmp
        };
        // TODO: if statement not yet translated:
        //
        //     if (sprtemp[frame].lump[rotation] != -1)
        // 	I_Error ("R_InitSprites: Sprite %s : %c : %c "
        // 		 "has two lumps mapped to it",
        // 		 spritename, 'A'+frame, '1'+rotation);
        todo!("if statement not yet translated");
        sprtemp[(frame) as usize].lump[(rotation) as usize] = (lump - firstspritelump);
        sprtemp[(frame) as usize].flip[(rotation) as usize] = ((flipped) as byte);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn R_InitSpriteDefs(mut namelist: *mut *mut std::ffi::c_char) {
    unsafe {
        let mut check: *mut *mut std::ffi::c_char = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut l: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut intname: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut frame: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut rotation: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut start: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut end: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut patched: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        check = namelist;
        // TODO: while statement not yet translated:
        //
        //     while (*check != NULL)
        // 	check++;
        todo!("while statement not yet translated");
        numsprites = (check - namelist);
        // TODO: if statement not yet translated:
        //
        //
        //     if (!numsprites)
        // 	return;
        todo!("if statement not yet translated");
        sprites = Z_Malloc(
            (numsprites * std::mem::size_of_val(&(*(sprites)))),
            PU_STATIC,
            NULL,
        );
        start = (firstspritelump - 1);
        end = (lastspritelump + 1);
        // TODO: for statement not yet translated:
        //
        //
        //     // scan all the lump names for each of the names,
        //     //  noting the highest frame letter.
        //     // Just compare 4 characters as ints
        //     for (i=0 ; i<numsprites ; i++)
        //     {
        // 	spritename = namelist[i];
        // 	memset (sprtemp,-1, sizeof(sprtemp));
        //
        // 	maxframe = -1;
        // 	intname = *(int *)namelist[i];
        //
        // 	// scan the lumps,
        // 	//  filling in the frames for whatever is found
        // 	for (l=start+1 ; l<end ; l++)
        // 	{
        // 	    if (*(int *)lumpinfo[l].name == intname)
        // 	    {
        // 		frame = lumpinfo[l].name[4] - 'A';
        // 		rotation = lumpinfo[l].name[5] - '0';
        //
        // 		if (modifiedgame)
        // 		    patched = W_GetNumForName (lumpinfo[l].name);
        // 		else
        // 		    patched = l;
        //
        // 		R_InstallSpriteLump (patched, frame, rotation, false);
        //
        // 		if (lumpinfo[l].name[6])
        // 		{
        // 		    frame = lumpinfo[l].name[6] - 'A';
        // 		    rotation = lumpinfo[l].name[7] - '0';
        // 		    R_InstallSpriteLump (l, frame, rotation, true);
        // 		}
        // 	    }
        // 	}
        //
        // 	// check the frames that were found for completeness
        // 	if (maxframe == -1)
        // 	{
        // 	    sprites[i].numframes = 0;
        // 	    continue;
        // 	}
        //
        // 	maxframe++;
        //
        // 	for (frame = 0 ; frame < maxframe ; frame++)
        // 	{
        // 	    switch ((int)sprtemp[frame].rotate)
        // 	    {
        // 	      case -1:
        // 		// no rotations were found for that frame at all
        // 		I_Error ("R_InitSprites: No patches found "
        // 			 "for %s frame %c", namelist[i], frame+'A');
        // 		break;
        //
        // 	      case 0:
        // 		// only the first rotation is needed
        // 		break;
        //
        // 	      case 1:
        // 		// must have all 8 frames
        // 		for (rotation=0 ; rotation<8 ; rotation++)
        // 		    if (sprtemp[frame].lump[rotation] == -1)
        // 			I_Error ("R_InitSprites: Sprite %s frame %c "
        // 				 "is missing rotations",
        // 				 namelist[i], frame+'A');
        // 		break;
        // 	    }
        // 	}
        //
        // 	// allocate space for the frames present and copy sprtemp to it
        // 	sprites[i].numframes = maxframe;
        // 	sprites[i].spriteframes =
        // 	    Z_Malloc (maxframe * sizeof(spriteframe_t), PU_STATIC, NULL);
        // 	memcpy (sprites[i].spriteframes, sprtemp, maxframe*sizeof(spriteframe_t));
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub static mut vissprites: [vissprite_t; (MAXVISSPRITES) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut vissprite_p: *mut vissprite_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut newvissprite: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn R_InitSprites(mut namelist: *mut *mut std::ffi::c_char) {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<SCREENWIDTH ; i++)
        //     {
        // 	negonearray[i] = -1;
        //     }
        todo!("for statement not yet translated");
        R_InitSpriteDefs(namelist);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn R_ClearSprites() {
    unsafe {
        vissprite_p = vissprites;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub static mut overflowsprite: vissprite_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn R_NewVisSprite() -> *mut vissprite_t {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (vissprite_p == &vissprites[MAXVISSPRITES])
        // 	return &overflowsprite;
        todo!("if statement not yet translated");
        {
            let __macro_tmp = vissprite_p;
            vissprite_p += 1;
            __macro_tmp
        };
        return (vissprite_p - 1);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub static mut mfloorclip: *mut std::ffi::c_short = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut mceilingclip: *mut std::ffi::c_short = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut spryscale: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut sprtopscreen: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn R_DrawMaskedColumn(mut column: *mut column_t) {
    unsafe {
        let mut topscreen: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut bottomscreen: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut basetexturemid: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        basetexturemid = dc_texturemid;
        // TODO: for statement not yet translated:
        //
        //
        //     for ( ; column->topdelta != 0xff ; )
        //     {
        // 	// calculate unclipped screen coordinates
        // 	//  for post
        // 	topscreen = sprtopscreen + spryscale*column->topdelta;
        // 	bottomscreen = topscreen + spryscale*column->length;
        //
        // 	dc_yl = (topscreen+FRACUNIT-1)>>FRACBITS;
        // 	dc_yh = (bottomscreen-1)>>FRACBITS;
        //
        // 	if (dc_yh >= mfloorclip[dc_x])
        // 	    dc_yh = mfloorclip[dc_x]-1;
        // 	if (dc_yl <= mceilingclip[dc_x])
        // 	    dc_yl = mceilingclip[dc_x]+1;
        //
        // 	if (dc_yl <= dc_yh)
        // 	{
        // 	    dc_source = (byte *)column + 3;
        // 	    dc_texturemid = basetexturemid - (column->topdelta<<FRACBITS);
        // 	    // dc_source = (byte *)column + 3 - column->topdelta;
        //
        // 	    // Drawn by either R_DrawColumn
        // 	    //  or (SHADOW) R_DrawFuzzColumn.
        // 	    colfunc ();
        // 	}
        // 	column = (column_t *)(  (byte *)column + column->length + 4);
        //     }
        todo!("for statement not yet translated");
        dc_texturemid = basetexturemid;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn R_DrawVisSprite(
    mut vis: *mut vissprite_t,
    mut x1: std::ffi::c_int,
    mut x2: std::ffi::c_int,
) {
    unsafe {
        let mut column: *mut column_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut texturecolumn: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut frac: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut patch: *mut patch_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        patch = W_CacheLumpNum(((*vis).patch + firstspritelump), PU_CACHE);
        dc_colormap = (*vis).colormap;
        // TODO: if statement not yet translated:
        //
        //
        //     if (!dc_colormap)
        //     {
        // 	// NULL colormap = shadow draw
        // 	colfunc = fuzzcolfunc;
        //     }
        //     else if (vis->mobjflags & MF_TRANSLATION)
        //     {
        // 	colfunc = R_DrawTranslatedColumn;
        // 	dc_translation = translationtables - 256 +
        // 	    ( (vis->mobjflags & MF_TRANSLATION) >> (MF_TRANSSHIFT-8) );
        //     }
        todo!("if statement not yet translated");
        dc_iscale = (abs((*vis).xiscale) >> detailshift);
        dc_texturemid = (*vis).texturemid;
        frac = (*vis).startfrac;
        spryscale = (*vis).scale;
        sprtopscreen = (centeryfrac - FixedMul(dc_texturemid, spryscale));
        // TODO: for statement not yet translated:
        //
        //
        //     for (dc_x=vis->x1 ; dc_x<=vis->x2 ; dc_x++, frac += vis->xiscale)
        //     {
        // 	texturecolumn = frac>>FRACBITS;
        // #ifdef RANGECHECK
        // 	if (texturecolumn < 0 || texturecolumn >= SHORT(patch->width))
        // 	    I_Error ("R_DrawSpriteRange: bad texturecolumn");
        // #endif
        // 	column = (column_t *) ((byte *)patch +
        // 			       LONG(patch->columnofs[texturecolumn]));
        // 	R_DrawMaskedColumn (column);
        //     }
        todo!("for statement not yet translated");
        colfunc = basecolfunc;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn R_ProjectSprite(mut thing: *mut mobj_t) {
    unsafe {
        let mut tr_x: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut tr_y: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut gxt: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut gyt: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut tx: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut tz: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut xscale: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut x1: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut x2: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut sprdef: *mut spritedef_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut sprframe: *mut spriteframe_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut lump: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut rot: std::ffi::c_uint = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut flip: boolean = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut index: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut vis: *mut vissprite_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut ang: angle_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut iscale: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        tr_x = ((*thing).x - viewx);
        tr_y = ((*thing).y - viewy);
        gxt = FixedMul(tr_x, viewcos);
        gyt = (-(FixedMul(tr_y, viewsin)));
        tz = (gxt - gyt);
        // TODO: if statement not yet translated:
        //
        //
        //     // thing is behind view plane?
        //     if (tz < MINZ)
        // 	return;
        todo!("if statement not yet translated");
        xscale = FixedDiv(projection, tz);
        gxt = (-(FixedMul(tr_x, viewsin)));
        gyt = FixedMul(tr_y, viewcos);
        tx = (-(gyt + gxt));
        // TODO: if statement not yet translated:
        //
        //
        //     // too far off the side?
        //     if (abs(tx)>(tz<<2))
        // 	return;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //     if ((unsigned)thing->sprite >= numsprites)
        // 	I_Error ("R_ProjectSprite: invalid sprite number %i ",
        // 		 thing->sprite);
        todo!("if statement not yet translated");
        sprdef = (&(sprites[((*thing).sprite) as usize]) as *const _ as *mut _);
        // TODO: if statement not yet translated:
        //     if ( (thing->frame&FF_FRAMEMASK) >= sprdef->numframes )
        // 	I_Error ("R_ProjectSprite: invalid sprite frame %i : %i ",
        // 		 thing->sprite, thing->frame);
        todo!("if statement not yet translated");
        sprframe = (&((*sprdef).spriteframes[((*thing).frame & FF_FRAMEMASK) as usize]) as *const _
            as *mut _);
        // TODO: if statement not yet translated:
        //
        //
        //     if (sprframe->rotate)
        //     {
        // 	// choose a different rotation based on player view
        // 	ang = R_PointToAngle (thing->x, thing->y);
        // 	rot = (ang-thing->angle+(unsigned)(ANG45/2)*9)>>29;
        // 	lump = sprframe->lump[rot];
        // 	flip = (boolean)sprframe->flip[rot];
        //     }
        //     else
        //     {
        // 	// use single rotation for all views
        // 	lump = sprframe->lump[0];
        // 	flip = (boolean)sprframe->flip[0];
        //     }
        todo!("if statement not yet translated");
        tx -= spriteoffset[(lump) as usize];
        x1 = ((centerxfrac + FixedMul(tx, xscale)) >> FRACBITS);
        // TODO: if statement not yet translated:
        //
        //
        //     // off the right side?
        //     if (x1 > viewwidth)
        // 	return;
        todo!("if statement not yet translated");
        tx += spritewidth[(lump) as usize];
        x2 = (((centerxfrac + FixedMul(tx, xscale)) >> FRACBITS) - 1);
        // TODO: if statement not yet translated:
        //
        //
        //     // off the left side
        //     if (x2 < 0)
        // 	return;
        todo!("if statement not yet translated");
        vis = R_NewVisSprite();
        (*vis).mobjflags = (*thing).flags;
        (*vis).scale = (xscale << detailshift);
        (*vis).gx = (*thing).x;
        (*vis).gy = (*thing).y;
        (*vis).gz = (*thing).z;
        (*vis).gzt = ((*thing).z + spritetopoffset[(lump) as usize]);
        (*vis).texturemid = ((*vis).gzt - viewz);
        (*vis).x1 = (if (x1 < 0) { 0 } else { x1 });
        (*vis).x2 = (if (x2 >= viewwidth) {
            (viewwidth - 1)
        } else {
            x2
        });
        iscale = FixedDiv(FRACUNIT, xscale);
        // TODO: if statement not yet translated:
        //
        //
        //     if (flip)
        //     {
        // 	vis->startfrac = spritewidth[lump]-1;
        // 	vis->xiscale = -iscale;
        //     }
        //     else
        //     {
        // 	vis->startfrac = 0;
        // 	vis->xiscale = iscale;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (vis->x1 > x1)
        // 	vis->startfrac += vis->xiscale*(vis->x1-x1);
        todo!("if statement not yet translated");
        (*vis).patch = lump;
        // TODO: if statement not yet translated:
        //
        //
        //     // get light level
        //     if (thing->flags & MF_SHADOW)
        //     {
        // 	// shadow draw
        // 	vis->colormap = NULL;
        //     }
        //     else if (fixedcolormap)
        //     {
        // 	// fixed map
        // 	vis->colormap = fixedcolormap;
        //     }
        //     else if (thing->frame & FF_FULLBRIGHT)
        //     {
        // 	// full bright
        // 	vis->colormap = colormaps;
        //     }
        //
        //     else
        //     {
        // 	// diminished light
        // 	index = xscale>>(LIGHTSCALESHIFT-detailshift);
        //
        // 	if (index >= MAXLIGHTSCALE)
        // 	    index = MAXLIGHTSCALE-1;
        //
        // 	vis->colormap = spritelights[index];
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn R_AddSprites(mut sec: *mut sector_t) {
    unsafe {
        let mut thing: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut lightnum: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     // BSP is traversed by subsector.
        //     // A sector might have been split into several
        //     //  subsectors during BSP building.
        //     // Thus we check whether its already added.
        //     if (sec->validcount == validcount)
        // 	return;
        todo!("if statement not yet translated");
        (*sec).validcount = validcount;
        lightnum = (((*sec).lightlevel >> LIGHTSEGSHIFT) + extralight);
        // TODO: if statement not yet translated:
        //
        //
        //     if (lightnum < 0)
        // 	spritelights = scalelight[0];
        //     else if (lightnum >= LIGHTLEVELS)
        // 	spritelights = scalelight[LIGHTLEVELS-1];
        //     else
        // 	spritelights = scalelight[lightnum];
        todo!("if statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //
        //     // Handle all things in sector.
        //     for (thing = sec->thinglist ; thing ; thing = thing->snext)
        // 	R_ProjectSprite (thing);
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn R_DrawPSprite(mut psp: *mut pspdef_t) {
    unsafe {
        let mut tx: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut x1: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut x2: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut sprdef: *mut spritedef_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut sprframe: *mut spriteframe_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut lump: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut flip: boolean = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut vis: *mut vissprite_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut avis: vissprite_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //     if ( (unsigned)psp->state->sprite >= numsprites)
        // 	I_Error ("R_ProjectSprite: invalid sprite number %i ",
        // 		 psp->state->sprite);
        todo!("if statement not yet translated");
        sprdef = (&(sprites[((*(*psp).state).sprite) as usize]) as *const _ as *mut _);
        // TODO: if statement not yet translated:
        //     if ( (psp->state->frame & FF_FRAMEMASK)  >= sprdef->numframes)
        // 	I_Error ("R_ProjectSprite: invalid sprite frame %i : %i ",
        // 		 psp->state->sprite, psp->state->frame);
        todo!("if statement not yet translated");
        sprframe = (&((*sprdef).spriteframes[((*(*psp).state).frame & FF_FRAMEMASK) as usize])
            as *const _ as *mut _);
        lump = (*sprframe).lump[(0) as usize];
        flip = (((*sprframe).flip[(0) as usize]) as boolean);
        tx = ((*psp).sx - (160 * FRACUNIT));
        tx -= spriteoffset[(lump) as usize];
        x1 = ((centerxfrac + FixedMul(tx, pspritescale)) >> FRACBITS);
        // TODO: if statement not yet translated:
        //
        //
        //     // off the right side
        //     if (x1 > viewwidth)
        // 	return;
        todo!("if statement not yet translated");
        tx += spritewidth[(lump) as usize];
        x2 = (((centerxfrac + FixedMul(tx, pspritescale)) >> FRACBITS) - 1);
        // TODO: if statement not yet translated:
        //
        //
        //     // off the left side
        //     if (x2 < 0)
        // 	return;
        todo!("if statement not yet translated");
        vis = (&(avis) as *const _ as *mut _);
        (*vis).mobjflags = 0;
        (*vis).texturemid = (((BASEYCENTER << FRACBITS) + (FRACUNIT / 2))
            - ((*psp).sy - spritetopoffset[(lump) as usize]));
        (*vis).x1 = (if (x1 < 0) { 0 } else { x1 });
        (*vis).x2 = (if (x2 >= viewwidth) {
            (viewwidth - 1)
        } else {
            x2
        });
        (*vis).scale = (pspritescale << detailshift);
        // TODO: if statement not yet translated:
        //
        //
        //     if (flip)
        //     {
        // 	vis->xiscale = -pspriteiscale;
        // 	vis->startfrac = spritewidth[lump]-1;
        //     }
        //     else
        //     {
        // 	vis->xiscale = pspriteiscale;
        // 	vis->startfrac = 0;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (vis->x1 > x1)
        // 	vis->startfrac += vis->xiscale*(vis->x1-x1);
        todo!("if statement not yet translated");
        (*vis).patch = lump;
        // TODO: if statement not yet translated:
        //
        //
        //     if (viewplayer->powers[pw_invisibility] > 4*32
        // 	|| viewplayer->powers[pw_invisibility] & 8)
        //     {
        // 	// shadow draw
        // 	vis->colormap = NULL;
        //     }
        //     else if (fixedcolormap)
        //     {
        // 	// fixed color
        // 	vis->colormap = fixedcolormap;
        //     }
        //     else if (psp->state->frame & FF_FULLBRIGHT)
        //     {
        // 	// full bright
        // 	vis->colormap = colormaps;
        //     }
        //     else
        //     {
        // 	// local light
        // 	vis->colormap = spritelights[MAXLIGHTSCALE-1];
        //     }
        todo!("if statement not yet translated");
        R_DrawVisSprite(vis, (*vis).x1, (*vis).x2);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn R_DrawPlayerSprites() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut lightnum: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut psp: *mut pspdef_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        lightnum =
            (((*(*(*(*viewplayer).mo).subsector).sector).lightlevel >> LIGHTSEGSHIFT) + extralight);
        // TODO: if statement not yet translated:
        //
        //
        //     if (lightnum < 0)
        // 	spritelights = scalelight[0];
        //     else if (lightnum >= LIGHTLEVELS)
        // 	spritelights = scalelight[LIGHTLEVELS-1];
        //     else
        // 	spritelights = scalelight[lightnum];
        todo!("if statement not yet translated");
        mfloorclip = screenheightarray;
        mceilingclip = negonearray;
        // TODO: for statement not yet translated:
        //
        //
        //     // add all active psprites
        //     for (i=0, psp=viewplayer->psprites;
        // 	 i<NUMPSPRITES;
        // 	 i++,psp++)
        //     {
        // 	if (psp->state)
        // 	    R_DrawPSprite (psp);
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub static mut vsprsortedhead: vissprite_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn R_SortVisSprites() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut count: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut ds: *mut vissprite_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut best: *mut vissprite_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut unsorted: vissprite_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut bestscale: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        count = (vissprite_p - vissprites);
        unsorted.next = unsorted.prev = (&(unsorted) as *const _ as *mut _);
        // TODO: if statement not yet translated:
        //
        //
        //     if (!count)
        // 	return;
        todo!("if statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //
        //     for (ds=vissprites ; ds<vissprite_p ; ds++)
        //     {
        // 	ds->next = ds+1;
        // 	ds->prev = ds-1;
        //     }
        todo!("for statement not yet translated");
        vissprites[(0) as usize].prev = (&(unsorted) as *const _ as *mut _);
        unsorted.next = (&(vissprites[(0) as usize]) as *const _ as *mut _);
        (*(vissprite_p - 1)).next = (&(unsorted) as *const _ as *mut _);
        unsorted.prev = (vissprite_p - 1);
        vsprsortedhead.next =
            vsprsortedhead.prev = (&(vsprsortedhead) as *const vissprite_t as *mut vissprite_t);
        // TODO: for statement not yet translated:
        //
        //     for (i=0 ; i<count ; i++)
        //     {
        // 	bestscale = MAXINT;
        // 	for (ds=unsorted.next ; ds!= &unsorted ; ds=ds->next)
        // 	{
        // 	    if (ds->scale < bestscale)
        // 	    {
        // 		bestscale = ds->scale;
        // 		best = ds;
        // 	    }
        // 	}
        // 	best->next->prev = best->prev;
        // 	best->prev->next = best->next;
        // 	best->next = &vsprsortedhead;
        // 	best->prev = vsprsortedhead.prev;
        // 	vsprsortedhead.prev->next = best;
        // 	vsprsortedhead.prev = best;
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn R_DrawSprite(mut spr: *mut vissprite_t) {
    unsafe {
        let mut ds: *mut drawseg_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut clipbot: [std::ffi::c_short; (SCREENWIDTH) as usize] = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut cliptop: [std::ffi::c_short; (SCREENWIDTH) as usize] = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut x: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut r1: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut r2: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut scale: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut lowscale: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut silhouette: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: for statement not yet translated:
        //
        //
        //     for (x = spr->x1 ; x<=spr->x2 ; x++)
        // 	clipbot[x] = cliptop[x] = -2;
        todo!("for statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //
        //     // Scan drawsegs from end to start for obscuring segs.
        //     // The first drawseg that has a greater scale
        //     //  is the clip seg.
        //     for (ds=ds_p-1 ; ds >= drawsegs ; ds--)
        //     {
        // 	// determine if the drawseg obscures the sprite
        // 	if (ds->x1 > spr->x2
        // 	    || ds->x2 < spr->x1
        // 	    || (!ds->silhouette
        // 		&& !ds->maskedtexturecol) )
        // 	{
        // 	    // does not cover sprite
        // 	    continue;
        // 	}
        //
        // 	r1 = ds->x1 < spr->x1 ? spr->x1 : ds->x1;
        // 	r2 = ds->x2 > spr->x2 ? spr->x2 : ds->x2;
        //
        // 	if (ds->scale1 > ds->scale2)
        // 	{
        // 	    lowscale = ds->scale2;
        // 	    scale = ds->scale1;
        // 	}
        // 	else
        // 	{
        // 	    lowscale = ds->scale1;
        // 	    scale = ds->scale2;
        // 	}
        //
        // 	if (scale < spr->scale
        // 	    || ( lowscale < spr->scale
        // 		 && !R_PointOnSegSide (spr->gx, spr->gy, ds->curline) ) )
        // 	{
        // 	    // masked mid texture?
        // 	    if (ds->maskedtexturecol)
        // 		R_RenderMaskedSegRange (ds, r1, r2);
        // 	    // seg is behind sprite
        // 	    continue;
        // 	}
        //
        //
        // 	// clip this piece of the sprite
        // 	silhouette = ds->silhouette;
        //
        // 	if (spr->gz >= ds->bsilheight)
        // 	    silhouette &= ~SIL_BOTTOM;
        //
        // 	if (spr->gzt <= ds->tsilheight)
        // 	    silhouette &= ~SIL_TOP;
        //
        // 	if (silhouette == 1)
        // 	{
        // 	    // bottom sil
        // 	    for (x=r1 ; x<=r2 ; x++)
        // 		if (clipbot[x] == -2)
        // 		    clipbot[x] = ds->sprbottomclip[x];
        // 	}
        // 	else if (silhouette == 2)
        // 	{
        // 	    // top sil
        // 	    for (x=r1 ; x<=r2 ; x++)
        // 		if (cliptop[x] == -2)
        // 		    cliptop[x] = ds->sprtopclip[x];
        // 	}
        // 	else if (silhouette == 3)
        // 	{
        // 	    // both
        // 	    for (x=r1 ; x<=r2 ; x++)
        // 	    {
        // 		if (clipbot[x] == -2)
        // 		    clipbot[x] = ds->sprbottomclip[x];
        // 		if (cliptop[x] == -2)
        // 		    cliptop[x] = ds->sprtopclip[x];
        // 	    }
        // 	}
        //
        //     }
        todo!("for statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //
        //     // all clipping has been performed, so draw the sprite
        //
        //     // check for unclipped columns
        //     for (x = spr->x1 ; x<=spr->x2 ; x++)
        //     {
        // 	if (clipbot[x] == -2)
        // 	    clipbot[x] = viewheight;
        //
        // 	if (cliptop[x] == -2)
        // 	    cliptop[x] = -1;
        //     }
        todo!("for statement not yet translated");
        mfloorclip = clipbot;
        mceilingclip = cliptop;
        R_DrawVisSprite(spr, (*spr).x1, (*spr).x2);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn R_DrawMasked() {
    unsafe {
        let mut spr: *mut vissprite_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut ds: *mut drawseg_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        R_SortVisSprites();
        // TODO: if statement not yet translated:
        //
        //
        //     if (vissprite_p > vissprites)
        //     {
        // 	// draw all vissprites back to front
        // 	for (spr = vsprsortedhead.next ;
        // 	     spr != &vsprsortedhead ;
        // 	     spr=spr->next)
        // 	{
        //
        // 	    R_DrawSprite (spr);
        // 	}
        //     }
        todo!("if statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //
        //     // render any remaining masked mid textures
        //     for (ds=ds_p-1 ; ds >= drawsegs ; ds--)
        // 	if (ds->maskedtexturecol)
        // 	    R_RenderMaskedSegRange (ds, ds->x1, ds->x2);
        todo!("for statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // draw the psprites on top of everything
        //     //  but does not draw on side views
        //     if (!viewangleoffset)
        // 	R_DrawPlayerSprites ();
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}
