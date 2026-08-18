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
use crate::z_zone::*;

static mut rcsid: [std::ffi::c_char; 49] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        112 as std::ffi::c_char,
        95 as std::ffi::c_char,
        116 as std::ffi::c_char,
        105 as std::ffi::c_char,
        99 as std::ffi::c_char,
        107 as std::ffi::c_char,
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

pub static mut leveltime: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut thinkercap: thinker_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn P_InitThinkers() {
    unsafe {
        thinkercap.prev = thinkercap.next = (&(thinkercap) as *const thinker_t as *mut thinker_t);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_AddThinker(mut thinker: *mut thinker_t) {
    unsafe {
        (*thinkercap.prev).next = thinker;
        (*thinker).next = (&(thinkercap) as *const thinker_t as *mut thinker_t);
        (*thinker).prev = thinkercap.prev;
        thinkercap.prev = thinker;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_RemoveThinker(mut thinker: *mut thinker_t) {
    unsafe {
        (*thinker).function.acv = ((-(1)) as actionf_v);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_AllocateThinker(mut thinker: *mut thinker_t) {
    unsafe {
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_RunThinkers() {
    unsafe {
        let mut currentthinker: *mut thinker_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        currentthinker = thinkercap.next;
        // TODO: while statement not yet translated:
        //
        //     while (currentthinker != &thinkercap)
        //     {
        // 	if ( currentthinker->function.acv == (actionf_v)(-1) )
        // 	{
        // 	    // time to remove it
        // 	    currentthinker->next->prev = currentthinker->prev;
        // 	    currentthinker->prev->next = currentthinker->next;
        // 	    Z_Free (currentthinker);
        // 	}
        // 	else
        // 	{
        // 	    if (currentthinker->function.acp1)
        // 		currentthinker->function.acp1 (currentthinker);
        // 	}
        // 	currentthinker = currentthinker->next;
        //     }
        todo!("while statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_Ticker() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     // run the tic
        //     if (paused)
        // 	return;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // pause if in menu and at least one tic has been run
        //     if ( !netgame
        // 	 && menuactive
        // 	 && !demoplayback
        // 	 && players[consoleplayer].viewz != 1)
        //     {
        // 	return;
        //     }
        todo!("if statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //
        //
        //     for (i=0 ; i<MAXPLAYERS ; i++)
        // 	if (playeringame[i])
        // 	    P_PlayerThink (&players[i]);
        todo!("for statement not yet translated");
        P_RunThinkers();
        P_UpdateSpecials();
        P_RespawnSpecials();
        {
            let __macro_tmp = leveltime;
            leveltime += 1;
            __macro_tmp
        };
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}
