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
use crate::hu_stuff::*;
use crate::i_system::*;
use crate::info::*;
use crate::m_fixed::*;
use crate::m_random::*;
use crate::p_local::*;
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
use crate::st_stuff::*;
use crate::tables::*;
use crate::z_zone::*;

pub const MF_SPECIAL: std::ffi::c_int = 1;
pub const MF_SOLID: std::ffi::c_int = 2;
pub const MF_SHOOTABLE: std::ffi::c_int = 4;
pub const MF_NOSECTOR: std::ffi::c_int = 8;
pub const MF_NOBLOCKMAP: std::ffi::c_int = 16;
pub const MF_AMBUSH: std::ffi::c_int = 32;
pub const MF_JUSTHIT: std::ffi::c_int = 64;
pub const MF_JUSTATTACKED: std::ffi::c_int = 128;
pub const MF_SPAWNCEILING: std::ffi::c_int = 256;
pub const MF_NOGRAVITY: std::ffi::c_int = 512;
pub const MF_DROPOFF: std::ffi::c_int = 0x400;
pub const MF_PICKUP: std::ffi::c_int = 0x800;
pub const MF_NOCLIP: std::ffi::c_int = 0x1000;
pub const MF_SLIDE: std::ffi::c_int = 0x2000;
pub const MF_FLOAT: std::ffi::c_int = 0x4000;
pub const MF_TELEPORT: std::ffi::c_int = 0x8000;
pub const MF_MISSILE: std::ffi::c_int = 0x10000;
pub const MF_DROPPED: std::ffi::c_int = 0x20000;
pub const MF_SHADOW: std::ffi::c_int = 0x40000;
pub const MF_NOBLOOD: std::ffi::c_int = 0x80000;
pub const MF_CORPSE: std::ffi::c_int = 0x100000;
pub const MF_INFLOAT: std::ffi::c_int = 0x200000;
pub const MF_COUNTKILL: std::ffi::c_int = 0x400000;
pub const MF_COUNTITEM: std::ffi::c_int = 0x800000;
pub const MF_SKULLFLY: std::ffi::c_int = 0x1000000;
pub const MF_NOTDMATCH: std::ffi::c_int = 0x2000000;
pub const MF_TRANSLATION: std::ffi::c_int = 0xc000000;
pub const MF_TRANSSHIFT: std::ffi::c_int = 26;

pub type mobjflag_t = std::ffi::c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mobj_t {
    pub thinker: thinker_t,
    pub x: fixed_t,
    pub y: fixed_t,
    pub z: fixed_t,
    pub snext: *mut mobj_s,
    pub sprev: *mut mobj_s,
    pub angle: angle_t,
    pub sprite: spritenum_t,
    pub frame: std::ffi::c_int,
    pub bnext: *mut mobj_s,
    pub bprev: *mut mobj_s,
    pub subsector: *mut subsector_s,
    pub floorz: fixed_t,
    pub ceilingz: fixed_t,
    pub radius: fixed_t,
    pub height: fixed_t,
    pub momx: fixed_t,
    pub momy: fixed_t,
    pub momz: fixed_t,
    pub validcount: std::ffi::c_int,
    pub type_: mobjtype_t,
    pub info: *mut mobjinfo_t,
    pub tics: std::ffi::c_int,
    pub state: *mut state_t,
    pub flags: std::ffi::c_int,
    pub health: std::ffi::c_int,
    pub movedir: std::ffi::c_int,
    pub movecount: std::ffi::c_int,
    pub target: *mut mobj_s,
    pub reactiontime: std::ffi::c_int,
    pub threshold: std::ffi::c_int,
    pub player: *mut player_s,
    pub lastlook: std::ffi::c_int,
    pub spawnpoint: mapthing_t,
    pub tracer: *mut mobj_s,
}

pub type mobj_s = mobj_t;

static mut rcsid: [std::ffi::c_char; 49] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        112 as std::ffi::c_char,
        95 as std::ffi::c_char,
        109 as std::ffi::c_char,
        111 as std::ffi::c_char,
        98 as std::ffi::c_char,
        106 as std::ffi::c_char,
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
    pub fn G_PlayerReborn(player: std::ffi::c_int);
}

pub static mut test: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn P_SetMobjState(mut mobj: *mut mobj_t, mut state: statenum_t) -> boolean {
    unsafe {
        let mut st: *mut state_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: do-while statement not yet translated:
        //
        //
        //     do
        //     {
        // 	if (state == S_NULL)
        // 	{
        // 	    mobj->state = (state_t *) S_NULL;
        // 	    P_RemoveMobj (mobj);
        // 	    return false;
        // 	}
        //
        // 	st = &states[state];
        // 	mobj->state = st;
        // 	mobj->tics = st->tics;
        // 	mobj->sprite = st->sprite;
        // 	mobj->frame = st->frame;
        //
        // 	// Modified handling.
        // 	// Call action functions when the state is set
        // 	if (st->action.acp1)
        // 	    st->action.acp1(mobj);
        //
        // 	state = st->nextstate;
        //     } while (!mobj->tics);
        todo!("do-while statement not yet translated");
        return true_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_ExplodeMissile(mut mo: *mut mobj_t) {
    unsafe {
        (*mo).momx = (*mo).momy = (*mo).momz = 0;
        P_SetMobjState(mo, mobjinfo[((*mo).type_) as usize].deathstate);
        (*mo).tics -= (P_Random() & 3);
        // TODO: if statement not yet translated:
        //
        //
        //     if (mo->tics < 1)
        // 	mo->tics = 1;
        todo!("if statement not yet translated");
        (*mo).flags &= (!(MF_MISSILE));
        // TODO: if statement not yet translated:
        //
        //
        //     if (mo->info->deathsound)
        // 	S_StartSound (mo, mo->info->deathsound);
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub const STOPSPEED: std::ffi::c_int = 0x1000;

pub const FRICTION: std::ffi::c_int = 0xe800;

pub unsafe extern "C" fn P_XYMovement(mut mo: *mut mobj_t) {
    unsafe {
        let mut ptryx: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut ptryy: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut player: *mut player_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut xmove: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut ymove: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (!mo->momx && !mo->momy)
        //     {
        // 	if (mo->flags & MF_SKULLFLY)
        // 	{
        // 	    // the skull slammed into something
        // 	    mo->flags &= ~MF_SKULLFLY;
        // 	    mo->momx = mo->momy = mo->momz = 0;
        //
        // 	    P_SetMobjState (mo, mo->info->spawnstate);
        // 	}
        // 	return;
        //     }
        todo!("if statement not yet translated");
        player = (*mo).player;
        // TODO: if statement not yet translated:
        //
        //
        //     if (mo->momx > MAXMOVE)
        // 	mo->momx = MAXMOVE;
        //     else if (mo->momx < -MAXMOVE)
        // 	mo->momx = -MAXMOVE;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (mo->momy > MAXMOVE)
        // 	mo->momy = MAXMOVE;
        //     else if (mo->momy < -MAXMOVE)
        // 	mo->momy = -MAXMOVE;
        todo!("if statement not yet translated");
        xmove = (*mo).momx;
        ymove = (*mo).momy;
        // TODO: do-while statement not yet translated:
        //
        //
        //     do
        //     {
        // 	if (xmove > MAXMOVE/2 || ymove > MAXMOVE/2)
        // 	{
        // 	    ptryx = mo->x + xmove/2;
        // 	    ptryy = mo->y + ymove/2;
        // 	    xmove >>= 1;
        // 	    ymove >>= 1;
        // 	}
        // 	else
        // 	{
        // 	    ptryx = mo->x + xmove;
        // 	    ptryy = mo->y + ymove;
        // 	    xmove = ymove = 0;
        // 	}
        //
        // 	if (!P_TryMove (mo, ptryx, ptryy))
        // 	{
        // 	    // blocked move
        // 	    if (mo->player)
        // 	    {	// try to slide along it
        // 		P_SlideMove (mo);
        // 	    }
        // 	    else if (mo->flags & MF_MISSILE)
        // 	    {
        // 		// explode a missile
        // 		if (ceilingline &&
        // 		    ceilingline->backsector &&
        // 		    ceilingline->backsector->ceilingpic == skyflatnum)
        // 		{
        // 		    // Hack to prevent missiles exploding
        // 		    // against the sky.
        // 		    // Does not handle sky floors.
        // 		    P_RemoveMobj (mo);
        // 		    return;
        // 		}
        // 		P_ExplodeMissile (mo);
        // 	    }
        // 	    else
        // 		mo->momx = mo->momy = 0;
        // 	}
        //     } while (xmove || ymove);
        todo!("do-while statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // slow down
        //     if (player && player->cheats & CF_NOMOMENTUM)
        //     {
        // 	// debug option for no sliding at all
        // 	mo->momx = mo->momy = 0;
        // 	return;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (mo->flags & (MF_MISSILE | MF_SKULLFLY) )
        // 	return; 	// no friction for missiles ever
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (mo->z > mo->floorz)
        // 	return;		// no friction when airborne
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (mo->flags & MF_CORPSE)
        //     {
        // 	// do not stop sliding
        // 	//  if halfway off a step with some momentum
        // 	if (mo->momx > FRACUNIT/4
        // 	    || mo->momx < -FRACUNIT/4
        // 	    || mo->momy > FRACUNIT/4
        // 	    || mo->momy < -FRACUNIT/4)
        // 	{
        // 	    if (mo->floorz != mo->subsector->sector->floorheight)
        // 		return;
        // 	}
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (mo->momx > -STOPSPEED
        // 	&& mo->momx < STOPSPEED
        // 	&& mo->momy > -STOPSPEED
        // 	&& mo->momy < STOPSPEED
        // 	&& (!player
        // 	    || (player->cmd.forwardmove== 0
        // 		&& player->cmd.sidemove == 0 ) ) )
        //     {
        // 	// if in a walking frame, stop moving
        // 	if ( player&&(unsigned)((player->mo->state - states)- S_PLAY_RUN1) < 4)
        // 	    P_SetMobjState (player->mo, S_PLAY);
        //
        // 	mo->momx = 0;
        // 	mo->momy = 0;
        //     }
        //     else
        //     {
        // 	mo->momx = FixedMul (mo->momx, FRICTION);
        // 	mo->momy = FixedMul (mo->momy, FRICTION);
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_ZMovement(mut mo: *mut mobj_t) {
    unsafe {
        let mut dist: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut delta: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     // check for smooth step up
        //     if (mo->player && mo->z < mo->floorz)
        //     {
        // 	mo->player->viewheight -= mo->floorz-mo->z;
        //
        // 	mo->player->deltaviewheight
        // 	    = (VIEWHEIGHT - mo->player->viewheight)>>3;
        //     }
        todo!("if statement not yet translated");
        (*mo).z += (*mo).momz;
        // TODO: if statement not yet translated:
        //
        //
        //     if ( mo->flags & MF_FLOAT
        // 	 && mo->target)
        //     {
        // 	// float down towards target if too close
        // 	if ( !(mo->flags & MF_SKULLFLY)
        // 	     && !(mo->flags & MF_INFLOAT) )
        // 	{
        // 	    dist = P_AproxDistance (mo->x - mo->target->x,
        // 				    mo->y - mo->target->y);
        //
        // 	    delta =(mo->target->z + (mo->height>>1)) - mo->z;
        //
        // 	    if (delta<0 && dist < -(delta*3) )
        // 		mo->z -= FLOATSPEED;
        // 	    else if (delta>0 && dist < (delta*3) )
        // 		mo->z += FLOATSPEED;
        // 	}
        //
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // clip movement
        //     if (mo->z <= mo->floorz)
        //     {
        // 	// hit the floor
        //
        // 	// Note (id):
        // 	//  somebody left this after the setting momz to 0,
        // 	//  kinda useless there.
        // 	if (mo->flags & MF_SKULLFLY)
        // 	{
        // 	    // the skull slammed into something
        // 	    mo->momz = -mo->momz;
        // 	}
        //
        // 	if (mo->momz < 0)
        // 	{
        // 	    if (mo->player
        // 		&& mo->momz < -GRAVITY*8)
        // 	    {
        // 		// Squat down.
        // 		// Decrease viewheight for a moment
        // 		// after hitting the ground (hard),
        // 		// and utter appropriate sound.
        // 		mo->player->deltaviewheight = mo->momz>>3;
        // 		S_StartSound (mo, sfx_oof);
        // 	    }
        // 	    mo->momz = 0;
        // 	}
        // 	mo->z = mo->floorz;
        //
        // 	if ( (mo->flags & MF_MISSILE)
        // 	     && !(mo->flags & MF_NOCLIP) )
        // 	{
        // 	    P_ExplodeMissile (mo);
        // 	    return;
        // 	}
        //     }
        //     else if (! (mo->flags & MF_NOGRAVITY) )
        //     {
        // 	if (mo->momz == 0)
        // 	    mo->momz = -GRAVITY*2;
        // 	else
        // 	    mo->momz -= GRAVITY;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (mo->z + mo->height > mo->ceilingz)
        //     {
        // 	// hit the ceiling
        // 	if (mo->momz > 0)
        // 	    mo->momz = 0;
        // 	{
        // 	    mo->z = mo->ceilingz - mo->height;
        // 	}
        //
        // 	if (mo->flags & MF_SKULLFLY)
        // 	{	// the skull slammed into something
        // 	    mo->momz = -mo->momz;
        // 	}
        //
        // 	if ( (mo->flags & MF_MISSILE)
        // 	     && !(mo->flags & MF_NOCLIP) )
        // 	{
        // 	    P_ExplodeMissile (mo);
        // 	    return;
        // 	}
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_NightmareRespawn(mut mobj: *mut mobj_t) {
    unsafe {
        let mut x: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut y: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut z: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut ss: *mut subsector_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut mo: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut mthing: *mut mapthing_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        x = ((*mobj).spawnpoint.x << FRACBITS);
        y = ((*mobj).spawnpoint.y << FRACBITS);
        // TODO: if statement not yet translated:
        //
        //
        //     // somthing is occupying it's position?
        //     if (!P_CheckPosition (mobj, x, y) )
        // 	return;	// no respwan
        todo!("if statement not yet translated");
        mo = P_SpawnMobj(
            (*mobj).x,
            (*mobj).y,
            (*(*(*mobj).subsector).sector).floorheight,
            MT_TFOG,
        );
        S_StartSound(mo, sfx_telept);
        ss = R_PointInSubsector(x, y);
        mo = P_SpawnMobj(x, y, (*(*ss).sector).floorheight, MT_TFOG);
        S_StartSound(mo, sfx_telept);
        mthing = (&((*mobj).spawnpoint) as *const _ as *mut _);
        // TODO: if statement not yet translated:
        //
        //
        //     // spawn it
        //     if (mobj->info->flags & MF_SPAWNCEILING)
        // 	z = ONCEILINGZ;
        //     else
        // 	z = ONFLOORZ;
        todo!("if statement not yet translated");
        mo = P_SpawnMobj(x, y, z, (*mobj).type_);
        (*mo).spawnpoint = (*mobj).spawnpoint;
        (*mo).angle = (ANG45 * ((*mthing).angle / 45));
        // TODO: if statement not yet translated:
        //
        //
        //     if (mthing->options & MTF_AMBUSH)
        // 	mo->flags |= MF_AMBUSH;
        todo!("if statement not yet translated");
        (*mo).reactiontime = 18;
        P_RemoveMobj(mobj);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_MobjThinker(mut mobj: *mut mobj_t) {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     // momentum movement
        //     if (mobj->momx
        // 	|| mobj->momy
        // 	|| (mobj->flags&MF_SKULLFLY) )
        //     {
        // 	P_XYMovement (mobj);
        //
        // 	// FIXME: decent NOP/NULL/Nil function pointer please.
        // 	if (mobj->thinker.function.acv == (actionf_v) (-1))
        // 	    return;		// mobj was removed
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if ( (mobj->z != mobj->floorz)
        // 	 || mobj->momz )
        //     {
        // 	P_ZMovement (mobj);
        //
        // 	// FIXME: decent NOP/NULL/Nil function pointer please.
        // 	if (mobj->thinker.function.acv == (actionf_v) (-1))
        // 	    return;		// mobj was removed
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //
        //     // cycle through states,
        //     // calling action functions at transitions
        //     if (mobj->tics != -1)
        //     {
        // 	mobj->tics--;
        //
        // 	// you can cycle through multiple states in a tic
        // 	if (!mobj->tics)
        // 	    if (!P_SetMobjState (mobj, mobj->state->nextstate) )
        // 		return;		// freed itself
        //     }
        //     else
        //     {
        // 	// check for nightmare respawn
        // 	if (! (mobj->flags & MF_COUNTKILL) )
        // 	    return;
        //
        // 	if (!respawnmonsters)
        // 	    return;
        //
        // 	mobj->movecount++;
        //
        // 	if (mobj->movecount < 12*35)
        // 	    return;
        //
        // 	if ( leveltime&31 )
        // 	    return;
        //
        // 	if (P_Random () > 4)
        // 	    return;
        //
        // 	P_NightmareRespawn (mobj);
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_SpawnMobj(
    mut x: fixed_t,
    mut y: fixed_t,
    mut z: fixed_t,
    mut type_: mobjtype_t,
) -> *mut mobj_t {
    unsafe {
        let mut mobj: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut st: *mut state_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut info: *mut mobjinfo_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        mobj = Z_Malloc(std::mem::size_of_val(&(*(mobj))), PU_LEVEL, NULL);
        memset(mobj, 0, std::mem::size_of_val(&(*(mobj))));
        info = (&(mobjinfo[(type_) as usize]) as *const _ as *mut _);
        (*mobj).type_ = type_;
        (*mobj).info = info;
        (*mobj).x = x;
        (*mobj).y = y;
        (*mobj).radius = (*info).radius;
        (*mobj).height = (*info).height;
        (*mobj).flags = (*info).flags;
        (*mobj).health = (*info).spawnhealth;
        // TODO: if statement not yet translated:
        //
        //
        //     if (gameskill != sk_nightmare)
        // 	mobj->reactiontime = info->reactiontime;
        todo!("if statement not yet translated");
        (*mobj).lastlook = (P_Random() % MAXPLAYERS);
        st = (&(states[((*info).spawnstate) as usize]) as *const _ as *mut _);
        (*mobj).state = st;
        (*mobj).tics = (*st).tics;
        (*mobj).sprite = (*st).sprite;
        (*mobj).frame = (*st).frame;
        P_SetThingPosition(mobj);
        (*mobj).floorz = (*(*(*mobj).subsector).sector).floorheight;
        (*mobj).ceilingz = (*(*(*mobj).subsector).sector).ceilingheight;
        // TODO: if statement not yet translated:
        //
        //
        //     if (z == ONFLOORZ)
        // 	mobj->z = mobj->floorz;
        //     else if (z == ONCEILINGZ)
        // 	mobj->z = mobj->ceilingz - mobj->info->height;
        //     else
        // 	mobj->z = z;
        todo!("if statement not yet translated");
        (*mobj).thinker.function.acp1 = ((P_MobjThinker) as actionf_p1);
        P_AddThinker((&((*mobj).thinker) as *const _ as *mut _));
        return mobj;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub static mut itemrespawnque: [mapthing_t; (ITEMQUESIZE) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut itemrespawntime: [std::ffi::c_int; (ITEMQUESIZE) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut iquehead: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut iquetail: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn P_RemoveMobj(mut mobj: *mut mobj_t) {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if ((mobj->flags & MF_SPECIAL)
        // 	&& !(mobj->flags & MF_DROPPED)
        // 	&& (mobj->type != MT_INV)
        // 	&& (mobj->type != MT_INS))
        //     {
        // 	itemrespawnque[iquehead] = mobj->spawnpoint;
        // 	itemrespawntime[iquehead] = leveltime;
        // 	iquehead = (iquehead+1)&(ITEMQUESIZE-1);
        //
        // 	// lose one off the end?
        // 	if (iquehead == iquetail)
        // 	    iquetail = (iquetail+1)&(ITEMQUESIZE-1);
        //     }
        todo!("if statement not yet translated");
        P_UnsetThingPosition(mobj);
        S_StopSound(mobj);
        P_RemoveThinker(((mobj) as *mut thinker_t));
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_RespawnSpecials() {
    unsafe {
        let mut x: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut y: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut z: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut ss: *mut subsector_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut mo: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut mthing: *mut mapthing_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     // only respawn items in deathmatch
        //     if (deathmatch != 2)
        // 	return;	//
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     // nothing left to respawn?
        //     if (iquehead == iquetail)
        // 	return;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // wait at least 30 seconds
        //     if (leveltime - itemrespawntime[iquetail] < 30*35)
        // 	return;
        todo!("if statement not yet translated");
        mthing = (&(itemrespawnque[(iquetail) as usize]) as *const _ as *mut _);
        x = ((*mthing).x << FRACBITS);
        y = ((*mthing).y << FRACBITS);
        ss = R_PointInSubsector(x, y);
        mo = P_SpawnMobj(x, y, (*(*ss).sector).floorheight, MT_IFOG);
        S_StartSound(mo, sfx_itmbk);
        // TODO: for statement not yet translated:
        //
        //
        //     // find which type to spawn
        //     for (i=0 ; i< NUMMOBJTYPES ; i++)
        //     {
        // 	if (mthing->type == mobjinfo[i].doomednum)
        // 	    break;
        //     }
        todo!("for statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // spawn it
        //     if (mobjinfo[i].flags & MF_SPAWNCEILING)
        // 	z = ONCEILINGZ;
        //     else
        // 	z = ONFLOORZ;
        todo!("if statement not yet translated");
        mo = P_SpawnMobj(x, y, z, i);
        (*mo).spawnpoint = (*(mthing));
        (*mo).angle = (ANG45 * ((*mthing).angle / 45));
        iquetail = ((iquetail + 1) & (ITEMQUESIZE - 1));
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_SpawnPlayer(mut mthing: *mut mapthing_t) {
    unsafe {
        let mut p: *mut player_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut x: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut y: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut z: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut mobj: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     // not playing?
        //     if (!playeringame[mthing->type-1])
        // 	return;
        todo!("if statement not yet translated");
        p = (&(players[((*mthing).type_ - 1) as usize]) as *const _ as *mut _);
        // TODO: if statement not yet translated:
        //
        //
        //     if (p->playerstate == PST_REBORN)
        // 	G_PlayerReborn (mthing->type-1);
        todo!("if statement not yet translated");
        x = ((*mthing).x << FRACBITS);
        y = ((*mthing).y << FRACBITS);
        z = ONFLOORZ;
        mobj = P_SpawnMobj(x, y, z, MT_PLAYER);
        // TODO: if statement not yet translated:
        //
        //
        //     // set color translations for player sprites
        //     if (mthing->type > 1)
        // 	mobj->flags |= (mthing->type-1)<<MF_TRANSSHIFT;
        todo!("if statement not yet translated");
        (*mobj).angle = (ANG45 * ((*mthing).angle / 45));
        (*mobj).player = p;
        (*mobj).health = (*p).health;
        (*p).mo = mobj;
        (*p).playerstate = PST_LIVE;
        (*p).refire = 0;
        (*p).message = NULL;
        (*p).damagecount = 0;
        (*p).bonuscount = 0;
        (*p).extralight = 0;
        (*p).fixedcolormap = 0;
        (*p).viewheight = VIEWHEIGHT;
        P_SetupPsprites(p);
        // TODO: if statement not yet translated:
        //
        //
        //     // give all cards in death match mode
        //     if (deathmatch)
        // 	for (i=0 ; i<NUMCARDS ; i++)
        // 	    p->cards[i] = true;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (mthing->type-1 == consoleplayer)
        //     {
        // 	// wake up the status bar
        // 	ST_Start ();
        // 	// wake up the heads up text
        // 	HU_Start ();
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_SpawnMapThing(mut mthing: *mut mapthing_t) {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut bit: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut mobj: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut x: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut y: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut z: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     // count deathmatch start positions
        //     if (mthing->type == 11)
        //     {
        // 	if (deathmatch_p < &deathmatchstarts[10])
        // 	{
        // 	    memcpy (deathmatch_p, mthing, sizeof(*mthing));
        // 	    deathmatch_p++;
        // 	}
        // 	return;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // check for players specially
        //     if (mthing->type <= 4)
        //     {
        // 	// save spots for respawning in network games
        // 	playerstarts[mthing->type-1] = *mthing;
        // 	if (!deathmatch)
        // 	    P_SpawnPlayer (mthing);
        //
        // 	return;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // check for apropriate skill level
        //     if (!netgame && (mthing->options & 16) )
        // 	return;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (gameskill == sk_baby)
        // 	bit = 1;
        //     else if (gameskill == sk_nightmare)
        // 	bit = 4;
        //     else
        // 	bit = 1<<(gameskill-1);
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (!(mthing->options & bit) )
        // 	return;
        todo!("if statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //
        //     // find which type to spawn
        //     for (i=0 ; i< NUMMOBJTYPES ; i++)
        // 	if (mthing->type == mobjinfo[i].doomednum)
        // 	    break;
        todo!("for statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (i==NUMMOBJTYPES)
        // 	I_Error ("P_SpawnMapThing: Unknown type %i at (%i, %i)",
        // 		 mthing->type,
        // 		 mthing->x, mthing->y);
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // don't spawn keycards and players in deathmatch
        //     if (deathmatch && mobjinfo[i].flags & MF_NOTDMATCH)
        // 	return;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // don't spawn any monsters if -nomonsters
        //     if (nomonsters
        // 	&& ( i == MT_SKULL
        // 	     || (mobjinfo[i].flags & MF_COUNTKILL)) )
        //     {
        // 	return;
        //     }
        todo!("if statement not yet translated");
        x = ((*mthing).x << FRACBITS);
        y = ((*mthing).y << FRACBITS);
        // TODO: if statement not yet translated:
        //
        //
        //     if (mobjinfo[i].flags & MF_SPAWNCEILING)
        // 	z = ONCEILINGZ;
        //     else
        // 	z = ONFLOORZ;
        todo!("if statement not yet translated");
        mobj = P_SpawnMobj(x, y, z, i);
        (*mobj).spawnpoint = (*(mthing));
        // TODO: if statement not yet translated:
        //
        //
        //     if (mobj->tics > 0)
        // 	mobj->tics = 1 + (P_Random () % mobj->tics);
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (mobj->flags & MF_COUNTKILL)
        // 	totalkills++;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (mobj->flags & MF_COUNTITEM)
        // 	totalitems++;
        todo!("if statement not yet translated");
        (*mobj).angle = (ANG45 * ((*mthing).angle / 45));
        // TODO: if statement not yet translated:
        //
        //     if (mthing->options & MTF_AMBUSH)
        // 	mobj->flags |= MF_AMBUSH;
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

unsafe extern "C" {
    pub static mut attackrange: fixed_t;
}

pub unsafe extern "C" fn P_SpawnPuff(mut x: fixed_t, mut y: fixed_t, mut z: fixed_t) {
    unsafe {
        let mut th: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        z += ((P_Random() - P_Random()) << 10);
        th = P_SpawnMobj(x, y, z, MT_PUFF);
        (*th).momz = FRACUNIT;
        (*th).tics -= (P_Random() & 3);
        // TODO: if statement not yet translated:
        //
        //
        //     if (th->tics < 1)
        // 	th->tics = 1;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // don't make punches spark on the wall
        //     if (attackrange == MELEERANGE)
        // 	P_SetMobjState (th, S_PUFF3);
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_SpawnBlood(
    mut x: fixed_t,
    mut y: fixed_t,
    mut z: fixed_t,
    mut damage: std::ffi::c_int,
) {
    unsafe {
        let mut th: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        z += ((P_Random() - P_Random()) << 10);
        th = P_SpawnMobj(x, y, z, MT_BLOOD);
        (*th).momz = (FRACUNIT * 2);
        (*th).tics -= (P_Random() & 3);
        // TODO: if statement not yet translated:
        //
        //
        //     if (th->tics < 1)
        // 	th->tics = 1;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (damage <= 12 && damage >= 9)
        // 	P_SetMobjState (th,S_BLOOD2);
        //     else if (damage < 9)
        // 	P_SetMobjState (th,S_BLOOD3);
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_CheckMissileSpawn(mut th: *mut mobj_t) {
    unsafe {
        (*th).tics -= (P_Random() & 3);
        // TODO: if statement not yet translated:
        //
        //     if (th->tics < 1)
        // 	th->tics = 1;
        todo!("if statement not yet translated");
        (*th).x += ((*th).momx >> 1);
        (*th).y += ((*th).momy >> 1);
        (*th).z += ((*th).momz >> 1);
        // TODO: if statement not yet translated:
        //
        //
        //     if (!P_TryMove (th, th->x, th->y))
        // 	P_ExplodeMissile (th);
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_SpawnMissile(
    mut source: *mut mobj_t,
    mut dest: *mut mobj_t,
    mut type_: mobjtype_t,
) -> *mut mobj_t {
    unsafe {
        let mut th: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut an: angle_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut dist: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        th = P_SpawnMobj(
            (*source).x,
            (*source).y,
            ((*source).z + ((4 * 8) * FRACUNIT)),
            type_,
        );
        // TODO: if statement not yet translated:
        //
        //
        //     if (th->info->seesound)
        // 	S_StartSound (th, th->info->seesound);
        todo!("if statement not yet translated");
        (*th).target = source;
        an = R_PointToAngle2((*source).x, (*source).y, (*dest).x, (*dest).y);
        // TODO: if statement not yet translated:
        //
        //
        //     // fuzzy player
        //     if (dest->flags & MF_SHADOW)
        // 	an += (P_Random()-P_Random())<<20;
        todo!("if statement not yet translated");
        (*th).angle = an;
        an >>= ANGLETOFINESHIFT;
        (*th).momx = FixedMul((*(*th).info).speed, finecosine[(an) as usize]);
        (*th).momy = FixedMul((*(*th).info).speed, finesine[(an) as usize]);
        dist = P_AproxDistance(((*dest).x - (*source).x), ((*dest).y - (*source).y));
        dist = (dist / (*(*th).info).speed);
        // TODO: if statement not yet translated:
        //
        //
        //     if (dist < 1)
        // 	dist = 1;
        todo!("if statement not yet translated");
        (*th).momz = (((*dest).z - (*source).z) / dist);
        P_CheckMissileSpawn(th);
        return th;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_SpawnPlayerMissile(mut source: *mut mobj_t, mut type_: mobjtype_t) {
    unsafe {
        let mut th: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut an: angle_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut x: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut y: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut z: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut slope: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        an = (*source).angle;
        slope = P_AimLineAttack(source, an, ((16 * 64) * FRACUNIT));
        // TODO: if statement not yet translated:
        //
        //
        //     if (!linetarget)
        //     {
        // 	an += 1<<26;
        // 	slope = P_AimLineAttack (source, an, 16*64*FRACUNIT);
        //
        // 	if (!linetarget)
        // 	{
        // 	    an -= 2<<26;
        // 	    slope = P_AimLineAttack (source, an, 16*64*FRACUNIT);
        // 	}
        //
        // 	if (!linetarget)
        // 	{
        // 	    an = source->angle;
        // 	    slope = 0;
        // 	}
        //     }
        todo!("if statement not yet translated");
        x = (*source).x;
        y = (*source).y;
        z = ((*source).z + ((4 * 8) * FRACUNIT));
        th = P_SpawnMobj(x, y, z, type_);
        // TODO: if statement not yet translated:
        //
        //
        //     if (th->info->seesound)
        // 	S_StartSound (th, th->info->seesound);
        todo!("if statement not yet translated");
        (*th).target = source;
        (*th).angle = an;
        (*th).momx = FixedMul(
            (*(*th).info).speed,
            finecosine[(an >> ANGLETOFINESHIFT) as usize],
        );
        (*th).momy = FixedMul(
            (*(*th).info).speed,
            finesine[(an >> ANGLETOFINESHIFT) as usize],
        );
        (*th).momz = FixedMul((*(*th).info).speed, slope);
        P_CheckMissileSpawn(th);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}
