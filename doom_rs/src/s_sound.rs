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

static mut rcsid: [std::ffi::c_char; 50] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        115 as std::ffi::c_char,
        95 as std::ffi::c_char,
        115 as std::ffi::c_char,
        111 as std::ffi::c_char,
        117 as std::ffi::c_char,
        110 as std::ffi::c_char,
        100 as std::ffi::c_char,
        46 as std::ffi::c_char,
        99 as std::ffi::c_char,
        44 as std::ffi::c_char,
        118 as std::ffi::c_char,
        32 as std::ffi::c_char,
        49 as std::ffi::c_char,
        46 as std::ffi::c_char,
        54 as std::ffi::c_char,
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

pub static mut snd_prefixen: [std::ffi::c_char; 12] = unsafe {
    [
        (b'P' as std::ffi::c_int),
        (b'P' as std::ffi::c_int),
        (b'A' as std::ffi::c_int),
        (b'S' as std::ffi::c_int),
        (b'S' as std::ffi::c_int),
        (b'S' as std::ffi::c_int),
        (b'M' as std::ffi::c_int),
        (b'M' as std::ffi::c_int),
        (b'M' as std::ffi::c_int),
        (b'S' as std::ffi::c_int),
        (b'S' as std::ffi::c_int),
        (b'S' as std::ffi::c_int),
    ]
};

pub const S_MAX_VOLUME: std::ffi::c_int = 127;

pub const S_CLIPPING_DIST: std::ffi::c_int = (1200 * 0x10000);

pub const S_CLOSE_DIST: std::ffi::c_int = (160 * 0x10000);

pub const S_ATTENUATOR: std::ffi::c_int = ((S_CLIPPING_DIST - S_CLOSE_DIST) >> FRACBITS);

/* TODO: unparsed macro value, references an identifier with no known definition anywhere in this module's visible corpus (likely dead code never expanded in the original C):
#define NORM_VOLUME snd_MaxVolume
*/

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

pub unsafe extern "C" fn S_Init(mut sfxVolume: std::ffi::c_int, mut musicVolume: std::ffi::c_int) {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        fprintf(
            stderr,
            (c"S_Init: default sfx volume %d\n").as_ptr(),
            sfxVolume,
        );
        I_SetChannels();
        S_SetSfxVolume(sfxVolume);
        S_SetMusicVolume(musicVolume);
        channels = ((Z_Malloc(
            (numChannels * std::mem::size_of::<channel_t>()),
            PU_STATIC,
            0,
        )) as *mut channel_t);
        // TODO: for statement not yet translated:
        //
        //
        //   // Free all channels for use
        //   for (i=0 ; i<numChannels ; i++)
        //     channels[i].sfxinfo = 0;
        todo!("for statement not yet translated");
        mus_paused = 0;
        // TODO: for statement not yet translated:
        //
        //
        //   // Note that sounds have not been cached (yet).
        //   for (i=1 ; i<NUMSFX ; i++)
        //     S_sfx[i].lumpnum = S_sfx[i].usefulness = -1;
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn S_Start() {
    unsafe {
        let mut cnum: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut mnum: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: for statement not yet translated:
        //
        //
        //   // kill all playing sounds at start of level
        //   //  (trust me - a good idea)
        //   for (cnum=0 ; cnum<numChannels ; cnum++)
        //     if (channels[cnum].sfxinfo)
        //       S_StopChannel(cnum);
        todo!("for statement not yet translated");
        mus_paused = 0;
        // TODO: if statement not yet translated:
        //
        //
        //   if (gamemode == commercial)
        //     mnum = mus_runnin + gamemap - 1;
        //   else
        //   {
        //     int spmus[]=
        //     {
        //       // Song - Who? - Where?
        //
        //       mus_e3m4,	// American	e4m1
        //       mus_e3m2,	// Romero	e4m2
        //       mus_e3m3,	// Shawn	e4m3
        //       mus_e1m5,	// American	e4m4
        //       mus_e2m7,	// Tim 	e4m5
        //       mus_e2m4,	// Romero	e4m6
        //       mus_e2m6,	// J.Anderson	e4m7 CHIRON.WAD
        //       mus_e2m5,	// Shawn	e4m8
        //       mus_e1m9	// Tim		e4m9
        //     };
        //
        //     if (gameepisode < 4)
        //       mnum = mus_e1m1 + (gameepisode-1)*9 + gamemap-1;
        //     else
        //       mnum = spmus[gamemap-1];
        //     }
        todo!("if statement not yet translated");
        S_ChangeMusic(mnum, true_);
        nextcleanup = 15;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn S_StartSoundAtVolume(
    mut origin_p: *mut std::ffi::c_void,
    mut sfx_id: std::ffi::c_int,
    mut volume: std::ffi::c_int,
) {
    unsafe {
        let mut rc: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut sep: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut pitch: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut priority: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut sfx: *mut sfxinfo_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut cnum: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut origin: *mut mobj_t = unsafe { ((origin_p) as *mut mobj_t) };
        // TODO: if statement not yet translated:
        //
        //
        //
        //   // Debug.
        //   /*fprintf( stderr,
        //   	   "S_StartSoundAtVolume: playing sound %d (%s)\n",
        //   	   sfx_id, S_sfx[sfx_id].name );*/
        //
        //   // check for bogus sound #
        //   if (sfx_id < 1 || sfx_id > NUMSFX)
        //     I_Error("Bad sfx #: %d", sfx_id);
        todo!("if statement not yet translated");
        sfx = S_sfx.add((sfx_id) as usize);
        // TODO: if statement not yet translated:
        //
        //
        //   // Initialize sound parameters
        //   if (sfx->link)
        //   {
        //     pitch = sfx->pitch;
        //     priority = sfx->priority;
        //     volume += sfx->volume;
        //
        //     if (volume < 1)
        //       return;
        //
        //     if (volume > snd_SfxVolume)
        //       volume = snd_SfxVolume;
        //   }
        //   else
        //   {
        //     pitch = NORM_PITCH;
        //     priority = NORM_PRIORITY;
        //   }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //
        //   // Check to see if it is audible,
        //   //  and if not, modify the params
        //   if (origin && origin != players[consoleplayer].mo)
        //   {
        //     rc = S_AdjustSoundParams(players[consoleplayer].mo,
        // 			     origin,
        // 			     &volume,
        // 			     &sep,
        // 			     &pitch);
        //
        //     if ( origin->x == players[consoleplayer].mo->x
        // 	 && origin->y == players[consoleplayer].mo->y)
        //     {
        //       sep 	= NORM_SEP;
        //     }
        //
        //     if (!rc)
        //       return;
        //   }
        //   else
        //   {
        //     sep = NORM_SEP;
        //   }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //   // hacks to vary the sfx pitches
        //   if (sfx_id >= sfx_sawup
        //       && sfx_id <= sfx_sawhit)
        //   {
        //     pitch += 8 - (M_Random()&15);
        //
        //     if (pitch<0)
        //       pitch = 0;
        //     else if (pitch>255)
        //       pitch = 255;
        //   }
        //   else if (sfx_id != sfx_itemup
        // 	   && sfx_id != sfx_tink)
        //   {
        //     pitch += 16 - (M_Random()&31);
        //
        //     if (pitch<0)
        //       pitch = 0;
        //     else if (pitch>255)
        //       pitch = 255;
        //   }
        todo!("if statement not yet translated");
        S_StopSound(origin);
        cnum = S_getChannel(origin, sfx);
        // TODO: if statement not yet translated:
        //
        //
        //   if (cnum<0)
        //     return;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //   //
        //   // This is supposed to handle the loading/caching.
        //   // For some odd reason, the caching is done nearly
        //   //  each time the sound is needed?
        //   //
        //
        //   // get lumpnum if necessary
        //   if (sfx->lumpnum < 0)
        //     sfx->lumpnum = I_GetSfxLumpNum(sfx);
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //   if (!sfx->data)
        //   {
        //     fprintf( stderr,
        // 	     "S_StartSoundAtVolume: 16bit and not pre-cached - wtf?\n");
        //
        //     // DOS remains, 8bit handling
        //     //sfx->data = (void *) W_CacheLumpNum(sfx->lumpnum, PU_MUSIC);
        //     // fprintf( stderr,
        //     //	     "S_StartSoundAtVolume: loading %d (lump %d) : 0x%x\n",
        //     //       sfx_id, sfx->lumpnum, (int)sfx->data );
        //
        //   }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //   // increase the usefulness
        //   if (sfx->usefulness++ < 0)
        //     sfx->usefulness = 1;
        todo!("if statement not yet translated");
        channels[(cnum) as usize].handle = I_StartSound(sfx_id, volume, sep, pitch, priority);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn S_StartSound(
    mut origin: *mut std::ffi::c_void,
    mut sfx_id: std::ffi::c_int,
) {
    unsafe {
        S_StartSoundAtVolume(origin, sfx_id, snd_SfxVolume);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn S_StopSound(mut origin: *mut std::ffi::c_void) {
    unsafe {
        let mut cnum: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: for statement not yet translated:
        //
        //
        //     for (cnum=0 ; cnum<numChannels ; cnum++)
        //     {
        // 	if (channels[cnum].sfxinfo && channels[cnum].origin == origin)
        // 	{
        // 	    S_StopChannel(cnum);
        // 	    break;
        // 	}
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn S_PauseSound() {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (mus_playing && !mus_paused)
        //     {
        // 	I_PauseSong(mus_playing->handle);
        // 	mus_paused = true;
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn S_ResumeSound() {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (mus_playing && mus_paused)
        //     {
        // 	I_ResumeSong(mus_playing->handle);
        // 	mus_paused = false;
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn S_UpdateSounds(mut listener_p: *mut std::ffi::c_void) {
    unsafe {
        let mut audible: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut cnum: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut volume: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut sep: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut pitch: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut sfx: *mut sfxinfo_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut c: *mut channel_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut listener: *mut mobj_t = unsafe { ((listener_p) as *mut mobj_t) };
        // TODO: for statement not yet translated:
        //
        //
        //
        //
        //     // Clean up unused data.
        //     // This is currently not done for 16bit (sounds cached static).
        //     // DOS 8bit remains.
        //     /*if (gametic > nextcleanup)
        //     {
        // 	for (i=1 ; i<NUMSFX ; i++)
        // 	{
        // 	    if (S_sfx[i].usefulness < 1
        // 		&& S_sfx[i].usefulness > -1)
        // 	    {
        // 		if (--S_sfx[i].usefulness == -1)
        // 		{
        // 		    Z_ChangeTag(S_sfx[i].data, PU_CACHE);
        // 		    S_sfx[i].data = 0;
        // 		}
        // 	    }
        // 	}
        // 	nextcleanup = gametic + 15;
        //     }*/
        //
        //     for (cnum=0 ; cnum<numChannels ; cnum++)
        //     {
        // 	c = &channels[cnum];
        // 	sfx = c->sfxinfo;
        //
        // 	if (c->sfxinfo)
        // 	{
        // 	    if (I_SoundIsPlaying(c->handle))
        // 	    {
        // 		// initialize parameters
        // 		volume = snd_SfxVolume;
        // 		pitch = NORM_PITCH;
        // 		sep = NORM_SEP;
        //
        // 		if (sfx->link)
        // 		{
        // 		    pitch = sfx->pitch;
        // 		    volume += sfx->volume;
        // 		    if (volume < 1)
        // 		    {
        // 			S_StopChannel(cnum);
        // 			continue;
        // 		    }
        // 		    else if (volume > snd_SfxVolume)
        // 		    {
        // 			volume = snd_SfxVolume;
        // 		    }
        // 		}
        //
        // 		// check non-local sounds for distance clipping
        // 		//  or modify their params
        // 		if (c->origin && listener_p != c->origin)
        // 		{
        // 		    audible = S_AdjustSoundParams(listener,
        // 						  c->origin,
        // 						  &volume,
        // 						  &sep,
        // 						  &pitch);
        //
        // 		    if (!audible)
        // 		    {
        // 			S_StopChannel(cnum);
        // 		    }
        // 		    else
        // 			I_UpdateSoundParams(c->handle, volume, sep, pitch);
        // 		}
        // 	    }
        // 	    else
        // 	    {
        // 		// if channel is allocated but sound has stopped,
        // 		//  free it
        // 		S_StopChannel(cnum);
        // 	    }
        // 	}
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        //     // kill music if it is a single-play && finished
        //     // if (	mus_playing
        //     //      && !I_QrySongPlaying(mus_playing->handle)
        //     //      && !mus_paused )
        //     // S_StopMusic();
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn S_SetMusicVolume(mut volume: std::ffi::c_int) {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (volume < 0 || volume > 127)
        //     {
        // 	I_Error("Attempt to set music volume at %d",
        // 		volume);
        //     }
        todo!("if statement not yet translated");
        I_SetMusicVolume(127);
        I_SetMusicVolume(volume);
        snd_MusicVolume = volume;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn S_SetSfxVolume(mut volume: std::ffi::c_int) {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //
        //     if (volume < 0 || volume > 127)
        // 	I_Error("Attempt to set sfx volume at %d", volume);
        todo!("if statement not yet translated");
        snd_SfxVolume = volume;
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn S_StartMusic(mut m_id: std::ffi::c_int) {
    unsafe {
        S_ChangeMusic(m_id, false_);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn S_ChangeMusic(
    mut musicnum: std::ffi::c_int,
    mut looping: std::ffi::c_int,
) {
    unsafe {
        let mut music: *mut musicinfo_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut namebuf: [std::ffi::c_char; (9) as usize] = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if ( (musicnum <= mus_None)
        // 	 || (musicnum >= NUMMUSIC) )
        //     {
        // 	I_Error("Bad music number %d", musicnum);
        //     }
        //     else
        // 	music = &S_music[musicnum];
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (mus_playing == music)
        // 	return;
        todo!("if statement not yet translated");
        S_StopMusic();
        // TODO: if statement not yet translated:
        //
        //
        //     // get lumpnum if neccessary
        //     if (!music->lumpnum)
        //     {
        // 	sprintf(namebuf, "d_%s", music->name);
        // 	music->lumpnum = W_GetNumForName(namebuf);
        //     }
        todo!("if statement not yet translated");
        (*music).data = ((W_CacheLumpNum((*music).lumpnum, PU_MUSIC)) as *mut std::ffi::c_void);
        (*music).handle = I_RegisterSong((*music).data);
        I_PlaySong((*music).handle, looping);
        mus_playing = music;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn S_StopMusic() {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (mus_playing)
        //     {
        // 	if (mus_paused)
        // 	    I_ResumeSong(mus_playing->handle);
        //
        // 	I_StopSong(mus_playing->handle);
        // 	I_UnRegisterSong(mus_playing->handle);
        // 	Z_ChangeTag(mus_playing->data, PU_CACHE);
        //
        // 	mus_playing->data = 0;
        // 	mus_playing = 0;
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn S_StopChannel(mut cnum: std::ffi::c_int) {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut c: *mut channel_t = unsafe { (&(channels[(cnum) as usize]) as *const _ as *mut _) };
        // TODO: if statement not yet translated:
        //
        //
        //     if (c->sfxinfo)
        //     {
        // 	// stop the sound playing
        // 	if (I_SoundIsPlaying(c->handle))
        // 	{
        // #ifdef SAWDEBUG
        // 	    if (c->sfxinfo == &S_sfx[sfx_sawful])
        // 		fprintf(stderr, "stopped\n");
        // #endif
        // 	    I_StopSound(c->handle);
        // 	}
        //
        // 	// check to see
        // 	//  if other channels are playing the sound
        // 	for (i=0 ; i<numChannels ; i++)
        // 	{
        // 	    if (cnum != i
        // 		&& c->sfxinfo == channels[i].sfxinfo)
        // 	    {
        // 		break;
        // 	    }
        // 	}
        //
        // 	// degrade usefulness of sound data
        // 	c->sfxinfo->usefulness--;
        //
        // 	c->sfxinfo = 0;
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn S_AdjustSoundParams(
    mut listener: *mut mobj_t,
    mut source: *mut mobj_t,
    mut vol: *mut std::ffi::c_int,
    mut sep: *mut std::ffi::c_int,
    mut pitch: *mut std::ffi::c_int,
) -> std::ffi::c_int {
    unsafe {
        let mut approx_dist: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut adx: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut ady: fixed_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut angle: angle_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        adx = abs(((*listener).x - (*source).x));
        ady = abs(((*listener).y - (*source).y));
        approx_dist = ((adx + ady) - ((if (adx < ady) { adx } else { ady }) >> 1));
        // TODO: if statement not yet translated:
        //
        //
        //     if (gamemap != 8
        // 	&& approx_dist > S_CLIPPING_DIST)
        //     {
        // 	return 0;
        //     }
        todo!("if statement not yet translated");
        angle = R_PointToAngle2((*listener).x, (*listener).y, (*source).x, (*source).y);
        // TODO: if statement not yet translated:
        //
        //
        //     if (angle > listener->angle)
        // 	angle = angle - listener->angle;
        //     else
        // 	angle = angle + (0xffffffff - listener->angle);
        todo!("if statement not yet translated");
        angle >>= ANGLETOFINESHIFT;
        (*(sep)) = (128 - (FixedMul(S_STEREO_SWING, finesine[(angle) as usize]) >> FRACBITS));
        // TODO: if statement not yet translated:
        //
        //
        //     // volume calculation
        //     if (approx_dist < S_CLOSE_DIST)
        //     {
        // 	*vol = snd_SfxVolume;
        //     }
        //     else if (gamemap == 8)
        //     {
        // 	if (approx_dist > S_CLIPPING_DIST)
        // 	    approx_dist = S_CLIPPING_DIST;
        //
        // 	*vol = 15+ ((snd_SfxVolume-15)
        // 		    *((S_CLIPPING_DIST - approx_dist)>>FRACBITS))
        // 	    / S_ATTENUATOR;
        //     }
        //     else
        //     {
        // 	// distance effect
        // 	*vol = (snd_SfxVolume
        // 		* ((S_CLIPPING_DIST - approx_dist)>>FRACBITS))
        // 	    / S_ATTENUATOR;
        //     }
        todo!("if statement not yet translated");
        return ((*(vol)) > 0);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn S_getChannel(
    mut origin: *mut std::ffi::c_void,
    mut sfxinfo: *mut sfxinfo_t,
) -> std::ffi::c_int {
    unsafe {
        let mut cnum: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut c: *mut channel_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: for statement not yet translated:
        //
        //
        //     // Find an open channel
        //     for (cnum=0 ; cnum<numChannels ; cnum++)
        //     {
        // 	if (!channels[cnum].sfxinfo)
        // 	    break;
        // 	else if (origin &&  channels[cnum].origin ==  origin)
        // 	{
        // 	    S_StopChannel(cnum);
        // 	    break;
        // 	}
        //     }
        todo!("for statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // None available
        //     if (cnum == numChannels)
        //     {
        // 	// Look for lower priority
        // 	for (cnum=0 ; cnum<numChannels ; cnum++)
        // 	    if (channels[cnum].sfxinfo->priority >= sfxinfo->priority) break;
        //
        // 	if (cnum == numChannels)
        // 	{
        // 	    // FUCK!  No lower priority.  Sorry, Charlie.
        // 	    return -1;
        // 	}
        // 	else
        // 	{
        // 	    // Otherwise, kick out lower priority.
        // 	    S_StopChannel(cnum);
        // 	}
        //     }
        todo!("if statement not yet translated");
        c = (&(channels[(cnum) as usize]) as *const _ as *mut _);
        (*c).sfxinfo = sfxinfo;
        (*c).origin = origin;
        return cnum;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}
