use crate::d_items::*;
use crate::d_player::*;
use crate::d_think::*;
use crate::d_ticcmd::*;
use crate::doomdata::*;
use crate::doomdef::*;
use crate::doomtype::*;
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
use crate::s_sound::*;
use crate::sounds::*;
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
        116 as std::ffi::c_char,
        101 as std::ffi::c_char,
        108 as std::ffi::c_char,
        101 as std::ffi::c_char,
        112 as std::ffi::c_char,
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

pub unsafe extern "C" fn EV_Teleport(
    mut line: *mut line_t,
    mut side: std::ffi::c_int,
    mut thing: *mut mobj_t,
) -> std::ffi::c_int {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut tag: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut m: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut fog: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut an: std::ffi::c_uint = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut thinker: *mut thinker_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut sector: *mut sector_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut oldx: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut oldy: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut oldz: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     // don't teleport missiles
        //     if (thing->flags & MF_MISSILE)
        // 	return 0;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // Don't teleport if hit back of line,
        //     //  so you can get out of teleporter.
        //     if (side == 1)
        // 	return 0;
        todo!("if statement not yet translated");
        tag = (*line).tag;
        // TODO: for statement not yet translated:
        //
        //     for (i = 0; i < numsectors; i++)
        //     {
        // 	if (sectors[ i ].tag == tag )
        // 	{
        // 	    thinker = thinkercap.next;
        // 	    for (thinker = thinkercap.next;
        // 		 thinker != &thinkercap;
        // 		 thinker = thinker->next)
        // 	    {
        // 		// not a mobj
        // 		if (thinker->function.acp1 != (actionf_p1)P_MobjThinker)
        // 		    continue;
        //
        // 		m = (mobj_t *)thinker;
        //
        // 		// not a teleportman
        // 		if (m->type != MT_TELEPORTMAN )
        // 		    continue;
        //
        // 		sector = m->subsector->sector;
        // 		// wrong sector
        // 		if (sector-sectors != i )
        // 		    continue;
        //
        // 		oldx = thing->x;
        // 		oldy = thing->y;
        // 		oldz = thing->z;
        //
        // 		if (!P_TeleportMove (thing, m->x, m->y))
        // 		    return 0;
        //
        // 		thing->z = thing->floorz;  //fixme: not needed?
        // 		if (thing->player)
        // 		    thing->player->viewz = thing->z+thing->player->viewheight;
        //
        // 		// spawn teleport fog at source and destination
        // 		fog = P_SpawnMobj (oldx, oldy, oldz, MT_TFOG);
        // 		S_StartSound (fog, sfx_telept);
        // 		an = m->angle >> ANGLETOFINESHIFT;
        // 		fog = P_SpawnMobj (m->x+20*finecosine[an], m->y+20*finesine[an]
        // 				   , thing->z, MT_TFOG);
        //
        // 		// emit sound, where?
        // 		S_StartSound (fog, sfx_telept);
        //
        // 		// don't move for a bit
        // 		if (thing->player)
        // 		    thing->reactiontime = 18;
        //
        // 		thing->angle = m->angle;
        // 		thing->momx = thing->momy = thing->momz = 0;
        // 		return 1;
        // 	    }
        // 	}
        //     }
        todo!("for statement not yet translated");
        return 0;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}
