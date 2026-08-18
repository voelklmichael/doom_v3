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
        115 as std::ffi::c_char,
        119 as std::ffi::c_char,
        105 as std::ffi::c_char,
        116 as std::ffi::c_char,
        99 as std::ffi::c_char,
        104 as std::ffi::c_char,
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

pub static mut alphSwitchList: [switchlist_t; 41] = unsafe {
    [
        switchlist_t {
            name1: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                49 as std::ffi::c_char,
                66 as std::ffi::c_char,
                82 as std::ffi::c_char,
                67 as std::ffi::c_char,
                79 as std::ffi::c_char,
                77 as std::ffi::c_char,
                0,
            ],
            name2: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                50 as std::ffi::c_char,
                66 as std::ffi::c_char,
                82 as std::ffi::c_char,
                67 as std::ffi::c_char,
                79 as std::ffi::c_char,
                77 as std::ffi::c_char,
                0,
            ],
            episode: 1,
        },
        switchlist_t {
            name1: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                49 as std::ffi::c_char,
                66 as std::ffi::c_char,
                82 as std::ffi::c_char,
                78 as std::ffi::c_char,
                49 as std::ffi::c_char,
                0,
                0,
            ],
            name2: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                50 as std::ffi::c_char,
                66 as std::ffi::c_char,
                82 as std::ffi::c_char,
                78 as std::ffi::c_char,
                49 as std::ffi::c_char,
                0,
                0,
            ],
            episode: 1,
        },
        switchlist_t {
            name1: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                49 as std::ffi::c_char,
                66 as std::ffi::c_char,
                82 as std::ffi::c_char,
                78 as std::ffi::c_char,
                50 as std::ffi::c_char,
                0,
                0,
            ],
            name2: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                50 as std::ffi::c_char,
                66 as std::ffi::c_char,
                82 as std::ffi::c_char,
                78 as std::ffi::c_char,
                50 as std::ffi::c_char,
                0,
                0,
            ],
            episode: 1,
        },
        switchlist_t {
            name1: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                49 as std::ffi::c_char,
                66 as std::ffi::c_char,
                82 as std::ffi::c_char,
                78 as std::ffi::c_char,
                71 as std::ffi::c_char,
                78 as std::ffi::c_char,
                0,
            ],
            name2: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                50 as std::ffi::c_char,
                66 as std::ffi::c_char,
                82 as std::ffi::c_char,
                78 as std::ffi::c_char,
                71 as std::ffi::c_char,
                78 as std::ffi::c_char,
                0,
            ],
            episode: 1,
        },
        switchlist_t {
            name1: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                49 as std::ffi::c_char,
                66 as std::ffi::c_char,
                82 as std::ffi::c_char,
                79 as std::ffi::c_char,
                87 as std::ffi::c_char,
                78 as std::ffi::c_char,
                0,
            ],
            name2: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                50 as std::ffi::c_char,
                66 as std::ffi::c_char,
                82 as std::ffi::c_char,
                79 as std::ffi::c_char,
                87 as std::ffi::c_char,
                78 as std::ffi::c_char,
                0,
            ],
            episode: 1,
        },
        switchlist_t {
            name1: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                49 as std::ffi::c_char,
                67 as std::ffi::c_char,
                79 as std::ffi::c_char,
                77 as std::ffi::c_char,
                77 as std::ffi::c_char,
                0,
                0,
            ],
            name2: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                50 as std::ffi::c_char,
                67 as std::ffi::c_char,
                79 as std::ffi::c_char,
                77 as std::ffi::c_char,
                77 as std::ffi::c_char,
                0,
                0,
            ],
            episode: 1,
        },
        switchlist_t {
            name1: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                49 as std::ffi::c_char,
                67 as std::ffi::c_char,
                79 as std::ffi::c_char,
                77 as std::ffi::c_char,
                80 as std::ffi::c_char,
                0,
                0,
            ],
            name2: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                50 as std::ffi::c_char,
                67 as std::ffi::c_char,
                79 as std::ffi::c_char,
                77 as std::ffi::c_char,
                80 as std::ffi::c_char,
                0,
                0,
            ],
            episode: 1,
        },
        switchlist_t {
            name1: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                49 as std::ffi::c_char,
                68 as std::ffi::c_char,
                73 as std::ffi::c_char,
                82 as std::ffi::c_char,
                84 as std::ffi::c_char,
                0,
                0,
            ],
            name2: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                50 as std::ffi::c_char,
                68 as std::ffi::c_char,
                73 as std::ffi::c_char,
                82 as std::ffi::c_char,
                84 as std::ffi::c_char,
                0,
                0,
            ],
            episode: 1,
        },
        switchlist_t {
            name1: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                49 as std::ffi::c_char,
                69 as std::ffi::c_char,
                88 as std::ffi::c_char,
                73 as std::ffi::c_char,
                84 as std::ffi::c_char,
                0,
                0,
            ],
            name2: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                50 as std::ffi::c_char,
                69 as std::ffi::c_char,
                88 as std::ffi::c_char,
                73 as std::ffi::c_char,
                84 as std::ffi::c_char,
                0,
                0,
            ],
            episode: 1,
        },
        switchlist_t {
            name1: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                49 as std::ffi::c_char,
                71 as std::ffi::c_char,
                82 as std::ffi::c_char,
                65 as std::ffi::c_char,
                89 as std::ffi::c_char,
                0,
                0,
            ],
            name2: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                50 as std::ffi::c_char,
                71 as std::ffi::c_char,
                82 as std::ffi::c_char,
                65 as std::ffi::c_char,
                89 as std::ffi::c_char,
                0,
                0,
            ],
            episode: 1,
        },
        switchlist_t {
            name1: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                49 as std::ffi::c_char,
                71 as std::ffi::c_char,
                82 as std::ffi::c_char,
                65 as std::ffi::c_char,
                89 as std::ffi::c_char,
                49 as std::ffi::c_char,
                0,
            ],
            name2: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                50 as std::ffi::c_char,
                71 as std::ffi::c_char,
                82 as std::ffi::c_char,
                65 as std::ffi::c_char,
                89 as std::ffi::c_char,
                49 as std::ffi::c_char,
                0,
            ],
            episode: 1,
        },
        switchlist_t {
            name1: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                49 as std::ffi::c_char,
                77 as std::ffi::c_char,
                69 as std::ffi::c_char,
                84 as std::ffi::c_char,
                65 as std::ffi::c_char,
                76 as std::ffi::c_char,
                0,
            ],
            name2: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                50 as std::ffi::c_char,
                77 as std::ffi::c_char,
                69 as std::ffi::c_char,
                84 as std::ffi::c_char,
                65 as std::ffi::c_char,
                76 as std::ffi::c_char,
                0,
            ],
            episode: 1,
        },
        switchlist_t {
            name1: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                49 as std::ffi::c_char,
                80 as std::ffi::c_char,
                73 as std::ffi::c_char,
                80 as std::ffi::c_char,
                69 as std::ffi::c_char,
                0,
                0,
            ],
            name2: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                50 as std::ffi::c_char,
                80 as std::ffi::c_char,
                73 as std::ffi::c_char,
                80 as std::ffi::c_char,
                69 as std::ffi::c_char,
                0,
                0,
            ],
            episode: 1,
        },
        switchlist_t {
            name1: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                49 as std::ffi::c_char,
                83 as std::ffi::c_char,
                76 as std::ffi::c_char,
                65 as std::ffi::c_char,
                68 as std::ffi::c_char,
                0,
                0,
            ],
            name2: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                50 as std::ffi::c_char,
                83 as std::ffi::c_char,
                76 as std::ffi::c_char,
                65 as std::ffi::c_char,
                68 as std::ffi::c_char,
                0,
                0,
            ],
            episode: 1,
        },
        switchlist_t {
            name1: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                49 as std::ffi::c_char,
                83 as std::ffi::c_char,
                84 as std::ffi::c_char,
                65 as std::ffi::c_char,
                82 as std::ffi::c_char,
                71 as std::ffi::c_char,
                0,
            ],
            name2: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                50 as std::ffi::c_char,
                83 as std::ffi::c_char,
                84 as std::ffi::c_char,
                65 as std::ffi::c_char,
                82 as std::ffi::c_char,
                71 as std::ffi::c_char,
                0,
            ],
            episode: 1,
        },
        switchlist_t {
            name1: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                49 as std::ffi::c_char,
                83 as std::ffi::c_char,
                84 as std::ffi::c_char,
                79 as std::ffi::c_char,
                78 as std::ffi::c_char,
                49 as std::ffi::c_char,
                0,
            ],
            name2: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                50 as std::ffi::c_char,
                83 as std::ffi::c_char,
                84 as std::ffi::c_char,
                79 as std::ffi::c_char,
                78 as std::ffi::c_char,
                49 as std::ffi::c_char,
                0,
            ],
            episode: 1,
        },
        switchlist_t {
            name1: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                49 as std::ffi::c_char,
                83 as std::ffi::c_char,
                84 as std::ffi::c_char,
                79 as std::ffi::c_char,
                78 as std::ffi::c_char,
                50 as std::ffi::c_char,
                0,
            ],
            name2: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                50 as std::ffi::c_char,
                83 as std::ffi::c_char,
                84 as std::ffi::c_char,
                79 as std::ffi::c_char,
                78 as std::ffi::c_char,
                50 as std::ffi::c_char,
                0,
            ],
            episode: 1,
        },
        switchlist_t {
            name1: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                49 as std::ffi::c_char,
                83 as std::ffi::c_char,
                84 as std::ffi::c_char,
                79 as std::ffi::c_char,
                78 as std::ffi::c_char,
                69 as std::ffi::c_char,
                0,
            ],
            name2: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                50 as std::ffi::c_char,
                83 as std::ffi::c_char,
                84 as std::ffi::c_char,
                79 as std::ffi::c_char,
                78 as std::ffi::c_char,
                69 as std::ffi::c_char,
                0,
            ],
            episode: 1,
        },
        switchlist_t {
            name1: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                49 as std::ffi::c_char,
                83 as std::ffi::c_char,
                84 as std::ffi::c_char,
                82 as std::ffi::c_char,
                84 as std::ffi::c_char,
                78 as std::ffi::c_char,
                0,
            ],
            name2: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                50 as std::ffi::c_char,
                83 as std::ffi::c_char,
                84 as std::ffi::c_char,
                82 as std::ffi::c_char,
                84 as std::ffi::c_char,
                78 as std::ffi::c_char,
                0,
            ],
            episode: 1,
        },
        switchlist_t {
            name1: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                49 as std::ffi::c_char,
                66 as std::ffi::c_char,
                76 as std::ffi::c_char,
                85 as std::ffi::c_char,
                69 as std::ffi::c_char,
                0,
                0,
            ],
            name2: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                50 as std::ffi::c_char,
                66 as std::ffi::c_char,
                76 as std::ffi::c_char,
                85 as std::ffi::c_char,
                69 as std::ffi::c_char,
                0,
                0,
            ],
            episode: 2,
        },
        switchlist_t {
            name1: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                49 as std::ffi::c_char,
                67 as std::ffi::c_char,
                77 as std::ffi::c_char,
                84 as std::ffi::c_char,
                0,
                0,
                0,
            ],
            name2: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                50 as std::ffi::c_char,
                67 as std::ffi::c_char,
                77 as std::ffi::c_char,
                84 as std::ffi::c_char,
                0,
                0,
                0,
            ],
            episode: 2,
        },
        switchlist_t {
            name1: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                49 as std::ffi::c_char,
                71 as std::ffi::c_char,
                65 as std::ffi::c_char,
                82 as std::ffi::c_char,
                71 as std::ffi::c_char,
                0,
                0,
            ],
            name2: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                50 as std::ffi::c_char,
                71 as std::ffi::c_char,
                65 as std::ffi::c_char,
                82 as std::ffi::c_char,
                71 as std::ffi::c_char,
                0,
                0,
            ],
            episode: 2,
        },
        switchlist_t {
            name1: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                49 as std::ffi::c_char,
                71 as std::ffi::c_char,
                83 as std::ffi::c_char,
                84 as std::ffi::c_char,
                79 as std::ffi::c_char,
                78 as std::ffi::c_char,
                0,
            ],
            name2: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                50 as std::ffi::c_char,
                71 as std::ffi::c_char,
                83 as std::ffi::c_char,
                84 as std::ffi::c_char,
                79 as std::ffi::c_char,
                78 as std::ffi::c_char,
                0,
            ],
            episode: 2,
        },
        switchlist_t {
            name1: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                49 as std::ffi::c_char,
                72 as std::ffi::c_char,
                79 as std::ffi::c_char,
                84 as std::ffi::c_char,
                0,
                0,
                0,
            ],
            name2: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                50 as std::ffi::c_char,
                72 as std::ffi::c_char,
                79 as std::ffi::c_char,
                84 as std::ffi::c_char,
                0,
                0,
                0,
            ],
            episode: 2,
        },
        switchlist_t {
            name1: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                49 as std::ffi::c_char,
                76 as std::ffi::c_char,
                73 as std::ffi::c_char,
                79 as std::ffi::c_char,
                78 as std::ffi::c_char,
                0,
                0,
            ],
            name2: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                50 as std::ffi::c_char,
                76 as std::ffi::c_char,
                73 as std::ffi::c_char,
                79 as std::ffi::c_char,
                78 as std::ffi::c_char,
                0,
                0,
            ],
            episode: 2,
        },
        switchlist_t {
            name1: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                49 as std::ffi::c_char,
                83 as std::ffi::c_char,
                65 as std::ffi::c_char,
                84 as std::ffi::c_char,
                89 as std::ffi::c_char,
                82 as std::ffi::c_char,
                0,
            ],
            name2: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                50 as std::ffi::c_char,
                83 as std::ffi::c_char,
                65 as std::ffi::c_char,
                84 as std::ffi::c_char,
                89 as std::ffi::c_char,
                82 as std::ffi::c_char,
                0,
            ],
            episode: 2,
        },
        switchlist_t {
            name1: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                49 as std::ffi::c_char,
                83 as std::ffi::c_char,
                75 as std::ffi::c_char,
                73 as std::ffi::c_char,
                78 as std::ffi::c_char,
                0,
                0,
            ],
            name2: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                50 as std::ffi::c_char,
                83 as std::ffi::c_char,
                75 as std::ffi::c_char,
                73 as std::ffi::c_char,
                78 as std::ffi::c_char,
                0,
                0,
            ],
            episode: 2,
        },
        switchlist_t {
            name1: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                49 as std::ffi::c_char,
                86 as std::ffi::c_char,
                73 as std::ffi::c_char,
                78 as std::ffi::c_char,
                69 as std::ffi::c_char,
                0,
                0,
            ],
            name2: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                50 as std::ffi::c_char,
                86 as std::ffi::c_char,
                73 as std::ffi::c_char,
                78 as std::ffi::c_char,
                69 as std::ffi::c_char,
                0,
                0,
            ],
            episode: 2,
        },
        switchlist_t {
            name1: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                49 as std::ffi::c_char,
                87 as std::ffi::c_char,
                79 as std::ffi::c_char,
                79 as std::ffi::c_char,
                68 as std::ffi::c_char,
                0,
                0,
            ],
            name2: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                50 as std::ffi::c_char,
                87 as std::ffi::c_char,
                79 as std::ffi::c_char,
                79 as std::ffi::c_char,
                68 as std::ffi::c_char,
                0,
                0,
            ],
            episode: 2,
        },
        switchlist_t {
            name1: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                49 as std::ffi::c_char,
                80 as std::ffi::c_char,
                65 as std::ffi::c_char,
                78 as std::ffi::c_char,
                69 as std::ffi::c_char,
                76 as std::ffi::c_char,
                0,
            ],
            name2: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                50 as std::ffi::c_char,
                80 as std::ffi::c_char,
                65 as std::ffi::c_char,
                78 as std::ffi::c_char,
                69 as std::ffi::c_char,
                76 as std::ffi::c_char,
                0,
            ],
            episode: 3,
        },
        switchlist_t {
            name1: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                49 as std::ffi::c_char,
                82 as std::ffi::c_char,
                79 as std::ffi::c_char,
                67 as std::ffi::c_char,
                75 as std::ffi::c_char,
                0,
                0,
            ],
            name2: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                50 as std::ffi::c_char,
                82 as std::ffi::c_char,
                79 as std::ffi::c_char,
                67 as std::ffi::c_char,
                75 as std::ffi::c_char,
                0,
                0,
            ],
            episode: 3,
        },
        switchlist_t {
            name1: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                49 as std::ffi::c_char,
                77 as std::ffi::c_char,
                69 as std::ffi::c_char,
                84 as std::ffi::c_char,
                50 as std::ffi::c_char,
                0,
                0,
            ],
            name2: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                50 as std::ffi::c_char,
                77 as std::ffi::c_char,
                69 as std::ffi::c_char,
                84 as std::ffi::c_char,
                50 as std::ffi::c_char,
                0,
                0,
            ],
            episode: 3,
        },
        switchlist_t {
            name1: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                49 as std::ffi::c_char,
                87 as std::ffi::c_char,
                68 as std::ffi::c_char,
                77 as std::ffi::c_char,
                69 as std::ffi::c_char,
                84 as std::ffi::c_char,
                0,
            ],
            name2: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                50 as std::ffi::c_char,
                87 as std::ffi::c_char,
                68 as std::ffi::c_char,
                77 as std::ffi::c_char,
                69 as std::ffi::c_char,
                84 as std::ffi::c_char,
                0,
            ],
            episode: 3,
        },
        switchlist_t {
            name1: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                49 as std::ffi::c_char,
                66 as std::ffi::c_char,
                82 as std::ffi::c_char,
                73 as std::ffi::c_char,
                75 as std::ffi::c_char,
                0,
                0,
            ],
            name2: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                50 as std::ffi::c_char,
                66 as std::ffi::c_char,
                82 as std::ffi::c_char,
                73 as std::ffi::c_char,
                75 as std::ffi::c_char,
                0,
                0,
            ],
            episode: 3,
        },
        switchlist_t {
            name1: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                49 as std::ffi::c_char,
                77 as std::ffi::c_char,
                79 as std::ffi::c_char,
                68 as std::ffi::c_char,
                49 as std::ffi::c_char,
                0,
                0,
            ],
            name2: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                50 as std::ffi::c_char,
                77 as std::ffi::c_char,
                79 as std::ffi::c_char,
                68 as std::ffi::c_char,
                49 as std::ffi::c_char,
                0,
                0,
            ],
            episode: 3,
        },
        switchlist_t {
            name1: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                49 as std::ffi::c_char,
                90 as std::ffi::c_char,
                73 as std::ffi::c_char,
                77 as std::ffi::c_char,
                0,
                0,
                0,
            ],
            name2: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                50 as std::ffi::c_char,
                90 as std::ffi::c_char,
                73 as std::ffi::c_char,
                77 as std::ffi::c_char,
                0,
                0,
                0,
            ],
            episode: 3,
        },
        switchlist_t {
            name1: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                49 as std::ffi::c_char,
                83 as std::ffi::c_char,
                84 as std::ffi::c_char,
                79 as std::ffi::c_char,
                78 as std::ffi::c_char,
                54 as std::ffi::c_char,
                0,
            ],
            name2: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                50 as std::ffi::c_char,
                83 as std::ffi::c_char,
                84 as std::ffi::c_char,
                79 as std::ffi::c_char,
                78 as std::ffi::c_char,
                54 as std::ffi::c_char,
                0,
            ],
            episode: 3,
        },
        switchlist_t {
            name1: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                49 as std::ffi::c_char,
                84 as std::ffi::c_char,
                69 as std::ffi::c_char,
                75 as std::ffi::c_char,
                0,
                0,
                0,
            ],
            name2: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                50 as std::ffi::c_char,
                84 as std::ffi::c_char,
                69 as std::ffi::c_char,
                75 as std::ffi::c_char,
                0,
                0,
                0,
            ],
            episode: 3,
        },
        switchlist_t {
            name1: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                49 as std::ffi::c_char,
                77 as std::ffi::c_char,
                65 as std::ffi::c_char,
                82 as std::ffi::c_char,
                66 as std::ffi::c_char,
                0,
                0,
            ],
            name2: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                50 as std::ffi::c_char,
                77 as std::ffi::c_char,
                65 as std::ffi::c_char,
                82 as std::ffi::c_char,
                66 as std::ffi::c_char,
                0,
                0,
            ],
            episode: 3,
        },
        switchlist_t {
            name1: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                49 as std::ffi::c_char,
                83 as std::ffi::c_char,
                75 as std::ffi::c_char,
                85 as std::ffi::c_char,
                76 as std::ffi::c_char,
                76 as std::ffi::c_char,
                0,
            ],
            name2: [
                83 as std::ffi::c_char,
                87 as std::ffi::c_char,
                50 as std::ffi::c_char,
                83 as std::ffi::c_char,
                75 as std::ffi::c_char,
                85 as std::ffi::c_char,
                76 as std::ffi::c_char,
                76 as std::ffi::c_char,
                0,
            ],
            episode: 3,
        },
        switchlist_t {
            name1: [0 as std::ffi::c_char, 0, 0, 0, 0, 0, 0, 0, 0],
            name2: [0 as std::ffi::c_char, 0, 0, 0, 0, 0, 0, 0, 0],
            episode: 0,
        },
    ]
};

pub static mut switchlist: [std::ffi::c_int; (MAXSWITCHES * 2) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut numswitches: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut buttonlist: [button_t; (MAXBUTTONS) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn P_InitSwitchList() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut index: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut episode: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        episode = 1;
        // TODO: if statement not yet translated:
        //
        //
        //     if (gamemode == registered)
        // 	episode = 2;
        //     else
        // 	if ( gamemode == commercial )
        // 	    episode = 3;
        todo!("if statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //
        //     for (index = 0,i = 0;i < MAXSWITCHES;i++)
        //     {
        // 	if (!alphSwitchList[i].episode)
        // 	{
        // 	    numswitches = index/2;
        // 	    switchlist[index] = -1;
        // 	    break;
        // 	}
        //
        // 	if (alphSwitchList[i].episode <= episode)
        // 	{
        // #if 0	// UNUSED - debug?
        // 	    int		value;
        //
        // 	    if (R_CheckTextureNumForName(alphSwitchList[i].name1) < 0)
        // 	    {
        // 		I_Error("Can't find switch texture '%s'!",
        // 			alphSwitchList[i].name1);
        // 		continue;
        // 	    }
        //
        // 	    value = R_TextureNumForName(alphSwitchList[i].name1);
        // #endif
        // 	    switchlist[index++] = R_TextureNumForName(alphSwitchList[i].name1);
        // 	    switchlist[index++] = R_TextureNumForName(alphSwitchList[i].name2);
        // 	}
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_StartButton(
    mut line: *mut line_t,
    mut w: bwhere_e,
    mut texture: std::ffi::c_int,
    mut time: std::ffi::c_int,
) {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: for statement not yet translated:
        //
        //
        //     // See if button is already pressed
        //     for (i = 0;i < MAXBUTTONS;i++)
        //     {
        // 	if (buttonlist[i].btimer
        // 	    && buttonlist[i].line == line)
        // 	{
        //
        // 	    return;
        // 	}
        //     }
        todo!("for statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //
        //
        //
        //     for (i = 0;i < MAXBUTTONS;i++)
        //     {
        // 	if (!buttonlist[i].btimer)
        // 	{
        // 	    buttonlist[i].line = line;
        // 	    buttonlist[i].where = w;
        // 	    buttonlist[i].btexture = texture;
        // 	    buttonlist[i].btimer = time;
        // 	    buttonlist[i].soundorg = (mobj_t *)&line->frontsector->soundorg;
        // 	    return;
        // 	}
        //     }
        todo!("for statement not yet translated");
        I_Error((c"P_StartButton: no button slots left!").as_ptr());
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_ChangeSwitchTexture(
    mut line: *mut line_t,
    mut useAgain: std::ffi::c_int,
) {
    unsafe {
        let mut texTop: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut texMid: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut texBot: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut sound: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (!useAgain)
        // 	line->special = 0;
        todo!("if statement not yet translated");
        texTop = sides[((*line).sidenum[(0) as usize]) as usize].toptexture;
        texMid = sides[((*line).sidenum[(0) as usize]) as usize].midtexture;
        texBot = sides[((*line).sidenum[(0) as usize]) as usize].bottomtexture;
        sound = sfx_swtchn;
        // TODO: if statement not yet translated:
        //
        //
        //     // EXIT SWITCH?
        //     if (line->special == 11)
        // 	sound = sfx_swtchx;
        todo!("if statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //
        //     for (i = 0;i < numswitches*2;i++)
        //     {
        // 	if (switchlist[i] == texTop)
        // 	{
        // 	    S_StartSound(buttonlist->soundorg,sound);
        // 	    sides[line->sidenum[0]].toptexture = switchlist[i^1];
        //
        // 	    if (useAgain)
        // 		P_StartButton(line,top,switchlist[i],BUTTONTIME);
        //
        // 	    return;
        // 	}
        // 	else
        // 	{
        // 	    if (switchlist[i] == texMid)
        // 	    {
        // 		S_StartSound(buttonlist->soundorg,sound);
        // 		sides[line->sidenum[0]].midtexture = switchlist[i^1];
        //
        // 		if (useAgain)
        // 		    P_StartButton(line, middle,switchlist[i],BUTTONTIME);
        //
        // 		return;
        // 	    }
        // 	    else
        // 	    {
        // 		if (switchlist[i] == texBot)
        // 		{
        // 		    S_StartSound(buttonlist->soundorg,sound);
        // 		    sides[line->sidenum[0]].bottomtexture = switchlist[i^1];
        //
        // 		    if (useAgain)
        // 			P_StartButton(line, bottom,switchlist[i],BUTTONTIME);
        //
        // 		    return;
        // 		}
        // 	    }
        // 	}
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn P_UseSpecialLine(
    mut thing: *mut mobj_t,
    mut line: *mut line_t,
    mut side: std::ffi::c_int,
) -> boolean {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //
        //     // Err...
        //     // Use the back sides of VERY SPECIAL lines...
        //     if (side)
        //     {
        // 	switch(line->special)
        // 	{
        // 	  case 124:
        // 	    // Sliding door open&close
        // 	    // UNUSED?
        // 	    break;
        //
        // 	  default:
        // 	    return false;
        // 	    break;
        // 	}
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //
        //     // Switches that other things can activate.
        //     if (!thing->player)
        //     {
        // 	// never open secret doors
        // 	if (line->flags & ML_SECRET)
        // 	    return false;
        //
        // 	switch(line->special)
        // 	{
        // 	  case 1: 	// MANUAL DOOR RAISE
        // 	  case 32:	// MANUAL BLUE
        // 	  case 33:	// MANUAL RED
        // 	  case 34:	// MANUAL YELLOW
        // 	    break;
        //
        // 	  default:
        // 	    return false;
        // 	    break;
        // 	}
        //     }
        todo!("if statement not yet translated");
        // TODO: switch statement not yet translated:
        //
        //
        //
        //     // do something
        //     switch (line->special)
        //     {
        // 	// MANUALS
        //       case 1:		// Vertical Door
        //       case 26:		// Blue Door/Locked
        //       case 27:		// Yellow Door /Locked
        //       case 28:		// Red Door /Locked
        //
        //       case 31:		// Manual door open
        //       case 32:		// Blue locked door open
        //       case 33:		// Red locked door open
        //       case 34:		// Yellow locked door open
        //
        //       case 117:		// Blazing door raise
        //       case 118:		// Blazing door open
        // 	EV_VerticalDoor (line, thing);
        // 	break;
        //
        // 	//UNUSED - Door Slide Open&Close
        // 	// case 124:
        // 	// EV_SlidingDoor (line, thing);
        // 	// break;
        //
        // 	// SWITCHES
        //       case 7:
        // 	// Build Stairs
        // 	if (EV_BuildStairs(line,build8))
        // 	    P_ChangeSwitchTexture(line,0);
        // 	break;
        //
        //       case 9:
        // 	// Change Donut
        // 	if (EV_DoDonut(line))
        // 	    P_ChangeSwitchTexture(line,0);
        // 	break;
        //
        //       case 11:
        // 	// Exit level
        // 	P_ChangeSwitchTexture(line,0);
        // 	G_ExitLevel ();
        // 	break;
        //
        //       case 14:
        // 	// Raise Floor 32 and change texture
        // 	if (EV_DoPlat(line,raiseAndChange,32))
        // 	    P_ChangeSwitchTexture(line,0);
        // 	break;
        //
        //       case 15:
        // 	// Raise Floor 24 and change texture
        // 	if (EV_DoPlat(line,raiseAndChange,24))
        // 	    P_ChangeSwitchTexture(line,0);
        // 	break;
        //
        //       case 18:
        // 	// Raise Floor to next highest floor
        // 	if (EV_DoFloor(line, raiseFloorToNearest))
        // 	    P_ChangeSwitchTexture(line,0);
        // 	break;
        //
        //       case 20:
        // 	// Raise Plat next highest floor and change texture
        // 	if (EV_DoPlat(line,raiseToNearestAndChange,0))
        // 	    P_ChangeSwitchTexture(line,0);
        // 	break;
        //
        //       case 21:
        // 	// PlatDownWaitUpStay
        // 	if (EV_DoPlat(line,downWaitUpStay,0))
        // 	    P_ChangeSwitchTexture(line,0);
        // 	break;
        //
        //       case 23:
        // 	// Lower Floor to Lowest
        // 	if (EV_DoFloor(line,lowerFloorToLowest))
        // 	    P_ChangeSwitchTexture(line,0);
        // 	break;
        //
        //       case 29:
        // 	// Raise Door
        // 	if (EV_DoDoor(line,normal))
        // 	    P_ChangeSwitchTexture(line,0);
        // 	break;
        //
        //       case 41:
        // 	// Lower Ceiling to Floor
        // 	if (EV_DoCeiling(line,lowerToFloor))
        // 	    P_ChangeSwitchTexture(line,0);
        // 	break;
        //
        //       case 71:
        // 	// Turbo Lower Floor
        // 	if (EV_DoFloor(line,turboLower))
        // 	    P_ChangeSwitchTexture(line,0);
        // 	break;
        //
        //       case 49:
        // 	// Ceiling Crush And Raise
        // 	if (EV_DoCeiling(line,crushAndRaise))
        // 	    P_ChangeSwitchTexture(line,0);
        // 	break;
        //
        //       case 50:
        // 	// Close Door
        // 	if (EV_DoDoor(line,close))
        // 	    P_ChangeSwitchTexture(line,0);
        // 	break;
        //
        //       case 51:
        // 	// Secret EXIT
        // 	P_ChangeSwitchTexture(line,0);
        // 	G_SecretExitLevel ();
        // 	break;
        //
        //       case 55:
        // 	// Raise Floor Crush
        // 	if (EV_DoFloor(line,raiseFloorCrush))
        // 	    P_ChangeSwitchTexture(line,0);
        // 	break;
        //
        //       case 101:
        // 	// Raise Floor
        // 	if (EV_DoFloor(line,raiseFloor))
        // 	    P_ChangeSwitchTexture(line,0);
        // 	break;
        //
        //       case 102:
        // 	// Lower Floor to Surrounding floor height
        // 	if (EV_DoFloor(line,lowerFloor))
        // 	    P_ChangeSwitchTexture(line,0);
        // 	break;
        //
        //       case 103:
        // 	// Open Door
        // 	if (EV_DoDoor(line,open))
        // 	    P_ChangeSwitchTexture(line,0);
        // 	break;
        //
        //       case 111:
        // 	// Blazing Door Raise (faster than TURBO!)
        // 	if (EV_DoDoor (line,blazeRaise))
        // 	    P_ChangeSwitchTexture(line,0);
        // 	break;
        //
        //       case 112:
        // 	// Blazing Door Open (faster than TURBO!)
        // 	if (EV_DoDoor (line,blazeOpen))
        // 	    P_ChangeSwitchTexture(line,0);
        // 	break;
        //
        //       case 113:
        // 	// Blazing Door Close (faster than TURBO!)
        // 	if (EV_DoDoor (line,blazeClose))
        // 	    P_ChangeSwitchTexture(line,0);
        // 	break;
        //
        //       case 122:
        // 	// Blazing PlatDownWaitUpStay
        // 	if (EV_DoPlat(line,blazeDWUS,0))
        // 	    P_ChangeSwitchTexture(line,0);
        // 	break;
        //
        //       case 127:
        // 	// Build Stairs Turbo 16
        // 	if (EV_BuildStairs(line,turbo16))
        // 	    P_ChangeSwitchTexture(line,0);
        // 	break;
        //
        //       case 131:
        // 	// Raise Floor Turbo
        // 	if (EV_DoFloor(line,raiseFloorTurbo))
        // 	    P_ChangeSwitchTexture(line,0);
        // 	break;
        //
        //       case 133:
        // 	// BlzOpenDoor BLUE
        //       case 135:
        // 	// BlzOpenDoor RED
        //       case 137:
        // 	// BlzOpenDoor YELLOW
        // 	if (EV_DoLockedDoor (line,blazeOpen,thing))
        // 	    P_ChangeSwitchTexture(line,0);
        // 	break;
        //
        //       case 140:
        // 	// Raise Floor 512
        // 	if (EV_DoFloor(line,raiseFloor512))
        // 	    P_ChangeSwitchTexture(line,0);
        // 	break;
        //
        // 	// BUTTONS
        //       case 42:
        // 	// Close Door
        // 	if (EV_DoDoor(line,close))
        // 	    P_ChangeSwitchTexture(line,1);
        // 	break;
        //
        //       case 43:
        // 	// Lower Ceiling to Floor
        // 	if (EV_DoCeiling(line,lowerToFloor))
        // 	    P_ChangeSwitchTexture(line,1);
        // 	break;
        //
        //       case 45:
        // 	// Lower Floor to Surrounding floor height
        // 	if (EV_DoFloor(line,lowerFloor))
        // 	    P_ChangeSwitchTexture(line,1);
        // 	break;
        //
        //       case 60:
        // 	// Lower Floor to Lowest
        // 	if (EV_DoFloor(line,lowerFloorToLowest))
        // 	    P_ChangeSwitchTexture(line,1);
        // 	break;
        //
        //       case 61:
        // 	// Open Door
        // 	if (EV_DoDoor(line,open))
        // 	    P_ChangeSwitchTexture(line,1);
        // 	break;
        //
        //       case 62:
        // 	// PlatDownWaitUpStay
        // 	if (EV_DoPlat(line,downWaitUpStay,1))
        // 	    P_ChangeSwitchTexture(line,1);
        // 	break;
        //
        //       case 63:
        // 	// Raise Door
        // 	if (EV_DoDoor(line,normal))
        // 	    P_ChangeSwitchTexture(line,1);
        // 	break;
        //
        //       case 64:
        // 	// Raise Floor to ceiling
        // 	if (EV_DoFloor(line,raiseFloor))
        // 	    P_ChangeSwitchTexture(line,1);
        // 	break;
        //
        //       case 66:
        // 	// Raise Floor 24 and change texture
        // 	if (EV_DoPlat(line,raiseAndChange,24))
        // 	    P_ChangeSwitchTexture(line,1);
        // 	break;
        //
        //       case 67:
        // 	// Raise Floor 32 and change texture
        // 	if (EV_DoPlat(line,raiseAndChange,32))
        // 	    P_ChangeSwitchTexture(line,1);
        // 	break;
        //
        //       case 65:
        // 	// Raise Floor Crush
        // 	if (EV_DoFloor(line,raiseFloorCrush))
        // 	    P_ChangeSwitchTexture(line,1);
        // 	break;
        //
        //       case 68:
        // 	// Raise Plat to next highest floor and change texture
        // 	if (EV_DoPlat(line,raiseToNearestAndChange,0))
        // 	    P_ChangeSwitchTexture(line,1);
        // 	break;
        //
        //       case 69:
        // 	// Raise Floor to next highest floor
        // 	if (EV_DoFloor(line, raiseFloorToNearest))
        // 	    P_ChangeSwitchTexture(line,1);
        // 	break;
        //
        //       case 70:
        // 	// Turbo Lower Floor
        // 	if (EV_DoFloor(line,turboLower))
        // 	    P_ChangeSwitchTexture(line,1);
        // 	break;
        //
        //       case 114:
        // 	// Blazing Door Raise (faster than TURBO!)
        // 	if (EV_DoDoor (line,blazeRaise))
        // 	    P_ChangeSwitchTexture(line,1);
        // 	break;
        //
        //       case 115:
        // 	// Blazing Door Open (faster than TURBO!)
        // 	if (EV_DoDoor (line,blazeOpen))
        // 	    P_ChangeSwitchTexture(line,1);
        // 	break;
        //
        //       case 116:
        // 	// Blazing Door Close (faster than TURBO!)
        // 	if (EV_DoDoor (line,blazeClose))
        // 	    P_ChangeSwitchTexture(line,1);
        // 	break;
        //
        //       case 123:
        // 	// Blazing PlatDownWaitUpStay
        // 	if (EV_DoPlat(line,blazeDWUS,0))
        // 	    P_ChangeSwitchTexture(line,1);
        // 	break;
        //
        //       case 132:
        // 	// Raise Floor Turbo
        // 	if (EV_DoFloor(line,raiseFloorTurbo))
        // 	    P_ChangeSwitchTexture(line,1);
        // 	break;
        //
        //       case 99:
        // 	// BlzOpenDoor BLUE
        //       case 134:
        // 	// BlzOpenDoor RED
        //       case 136:
        // 	// BlzOpenDoor YELLOW
        // 	if (EV_DoLockedDoor (line,blazeOpen,thing))
        // 	    P_ChangeSwitchTexture(line,1);
        // 	break;
        //
        //       case 138:
        // 	// Light Turn On
        // 	EV_LightTurnOn(line,255);
        // 	P_ChangeSwitchTexture(line,1);
        // 	break;
        //
        //       case 139:
        // 	// Light Turn Off
        // 	EV_LightTurnOn(line,35);
        // 	P_ChangeSwitchTexture(line,1);
        // 	break;
        //
        //     }
        todo!("switch statement not yet translated");
        return true_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}
