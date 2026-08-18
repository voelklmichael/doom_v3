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
use crate::m_bbox::*;
use crate::m_fixed::*;
use crate::m_swap::*;
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
use crate::tables::*;
use crate::w_wad::*;
use crate::z_zone::*;

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
        101 as std::ffi::c_char,
        116 as std::ffi::c_char,
        117 as std::ffi::c_char,
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

unsafe extern "C" {
    pub fn P_SpawnMapThing(mthing: *mut mapthing_t);
}

pub static mut numvertexes: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut vertexes: *mut vertex_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut numsegs: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut segs: *mut seg_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut numsectors: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut sectors: *mut sector_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut numsubsectors: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut subsectors: *mut subsector_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut numnodes: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut nodes: *mut node_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut numlines: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut lines: *mut line_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut numsides: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut sides: *mut side_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut bmapwidth: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut bmapheight: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut blockmap: *mut std::ffi::c_short = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut blockmaplump: *mut std::ffi::c_short = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut bmaporgx: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut bmaporgy: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut blocklinks: *mut *mut mobj_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut rejectmatrix: *mut byte = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub const MAX_DEATHMATCH_STARTS: std::ffi::c_int = 10;

pub static mut deathmatchstarts: [mapthing_t; (MAX_DEATHMATCH_STARTS) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut deathmatch_p: *mut mapthing_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut playerstarts: [mapthing_t; (MAXPLAYERS) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn P_LoadVertexes(mut lump: std::ffi::c_int) {
    unsafe {
        let mut data: *mut byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut ml: *mut mapvertex_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut li: *mut vertex_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        numvertexes = (W_LumpLength(lump) / std::mem::size_of::<mapvertex_t>());
        vertexes = Z_Malloc((numvertexes * std::mem::size_of::<vertex_t>()), PU_LEVEL, 0);
        data = W_CacheLumpNum(lump, PU_STATIC);
        ml = ((data) as *mut mapvertex_t);
        li = vertexes;
        // TODO: for statement not yet translated:
        //
        //
        //     // Copy and convert vertex coordinates,
        //     // internal representation as fixed.
        //     for (i=0 ; i<numvertexes ; i++, li++, ml++)
        //     {
        // 	li->x = SHORT(ml->x)<<FRACBITS;
        // 	li->y = SHORT(ml->y)<<FRACBITS;
        //     }
        todo!("for statement not yet translated");
        Z_Free(data);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_LoadSegs(mut lump: std::ffi::c_int) {
    unsafe {
        let mut data: *mut byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut ml: *mut mapseg_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut li: *mut seg_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut ldef: *mut line_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut linedef: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut side: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        numsegs = (W_LumpLength(lump) / std::mem::size_of::<mapseg_t>());
        segs = Z_Malloc((numsegs * std::mem::size_of::<seg_t>()), PU_LEVEL, 0);
        memset(segs, 0, (numsegs * std::mem::size_of::<seg_t>()));
        data = W_CacheLumpNum(lump, PU_STATIC);
        ml = ((data) as *mut mapseg_t);
        li = segs;
        // TODO: for statement not yet translated:
        //
        //     for (i=0 ; i<numsegs ; i++, li++, ml++)
        //     {
        // 	li->v1 = &vertexes[SHORT(ml->v1)];
        // 	li->v2 = &vertexes[SHORT(ml->v2)];
        //
        // 	li->angle = (SHORT(ml->angle))<<16;
        // 	li->offset = (SHORT(ml->offset))<<16;
        // 	linedef = SHORT(ml->linedef);
        // 	ldef = &lines[linedef];
        // 	li->linedef = ldef;
        // 	side = SHORT(ml->side);
        // 	li->sidedef = &sides[ldef->sidenum[side]];
        // 	li->frontsector = sides[ldef->sidenum[side]].sector;
        // 	if (ldef-> flags & ML_TWOSIDED)
        // 	    li->backsector = sides[ldef->sidenum[side^1]].sector;
        // 	else
        // 	    li->backsector = 0;
        //     }
        todo!("for statement not yet translated");
        Z_Free(data);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_LoadSubsectors(mut lump: std::ffi::c_int) {
    unsafe {
        let mut data: *mut byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut ms: *mut mapsubsector_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut ss: *mut subsector_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        numsubsectors = (W_LumpLength(lump) / std::mem::size_of::<mapsubsector_t>());
        subsectors = Z_Malloc(
            (numsubsectors * std::mem::size_of::<subsector_t>()),
            PU_LEVEL,
            0,
        );
        data = W_CacheLumpNum(lump, PU_STATIC);
        ms = ((data) as *mut mapsubsector_t);
        memset(
            subsectors,
            0,
            (numsubsectors * std::mem::size_of::<subsector_t>()),
        );
        ss = subsectors;
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<numsubsectors ; i++, ss++, ms++)
        //     {
        // 	ss->numlines = SHORT(ms->numsegs);
        // 	ss->firstline = SHORT(ms->firstseg);
        //     }
        todo!("for statement not yet translated");
        Z_Free(data);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_LoadSectors(mut lump: std::ffi::c_int) {
    unsafe {
        let mut data: *mut byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut ms: *mut mapsector_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut ss: *mut sector_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        numsectors = (W_LumpLength(lump) / std::mem::size_of::<mapsector_t>());
        sectors = Z_Malloc((numsectors * std::mem::size_of::<sector_t>()), PU_LEVEL, 0);
        memset(sectors, 0, (numsectors * std::mem::size_of::<sector_t>()));
        data = W_CacheLumpNum(lump, PU_STATIC);
        ms = ((data) as *mut mapsector_t);
        ss = sectors;
        // TODO: for statement not yet translated:
        //
        //     for (i=0 ; i<numsectors ; i++, ss++, ms++)
        //     {
        // 	ss->floorheight = SHORT(ms->floorheight)<<FRACBITS;
        // 	ss->ceilingheight = SHORT(ms->ceilingheight)<<FRACBITS;
        // 	ss->floorpic = R_FlatNumForName(ms->floorpic);
        // 	ss->ceilingpic = R_FlatNumForName(ms->ceilingpic);
        // 	ss->lightlevel = SHORT(ms->lightlevel);
        // 	ss->special = SHORT(ms->special);
        // 	ss->tag = SHORT(ms->tag);
        // 	ss->thinglist = NULL;
        //     }
        todo!("for statement not yet translated");
        Z_Free(data);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_LoadNodes(mut lump: std::ffi::c_int) {
    unsafe {
        let mut data: *mut byte = unsafe {
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
        let mut mn: *mut mapnode_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut no: *mut node_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        numnodes = (W_LumpLength(lump) / std::mem::size_of::<mapnode_t>());
        nodes = Z_Malloc((numnodes * std::mem::size_of::<node_t>()), PU_LEVEL, 0);
        data = W_CacheLumpNum(lump, PU_STATIC);
        mn = ((data) as *mut mapnode_t);
        no = nodes;
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<numnodes ; i++, no++, mn++)
        //     {
        // 	no->x = SHORT(mn->x)<<FRACBITS;
        // 	no->y = SHORT(mn->y)<<FRACBITS;
        // 	no->dx = SHORT(mn->dx)<<FRACBITS;
        // 	no->dy = SHORT(mn->dy)<<FRACBITS;
        // 	for (j=0 ; j<2 ; j++)
        // 	{
        // 	    no->children[j] = SHORT(mn->children[j]);
        // 	    for (k=0 ; k<4 ; k++)
        // 		no->bbox[j][k] = SHORT(mn->bbox[j][k])<<FRACBITS;
        // 	}
        //     }
        todo!("for statement not yet translated");
        Z_Free(data);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_LoadThings(mut lump: std::ffi::c_int) {
    unsafe {
        let mut data: *mut byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut mt: *mut mapthing_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut numthings: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut spawn: boolean = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        data = W_CacheLumpNum(lump, PU_STATIC);
        numthings = (W_LumpLength(lump) / std::mem::size_of::<mapthing_t>());
        mt = ((data) as *mut mapthing_t);
        // TODO: for statement not yet translated:
        //
        //     for (i=0 ; i<numthings ; i++, mt++)
        //     {
        // 	spawn = true;
        //
        // 	// Do not spawn cool, new monsters if !commercial
        // 	if ( gamemode != commercial)
        // 	{
        // 	    switch(mt->type)
        // 	    {
        // 	      case 68:	// Arachnotron
        // 	      case 64:	// Archvile
        // 	      case 88:	// Boss Brain
        // 	      case 89:	// Boss Shooter
        // 	      case 69:	// Hell Knight
        // 	      case 67:	// Mancubus
        // 	      case 71:	// Pain Elemental
        // 	      case 65:	// Former Human Commando
        // 	      case 66:	// Revenant
        // 	      case 84:	// Wolf SS
        // 		spawn = false;
        // 		break;
        // 	    }
        // 	}
        // 	if (spawn == false)
        // 	    break;
        //
        // 	// Do spawn all other stuff.
        // 	mt->x = SHORT(mt->x);
        // 	mt->y = SHORT(mt->y);
        // 	mt->angle = SHORT(mt->angle);
        // 	mt->type = SHORT(mt->type);
        // 	mt->options = SHORT(mt->options);
        //
        // 	P_SpawnMapThing (mt);
        //     }
        todo!("for statement not yet translated");
        Z_Free(data);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_LoadLineDefs(mut lump: std::ffi::c_int) {
    unsafe {
        let mut data: *mut byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut mld: *mut maplinedef_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut ld: *mut line_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut v1: *mut vertex_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut v2: *mut vertex_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        numlines = (W_LumpLength(lump) / std::mem::size_of::<maplinedef_t>());
        lines = Z_Malloc((numlines * std::mem::size_of::<line_t>()), PU_LEVEL, 0);
        memset(lines, 0, (numlines * std::mem::size_of::<line_t>()));
        data = W_CacheLumpNum(lump, PU_STATIC);
        mld = ((data) as *mut maplinedef_t);
        ld = lines;
        // TODO: for statement not yet translated:
        //
        //     for (i=0 ; i<numlines ; i++, mld++, ld++)
        //     {
        // 	ld->flags = SHORT(mld->flags);
        // 	ld->special = SHORT(mld->special);
        // 	ld->tag = SHORT(mld->tag);
        // 	v1 = ld->v1 = &vertexes[SHORT(mld->v1)];
        // 	v2 = ld->v2 = &vertexes[SHORT(mld->v2)];
        // 	ld->dx = v2->x - v1->x;
        // 	ld->dy = v2->y - v1->y;
        //
        // 	if (!ld->dx)
        // 	    ld->slopetype = ST_VERTICAL;
        // 	else if (!ld->dy)
        // 	    ld->slopetype = ST_HORIZONTAL;
        // 	else
        // 	{
        // 	    if (FixedDiv (ld->dy , ld->dx) > 0)
        // 		ld->slopetype = ST_POSITIVE;
        // 	    else
        // 		ld->slopetype = ST_NEGATIVE;
        // 	}
        //
        // 	if (v1->x < v2->x)
        // 	{
        // 	    ld->bbox[BOXLEFT] = v1->x;
        // 	    ld->bbox[BOXRIGHT] = v2->x;
        // 	}
        // 	else
        // 	{
        // 	    ld->bbox[BOXLEFT] = v2->x;
        // 	    ld->bbox[BOXRIGHT] = v1->x;
        // 	}
        //
        // 	if (v1->y < v2->y)
        // 	{
        // 	    ld->bbox[BOXBOTTOM] = v1->y;
        // 	    ld->bbox[BOXTOP] = v2->y;
        // 	}
        // 	else
        // 	{
        // 	    ld->bbox[BOXBOTTOM] = v2->y;
        // 	    ld->bbox[BOXTOP] = v1->y;
        // 	}
        //
        // 	ld->sidenum[0] = SHORT(mld->sidenum[0]);
        // 	ld->sidenum[1] = SHORT(mld->sidenum[1]);
        //
        // 	if (ld->sidenum[0] != -1)
        // 	    ld->frontsector = sides[ld->sidenum[0]].sector;
        // 	else
        // 	    ld->frontsector = 0;
        //
        // 	if (ld->sidenum[1] != -1)
        // 	    ld->backsector = sides[ld->sidenum[1]].sector;
        // 	else
        // 	    ld->backsector = 0;
        //     }
        todo!("for statement not yet translated");
        Z_Free(data);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_LoadSideDefs(mut lump: std::ffi::c_int) {
    unsafe {
        let mut data: *mut byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut msd: *mut mapsidedef_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut sd: *mut side_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        numsides = (W_LumpLength(lump) / std::mem::size_of::<mapsidedef_t>());
        sides = Z_Malloc((numsides * std::mem::size_of::<side_t>()), PU_LEVEL, 0);
        memset(sides, 0, (numsides * std::mem::size_of::<side_t>()));
        data = W_CacheLumpNum(lump, PU_STATIC);
        msd = ((data) as *mut mapsidedef_t);
        sd = sides;
        // TODO: for statement not yet translated:
        //
        //     for (i=0 ; i<numsides ; i++, msd++, sd++)
        //     {
        // 	sd->textureoffset = SHORT(msd->textureoffset)<<FRACBITS;
        // 	sd->rowoffset = SHORT(msd->rowoffset)<<FRACBITS;
        // 	sd->toptexture = R_TextureNumForName(msd->toptexture);
        // 	sd->bottomtexture = R_TextureNumForName(msd->bottomtexture);
        // 	sd->midtexture = R_TextureNumForName(msd->midtexture);
        // 	sd->sector = &sectors[SHORT(msd->sector)];
        //     }
        todo!("for statement not yet translated");
        Z_Free(data);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_LoadBlockMap(mut lump: std::ffi::c_int) {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut count: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        blockmaplump = W_CacheLumpNum(lump, PU_LEVEL);
        blockmap = (blockmaplump + 4);
        count = (W_LumpLength(lump) / 2);
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<count ; i++)
        // 	blockmaplump[i] = SHORT(blockmaplump[i]);
        todo!("for statement not yet translated");
        bmaporgx = (blockmaplump[(0) as usize] << FRACBITS);
        bmaporgy = (blockmaplump[(1) as usize] << FRACBITS);
        bmapwidth = blockmaplump[(2) as usize];
        bmapheight = blockmaplump[(3) as usize];
        count = ((std::mem::size_of_val(&(*(blocklinks))) * bmapwidth) * bmapheight);
        blocklinks = Z_Malloc(count, PU_LEVEL, 0);
        memset(blocklinks, 0, count);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_GroupLines() {
    unsafe {
        let mut linebuffer: *mut *mut line_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut j: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut total: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut li: *mut line_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut sector: *mut sector_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut ss: *mut subsector_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut seg: *mut seg_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut bbox: [fixed_t; (4) as usize] = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut block: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        ss = subsectors;
        // TODO: for statement not yet translated:
        //
        //     for (i=0 ; i<numsubsectors ; i++, ss++)
        //     {
        // 	seg = &segs[ss->firstline];
        // 	ss->sector = seg->sidedef->sector;
        //     }
        todo!("for statement not yet translated");
        li = lines;
        total = 0;
        // TODO: for statement not yet translated:
        //
        //     for (i=0 ; i<numlines ; i++, li++)
        //     {
        // 	total++;
        // 	li->frontsector->linecount++;
        //
        // 	if (li->backsector && li->backsector != li->frontsector)
        // 	{
        // 	    li->backsector->linecount++;
        // 	    total++;
        // 	}
        //     }
        todo!("for statement not yet translated");
        linebuffer = Z_Malloc((total * 4), PU_LEVEL, 0);
        sector = sectors;
        // TODO: for statement not yet translated:
        //
        //     for (i=0 ; i<numsectors ; i++, sector++)
        //     {
        // 	M_ClearBox (bbox);
        // 	sector->lines = linebuffer;
        // 	li = lines;
        // 	for (j=0 ; j<numlines ; j++, li++)
        // 	{
        // 	    if (li->frontsector == sector || li->backsector == sector)
        // 	    {
        // 		*linebuffer++ = li;
        // 		M_AddToBox (bbox, li->v1->x, li->v1->y);
        // 		M_AddToBox (bbox, li->v2->x, li->v2->y);
        // 	    }
        // 	}
        // 	if (linebuffer - sector->lines != sector->linecount)
        // 	    I_Error ("P_GroupLines: miscounted");
        //
        // 	// set the degenmobj_t to the middle of the bounding box
        // 	sector->soundorg.x = (bbox[BOXRIGHT]+bbox[BOXLEFT])/2;
        // 	sector->soundorg.y = (bbox[BOXTOP]+bbox[BOXBOTTOM])/2;
        //
        // 	// adjust bounding box to map blocks
        // 	block = (bbox[BOXTOP]-bmaporgy+MAXRADIUS)>>MAPBLOCKSHIFT;
        // 	block = block >= bmapheight ? bmapheight-1 : block;
        // 	sector->blockbox[BOXTOP]=block;
        //
        // 	block = (bbox[BOXBOTTOM]-bmaporgy-MAXRADIUS)>>MAPBLOCKSHIFT;
        // 	block = block < 0 ? 0 : block;
        // 	sector->blockbox[BOXBOTTOM]=block;
        //
        // 	block = (bbox[BOXRIGHT]-bmaporgx+MAXRADIUS)>>MAPBLOCKSHIFT;
        // 	block = block >= bmapwidth ? bmapwidth-1 : block;
        // 	sector->blockbox[BOXRIGHT]=block;
        //
        // 	block = (bbox[BOXLEFT]-bmaporgx-MAXRADIUS)>>MAPBLOCKSHIFT;
        // 	block = block < 0 ? 0 : block;
        // 	sector->blockbox[BOXLEFT]=block;
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_SetupLevel(
    mut episode: std::ffi::c_int,
    mut map: std::ffi::c_int,
    mut playermask: std::ffi::c_int,
    mut skill: skill_t,
) {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut lumpname: [std::ffi::c_char; (9) as usize] = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut lumpnum: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        totalkills = totalitems = totalsecret = wminfo.maxfrags = 0;
        wminfo.partime = 180;
        // TODO: for statement not yet translated:
        //
        //     for (i=0 ; i<MAXPLAYERS ; i++)
        //     {
        // 	players[i].killcount = players[i].secretcount
        // 	    = players[i].itemcount = 0;
        //     }
        todo!("for statement not yet translated");
        players[(consoleplayer) as usize].viewz = 1;
        S_Start();
        // C preprocessor directive at statement position (not executable, nothing lost):
        //
        //
        //
        // #if 0 // UNUSED
        // TODO: if statement not yet translated:
        //     if (debugfile)
        //     {
        // 	Z_FreeTags (PU_LEVEL, MAXINT);
        // 	Z_FileDumpHeap (debugfile);
        //     }
        //     else
        // #endif
        todo!("if statement not yet translated");
        Z_FreeTags(PU_LEVEL, (PU_PURGELEVEL - 1));
        P_InitThinkers();
        W_Reload();
        // TODO: if statement not yet translated:
        //
        //
        //     // find map name
        //     if ( gamemode == commercial)
        //     {
        // 	if (map<10)
        // 	    sprintf (lumpname,"map0%i", map);
        // 	else
        // 	    sprintf (lumpname,"map%i", map);
        //     }
        //     else
        //     {
        // 	lumpname[0] = 'E';
        // 	lumpname[1] = '0' + episode;
        // 	lumpname[2] = 'M';
        // 	lumpname[3] = '0' + map;
        // 	lumpname[4] = 0;
        //     }
        todo!("if statement not yet translated");
        lumpnum = W_GetNumForName(lumpname);
        leveltime = 0;
        P_LoadBlockMap((lumpnum + ML_BLOCKMAP));
        P_LoadVertexes((lumpnum + ML_VERTEXES));
        P_LoadSectors((lumpnum + ML_SECTORS));
        P_LoadSideDefs((lumpnum + ML_SIDEDEFS));
        P_LoadLineDefs((lumpnum + ML_LINEDEFS));
        P_LoadSubsectors((lumpnum + ML_SSECTORS));
        P_LoadNodes((lumpnum + ML_NODES));
        P_LoadSegs((lumpnum + ML_SEGS));
        rejectmatrix = W_CacheLumpNum((lumpnum + ML_REJECT), PU_LEVEL);
        P_GroupLines();
        bodyqueslot = 0;
        deathmatch_p = deathmatchstarts;
        P_LoadThings((lumpnum + ML_THINGS));
        // TODO: if statement not yet translated:
        //
        //
        //     // if deathmatch, randomly spawn the active players
        //     if (deathmatch)
        //     {
        // 	for (i=0 ; i<MAXPLAYERS ; i++)
        // 	    if (playeringame[i])
        // 	    {
        // 		players[i].mo = NULL;
        // 		G_DeathMatchSpawnPlayer (i);
        // 	    }
        //
        //     }
        todo!("if statement not yet translated");
        iquehead = iquetail = 0;
        P_SpawnSpecials();
        // TODO: if statement not yet translated:
        //
        //
        //     // build subsector connect matrix
        //     //	UNUSED P_ConnectSubsectors ();
        //
        //     // preload graphics
        //     if (precache)
        // 	R_PrecacheLevel ();
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        //     //printf ("free memory: 0x%x\n", Z_FreeMemory());
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_Init() {
    unsafe {
        P_InitSwitchList();
        P_InitPicAnims();
        R_InitSprites(sprnames);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}
