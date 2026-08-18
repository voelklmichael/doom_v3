use crate::d_event::*;
use crate::d_items::*;
use crate::d_player::*;
use crate::d_think::*;
use crate::d_ticcmd::*;
use crate::doomdata::*;
use crate::doomdef::*;
use crate::doomstat::*;
use crate::doomtype::*;
use crate::g_game::*;
use crate::i_net::*;
use crate::i_system::*;
use crate::i_video::*;
use crate::info::*;
use crate::m_fixed::*;
use crate::m_menu::*;
use crate::p_mobj::*;
use crate::p_pspr::*;
use crate::tables::*;

pub const DOOMCOM_ID: std::ffi::c_int = 0x12345678;

pub const MAXNETNODES: std::ffi::c_int = 8;

pub const BACKUPTICS: std::ffi::c_int = 12;

pub const CMD_SEND: std::ffi::c_int = 1;
pub const CMD_GET: std::ffi::c_int = 2;

pub type command_t = std::ffi::c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct doomdata_t {
    pub checksum: std::ffi::c_uint,
    pub retransmitfrom: byte,
    pub starttic: byte,
    pub player: byte,
    pub numtics: byte,
    pub cmds: [ticcmd_t; (BACKUPTICS) as usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct doomcom_t {
    pub id: std::ffi::c_long,
    pub intnum: std::ffi::c_short,
    pub command: std::ffi::c_short,
    pub remotenode: std::ffi::c_short,
    pub datalength: std::ffi::c_short,
    pub numnodes: std::ffi::c_short,
    pub ticdup: std::ffi::c_short,
    pub extratics: std::ffi::c_short,
    pub deathmatch: std::ffi::c_short,
    pub savegame: std::ffi::c_short,
    pub episode: std::ffi::c_short,
    pub map: std::ffi::c_short,
    pub skill: std::ffi::c_short,
    pub consoleplayer: std::ffi::c_short,
    pub numplayers: std::ffi::c_short,
    pub angleoffset: std::ffi::c_short,
    pub drone: std::ffi::c_short,
    pub data: doomdata_t,
}

/* TODO: unparsed C construct, needs manual translation:

//-----------------------------------------------------------------------------
//
// $Log:$
//
//-----------------------------------------------------------------------------


*/

static mut rcsid: [std::ffi::c_char; 48] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        100 as std::ffi::c_char,
        95 as std::ffi::c_char,
        110 as std::ffi::c_char,
        101 as std::ffi::c_char,
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
        50 as std::ffi::c_char,
        47 as std::ffi::c_char,
        48 as std::ffi::c_char,
        51 as std::ffi::c_char,
        32 as std::ffi::c_char,
        50 as std::ffi::c_char,
        50 as std::ffi::c_char,
        58 as std::ffi::c_char,
        48 as std::ffi::c_char,
        49 as std::ffi::c_char,
        58 as std::ffi::c_char,
        52 as std::ffi::c_char,
        55 as std::ffi::c_char,
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

pub const NCMD_EXIT: std::ffi::c_int = 0x80000000;

pub const NCMD_RETRANSMIT: std::ffi::c_int = 0x40000000;

pub const NCMD_SETUP: std::ffi::c_int = 0x20000000;

pub const NCMD_KILL: std::ffi::c_int = 0x10000000;

pub const NCMD_CHECKSUM: std::ffi::c_int = 0x0fffffff;

pub static mut doomcom: *mut doomcom_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut netbuffer: *mut doomdata_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub const RESENDCOUNT: std::ffi::c_int = 10;

pub const PL_DRONE: std::ffi::c_int = 0x80;

pub static mut localcmds: [ticcmd_t; (BACKUPTICS) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut netcmds: [[ticcmd_t; (BACKUPTICS) as usize]; (MAXPLAYERS) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut nettics: [std::ffi::c_int; (MAXNETNODES) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut nodeingame: [boolean; (MAXNETNODES) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut remoteresend: [boolean; (MAXNETNODES) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut resendto: [std::ffi::c_int; (MAXNETNODES) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut resendcount: [std::ffi::c_int; (MAXNETNODES) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut nodeforplayer: [std::ffi::c_int; (MAXPLAYERS) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut maketic: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut lastnettic: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut skiptics: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut ticdup: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut maxsend: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

unsafe extern "C" {
    pub fn D_ProcessEvents();
}

unsafe extern "C" {
    pub fn G_BuildTiccmd(cmd: *mut ticcmd_t);
}

unsafe extern "C" {
    pub fn D_DoAdvanceDemo();
}

pub static mut reboundpacket: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut reboundstore: doomdata_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn NetbufferSize() -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn NetbufferChecksum() -> std::ffi::c_uint {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn ExpandTics(low: std::ffi::c_int) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn HSendPacket(node: std::ffi::c_int, flags: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn HGetPacket() -> boolean {
    todo!("body not yet translated")
}

pub static mut exitmsg: [std::ffi::c_char; (80) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn GetPackets() {
    todo!("body not yet translated")
}

pub static mut gametime: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn NetUpdate() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn CheckAbort() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn D_ArbitrateNetStart() {
    todo!("body not yet translated")
}

unsafe extern "C" {
    pub static mut viewangleoffset: std::ffi::c_int;
}

pub unsafe extern "C" fn D_CheckNetGame() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn D_QuitNetGame() {
    todo!("body not yet translated")
}

pub static mut frametics: [std::ffi::c_int; (4) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut frameon: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut frameskip: [std::ffi::c_int; (4) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut oldnettics: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

unsafe extern "C" {
    pub static mut advancedemo: boolean;
}

pub unsafe extern "C" fn TryRunTics() {
    todo!("body not yet translated")
}
