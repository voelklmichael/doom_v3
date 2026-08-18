use crate::am_map::*;
use crate::d_englsh::*;
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
use crate::dstrings::*;
use crate::i_system::*;
use crate::info::*;
use crate::m_fixed::*;
use crate::m_random::*;
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

static mut rcsid: [std::ffi::c_char; 50] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        112 as std::ffi::c_char,
        95 as std::ffi::c_char,
        105 as std::ffi::c_char,
        110 as std::ffi::c_char,
        116 as std::ffi::c_char,
        101 as std::ffi::c_char,
        114 as std::ffi::c_char,
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
        50 as std::ffi::c_char,
        50 as std::ffi::c_char,
        58 as std::ffi::c_char,
        52 as std::ffi::c_char,
        53 as std::ffi::c_char,
        58 as std::ffi::c_char,
        49 as std::ffi::c_char,
        49 as std::ffi::c_char,
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

pub const BONUSADD: std::ffi::c_int = 6;

pub static mut maxammo: [std::ffi::c_int; (NUMAMMO) as usize] = unsafe { [200, 50, 300, 50] };

pub static mut clipammo: [std::ffi::c_int; (NUMAMMO) as usize] = unsafe { [10, 4, 20, 1] };

pub unsafe extern "C" fn P_GiveAmmo(
    mut player: *mut player_t,
    mut ammo: ammotype_t,
    mut num: std::ffi::c_int,
) -> boolean {
    unsafe {
        let mut oldammo: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (ammo == am_noammo)
        // 	return false;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (ammo < 0 || ammo > NUMAMMO)
        // 	I_Error ("P_GiveAmmo: bad type %i", ammo);
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if ( player->ammo[ammo] == player->maxammo[ammo]  )
        // 	return false;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (num)
        // 	num *= clipammo[ammo];
        //     else
        // 	num = clipammo[ammo]/2;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (gameskill == sk_baby
        // 	|| gameskill == sk_nightmare)
        //     {
        // 	// give double ammo in trainer mode,
        // 	// you'll need in nightmare
        // 	num <<= 1;
        //     }
        todo!("if statement not yet translated");
        oldammo = (*player).ammo[(ammo) as usize];
        (*player).ammo[(ammo) as usize] += num;
        // TODO: if statement not yet translated:
        //
        //
        //     if (player->ammo[ammo] > player->maxammo[ammo])
        // 	player->ammo[ammo] = player->maxammo[ammo];
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // If non zero ammo,
        //     // don't change up weapons,
        //     // player was lower on purpose.
        //     if (oldammo)
        // 	return true;
        todo!("if statement not yet translated");
        // TODO: switch statement not yet translated:
        //
        //
        //     // We were down to zero,
        //     // so select a new weapon.
        //     // Preferences are not user selectable.
        //     switch (ammo)
        //     {
        //       case am_clip:
        // 	if (player->readyweapon == wp_fist)
        // 	{
        // 	    if (player->weaponowned[wp_chaingun])
        // 		player->pendingweapon = wp_chaingun;
        // 	    else
        // 		player->pendingweapon = wp_pistol;
        // 	}
        // 	break;
        //
        //       case am_shell:
        // 	if (player->readyweapon == wp_fist
        // 	    || player->readyweapon == wp_pistol)
        // 	{
        // 	    if (player->weaponowned[wp_shotgun])
        // 		player->pendingweapon = wp_shotgun;
        // 	}
        // 	break;
        //
        //       case am_cell:
        // 	if (player->readyweapon == wp_fist
        // 	    || player->readyweapon == wp_pistol)
        // 	{
        // 	    if (player->weaponowned[wp_plasma])
        // 		player->pendingweapon = wp_plasma;
        // 	}
        // 	break;
        //
        //       case am_misl:
        // 	if (player->readyweapon == wp_fist)
        // 	{
        // 	    if (player->weaponowned[wp_missile])
        // 		player->pendingweapon = wp_missile;
        // 	}
        //       default:
        // 	break;
        //     }
        todo!("switch statement not yet translated");
        return true_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_GiveWeapon(
    mut player: *mut player_t,
    mut weapon: weapontype_t,
    mut dropped: boolean,
) -> boolean {
    unsafe {
        let mut gaveammo: boolean = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut gaveweapon: boolean = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (netgame
        // 	&& (deathmatch!=2)
        // 	 && !dropped )
        //     {
        // 	// leave placed weapons forever on net games
        // 	if (player->weaponowned[weapon])
        // 	    return false;
        //
        // 	player->bonuscount += BONUSADD;
        // 	player->weaponowned[weapon] = true;
        //
        // 	if (deathmatch)
        // 	    P_GiveAmmo (player, weaponinfo[weapon].ammo, 5);
        // 	else
        // 	    P_GiveAmmo (player, weaponinfo[weapon].ammo, 2);
        // 	player->pendingweapon = weapon;
        //
        // 	if (player == &players[consoleplayer])
        // 	    S_StartSound (NULL, sfx_wpnup);
        // 	return false;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (weaponinfo[weapon].ammo != am_noammo)
        //     {
        // 	// give one clip with a dropped weapon,
        // 	// two clips with a found weapon
        // 	if (dropped)
        // 	    gaveammo = P_GiveAmmo (player, weaponinfo[weapon].ammo, 1);
        // 	else
        // 	    gaveammo = P_GiveAmmo (player, weaponinfo[weapon].ammo, 2);
        //     }
        //     else
        // 	gaveammo = false;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (player->weaponowned[weapon])
        // 	gaveweapon = false;
        //     else
        //     {
        // 	gaveweapon = true;
        // 	player->weaponowned[weapon] = true;
        // 	player->pendingweapon = weapon;
        //     }
        todo!("if statement not yet translated");
        return (gaveweapon || gaveammo);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_GiveBody(
    mut player: *mut player_t,
    mut num: std::ffi::c_int,
) -> boolean {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (player->health >= MAXHEALTH)
        // 	return false;
        todo!("if statement not yet translated");
        (*player).health += num;
        // TODO: if statement not yet translated:
        //
        //     if (player->health > MAXHEALTH)
        // 	player->health = MAXHEALTH;
        todo!("if statement not yet translated");
        (*(*player).mo).health = (*player).health;
        return true_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_GiveArmor(
    mut player: *mut player_t,
    mut armortype: std::ffi::c_int,
) -> boolean {
    unsafe {
        let mut hits: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        hits = (armortype * 100);
        // TODO: if statement not yet translated:
        //
        //     if (player->armorpoints >= hits)
        // 	return false;	// don't pick up
        todo!("if statement not yet translated");
        (*player).armortype = armortype;
        (*player).armorpoints = hits;
        return true_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_GiveCard(mut player: *mut player_t, mut card: card_t) {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (player->cards[card])
        // 	return;
        todo!("if statement not yet translated");
        (*player).bonuscount = BONUSADD;
        (*player).cards[(card) as usize] = 1;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_GivePower(
    mut player: *mut player_t,
    mut power: std::ffi::c_int,
) -> boolean {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (power == pw_invulnerability)
        //     {
        // 	player->powers[power] = INVULNTICS;
        // 	return true;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (power == pw_invisibility)
        //     {
        // 	player->powers[power] = INVISTICS;
        // 	player->mo->flags |= MF_SHADOW;
        // 	return true;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (power == pw_infrared)
        //     {
        // 	player->powers[power] = INFRATICS;
        // 	return true;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (power == pw_ironfeet)
        //     {
        // 	player->powers[power] = IRONTICS;
        // 	return true;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (power == pw_strength)
        //     {
        // 	P_GiveBody (player, 100);
        // 	player->powers[power] = 1;
        // 	return true;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (player->powers[power])
        // 	return false;	// already got it
        todo!("if statement not yet translated");
        (*player).powers[(power) as usize] = 1;
        return true_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn P_TouchSpecialThing(mut special: *mut mobj_t, mut toucher: *mut mobj_t) {
    unsafe {
        let mut player: *mut player_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut delta: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut sound: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        delta = ((*special).z - (*toucher).z);
        // TODO: if statement not yet translated:
        //
        //
        //     if (delta > toucher->height
        // 	|| delta < -8*FRACUNIT)
        //     {
        // 	// out of reach
        // 	return;
        //     }
        todo!("if statement not yet translated");
        sound = sfx_itemup;
        player = (*toucher).player;
        // TODO: if statement not yet translated:
        //
        //
        //     // Dead thing touching.
        //     // Can happen with a sliding player corpse.
        //     if (toucher->health <= 0)
        // 	return;
        todo!("if statement not yet translated");
        // TODO: switch statement not yet translated:
        //
        //
        //     // Identify by sprite.
        //     switch (special->sprite)
        //     {
        // 	// armor
        //       case SPR_ARM1:
        // 	if (!P_GiveArmor (player, 1))
        // 	    return;
        // 	player->message = GOTARMOR;
        // 	break;
        //
        //       case SPR_ARM2:
        // 	if (!P_GiveArmor (player, 2))
        // 	    return;
        // 	player->message = GOTMEGA;
        // 	break;
        //
        // 	// bonus items
        //       case SPR_BON1:
        // 	player->health++;		// can go over 100%
        // 	if (player->health > 200)
        // 	    player->health = 200;
        // 	player->mo->health = player->health;
        // 	player->message = GOTHTHBONUS;
        // 	break;
        //
        //       case SPR_BON2:
        // 	player->armorpoints++;		// can go over 100%
        // 	if (player->armorpoints > 200)
        // 	    player->armorpoints = 200;
        // 	if (!player->armortype)
        // 	    player->armortype = 1;
        // 	player->message = GOTARMBONUS;
        // 	break;
        //
        //       case SPR_SOUL:
        // 	player->health += 100;
        // 	if (player->health > 200)
        // 	    player->health = 200;
        // 	player->mo->health = player->health;
        // 	player->message = GOTSUPER;
        // 	sound = sfx_getpow;
        // 	break;
        //
        //       case SPR_MEGA:
        // 	if (gamemode != commercial)
        // 	    return;
        // 	player->health = 200;
        // 	player->mo->health = player->health;
        // 	P_GiveArmor (player,2);
        // 	player->message = GOTMSPHERE;
        // 	sound = sfx_getpow;
        // 	break;
        //
        // 	// cards
        // 	// leave cards for everyone
        //       case SPR_BKEY:
        // 	if (!player->cards[it_bluecard])
        // 	    player->message = GOTBLUECARD;
        // 	P_GiveCard (player, it_bluecard);
        // 	if (!netgame)
        // 	    break;
        // 	return;
        //
        //       case SPR_YKEY:
        // 	if (!player->cards[it_yellowcard])
        // 	    player->message = GOTYELWCARD;
        // 	P_GiveCard (player, it_yellowcard);
        // 	if (!netgame)
        // 	    break;
        // 	return;
        //
        //       case SPR_RKEY:
        // 	if (!player->cards[it_redcard])
        // 	    player->message = GOTREDCARD;
        // 	P_GiveCard (player, it_redcard);
        // 	if (!netgame)
        // 	    break;
        // 	return;
        //
        //       case SPR_BSKU:
        // 	if (!player->cards[it_blueskull])
        // 	    player->message = GOTBLUESKUL;
        // 	P_GiveCard (player, it_blueskull);
        // 	if (!netgame)
        // 	    break;
        // 	return;
        //
        //       case SPR_YSKU:
        // 	if (!player->cards[it_yellowskull])
        // 	    player->message = GOTYELWSKUL;
        // 	P_GiveCard (player, it_yellowskull);
        // 	if (!netgame)
        // 	    break;
        // 	return;
        //
        //       case SPR_RSKU:
        // 	if (!player->cards[it_redskull])
        // 	    player->message = GOTREDSKULL;
        // 	P_GiveCard (player, it_redskull);
        // 	if (!netgame)
        // 	    break;
        // 	return;
        //
        // 	// medikits, heals
        //       case SPR_STIM:
        // 	if (!P_GiveBody (player, 10))
        // 	    return;
        // 	player->message = GOTSTIM;
        // 	break;
        //
        //       case SPR_MEDI:
        // 	if (!P_GiveBody (player, 25))
        // 	    return;
        //
        // 	if (player->health < 25)
        // 	    player->message = GOTMEDINEED;
        // 	else
        // 	    player->message = GOTMEDIKIT;
        // 	break;
        //
        //
        // 	// power ups
        //       case SPR_PINV:
        // 	if (!P_GivePower (player, pw_invulnerability))
        // 	    return;
        // 	player->message = GOTINVUL;
        // 	sound = sfx_getpow;
        // 	break;
        //
        //       case SPR_PSTR:
        // 	if (!P_GivePower (player, pw_strength))
        // 	    return;
        // 	player->message = GOTBERSERK;
        // 	if (player->readyweapon != wp_fist)
        // 	    player->pendingweapon = wp_fist;
        // 	sound = sfx_getpow;
        // 	break;
        //
        //       case SPR_PINS:
        // 	if (!P_GivePower (player, pw_invisibility))
        // 	    return;
        // 	player->message = GOTINVIS;
        // 	sound = sfx_getpow;
        // 	break;
        //
        //       case SPR_SUIT:
        // 	if (!P_GivePower (player, pw_ironfeet))
        // 	    return;
        // 	player->message = GOTSUIT;
        // 	sound = sfx_getpow;
        // 	break;
        //
        //       case SPR_PMAP:
        // 	if (!P_GivePower (player, pw_allmap))
        // 	    return;
        // 	player->message = GOTMAP;
        // 	sound = sfx_getpow;
        // 	break;
        //
        //       case SPR_PVIS:
        // 	if (!P_GivePower (player, pw_infrared))
        // 	    return;
        // 	player->message = GOTVISOR;
        // 	sound = sfx_getpow;
        // 	break;
        //
        // 	// ammo
        //       case SPR_CLIP:
        // 	if (special->flags & MF_DROPPED)
        // 	{
        // 	    if (!P_GiveAmmo (player,am_clip,0))
        // 		return;
        // 	}
        // 	else
        // 	{
        // 	    if (!P_GiveAmmo (player,am_clip,1))
        // 		return;
        // 	}
        // 	player->message = GOTCLIP;
        // 	break;
        //
        //       case SPR_AMMO:
        // 	if (!P_GiveAmmo (player, am_clip,5))
        // 	    return;
        // 	player->message = GOTCLIPBOX;
        // 	break;
        //
        //       case SPR_ROCK:
        // 	if (!P_GiveAmmo (player, am_misl,1))
        // 	    return;
        // 	player->message = GOTROCKET;
        // 	break;
        //
        //       case SPR_BROK:
        // 	if (!P_GiveAmmo (player, am_misl,5))
        // 	    return;
        // 	player->message = GOTROCKBOX;
        // 	break;
        //
        //       case SPR_CELL:
        // 	if (!P_GiveAmmo (player, am_cell,1))
        // 	    return;
        // 	player->message = GOTCELL;
        // 	break;
        //
        //       case SPR_CELP:
        // 	if (!P_GiveAmmo (player, am_cell,5))
        // 	    return;
        // 	player->message = GOTCELLBOX;
        // 	break;
        //
        //       case SPR_SHEL:
        // 	if (!P_GiveAmmo (player, am_shell,1))
        // 	    return;
        // 	player->message = GOTSHELLS;
        // 	break;
        //
        //       case SPR_SBOX:
        // 	if (!P_GiveAmmo (player, am_shell,5))
        // 	    return;
        // 	player->message = GOTSHELLBOX;
        // 	break;
        //
        //       case SPR_BPAK:
        // 	if (!player->backpack)
        // 	{
        // 	    for (i=0 ; i<NUMAMMO ; i++)
        // 		player->maxammo[i] *= 2;
        // 	    player->backpack = true;
        // 	}
        // 	for (i=0 ; i<NUMAMMO ; i++)
        // 	    P_GiveAmmo (player, i, 1);
        // 	player->message = GOTBACKPACK;
        // 	break;
        //
        // 	// weapons
        //       case SPR_BFUG:
        // 	if (!P_GiveWeapon (player, wp_bfg, false) )
        // 	    return;
        // 	player->message = GOTBFG9000;
        // 	sound = sfx_wpnup;
        // 	break;
        //
        //       case SPR_MGUN:
        // 	if (!P_GiveWeapon (player, wp_chaingun, special->flags&MF_DROPPED) )
        // 	    return;
        // 	player->message = GOTCHAINGUN;
        // 	sound = sfx_wpnup;
        // 	break;
        //
        //       case SPR_CSAW:
        // 	if (!P_GiveWeapon (player, wp_chainsaw, false) )
        // 	    return;
        // 	player->message = GOTCHAINSAW;
        // 	sound = sfx_wpnup;
        // 	break;
        //
        //       case SPR_LAUN:
        // 	if (!P_GiveWeapon (player, wp_missile, false) )
        // 	    return;
        // 	player->message = GOTLAUNCHER;
        // 	sound = sfx_wpnup;
        // 	break;
        //
        //       case SPR_PLAS:
        // 	if (!P_GiveWeapon (player, wp_plasma, false) )
        // 	    return;
        // 	player->message = GOTPLASMA;
        // 	sound = sfx_wpnup;
        // 	break;
        //
        //       case SPR_SHOT:
        // 	if (!P_GiveWeapon (player, wp_shotgun, special->flags&MF_DROPPED ) )
        // 	    return;
        // 	player->message = GOTSHOTGUN;
        // 	sound = sfx_wpnup;
        // 	break;
        //
        //       case SPR_SGN2:
        // 	if (!P_GiveWeapon (player, wp_supershotgun, special->flags&MF_DROPPED ) )
        // 	    return;
        // 	player->message = GOTSHOTGUN2;
        // 	sound = sfx_wpnup;
        // 	break;
        //
        //       default:
        // 	I_Error ("P_SpecialThing: Unknown gettable thing");
        //     }
        todo!("switch statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (special->flags & MF_COUNTITEM)
        // 	player->itemcount++;
        todo!("if statement not yet translated");
        P_RemoveMobj(special);
        (*player).bonuscount += BONUSADD;
        // TODO: if statement not yet translated:
        //
        //     if (player == &players[consoleplayer])
        // 	S_StartSound (NULL, sound);
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_KillMobj(mut source: *mut mobj_t, mut target: *mut mobj_t) {
    unsafe {
        let mut item: mobjtype_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut mo: *mut mobj_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        (*target).flags &= (!((MF_SHOOTABLE | MF_FLOAT) | MF_SKULLFLY));
        // TODO: if statement not yet translated:
        //
        //
        //     if (target->type != MT_SKULL)
        // 	target->flags &= ~MF_NOGRAVITY;
        todo!("if statement not yet translated");
        (*target).flags |= (MF_CORPSE | MF_DROPOFF);
        (*target).height >>= 2;
        // TODO: if statement not yet translated:
        //
        //
        //     if (source && source->player)
        //     {
        // 	// count for intermission
        // 	if (target->flags & MF_COUNTKILL)
        // 	    source->player->killcount++;
        //
        // 	if (target->player)
        // 	    source->player->frags[target->player-players]++;
        //     }
        //     else if (!netgame && (target->flags & MF_COUNTKILL) )
        //     {
        // 	// count all monster deaths,
        // 	// even those caused by other monsters
        // 	players[0].killcount++;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (target->player)
        //     {
        // 	// count environment kills against you
        // 	if (!source)
        // 	    target->player->frags[target->player-players]++;
        //
        // 	target->flags &= ~MF_SOLID;
        // 	target->player->playerstate = PST_DEAD;
        // 	P_DropWeapon (target->player);
        //
        // 	if (target->player == &players[consoleplayer]
        // 	    && automapactive)
        // 	{
        // 	    // don't die in auto map,
        // 	    // switch view prior to dying
        // 	    AM_Stop ();
        // 	}
        //
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (target->health < -target->info->spawnhealth
        // 	&& target->info->xdeathstate)
        //     {
        // 	P_SetMobjState (target, target->info->xdeathstate);
        //     }
        //     else
        // 	P_SetMobjState (target, target->info->deathstate);
        todo!("if statement not yet translated");
        (*target).tics -= (P_Random() & 3);
        // TODO: if statement not yet translated:
        //
        //
        //     if (target->tics < 1)
        // 	target->tics = 1;
        todo!("if statement not yet translated");
        // TODO: switch statement not yet translated:
        //
        //
        //     //	I_StartSound (&actor->r, actor->info->deathsound);
        //
        //
        //     // Drop stuff.
        //     // This determines the kind of object spawned
        //     // during the death frame of a thing.
        //     switch (target->type)
        //     {
        //       case MT_WOLFSS:
        //       case MT_POSSESSED:
        // 	item = MT_CLIP;
        // 	break;
        //
        //       case MT_SHOTGUY:
        // 	item = MT_SHOTGUN;
        // 	break;
        //
        //       case MT_CHAINGUY:
        // 	item = MT_CHAINGUN;
        // 	break;
        //
        //       default:
        // 	return;
        //     }
        todo!("switch statement not yet translated");
        mo = P_SpawnMobj((*target).x, (*target).y, ONFLOORZ, item);
        (*mo).flags |= MF_DROPPED;
    }
}

pub unsafe extern "C" fn P_DamageMobj(
    mut target: *mut mobj_t,
    mut inflictor: *mut mobj_t,
    mut source: *mut mobj_t,
    mut damage: std::ffi::c_int,
) {
    unsafe {
        let mut ang: std::ffi::c_uint = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut saved: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut player: *mut player_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut thrust: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut temp: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if ( !(target->flags & MF_SHOOTABLE) )
        // 	return;	// shouldn't happen...
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (target->health <= 0)
        // 	return;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if ( target->flags & MF_SKULLFLY )
        //     {
        // 	target->momx = target->momy = target->momz = 0;
        //     }
        todo!("if statement not yet translated");
        player = (*target).player;
        // TODO: if statement not yet translated:
        //
        //     if (player && gameskill == sk_baby)
        // 	damage >>= 1; 	// take half damage in trainer mode
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // Some close combat weapons should not
        //     // inflict thrust and push the victim out of reach,
        //     // thus kick away unless using the chainsaw.
        //     if (inflictor
        // 	&& !(target->flags & MF_NOCLIP)
        // 	&& (!source
        // 	    || !source->player
        // 	    || source->player->readyweapon != wp_chainsaw))
        //     {
        // 	ang = R_PointToAngle2 ( inflictor->x,
        // 				inflictor->y,
        // 				target->x,
        // 				target->y);
        //
        // 	thrust = damage*(FRACUNIT>>3)*100/target->info->mass;
        //
        // 	// make fall forwards sometimes
        // 	if ( damage < 40
        // 	     && damage > target->health
        // 	     && target->z - inflictor->z > 64*FRACUNIT
        // 	     && (P_Random ()&1) )
        // 	{
        // 	    ang += ANG180;
        // 	    thrust *= 4;
        // 	}
        //
        // 	ang >>= ANGLETOFINESHIFT;
        // 	target->momx += FixedMul (thrust, finecosine[ang]);
        // 	target->momy += FixedMul (thrust, finesine[ang]);
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // player specific
        //     if (player)
        //     {
        // 	// end of game hell hack
        // 	if (target->subsector->sector->special == 11
        // 	    && damage >= target->health)
        // 	{
        // 	    damage = target->health - 1;
        // 	}
        //
        //
        // 	// Below certain threshold,
        // 	// ignore damage in GOD mode, or with INVUL power.
        // 	if ( damage < 1000
        // 	     && ( (player->cheats&CF_GODMODE)
        // 		  || player->powers[pw_invulnerability] ) )
        // 	{
        // 	    return;
        // 	}
        //
        // 	if (player->armortype)
        // 	{
        // 	    if (player->armortype == 1)
        // 		saved = damage/3;
        // 	    else
        // 		saved = damage/2;
        //
        // 	    if (player->armorpoints <= saved)
        // 	    {
        // 		// armor is used up
        // 		saved = player->armorpoints;
        // 		player->armortype = 0;
        // 	    }
        // 	    player->armorpoints -= saved;
        // 	    damage -= saved;
        // 	}
        // 	player->health -= damage; 	// mirror mobj health here for Dave
        // 	if (player->health < 0)
        // 	    player->health = 0;
        //
        // 	player->attacker = source;
        // 	player->damagecount += damage;	// add damage after armor / invuln
        //
        // 	if (player->damagecount > 100)
        // 	    player->damagecount = 100;	// teleport stomp does 10k points...
        //
        // 	temp = damage < 100 ? damage : 100;
        //
        // 	if (player == &players[consoleplayer])
        // 	    I_Tactile (40,10,40+temp*2);
        //     }
        todo!("if statement not yet translated");
        (*target).health -= damage;
        // TODO: if statement not yet translated:
        //
        //     if (target->health <= 0)
        //     {
        // 	P_KillMobj (source, target);
        // 	return;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if ( (P_Random () < target->info->painchance)
        // 	 && !(target->flags&MF_SKULLFLY) )
        //     {
        // 	target->flags |= MF_JUSTHIT;	// fight back!
        //
        // 	P_SetMobjState (target, target->info->painstate);
        //     }
        todo!("if statement not yet translated");
        (*target).reactiontime = 0;
        // TODO: if statement not yet translated:
        //
        //     if ( (!target->threshold || target->type == MT_VILE)
        // 	 && source && source != target
        // 	 && source->type != MT_VILE)
        //     {
        // 	// if not intent on another player,
        // 	// chase after this one
        // 	target->target = source;
        // 	target->threshold = BASETHRESHOLD;
        // 	if (target->state == &states[target->info->spawnstate]
        // 	    && target->info->seestate != S_NULL)
        // 	    P_SetMobjState (target, target->info->seestate);
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}
