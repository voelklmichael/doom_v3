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
use crate::p_local::*;
use crate::p_mobj::*;
use crate::p_pspr::*;
use crate::p_spec::*;
use crate::r_bsp::*;
use crate::r_defs::*;
use crate::r_draw::*;
use crate::r_local::*;
use crate::r_main::*;
use crate::r_plane::*;
use crate::r_segs::*;
use crate::r_sky::*;
use crate::r_state::*;
use crate::r_things::*;
use crate::tables::*;
use crate::w_wad::*;
use crate::z_zone::*;

static mut rcsid: [std::ffi::c_char; 49] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        114 as std::ffi::c_char,
        95 as std::ffi::c_char,
        100 as std::ffi::c_char,
        97 as std::ffi::c_char,
        116 as std::ffi::c_char,
        97 as std::ffi::c_char,
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mappatch_t {
    pub originx: std::ffi::c_short,
    pub originy: std::ffi::c_short,
    pub patch: std::ffi::c_short,
    pub stepdir: std::ffi::c_short,
    pub colormap: std::ffi::c_short,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct maptexture_t {
    pub name: [std::ffi::c_char; (8) as usize],
    pub masked: boolean,
    pub width: std::ffi::c_short,
    pub height: std::ffi::c_short,
    pub columndirectory: *mut *mut std::ffi::c_void,
    pub patchcount: std::ffi::c_short,
    pub patches: [mappatch_t; (1) as usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct texpatch_t {
    pub originx: std::ffi::c_int,
    pub originy: std::ffi::c_int,
    pub patch: std::ffi::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct texture_t {
    pub name: [std::ffi::c_char; (8) as usize],
    pub width: std::ffi::c_short,
    pub height: std::ffi::c_short,
    pub patchcount: std::ffi::c_short,
    pub patches: [texpatch_t; (1) as usize],
}

pub static mut firstflat: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut lastflat: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut numflats: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut firstpatch: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut lastpatch: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut numpatches: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut firstspritelump: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut lastspritelump: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut numspritelumps: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut numtextures: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut textures: *mut *mut texture_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut texturewidthmask: *mut std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut textureheight: *mut fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut texturecompositesize: *mut std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut texturecolumnlump: *mut *mut std::ffi::c_short = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut texturecolumnofs: *mut *mut std::ffi::c_ushort = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut texturecomposite: *mut *mut byte = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut flattranslation: *mut std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut texturetranslation: *mut std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut spritewidth: *mut fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut spriteoffset: *mut fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut spritetopoffset: *mut fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut colormaps: *mut lighttable_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn R_DrawColumnInCache(
    mut patch: *mut column_t,
    mut cache: *mut byte,
    mut originy: std::ffi::c_int,
    mut cacheheight: std::ffi::c_int,
) {
    unsafe {
        let mut count: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut position: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut source: *mut byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut dest: *mut byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        dest = (((cache) as *mut byte) + 3);
        // TODO: while statement not yet translated:
        //
        //
        //     while (patch->topdelta != 0xff)
        //     {
        // 	source = (byte *)patch + 3;
        // 	count = patch->length;
        // 	position = originy + patch->topdelta;
        //
        // 	if (position < 0)
        // 	{
        // 	    count += position;
        // 	    position = 0;
        // 	}
        //
        // 	if (position + count > cacheheight)
        // 	    count = cacheheight - position;
        //
        // 	if (count > 0)
        // 	    memcpy (cache + position, source, count);
        //
        // 	patch = (column_t *)(  (byte *)patch + patch->length + 4);
        //     }
        todo!("while statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn R_GenerateComposite(mut texnum: std::ffi::c_int) {
    unsafe {
        let mut block: *mut byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut texture: *mut texture_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut patch: *mut texpatch_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut realpatch: *mut patch_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut x: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut x1: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut x2: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut patchcol: *mut column_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut collump: *mut std::ffi::c_short = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut colofs: *mut std::ffi::c_ushort = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        texture = textures[(texnum) as usize];
        block = Z_Malloc(
            texturecompositesize[(texnum) as usize],
            PU_STATIC,
            (&(texturecomposite[(texnum) as usize]) as *const _ as *mut _),
        );
        collump = texturecolumnlump[(texnum) as usize];
        colofs = texturecolumnofs[(texnum) as usize];
        patch = (*texture).patches;
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 , patch = texture->patches;
        // 	 i<texture->patchcount;
        // 	 i++, patch++)
        //     {
        // 	realpatch = W_CacheLumpNum (patch->patch, PU_CACHE);
        // 	x1 = patch->originx;
        // 	x2 = x1 + SHORT(realpatch->width);
        //
        // 	if (x1<0)
        // 	    x = 0;
        // 	else
        // 	    x = x1;
        //
        // 	if (x2 > texture->width)
        // 	    x2 = texture->width;
        //
        // 	for ( ; x<x2 ; x++)
        // 	{
        // 	    // Column does not have multiple patches?
        // 	    if (collump[x] >= 0)
        // 		continue;
        //
        // 	    patchcol = (column_t *)((byte *)realpatch
        // 				    + LONG(realpatch->columnofs[x-x1]));
        // 	    R_DrawColumnInCache (patchcol,
        // 				 block + colofs[x],
        // 				 patch->originy,
        // 				 texture->height);
        // 	}
        //
        //     }
        todo!("for statement not yet translated");
        Z_ChangeTag(block, PU_CACHE);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn R_GenerateLookup(mut texnum: std::ffi::c_int) {
    unsafe {
        let mut texture: *mut texture_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut patchcount: *mut byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut patch: *mut texpatch_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut realpatch: *mut patch_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut x: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut x1: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut x2: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut collump: *mut std::ffi::c_short = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut colofs: *mut std::ffi::c_ushort = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        texture = textures[(texnum) as usize];
        texturecomposite[(texnum) as usize] = 0;
        texturecompositesize[(texnum) as usize] = 0;
        collump = texturecolumnlump[(texnum) as usize];
        colofs = texturecolumnofs[(texnum) as usize];
        patchcount = ((alloca((*texture).width)) as *mut byte);
        memset(patchcount, 0, (*texture).width);
        patch = (*texture).patches;
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 , patch = texture->patches;
        // 	 i<texture->patchcount;
        // 	 i++, patch++)
        //     {
        // 	realpatch = W_CacheLumpNum (patch->patch, PU_CACHE);
        // 	x1 = patch->originx;
        // 	x2 = x1 + SHORT(realpatch->width);
        //
        // 	if (x1 < 0)
        // 	    x = 0;
        // 	else
        // 	    x = x1;
        //
        // 	if (x2 > texture->width)
        // 	    x2 = texture->width;
        // 	for ( ; x<x2 ; x++)
        // 	{
        // 	    patchcount[x]++;
        // 	    collump[x] = patch->patch;
        // 	    colofs[x] = LONG(realpatch->columnofs[x-x1])+3;
        // 	}
        //     }
        todo!("for statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //
        //     for (x=0 ; x<texture->width ; x++)
        //     {
        // 	if (!patchcount[x])
        // 	{
        // 	    printf ("R_GenerateLookup: column without a patch (%s)\n",
        // 		    texture->name);
        // 	    return;
        // 	}
        // 	// I_Error ("R_GenerateLookup: column without a patch");
        //
        // 	if (patchcount[x] > 1)
        // 	{
        // 	    // Use the cached block.
        // 	    collump[x] = -1;
        // 	    colofs[x] = texturecompositesize[texnum];
        //
        // 	    if (texturecompositesize[texnum] > 0x10000-texture->height)
        // 	    {
        // 		I_Error ("R_GenerateLookup: texture %i is >64k",
        // 			 texnum);
        // 	    }
        //
        // 	    texturecompositesize[texnum] += texture->height;
        // 	}
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn R_GetColumn(
    mut tex: std::ffi::c_int,
    mut col: std::ffi::c_int,
) -> *mut byte {
    unsafe {
        let mut lump: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut ofs: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        col &= texturewidthmask[(tex) as usize];
        lump = texturecolumnlump[(tex) as usize][(col) as usize];
        ofs = texturecolumnofs[(tex) as usize][(col) as usize];
        // TODO: if statement not yet translated:
        //
        //
        //     if (lump > 0)
        // 	return (byte *)W_CacheLumpNum(lump,PU_CACHE)+ofs;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (!texturecomposite[tex])
        // 	R_GenerateComposite (tex);
        todo!("if statement not yet translated");
        return (texturecomposite[(tex) as usize] + ofs);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn R_InitTextures() {
    unsafe {
        let mut mtexture: *mut maptexture_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut texture: *mut texture_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut mpatch: *mut mappatch_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut patch: *mut texpatch_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut j: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut maptex: *mut std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut maptex2: *mut std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut maptex1: *mut std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut name: [std::ffi::c_char; (9) as usize] = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut names: *mut std::ffi::c_char = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut name_p: *mut std::ffi::c_char = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut patchlookup: *mut std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut totalwidth: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut nummappatches: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut offset: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut maxoff: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut maxoff2: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut numtextures1: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut numtextures2: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut directory: *mut std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut temp1: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut temp2: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut temp3: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        name[(8) as usize] = 0;
        names = W_CacheLumpName((c"PNAMES").as_ptr(), PU_STATIC);
        nummappatches = LONG((*((names) as *mut std::ffi::c_int)));
        name_p = (names + 4);
        patchlookup = alloca((nummappatches * std::mem::size_of_val(&(*(patchlookup)))));
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<nummappatches ; i++)
        //     {
        // 	strncpy (name,name_p+i*8, 8);
        // 	patchlookup[i] = W_CheckNumForName (name);
        //     }
        todo!("for statement not yet translated");
        Z_Free(names);
        maptex = maptex1 = W_CacheLumpName((c"TEXTURE1").as_ptr(), PU_STATIC);
        numtextures1 = LONG((*(maptex)));
        maxoff = W_LumpLength(W_GetNumForName((c"TEXTURE1").as_ptr()));
        directory = (maptex + 1);
        // TODO: if statement not yet translated:
        //
        //
        //     if (W_CheckNumForName ("TEXTURE2") != -1)
        //     {
        // 	maptex2 = W_CacheLumpName ("TEXTURE2", PU_STATIC);
        // 	numtextures2 = LONG(*maptex2);
        // 	maxoff2 = W_LumpLength (W_GetNumForName ("TEXTURE2"));
        //     }
        //     else
        //     {
        // 	maptex2 = NULL;
        // 	numtextures2 = 0;
        // 	maxoff2 = 0;
        //     }
        todo!("if statement not yet translated");
        numtextures = (numtextures1 + numtextures2);
        textures = Z_Malloc((numtextures * 4), PU_STATIC, 0);
        texturecolumnlump = Z_Malloc((numtextures * 4), PU_STATIC, 0);
        texturecolumnofs = Z_Malloc((numtextures * 4), PU_STATIC, 0);
        texturecomposite = Z_Malloc((numtextures * 4), PU_STATIC, 0);
        texturecompositesize = Z_Malloc((numtextures * 4), PU_STATIC, 0);
        texturewidthmask = Z_Malloc((numtextures * 4), PU_STATIC, 0);
        textureheight = Z_Malloc((numtextures * 4), PU_STATIC, 0);
        totalwidth = 0;
        temp1 = W_GetNumForName((c"S_START").as_ptr());
        temp2 = (W_GetNumForName((c"S_END").as_ptr()) - 1);
        temp3 = ((((temp2 - temp1) + 63) / 64) + ((numtextures + 63) / 64));
        printf((c"[").as_ptr());
        // TODO: for statement not yet translated:
        //
        //     for (i = 0; i < temp3; i++)
        // 	printf(" ");
        todo!("for statement not yet translated");
        printf((c"         ]").as_ptr());
        // TODO: for statement not yet translated:
        //
        //     for (i = 0; i < temp3; i++)
        // 	printf("\x8");
        todo!("for statement not yet translated");
        printf((c"\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08").as_ptr());
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<numtextures ; i++, directory++)
        //     {
        // 	if (!(i&63))
        // 	    printf (".");
        //
        // 	if (i == numtextures1)
        // 	{
        // 	    // Start looking in second texture file.
        // 	    maptex = maptex2;
        // 	    maxoff = maxoff2;
        // 	    directory = maptex+1;
        // 	}
        //
        // 	offset = LONG(*directory);
        //
        // 	if (offset > maxoff)
        // 	    I_Error ("R_InitTextures: bad texture directory");
        //
        // 	mtexture = (maptexture_t *) ( (byte *)maptex + offset);
        //
        // 	texture = textures[i] =
        // 	    Z_Malloc (sizeof(texture_t)
        // 		      + sizeof(texpatch_t)*(SHORT(mtexture->patchcount)-1),
        // 		      PU_STATIC, 0);
        //
        // 	texture->width = SHORT(mtexture->width);
        // 	texture->height = SHORT(mtexture->height);
        // 	texture->patchcount = SHORT(mtexture->patchcount);
        //
        // 	memcpy (texture->name, mtexture->name, sizeof(texture->name));
        // 	mpatch = &mtexture->patches[0];
        // 	patch = &texture->patches[0];
        //
        // 	for (j=0 ; j<texture->patchcount ; j++, mpatch++, patch++)
        // 	{
        // 	    patch->originx = SHORT(mpatch->originx);
        // 	    patch->originy = SHORT(mpatch->originy);
        // 	    patch->patch = patchlookup[SHORT(mpatch->patch)];
        // 	    if (patch->patch == -1)
        // 	    {
        // 		I_Error ("R_InitTextures: Missing patch in texture %s",
        // 			 texture->name);
        // 	    }
        // 	}
        // 	texturecolumnlump[i] = Z_Malloc (texture->width*2, PU_STATIC,0);
        // 	texturecolumnofs[i] = Z_Malloc (texture->width*2, PU_STATIC,0);
        //
        // 	j = 1;
        // 	while (j*2 <= texture->width)
        // 	    j<<=1;
        //
        // 	texturewidthmask[i] = j-1;
        // 	textureheight[i] = texture->height<<FRACBITS;
        //
        // 	totalwidth += texture->width;
        //     }
        todo!("for statement not yet translated");
        Z_Free(maptex1);
        // TODO: if statement not yet translated:
        //
        //     if (maptex2)
        // 	Z_Free (maptex2);
        todo!("if statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //
        //     // Precalculate whatever possible.
        //     for (i=0 ; i<numtextures ; i++)
        // 	R_GenerateLookup (i);
        todo!("for statement not yet translated");
        texturetranslation = Z_Malloc(((numtextures + 1) * 4), PU_STATIC, 0);
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<numtextures ; i++)
        // 	texturetranslation[i] = i;
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn R_InitFlats() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        firstflat = (W_GetNumForName((c"F_START").as_ptr()) + 1);
        lastflat = (W_GetNumForName((c"F_END").as_ptr()) - 1);
        numflats = ((lastflat - firstflat) + 1);
        flattranslation = Z_Malloc(((numflats + 1) * 4), PU_STATIC, 0);
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<numflats ; i++)
        // 	flattranslation[i] = i;
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn R_InitSpriteLumps() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut patch: *mut patch_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        firstspritelump = (W_GetNumForName((c"S_START").as_ptr()) + 1);
        lastspritelump = (W_GetNumForName((c"S_END").as_ptr()) - 1);
        numspritelumps = ((lastspritelump - firstspritelump) + 1);
        spritewidth = Z_Malloc((numspritelumps * 4), PU_STATIC, 0);
        spriteoffset = Z_Malloc((numspritelumps * 4), PU_STATIC, 0);
        spritetopoffset = Z_Malloc((numspritelumps * 4), PU_STATIC, 0);
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i< numspritelumps ; i++)
        //     {
        // 	if (!(i&63))
        // 	    printf (".");
        //
        // 	patch = W_CacheLumpNum (firstspritelump+i, PU_CACHE);
        // 	spritewidth[i] = SHORT(patch->width)<<FRACBITS;
        // 	spriteoffset[i] = SHORT(patch->leftoffset)<<FRACBITS;
        // 	spritetopoffset[i] = SHORT(patch->topoffset)<<FRACBITS;
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn R_InitColormaps() {
    unsafe {
        let mut lump: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut length: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        lump = W_GetNumForName((c"COLORMAP").as_ptr());
        length = (W_LumpLength(lump) + 255);
        colormaps = Z_Malloc(length, PU_STATIC, 0);
        colormaps = (((((colormaps) as std::ffi::c_int) + 255) & (!(0xff))) as *mut byte);
        W_ReadLump(lump, colormaps);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn R_InitData() {
    unsafe {
        R_InitTextures();
        printf((c"\nInitTextures").as_ptr());
        R_InitFlats();
        printf((c"\nInitFlats").as_ptr());
        R_InitSpriteLumps();
        printf((c"\nInitSprites").as_ptr());
        R_InitColormaps();
        printf((c"\nInitColormaps").as_ptr());
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn R_FlatNumForName(mut name: *mut std::ffi::c_char) -> std::ffi::c_int {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut namet: [std::ffi::c_char; (9) as usize] = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        i = W_CheckNumForName(name);
        // TODO: if statement not yet translated:
        //
        //
        //     if (i == -1)
        //     {
        // 	namet[8] = 0;
        // 	memcpy (namet, name,8);
        // 	I_Error ("R_FlatNumForName: %s not found",namet);
        //     }
        todo!("if statement not yet translated");
        return (i - firstflat);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn R_CheckTextureNumForName(
    mut name: *mut std::ffi::c_char,
) -> std::ffi::c_int {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     // "NoTexture" marker.
        //     if (name[0] == '-')
        // 	return 0;
        todo!("if statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<numtextures ; i++)
        // 	if (!strncasecmp (textures[i]->name, name, 8) )
        // 	    return i;
        todo!("for statement not yet translated");
        return (-(1));
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn R_TextureNumForName(mut name: *mut std::ffi::c_char) -> std::ffi::c_int {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        i = R_CheckTextureNumForName(name);
        // TODO: if statement not yet translated:
        //
        //
        //     if (i==-1)
        //     {
        // 	I_Error ("R_TextureNumForName: %s not found",
        // 		 name);
        //     }
        todo!("if statement not yet translated");
        return i;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub static mut flatmemory: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut texturememory: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut spritememory: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn R_PrecacheLevel() {
    unsafe {
        let mut flatpresent: *mut std::ffi::c_char = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut texturepresent: *mut std::ffi::c_char = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut spritepresent: *mut std::ffi::c_char = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut j: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut k: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut lump: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut texture: *mut texture_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut th: *mut thinker_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut sf: *mut spriteframe_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (demoplayback)
        // 	return;
        todo!("if statement not yet translated");
        flatpresent = alloca(numflats);
        memset(flatpresent, 0, numflats);
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<numsectors ; i++)
        //     {
        // 	flatpresent[sectors[i].floorpic] = 1;
        // 	flatpresent[sectors[i].ceilingpic] = 1;
        //     }
        todo!("for statement not yet translated");
        flatmemory = 0;
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<numflats ; i++)
        //     {
        // 	if (flatpresent[i])
        // 	{
        // 	    lump = firstflat + i;
        // 	    flatmemory += lumpinfo[lump].size;
        // 	    W_CacheLumpNum(lump, PU_CACHE);
        // 	}
        //     }
        todo!("for statement not yet translated");
        texturepresent = alloca(numtextures);
        memset(texturepresent, 0, numtextures);
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<numsides ; i++)
        //     {
        // 	texturepresent[sides[i].toptexture] = 1;
        // 	texturepresent[sides[i].midtexture] = 1;
        // 	texturepresent[sides[i].bottomtexture] = 1;
        //     }
        todo!("for statement not yet translated");
        texturepresent[(skytexture) as usize] = 1;
        texturememory = 0;
        // TODO: for statement not yet translated:
        //
        //     for (i=0 ; i<numtextures ; i++)
        //     {
        // 	if (!texturepresent[i])
        // 	    continue;
        //
        // 	texture = textures[i];
        //
        // 	for (j=0 ; j<texture->patchcount ; j++)
        // 	{
        // 	    lump = texture->patches[j].patch;
        // 	    texturememory += lumpinfo[lump].size;
        // 	    W_CacheLumpNum(lump , PU_CACHE);
        // 	}
        //     }
        todo!("for statement not yet translated");
        spritepresent = alloca(numsprites);
        memset(spritepresent, 0, numsprites);
        // TODO: for statement not yet translated:
        //
        //
        //     for (th = thinkercap.next ; th != &thinkercap ; th=th->next)
        //     {
        // 	if (th->function.acp1 == (actionf_p1)P_MobjThinker)
        // 	    spritepresent[((mobj_t *)th)->sprite] = 1;
        //     }
        todo!("for statement not yet translated");
        spritememory = 0;
        // TODO: for statement not yet translated:
        //
        //     for (i=0 ; i<numsprites ; i++)
        //     {
        // 	if (!spritepresent[i])
        // 	    continue;
        //
        // 	for (j=0 ; j<sprites[i].numframes ; j++)
        // 	{
        // 	    sf = &sprites[i].spriteframes[j];
        // 	    for (k=0 ; k<8 ; k++)
        // 	    {
        // 		lump = firstspritelump + sf->lump[k];
        // 		spritememory += lumpinfo[lump].size;
        // 		W_CacheLumpNum(lump , PU_CACHE);
        // 	    }
        // 	}
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}
