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
use crate::m_argv::*;
use crate::m_fixed::*;
use crate::p_mobj::*;
use crate::p_pspr::*;
use crate::tables::*;

static mut rcsid: [std::ffi::c_char; 49] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        109 as std::ffi::c_char,
        95 as std::ffi::c_char,
        98 as std::ffi::c_char,
        98 as std::ffi::c_char,
        111 as std::ffi::c_char,
        120 as std::ffi::c_char,
        46 as std::ffi::c_char,
        99 as std::ffi::c_char,
        44 as std::ffi::c_char,
        118 as std::ffi::c_char,
        32 as std::ffi::c_char,
        49 as std::ffi::c_char,
        46 as std::ffi::c_char,
        49 as std::ffi::c_char,
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
        48 as std::ffi::c_char,
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

pub unsafe extern "C" fn ntohl(x: std::ffi::c_int) -> std::ffi::c_ulong {
    ((((((((x) as std::ffi::c_ulong) & 0x000000ff) << 24)
        | ((((x) as std::ffi::c_ulong) & 0x0000ff00) << 8))
        | ((((x) as std::ffi::c_ulong) & 0x00ff0000) >> 8))
        | ((((x) as std::ffi::c_ulong) & 0xff000000) >> 24)) as std::ffi::c_ulong)
}

pub unsafe extern "C" fn ntohs(x: std::ffi::c_int) -> std::ffi::c_ushort {
    ((((((x) as std::ffi::c_ushort) & 0x00ff) << 8) | ((((x) as std::ffi::c_ushort) & 0xff00) >> 8))
        as std::ffi::c_ushort)
}

pub unsafe extern "C" fn htonl(x: std::ffi::c_int) -> std::ffi::c_int {
    ntohl(x)
}

pub unsafe extern "C" fn htons(x: std::ffi::c_int) -> std::ffi::c_int {
    ntohs(x)
}

unsafe extern "C" {
    pub fn NetSend();
}

unsafe extern "C" {
    pub fn NetListen() -> boolean;
}

pub static mut DOOMPORT: std::ffi::c_int = unsafe { (5000 + 0x1d) };

pub static mut sendsocket: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut insocket: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut sendaddress: [libc::sockaddr_in; (MAXNETNODES) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut netget: Option<unsafe extern "C" fn()> = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut netsend: Option<unsafe extern "C" fn()> = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn UDPsocket() -> std::ffi::c_int {
    unsafe { todo!("body not yet translated") }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn BindToLocalPort(s: std::ffi::c_int, port: std::ffi::c_int) {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn PacketSend() {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn PacketGet() {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn GetLocalAddress() -> std::ffi::c_int {
    unsafe { todo!("body not yet translated") }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn I_InitNetwork() {
    unsafe { todo!("body not yet translated") }
}

pub unsafe extern "C" fn I_NetCmd() {
    unsafe { todo!("body not yet translated") }
}
