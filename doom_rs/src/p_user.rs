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

static mut rcsid: [std::ffi::c_char; 49] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        112 as std::ffi::c_char,
        95 as std::ffi::c_char,
        117 as std::ffi::c_char,
        115 as std::ffi::c_char,
        101 as std::ffi::c_char,
        114 as std::ffi::c_char,
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

pub const INVERSECOLORMAP: std::ffi::c_int = 32;

pub const MAXBOB: std::ffi::c_int = 0x100000;

pub static mut onground: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn P_Thrust(
    mut player: *mut player_t,
    mut angle: angle_t,
    mut move_: fixed_t,
) {
    unsafe {
        angle >>= ANGLETOFINESHIFT;
        (*(*player).mo).momx += FixedMul(move_, finecosine[(angle) as usize]);
        (*(*player).mo).momy += FixedMul(move_, finesine[(angle) as usize]);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_CalcHeight(mut player: *mut player_t) {
    unsafe {
        let mut angle: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut bob: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        (*player).bob = (FixedMul((*(*player).mo).momx, (*(*player).mo).momx)
            + FixedMul((*(*player).mo).momy, (*(*player).mo).momy));
        (*player).bob >>= 2;
        // TODO: if statement not yet translated:
        //
        //
        //     if (player->bob>MAXBOB)
        // 	player->bob = MAXBOB;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if ((player->cheats & CF_NOMOMENTUM) || !onground)
        //     {
        // 	player->viewz = player->mo->z + VIEWHEIGHT;
        //
        // 	if (player->viewz > player->mo->ceilingz-4*FRACUNIT)
        // 	    player->viewz = player->mo->ceilingz-4*FRACUNIT;
        //
        // 	player->viewz = player->mo->z + player->viewheight;
        // 	return;
        //     }
        todo!("if statement not yet translated");
        angle = (((FINEANGLES / 20) * leveltime) & FINEMASK);
        bob = FixedMul(((*player).bob / 2), finesine[(angle) as usize]);
        // TODO: if statement not yet translated:
        //
        //
        //
        //     // move viewheight
        //     if (player->playerstate == PST_LIVE)
        //     {
        // 	player->viewheight += player->deltaviewheight;
        //
        // 	if (player->viewheight > VIEWHEIGHT)
        // 	{
        // 	    player->viewheight = VIEWHEIGHT;
        // 	    player->deltaviewheight = 0;
        // 	}
        //
        // 	if (player->viewheight < VIEWHEIGHT/2)
        // 	{
        // 	    player->viewheight = VIEWHEIGHT/2;
        // 	    if (player->deltaviewheight <= 0)
        // 		player->deltaviewheight = 1;
        // 	}
        //
        // 	if (player->deltaviewheight)
        // 	{
        // 	    player->deltaviewheight += FRACUNIT/4;
        // 	    if (!player->deltaviewheight)
        // 		player->deltaviewheight = 1;
        // 	}
        //     }
        todo!("if statement not yet translated");
        (*player).viewz = (((*(*player).mo).z + (*player).viewheight) + bob);
        // TODO: if statement not yet translated:
        //
        //
        //     if (player->viewz > player->mo->ceilingz-4*FRACUNIT)
        // 	player->viewz = player->mo->ceilingz-4*FRACUNIT;
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_MovePlayer(mut player: *mut player_t) {
    unsafe {
        let mut cmd: *mut ticcmd_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        cmd = (&((*player).cmd) as *const _ as *mut _);
        (*(*player).mo).angle += ((*cmd).angleturn << 16);
        onground = ((*(*player).mo).z <= (*(*player).mo).floorz);
        // TODO: if statement not yet translated:
        //
        //
        //     if (cmd->forwardmove && onground)
        // 	P_Thrust (player, player->mo->angle, cmd->forwardmove*2048);
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (cmd->sidemove && onground)
        // 	P_Thrust (player, player->mo->angle-ANG90, cmd->sidemove*2048);
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if ( (cmd->forwardmove || cmd->sidemove)
        // 	 && player->mo->state == &states[S_PLAY] )
        //     {
        // 	P_SetMobjState (player->mo, S_PLAY_RUN1);
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub const ANG5: std::ffi::c_int = (ANG90 / 18);

pub unsafe extern "C" fn P_DeathThink(mut player: *mut player_t) {
    unsafe {
        let mut angle: angle_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut delta: angle_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        P_MovePsprites(player);
        // TODO: if statement not yet translated:
        //
        //
        //     // fall to the ground
        //     if (player->viewheight > 6*FRACUNIT)
        // 	player->viewheight -= FRACUNIT;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (player->viewheight < 6*FRACUNIT)
        // 	player->viewheight = 6*FRACUNIT;
        todo!("if statement not yet translated");
        (*player).deltaviewheight = 0;
        onground = ((*(*player).mo).z <= (*(*player).mo).floorz);
        P_CalcHeight(player);
        // TODO: if statement not yet translated:
        //
        //
        //     if (player->attacker && player->attacker != player->mo)
        //     {
        // 	angle = R_PointToAngle2 (player->mo->x,
        // 				 player->mo->y,
        // 				 player->attacker->x,
        // 				 player->attacker->y);
        //
        // 	delta = angle - player->mo->angle;
        //
        // 	if (delta < ANG5 || delta > (unsigned)-ANG5)
        // 	{
        // 	    // Looking at killer,
        // 	    //  so fade damage flash down.
        // 	    player->mo->angle = angle;
        //
        // 	    if (player->damagecount)
        // 		player->damagecount--;
        // 	}
        // 	else if (delta < ANG180)
        // 	    player->mo->angle += ANG5;
        // 	else
        // 	    player->mo->angle -= ANG5;
        //     }
        //     else if (player->damagecount)
        // 	player->damagecount--;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //
        //     if (player->cmd.buttons & BT_USE)
        // 	player->playerstate = PST_REBORN;
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_PlayerThink(mut player: *mut player_t) {
    unsafe {
        let mut cmd: *mut ticcmd_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut newweapon: weapontype_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     // fixme: do this in the cheat code
        //     if (player->cheats & CF_NOCLIP)
        // 	player->mo->flags |= MF_NOCLIP;
        //     else
        // 	player->mo->flags &= ~MF_NOCLIP;
        todo!("if statement not yet translated");
        cmd = (&((*player).cmd) as *const _ as *mut _);
        // TODO: if statement not yet translated:
        //
        //     if (player->mo->flags & MF_JUSTATTACKED)
        //     {
        // 	cmd->angleturn = 0;
        // 	cmd->forwardmove = 0xc800/512;
        // 	cmd->sidemove = 0;
        // 	player->mo->flags &= ~MF_JUSTATTACKED;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //
        //     if (player->playerstate == PST_DEAD)
        //     {
        // 	P_DeathThink (player);
        // 	return;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // Move around.
        //     // Reactiontime is used to prevent movement
        //     //  for a bit after a teleport.
        //     if (player->mo->reactiontime)
        // 	player->mo->reactiontime--;
        //     else
        // 	P_MovePlayer (player);
        todo!("if statement not yet translated");
        P_CalcHeight(player);
        // TODO: if statement not yet translated:
        //
        //
        //     if (player->mo->subsector->sector->special)
        // 	P_PlayerInSpecialSector (player);
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // Check for weapon change.
        //
        //     // A special event has no other buttons.
        //     if (cmd->buttons & BT_SPECIAL)
        // 	cmd->buttons = 0;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (cmd->buttons & BT_CHANGE)
        //     {
        // 	// The actual changing of the weapon is done
        // 	//  when the weapon psprite can do it
        // 	//  (read: not in the middle of an attack).
        // 	newweapon = (cmd->buttons&BT_WEAPONMASK)>>BT_WEAPONSHIFT;
        //
        // 	if (newweapon == wp_fist
        // 	    && player->weaponowned[wp_chainsaw]
        // 	    && !(player->readyweapon == wp_chainsaw
        // 		 && player->powers[pw_strength]))
        // 	{
        // 	    newweapon = wp_chainsaw;
        // 	}
        //
        // 	if ( (gamemode == commercial)
        // 	    && newweapon == wp_shotgun
        // 	    && player->weaponowned[wp_supershotgun]
        // 	    && player->readyweapon != wp_supershotgun)
        // 	{
        // 	    newweapon = wp_supershotgun;
        // 	}
        //
        //
        // 	if (player->weaponowned[newweapon]
        // 	    && newweapon != player->readyweapon)
        // 	{
        // 	    // Do not go to plasma or BFG in shareware,
        // 	    //  even if cheated.
        // 	    if ((newweapon != wp_plasma
        // 		 && newweapon != wp_bfg)
        // 		|| (gamemode != shareware) )
        // 	    {
        // 		player->pendingweapon = newweapon;
        // 	    }
        // 	}
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // check for use
        //     if (cmd->buttons & BT_USE)
        //     {
        // 	if (!player->usedown)
        // 	{
        // 	    P_UseLines (player);
        // 	    player->usedown = true;
        // 	}
        //     }
        //     else
        // 	player->usedown = false;
        todo!("if statement not yet translated");
        P_MovePsprites(player);
        // TODO: if statement not yet translated:
        //
        //
        //     // Counters, time dependend power ups.
        //
        //     // Strength counts up to diminish fade.
        //     if (player->powers[pw_strength])
        // 	player->powers[pw_strength]++;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (player->powers[pw_invulnerability])
        // 	player->powers[pw_invulnerability]--;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (player->powers[pw_invisibility])
        // 	if (! --player->powers[pw_invisibility] )
        // 	    player->mo->flags &= ~MF_SHADOW;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (player->powers[pw_infrared])
        // 	player->powers[pw_infrared]--;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (player->powers[pw_ironfeet])
        // 	player->powers[pw_ironfeet]--;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (player->damagecount)
        // 	player->damagecount--;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (player->bonuscount)
        // 	player->bonuscount--;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //
        //     // Handling colormaps.
        //     if (player->powers[pw_invulnerability])
        //     {
        // 	if (player->powers[pw_invulnerability] > 4*32
        // 	    || (player->powers[pw_invulnerability]&8) )
        // 	    player->fixedcolormap = INVERSECOLORMAP;
        // 	else
        // 	    player->fixedcolormap = 0;
        //     }
        //     else if (player->powers[pw_infrared])
        //     {
        // 	if (player->powers[pw_infrared] > 4*32
        // 	    || (player->powers[pw_infrared]&8) )
        // 	{
        // 	    // almost full bright
        // 	    player->fixedcolormap = 1;
        // 	}
        // 	else
        // 	    player->fixedcolormap = 0;
        //     }
        //     else
        // 	player->fixedcolormap = 0;
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}
