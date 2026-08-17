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
use crate::i_sound::*;
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
use crate::sounds::*;
use crate::tables::*;
use crate::w_wad::*;
use crate::z_zone::*;

static mut rcsid: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut snd_prefixen: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub const S_MAX_VOLUME: std::ffi::c_int = 127;

pub const S_CLIPPING_DIST: std::ffi::c_int = (1200 * 0x10000);

pub const S_CLOSE_DIST: std::ffi::c_int = (160 * 0x10000);

pub const S_ATTENUATOR: std::ffi::c_int = ((S_CLIPPING_DIST - S_CLOSE_DIST) >> FRACBITS);

pub const NORM_VOLUME: std::ffi::c_int = snd_MaxVolume;

pub const NORM_PITCH: std::ffi::c_int = 128;

pub const NORM_PRIORITY: std::ffi::c_int = 64;

pub const NORM_SEP: std::ffi::c_int = 128;

pub const S_PITCH_PERTURB: std::ffi::c_int = 1;

pub const S_STEREO_SWING: std::ffi::c_int = (96 * 0x10000);

pub const S_IFRACVOL: std::ffi::c_int = 30;

pub const NA: std::ffi::c_int = 0;

pub const S_NUMCHANNELS: std::ffi::c_int = 2;

unsafe extern "C" {
    pub static mut snd_MusicDevice: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut snd_SfxDevice: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut snd_DesiredMusicDevice: std::ffi::c_int;
}

unsafe extern "C" {
    pub static mut snd_DesiredSfxDevice: std::ffi::c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct channel_t {
    pub sfxinfo: *mut sfxinfo_t,
    pub origin: *mut std::ffi::c_void,
    pub handle: std::ffi::c_int,
}

static mut channels: *mut channel_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut snd_SfxVolume: std::ffi::c_int = unsafe { 15 };

pub static mut snd_MusicVolume: std::ffi::c_int = unsafe { 15 };

static mut mus_paused: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut mus_playing: *mut musicinfo_t = unsafe { std::ptr::null_mut() };

pub static mut numChannels: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

static mut nextcleanup: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn S_Init(sfxVolume: std::ffi::c_int, musicVolume: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn S_Start() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn S_StartSoundAtVolume(
    origin_p: *mut std::ffi::c_void,
    sfx_id: std::ffi::c_int,
    volume: std::ffi::c_int,
) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn S_StartSound(origin: *mut std::ffi::c_void, sfx_id: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn S_StopSound(origin: *mut std::ffi::c_void) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn S_PauseSound() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn S_ResumeSound() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn S_UpdateSounds(listener_p: *mut std::ffi::c_void) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn S_SetMusicVolume(volume: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn S_SetSfxVolume(volume: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn S_StartMusic(m_id: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn S_ChangeMusic(musicnum: std::ffi::c_int, looping: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn S_StopMusic() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn S_StopChannel(cnum: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn S_AdjustSoundParams(
    listener: *mut mobj_t,
    source: *mut mobj_t,
    vol: *mut std::ffi::c_int,
    sep: *mut std::ffi::c_int,
    pitch: *mut std::ffi::c_int,
) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn S_getChannel(
    origin: *mut std::ffi::c_void,
    sfxinfo: *mut sfxinfo_t,
) -> std::ffi::c_int {
    todo!("body not yet translated")
}
