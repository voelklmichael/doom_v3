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
use crate::m_misc::*;
use crate::p_mobj::*;
use crate::p_pspr::*;
use crate::sounds::*;
use crate::tables::*;
use crate::w_wad::*;
use crate::z_zone::*;

static mut rcsid: [std::ffi::c_char; 49] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        105 as std::ffi::c_char,
        95 as std::ffi::c_char,
        117 as std::ffi::c_char,
        110 as std::ffi::c_char,
        105 as std::ffi::c_char,
        120 as std::ffi::c_char,
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

pub static mut sndserver: *mut FILE = unsafe { std::ptr::null_mut() };

pub static mut sndserver_filename: *mut std::ffi::c_char =
    unsafe { (c"./sndserver ").as_ptr() as *mut std::ffi::c_char };

static mut flag: std::ffi::c_int = unsafe { 0 };

pub const SAMPLECOUNT: std::ffi::c_int = 512;

pub const NUM_CHANNELS: std::ffi::c_int = 8;

pub const BUFMUL: std::ffi::c_int = 4;

pub const MIXBUFFERSIZE: std::ffi::c_int = (SAMPLECOUNT * BUFMUL);

pub const SAMPLERATE: std::ffi::c_int = 11025;

pub const SAMPLESIZE: std::ffi::c_int = 2;

pub static mut lengths: [std::ffi::c_int; (NUMSFX) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut audio_fd: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut mixbuffer: [std::ffi::c_short; (MIXBUFFERSIZE) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut channelstep: [std::ffi::c_uint; (NUM_CHANNELS) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut channelstepremainder: [std::ffi::c_uint; (NUM_CHANNELS) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut channels: [*mut std::ffi::c_uchar; (NUM_CHANNELS) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut channelsend: [*mut std::ffi::c_uchar; (NUM_CHANNELS) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut channelstart: [std::ffi::c_int; (NUM_CHANNELS) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut channelhandles: [std::ffi::c_int; (NUM_CHANNELS) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut channelids: [std::ffi::c_int; (NUM_CHANNELS) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut steptable: [std::ffi::c_int; (256) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut vol_lookup: [std::ffi::c_int; (128 * 256) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut channelleftvol_lookup: [*mut std::ffi::c_int; (NUM_CHANNELS) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut channelrightvol_lookup: [*mut std::ffi::c_int; (NUM_CHANNELS) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn myioctl(
    fd: std::ffi::c_int,
    command: std::ffi::c_int,
    arg: *mut std::ffi::c_int,
) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn getsfx(
    sfxname: *mut std::ffi::c_char,
    len: *mut std::ffi::c_int,
) -> *mut std::ffi::c_void {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn addsfx(
    sfxid: std::ffi::c_int,
    volume: std::ffi::c_int,
    step: std::ffi::c_int,
    seperation: std::ffi::c_int,
) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn I_SetChannels() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn I_SetSfxVolume(volume: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn I_SetMusicVolume(volume: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn I_GetSfxLumpNum(sfx: *mut sfxinfo_t) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn I_StartSound(
    id: std::ffi::c_int,
    vol: std::ffi::c_int,
    sep: std::ffi::c_int,
    pitch: std::ffi::c_int,
    priority: std::ffi::c_int,
) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn I_StopSound(handle: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn I_SoundIsPlaying(handle: std::ffi::c_int) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn I_UpdateSound() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn I_SubmitSound() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn I_UpdateSoundParams(
    handle: std::ffi::c_int,
    vol: std::ffi::c_int,
    sep: std::ffi::c_int,
    pitch: std::ffi::c_int,
) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn I_ShutdownSound() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn I_InitSound() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn I_InitMusic() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn I_ShutdownMusic() {
    todo!("body not yet translated")
}

static mut looping: std::ffi::c_int = unsafe { 0 };

static mut musicdies: std::ffi::c_int = unsafe { (-(1)) };

pub unsafe extern "C" fn I_PlaySong(handle: std::ffi::c_int, looping: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn I_PauseSong(handle: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn I_ResumeSong(handle: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn I_StopSong(handle: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn I_UnRegisterSong(handle: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn I_RegisterSong(data: *mut std::ffi::c_void) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn I_QrySongPlaying(handle: std::ffi::c_int) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub type tSigSet = std::ffi::c_int;

static mut itimer: std::ffi::c_int = unsafe { ITIMER_REAL };

static mut sig: std::ffi::c_int = unsafe { SIGALRM };

pub unsafe extern "C" fn I_HandleSoundTimer(ignore: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn I_SoundSetTimer(duration_of_tick: std::ffi::c_int) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn I_SoundDelTimer() {
    todo!("body not yet translated")
}
