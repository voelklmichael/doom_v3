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
use crate::m_random::*;
use crate::p_local::*;
use crate::p_mobj::*;
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

pub const FF_FULLBRIGHT: std::ffi::c_int = 0x8000;

pub const FF_FRAMEMASK: std::ffi::c_int = 0x7fff;

pub const ps_weapon: std::ffi::c_int = 0;
pub const ps_flash: std::ffi::c_int = ps_weapon + 1;
pub const NUMPSPRITES: std::ffi::c_int = ps_flash + 1;

pub type psprnum_t = std::ffi::c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pspdef_t {
    pub state: *mut state_t,
    pub tics: std::ffi::c_int,
    pub sx: fixed_t,
    pub sy: fixed_t,
}

static mut rcsid: [std::ffi::c_char; 49] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        112 as std::ffi::c_char,
        95 as std::ffi::c_char,
        112 as std::ffi::c_char,
        115 as std::ffi::c_char,
        112 as std::ffi::c_char,
        114 as std::ffi::c_char,
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

pub const LOWERSPEED: std::ffi::c_int = (FRACUNIT * 6);

pub const RAISESPEED: std::ffi::c_int = (FRACUNIT * 6);

pub const WEAPONBOTTOM: std::ffi::c_int = (128 * FRACUNIT);

pub const WEAPONTOP: std::ffi::c_int = (32 * FRACUNIT);

pub const BFGCELLS: std::ffi::c_int = 40;

pub unsafe extern "C" fn P_SetPsprite(
    mut player: *mut player_t,
    mut position: std::ffi::c_int,
    mut stnum: statenum_t,
) {
    unsafe {
        let mut psp: *mut pspdef_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut state: *mut state_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        psp = (&((*player).psprites[(position) as usize]) as *const _ as *mut _);
        // TODO: do-while statement not yet translated:
        //
        //
        //     do
        //     {
        // 	if (!stnum)
        // 	{
        // 	    // object removed itself
        // 	    psp->state = NULL;
        // 	    break;
        // 	}
        //
        // 	state = &states[stnum];
        // 	psp->state = state;
        // 	psp->tics = state->tics;	// could be 0
        //
        // 	if (state->misc1)
        // 	{
        // 	    // coordinate set
        // 	    psp->sx = state->misc1 << FRACBITS;
        // 	    psp->sy = state->misc2 << FRACBITS;
        // 	}
        //
        // 	// Call action routine.
        // 	// Modified handling.
        // 	if (state->action.acp2)
        // 	{
        // 	    state->action.acp2(player, psp);
        // 	    if (!psp->state)
        // 		break;
        // 	}
        //
        // 	stnum = psp->state->nextstate;
        //
        //     } while (!psp->tics);
        todo!("do-while statement not yet translated");
        // TODO: statement not yet translated:
        //
        //     // an initial state of 0 could cycle through
        todo!("statement not yet translated");
    }
}

pub static mut swingx: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut swingy: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn P_CalcSwing(mut player: *mut player_t) {
    unsafe {
        let mut swing: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut angle: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        swing = (*player).bob;
        angle = (((FINEANGLES / 70) * leveltime) & FINEMASK);
        swingx = FixedMul(swing, finesine[(angle) as usize]);
        angle = ((((FINEANGLES / 70) * leveltime) + (FINEANGLES / 2)) & FINEMASK);
        swingy = (-(FixedMul(swingx, finesine[(angle) as usize])));
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_BringUpWeapon(mut player: *mut player_t) {
    unsafe {
        let mut newstate: statenum_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (player->pendingweapon == wp_nochange)
        // 	player->pendingweapon = player->readyweapon;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (player->pendingweapon == wp_chainsaw)
        // 	S_StartSound (player->mo, sfx_sawup);
        todo!("if statement not yet translated");
        newstate = weaponinfo[((*player).pendingweapon) as usize].upstate;
        (*player).pendingweapon = wp_nochange;
        (*player).psprites[(ps_weapon) as usize].sy = WEAPONBOTTOM;
        P_SetPsprite(player, ps_weapon, newstate);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_CheckAmmo(mut player: *mut player_t) -> boolean {
    unsafe {
        let mut ammo: ammotype_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut count: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        ammo = weaponinfo[((*player).readyweapon) as usize].ammo;
        // TODO: if statement not yet translated:
        //
        //
        //     // Minimal amount for one shot varies.
        //     if (player->readyweapon == wp_bfg)
        // 	count = BFGCELLS;
        //     else if (player->readyweapon == wp_supershotgun)
        // 	count = 2;	// Double barrel.
        //     else
        // 	count = 1;	// Regular.
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     // Some do not need ammunition anyway.
        //     // Return if current ammunition sufficient.
        //     if (ammo == am_noammo || player->ammo[ammo] >= count)
        // 	return true;
        todo!("if statement not yet translated");
        // TODO: do-while statement not yet translated:
        //
        //
        //     // Out of ammo, pick a weapon to change to.
        //     // Preferences are set here.
        //     do
        //     {
        // 	if (player->weaponowned[wp_plasma]
        // 	    && player->ammo[am_cell]
        // 	    && (gamemode != shareware) )
        // 	{
        // 	    player->pendingweapon = wp_plasma;
        // 	}
        // 	else if (player->weaponowned[wp_supershotgun]
        // 		 && player->ammo[am_shell]>2
        // 		 && (gamemode == commercial) )
        // 	{
        // 	    player->pendingweapon = wp_supershotgun;
        // 	}
        // 	else if (player->weaponowned[wp_chaingun]
        // 		 && player->ammo[am_clip])
        // 	{
        // 	    player->pendingweapon = wp_chaingun;
        // 	}
        // 	else if (player->weaponowned[wp_shotgun]
        // 		 && player->ammo[am_shell])
        // 	{
        // 	    player->pendingweapon = wp_shotgun;
        // 	}
        // 	else if (player->ammo[am_clip])
        // 	{
        // 	    player->pendingweapon = wp_pistol;
        // 	}
        // 	else if (player->weaponowned[wp_chainsaw])
        // 	{
        // 	    player->pendingweapon = wp_chainsaw;
        // 	}
        // 	else if (player->weaponowned[wp_missile]
        // 		 && player->ammo[am_misl])
        // 	{
        // 	    player->pendingweapon = wp_missile;
        // 	}
        // 	else if (player->weaponowned[wp_bfg]
        // 		 && player->ammo[am_cell]>40
        // 		 && (gamemode != shareware) )
        // 	{
        // 	    player->pendingweapon = wp_bfg;
        // 	}
        // 	else
        // 	{
        // 	    // If everything fails.
        // 	    player->pendingweapon = wp_fist;
        // 	}
        //
        //     } while (player->pendingweapon == wp_nochange);
        todo!("do-while statement not yet translated");
        P_SetPsprite(
            player,
            ps_weapon,
            weaponinfo[((*player).readyweapon) as usize].downstate,
        );
        return false_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_FireWeapon(mut player: *mut player_t) {
    unsafe {
        let mut newstate: statenum_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (!P_CheckAmmo (player))
        // 	return;
        todo!("if statement not yet translated");
        P_SetMobjState((*player).mo, S_PLAY_ATK1);
        newstate = weaponinfo[((*player).readyweapon) as usize].atkstate;
        P_SetPsprite(player, ps_weapon, newstate);
        P_NoiseAlert((*player).mo, (*player).mo);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_DropWeapon(mut player: *mut player_t) {
    unsafe {
        P_SetPsprite(
            player,
            ps_weapon,
            weaponinfo[((*player).readyweapon) as usize].downstate,
        );
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_WeaponReady(mut player: *mut player_t, mut psp: *mut pspdef_t) {
    unsafe {
        let mut newstate: statenum_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut angle: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     // get out of attack state
        //     if (player->mo->state == &states[S_PLAY_ATK1]
        // 	|| player->mo->state == &states[S_PLAY_ATK2] )
        //     {
        // 	P_SetMobjState (player->mo, S_PLAY);
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (player->readyweapon == wp_chainsaw
        // 	&& psp->state == &states[S_SAW])
        //     {
        // 	S_StartSound (player->mo, sfx_sawidl);
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // check for change
        //     //  if player is dead, put the weapon away
        //     if (player->pendingweapon != wp_nochange || !player->health)
        //     {
        // 	// change weapon
        // 	//  (pending weapon should allready be validated)
        // 	newstate = weaponinfo[player->readyweapon].downstate;
        // 	P_SetPsprite (player, ps_weapon, newstate);
        // 	return;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // check for fire
        //     //  the missile launcher and bfg do not auto fire
        //     if (player->cmd.buttons & BT_ATTACK)
        //     {
        // 	if ( !player->attackdown
        // 	     || (player->readyweapon != wp_missile
        // 		 && player->readyweapon != wp_bfg) )
        // 	{
        // 	    player->attackdown = true;
        // 	    P_FireWeapon (player);
        // 	    return;
        // 	}
        //     }
        //     else
        // 	player->attackdown = false;
        todo!("if statement not yet translated");
        angle = ((128 * leveltime) & FINEMASK);
        (*psp).sx = (FRACUNIT + FixedMul((*player).bob, finecosine[(angle) as usize]));
        angle &= ((FINEANGLES / 2) - 1);
        (*psp).sy = (WEAPONTOP + FixedMul((*player).bob, finesine[(angle) as usize]));
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_ReFire(mut player: *mut player_t, mut psp: *mut pspdef_t) {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //
        //     // check for fire
        //     //  (if a weaponchange is pending, let it go through instead)
        //     if ( (player->cmd.buttons & BT_ATTACK)
        // 	 && player->pendingweapon == wp_nochange
        // 	 && player->health)
        //     {
        // 	player->refire++;
        // 	P_FireWeapon (player);
        //     }
        //     else
        //     {
        // 	player->refire = 0;
        // 	P_CheckAmmo (player);
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_CheckReload(mut player: *mut player_t, mut psp: *mut pspdef_t) {
    unsafe {
        P_CheckAmmo(player);
    }
}

pub unsafe extern "C" fn A_Lower(mut player: *mut player_t, mut psp: *mut pspdef_t) {
    unsafe {
        (*psp).sy += LOWERSPEED;
        // TODO: if statement not yet translated:
        //
        //
        //     // Is already down.
        //     if (psp->sy < WEAPONBOTTOM )
        // 	return;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // Player is dead.
        //     if (player->playerstate == PST_DEAD)
        //     {
        // 	psp->sy = WEAPONBOTTOM;
        //
        // 	// don't bring weapon back up
        // 	return;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // The old weapon has been lowered off the screen,
        //     // so change the weapon and start raising it
        //     if (!player->health)
        //     {
        // 	// Player is dead, so keep the weapon off screen.
        // 	P_SetPsprite (player,  ps_weapon, S_NULL);
        // 	return;
        //     }
        todo!("if statement not yet translated");
        (*player).readyweapon = (*player).pendingweapon;
        P_BringUpWeapon(player);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_Raise(mut player: *mut player_t, mut psp: *mut pspdef_t) {
    unsafe {
        let mut newstate: statenum_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        (*psp).sy -= RAISESPEED;
        // TODO: if statement not yet translated:
        //
        //
        //     if (psp->sy > WEAPONTOP )
        // 	return;
        todo!("if statement not yet translated");
        (*psp).sy = WEAPONTOP;
        newstate = weaponinfo[((*player).readyweapon) as usize].readystate;
        P_SetPsprite(player, ps_weapon, newstate);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_GunFlash(mut player: *mut player_t, mut psp: *mut pspdef_t) {
    unsafe {
        P_SetMobjState((*player).mo, S_PLAY_ATK2);
        P_SetPsprite(
            player,
            ps_flash,
            weaponinfo[((*player).readyweapon) as usize].flashstate,
        );
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_Punch(mut player: *mut player_t, mut psp: *mut pspdef_t) {
    unsafe {
        let mut angle: angle_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut damage: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut slope: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        damage = (((P_Random() % 10) + 1) << 1);
        // TODO: if statement not yet translated:
        //
        //
        //     if (player->powers[pw_strength])
        // 	damage *= 10;
        todo!("if statement not yet translated");
        angle = (*(*player).mo).angle;
        angle += ((P_Random() - P_Random()) << 18);
        slope = P_AimLineAttack((*player).mo, angle, MELEERANGE);
        P_LineAttack((*player).mo, angle, MELEERANGE, slope, damage);
        // TODO: if statement not yet translated:
        //
        //
        //     // turn to face target
        //     if (linetarget)
        //     {
        // 	S_StartSound (player->mo, sfx_punch);
        // 	player->mo->angle = R_PointToAngle2 (player->mo->x,
        // 					     player->mo->y,
        // 					     linetarget->x,
        // 					     linetarget->y);
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_Saw(mut player: *mut player_t, mut psp: *mut pspdef_t) {
    unsafe {
        let mut angle: angle_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut damage: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut slope: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        damage = (2 * ((P_Random() % 10) + 1));
        angle = (*(*player).mo).angle;
        angle += ((P_Random() - P_Random()) << 18);
        slope = P_AimLineAttack((*player).mo, angle, (MELEERANGE + 1));
        P_LineAttack((*player).mo, angle, (MELEERANGE + 1), slope, damage);
        // TODO: if statement not yet translated:
        //
        //
        //     if (!linetarget)
        //     {
        // 	S_StartSound (player->mo, sfx_sawful);
        // 	return;
        //     }
        todo!("if statement not yet translated");
        S_StartSound((*player).mo, sfx_sawhit);
        angle = R_PointToAngle2(
            (*(*player).mo).x,
            (*(*player).mo).y,
            (*linetarget).x,
            (*linetarget).y,
        );
        // TODO: if statement not yet translated:
        //
        //     if (angle - player->mo->angle > ANG180)
        //     {
        // 	if (angle - player->mo->angle < -ANG90/20)
        // 	    player->mo->angle = angle + ANG90/21;
        // 	else
        // 	    player->mo->angle -= ANG90/20;
        //     }
        //     else
        //     {
        // 	if (angle - player->mo->angle > ANG90/20)
        // 	    player->mo->angle = angle - ANG90/21;
        // 	else
        // 	    player->mo->angle += ANG90/20;
        //     }
        todo!("if statement not yet translated");
        (*(*player).mo).flags |= MF_JUSTATTACKED;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_FireMissile(mut player: *mut player_t, mut psp: *mut pspdef_t) {
    unsafe {
        {
            let __macro_tmp =
                (*player).ammo[(weaponinfo[((*player).readyweapon) as usize].ammo) as usize];
            (*player).ammo[(weaponinfo[((*player).readyweapon) as usize].ammo) as usize] -= 1;
            __macro_tmp
        };
        P_SpawnPlayerMissile((*player).mo, MT_ROCKET);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_FireBFG(mut player: *mut player_t, mut psp: *mut pspdef_t) {
    unsafe {
        (*player).ammo[(weaponinfo[((*player).readyweapon) as usize].ammo) as usize] -= BFGCELLS;
        P_SpawnPlayerMissile((*player).mo, MT_BFG);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_FirePlasma(mut player: *mut player_t, mut psp: *mut pspdef_t) {
    unsafe {
        {
            let __macro_tmp =
                (*player).ammo[(weaponinfo[((*player).readyweapon) as usize].ammo) as usize];
            (*player).ammo[(weaponinfo[((*player).readyweapon) as usize].ammo) as usize] -= 1;
            __macro_tmp
        };
        P_SetPsprite(
            player,
            ps_flash,
            (weaponinfo[((*player).readyweapon) as usize].flashstate + (P_Random() & 1)),
        );
        P_SpawnPlayerMissile((*player).mo, MT_PLASMA);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub static mut bulletslope: fixed_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn P_BulletSlope(mut mo: *mut mobj_t) {
    unsafe {
        let mut an: angle_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        an = (*mo).angle;
        bulletslope = P_AimLineAttack(mo, an, ((16 * 64) * FRACUNIT));
        // TODO: if statement not yet translated:
        //
        //
        //     if (!linetarget)
        //     {
        // 	an += 1<<26;
        // 	bulletslope = P_AimLineAttack (mo, an, 16*64*FRACUNIT);
        // 	if (!linetarget)
        // 	{
        // 	    an -= 2<<26;
        // 	    bulletslope = P_AimLineAttack (mo, an, 16*64*FRACUNIT);
        // 	}
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_GunShot(mut mo: *mut mobj_t, mut accurate: boolean) {
    unsafe {
        let mut angle: angle_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut damage: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        damage = (5 * ((P_Random() % 3) + 1));
        angle = (*mo).angle;
        // TODO: if statement not yet translated:
        //
        //
        //     if (!accurate)
        // 	angle += (P_Random()-P_Random())<<18;
        todo!("if statement not yet translated");
        P_LineAttack(mo, angle, MISSILERANGE, bulletslope, damage);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_FirePistol(mut player: *mut player_t, mut psp: *mut pspdef_t) {
    unsafe {
        S_StartSound((*player).mo, sfx_pistol);
        P_SetMobjState((*player).mo, S_PLAY_ATK2);
        {
            let __macro_tmp =
                (*player).ammo[(weaponinfo[((*player).readyweapon) as usize].ammo) as usize];
            (*player).ammo[(weaponinfo[((*player).readyweapon) as usize].ammo) as usize] -= 1;
            __macro_tmp
        };
        P_SetPsprite(
            player,
            ps_flash,
            weaponinfo[((*player).readyweapon) as usize].flashstate,
        );
        P_BulletSlope((*player).mo);
        P_GunShot((*player).mo, ((((*player).refire) == 0) as std::ffi::c_int));
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_FireShotgun(mut player: *mut player_t, mut psp: *mut pspdef_t) {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        S_StartSound((*player).mo, sfx_shotgn);
        P_SetMobjState((*player).mo, S_PLAY_ATK2);
        {
            let __macro_tmp =
                (*player).ammo[(weaponinfo[((*player).readyweapon) as usize].ammo) as usize];
            (*player).ammo[(weaponinfo[((*player).readyweapon) as usize].ammo) as usize] -= 1;
            __macro_tmp
        };
        P_SetPsprite(
            player,
            ps_flash,
            weaponinfo[((*player).readyweapon) as usize].flashstate,
        );
        P_BulletSlope((*player).mo);
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<7 ; i++)
        // 	P_GunShot (player->mo, false);
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_FireShotgun2(mut player: *mut player_t, mut psp: *mut pspdef_t) {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut angle: angle_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut damage: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        S_StartSound((*player).mo, sfx_dshtgn);
        P_SetMobjState((*player).mo, S_PLAY_ATK2);
        (*player).ammo[(weaponinfo[((*player).readyweapon) as usize].ammo) as usize] -= 2;
        P_SetPsprite(
            player,
            ps_flash,
            weaponinfo[((*player).readyweapon) as usize].flashstate,
        );
        P_BulletSlope((*player).mo);
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<20 ; i++)
        //     {
        // 	damage = 5*(P_Random ()%3+1);
        // 	angle = player->mo->angle;
        // 	angle += (P_Random()-P_Random())<<19;
        // 	P_LineAttack (player->mo,
        // 		      angle,
        // 		      MISSILERANGE,
        // 		      bulletslope + ((P_Random()-P_Random())<<5), damage);
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_FireCGun(mut player: *mut player_t, mut psp: *mut pspdef_t) {
    unsafe {
        S_StartSound((*player).mo, sfx_pistol);
        // TODO: if statement not yet translated:
        //
        //
        //     if (!player->ammo[weaponinfo[player->readyweapon].ammo])
        // 	return;
        todo!("if statement not yet translated");
        P_SetMobjState((*player).mo, S_PLAY_ATK2);
        {
            let __macro_tmp =
                (*player).ammo[(weaponinfo[((*player).readyweapon) as usize].ammo) as usize];
            (*player).ammo[(weaponinfo[((*player).readyweapon) as usize].ammo) as usize] -= 1;
            __macro_tmp
        };
        P_SetPsprite(
            player,
            ps_flash,
            ((weaponinfo[((*player).readyweapon) as usize].flashstate + (*psp).state)
                - (&(states[(S_CHAIN1) as usize]) as *const _ as *mut _)),
        );
        P_BulletSlope((*player).mo);
        P_GunShot((*player).mo, ((((*player).refire) == 0) as std::ffi::c_int));
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_Light0(mut player: *mut player_t, mut psp: *mut pspdef_t) {
    unsafe {
        (*player).extralight = 0;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_Light1(mut player: *mut player_t, mut psp: *mut pspdef_t) {
    unsafe {
        (*player).extralight = 1;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_Light2(mut player: *mut player_t, mut psp: *mut pspdef_t) {
    unsafe {
        (*player).extralight = 2;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_BFGSpray(mut mo: *mut mobj_t) {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut j: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut damage: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut an: angle_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: for statement not yet translated:
        //
        //
        //     // offset angles from its attack angle
        //     for (i=0 ; i<40 ; i++)
        //     {
        // 	an = mo->angle - ANG90/2 + ANG90/40*i;
        //
        // 	// mo->target is the originator (player)
        // 	//  of the missile
        // 	P_AimLineAttack (mo->target, an, 16*64*FRACUNIT);
        //
        // 	if (!linetarget)
        // 	    continue;
        //
        // 	P_SpawnMobj (linetarget->x,
        // 		     linetarget->y,
        // 		     linetarget->z + (linetarget->height>>2),
        // 		     MT_EXTRABFG);
        //
        // 	damage = 0;
        // 	for (j=0;j<15;j++)
        // 	    damage += (P_Random()&7) + 1;
        //
        // 	P_DamageMobj (linetarget, mo->target,mo->target, damage);
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn A_BFGsound(mut player: *mut player_t, mut psp: *mut pspdef_t) {
    unsafe {
        S_StartSound((*player).mo, sfx_bfg);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_SetupPsprites(mut player: *mut player_t) {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: for statement not yet translated:
        //
        //
        //     // remove all psprites
        //     for (i=0 ; i<NUMPSPRITES ; i++)
        // 	player->psprites[i].state = NULL;
        todo!("for statement not yet translated");
        (*player).pendingweapon = (*player).readyweapon;
        P_BringUpWeapon(player);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_MovePsprites(mut player: *mut player_t) {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut psp: *mut pspdef_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut state: *mut state_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        psp = (&((*player).psprites[(0) as usize]) as *const _ as *mut _);
        // TODO: for statement not yet translated:
        //
        //     for (i=0 ; i<NUMPSPRITES ; i++, psp++)
        //     {
        // 	// a null state means not active
        // 	if ( (state = psp->state) )
        // 	{
        // 	    // drop tic count and possibly change state
        //
        // 	    // a -1 tic count never changes
        // 	    if (psp->tics != -1)
        // 	    {
        // 		psp->tics--;
        // 		if (!psp->tics)
        // 		    P_SetPsprite (player, i, psp->state->nextstate);
        // 	    }
        // 	}
        //     }
        todo!("for statement not yet translated");
        (*player).psprites[(ps_flash) as usize].sx = (*player).psprites[(ps_weapon) as usize].sx;
        (*player).psprites[(ps_flash) as usize].sy = (*player).psprites[(ps_weapon) as usize].sy;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}
