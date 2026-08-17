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

static mut rcsid: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

unsafe extern "C" {
    pub fn NetSend();
}

unsafe extern "C" {
    pub fn NetListen() -> boolean;
}

pub static mut DOOMPORT: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut sendsocket: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut insocket: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut sendaddress: [sockaddr_in; (MAXNETNODES) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut netget: Option<unsafe extern "C" fn()> = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut netsend: Option<unsafe extern "C" fn()> = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn UDPsocket() -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn BindToLocalPort(s: std::ffi::c_int, port: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn PacketSend() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn PacketGet() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn GetLocalAddress() -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn I_InitNetwork() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn I_NetCmd() {
    todo!("body not yet translated")
}
