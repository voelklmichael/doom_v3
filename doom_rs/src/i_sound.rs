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

pub static mut sndserver: *mut libc::FILE = unsafe { std::ptr::null_mut() };

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
    mut fd: std::ffi::c_int,
    mut command: std::ffi::c_int,
    mut arg: *mut std::ffi::c_int,
) {
    unsafe {
        let mut rc: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut errno: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        rc = ioctl(fd, command, arg);
        // TODO: if statement not yet translated:
        //
        //     if (rc < 0)
        //     {
        // 	fprintf(stderr, "ioctl(dsp,%d,arg) failed\n", command);
        // 	fprintf(stderr, "errno=%d\n", errno);
        // 	exit(-1);
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn getsfx(
    mut sfxname: *mut std::ffi::c_char,
    mut len: *mut std::ffi::c_int,
) -> *mut std::ffi::c_void {
    unsafe {
        let mut sfx: *mut std::ffi::c_uchar = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut paddedsfx: *mut std::ffi::c_uchar = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut size: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut paddedsize: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut name: [std::ffi::c_char; (20) as usize] = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut sfxlump: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        sprintf(name, (c"ds%s").as_ptr(), sfxname);
        // TODO: if statement not yet translated:
        //
        //
        //     // Now, there is a severe problem with the
        //     //  sound handling, in it is not (yet/anymore)
        //     //  gamemode aware. That means, sounds from
        //     //  DOOM II will be requested even with DOOM
        //     //  shareware.
        //     // The sound list is wired into sounds.c,
        //     //  which sets the external variable.
        //     // I do not do runtime patches to that
        //     //  variable. Instead, we will use a
        //     //  default sound for replacement.
        //     if ( W_CheckNumForName(name) == -1 )
        //       sfxlump = W_GetNumForName("dspistol");
        //     else
        //       sfxlump = W_GetNumForName(name);
        todo!("if statement not yet translated");
        size = W_LumpLength(sfxlump);
        sfx = ((W_CacheLumpNum(sfxlump, PU_STATIC)) as *mut std::ffi::c_uchar);
        paddedsize = ((((size - 8) + (SAMPLECOUNT - 1)) / SAMPLECOUNT) * SAMPLECOUNT);
        paddedsfx = ((Z_Malloc((paddedsize + 8), PU_STATIC, 0)) as *mut std::ffi::c_uchar);
        memcpy(paddedsfx, sfx, size);
        // TODO: for statement not yet translated:
        //
        //     for (i=size ; i<paddedsize+8 ; i++)
        //         paddedsfx[i] = 128;
        todo!("for statement not yet translated");
        Z_Free(sfx);
        (*(len)) = paddedsize;
        return ((paddedsfx + 8) as *mut std::ffi::c_void);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn addsfx(
    mut sfxid: std::ffi::c_int,
    mut volume: std::ffi::c_int,
    mut step: std::ffi::c_int,
    mut seperation: std::ffi::c_int,
) -> std::ffi::c_int {
    unsafe {
        static mut handlenums: std::ffi::c_ushort = unsafe { 0 };
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut rc: std::ffi::c_int = unsafe { (-(1)) };
        let mut oldest: std::ffi::c_int = unsafe { gametic };
        let mut oldestnum: std::ffi::c_int = unsafe { 0 };
        let mut slot: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut rightvol: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut leftvol: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     // Chainsaw troubles.
        //     // Play these sound effects only one at a time.
        //     if ( sfxid == sfx_sawup
        // 	 || sfxid == sfx_sawidl
        // 	 || sfxid == sfx_sawful
        // 	 || sfxid == sfx_sawhit
        // 	 || sfxid == sfx_stnmov
        // 	 || sfxid == sfx_pistol	 )
        //     {
        // 	// Loop all channels, check.
        // 	for (i=0 ; i<NUM_CHANNELS ; i++)
        // 	{
        // 	    // Active, and using the same SFX?
        // 	    if ( (channels[i])
        // 		 && (channelids[i] == sfxid) )
        // 	    {
        // 		// Reset.
        // 		channels[i] = 0;
        // 		// We are sure that iff,
        // 		//  there will only be one.
        // 		break;
        // 	    }
        // 	}
        //     }
        todo!("if statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //
        //     // Loop all channels to find oldest SFX.
        //     for (i=0; (i<NUM_CHANNELS) && (channels[i]); i++)
        //     {
        // 	if (channelstart[i] < oldest)
        // 	{
        // 	    oldestnum = i;
        // 	    oldest = channelstart[i];
        // 	}
        //     }
        todo!("for statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     // Tales from the cryptic.
        //     // If we found a channel, fine.
        //     // If not, we simply overwrite the first one, 0.
        //     // Probably only happens at startup.
        //     if (i == NUM_CHANNELS)
        // 	slot = oldestnum;
        //     else
        // 	slot = i;
        todo!("if statement not yet translated");
        channels[(slot) as usize] =
            (((*S_sfx.add((sfxid) as usize)).data) as *mut std::ffi::c_uchar);
        channelsend[(slot) as usize] = (channels[(slot) as usize] + lengths[(sfxid) as usize]);
        // TODO: if statement not yet translated:
        //
        //
        //     // Reset current handle number, limited to 0..100.
        //     if (!handlenums)
        // 	handlenums = 100;
        todo!("if statement not yet translated");
        channelhandles[(slot) as usize] = rc = {
            let __macro_tmp = handlenums;
            handlenums += 1;
            __macro_tmp
        };
        channelstep[(slot) as usize] = step;
        channelstepremainder[(slot) as usize] = 0;
        channelstart[(slot) as usize] = gametic;
        seperation += 1;
        leftvol = (volume - (((volume * seperation) * seperation) >> 16));
        seperation = (seperation - 257);
        rightvol = (volume - (((volume * seperation) * seperation) >> 16));
        // TODO: if statement not yet translated:
        //
        //
        //     // Sanity check, clamp volume.
        //     if (rightvol < 0 || rightvol > 127)
        // 	I_Error("rightvol out of bounds");
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (leftvol < 0 || leftvol > 127)
        // 	I_Error("leftvol out of bounds");
        todo!("if statement not yet translated");
        channelleftvol_lookup[(slot) as usize] =
            (&(vol_lookup[(leftvol * 256) as usize]) as *const _ as *mut _);
        channelrightvol_lookup[(slot) as usize] =
            (&(vol_lookup[(rightvol * 256) as usize]) as *const _ as *mut _);
        channelids[(slot) as usize] = sfxid;
        return rc;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn I_SetChannels() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut j: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut steptablemid: *mut std::ffi::c_int = unsafe { (steptable + 128) };
        // TODO: for statement not yet translated:
        //
        //
        //   // Okay, reset internal mixing channels to zero.
        //   /*for (i=0; i<NUM_CHANNELS; i++)
        //   {
        //     channels[i] = 0;
        //   }*/
        //
        //   // This table provides step widths for pitch parameters.
        //   // I fail to see that this is currently used.
        //   for (i=-128 ; i<128 ; i++)
        //     steptablemid[i] = (int)(pow(2.0, (i/64.0))*65536.0);
        todo!("for statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //
        //
        //   // Generates volume lookup tables
        //   //  which also turn the unsigned samples
        //   //  into signed samples.
        //   for (i=0 ; i<128 ; i++)
        //     for (j=0 ; j<256 ; j++)
        //       vol_lookup[i*256+j] = (i*(j-128)*256)/127;
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn I_SetSfxVolume(mut volume: std::ffi::c_int) {
    unsafe {
        snd_SfxVolume = volume;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn I_SetMusicVolume(mut volume: std::ffi::c_int) {
    unsafe {
        snd_MusicVolume = volume;
        // TODO: statement not yet translated:
        //
        //   // Now set volume on output device.
        //   // Whatever( snd_MusciVolume );
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn I_GetSfxLumpNum(mut sfx: *mut sfxinfo_t) -> std::ffi::c_int {
    unsafe {
        let mut namebuf: [std::ffi::c_char; (9) as usize] = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        sprintf(namebuf, (c"ds%s").as_ptr(), (*sfx).name);
        return W_GetNumForName(namebuf);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn I_StartSound(
    mut id: std::ffi::c_int,
    mut vol: std::ffi::c_int,
    mut sep: std::ffi::c_int,
    mut pitch: std::ffi::c_int,
    mut priority: std::ffi::c_int,
) -> std::ffi::c_int {
    unsafe {
        priority = 0;
        // TODO: if statement not yet translated:
        //     if (sndserver)
        //     {
        // 	fprintf(sndserver, "p%2.2x%2.2x%2.2x%2.2x\n", id, pitch, vol, sep);
        // 	fflush(sndserver);
        //     }
        todo!("if statement not yet translated");
        return id;
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn I_StopSound(mut handle: std::ffi::c_int) {
    unsafe {
        handle = 0;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn I_SoundIsPlaying(mut handle: std::ffi::c_int) -> std::ffi::c_int {
    unsafe {
        return (gametic < handle);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn I_UpdateSound() {
    unsafe {
        let mut sample: std::ffi::c_uint = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut dl: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut dr: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut leftout: *mut std::ffi::c_short = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut rightout: *mut std::ffi::c_short = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut leftend: *mut std::ffi::c_short = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut step: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut chan: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        leftout = mixbuffer;
        rightout = (mixbuffer + 1);
        step = 2;
        leftend = (mixbuffer + (SAMPLECOUNT * step));
        // TODO: while statement not yet translated:
        //
        //
        //     // Mix sounds into the mixing buffer.
        //     // Loop over step*SAMPLECOUNT,
        //     //  that is 512 values for two channels.
        //     while (leftout != leftend)
        //     {
        // 	// Reset left/right value.
        // 	dl = 0;
        // 	dr = 0;
        //
        // 	// Love thy L2 chache - made this a loop.
        // 	// Now more channels could be set at compile time
        // 	//  as well. Thus loop those  channels.
        // 	for ( chan = 0; chan < NUM_CHANNELS; chan++ )
        // 	{
        // 	    // Check channel, if active.
        // 	    if (channels[ chan ])
        // 	    {
        // 		// Get the raw data from the channel.
        // 		sample = *channels[ chan ];
        // 		// Add left and right part
        // 		//  for this channel (sound)
        // 		//  to the current data.
        // 		// Adjust volume accordingly.
        // 		dl += channelleftvol_lookup[ chan ][sample];
        // 		dr += channelrightvol_lookup[ chan ][sample];
        // 		// Increment index ???
        // 		channelstepremainder[ chan ] += channelstep[ chan ];
        // 		// MSB is next sample???
        // 		channels[ chan ] += channelstepremainder[ chan ] >> 16;
        // 		// Limit to LSB???
        // 		channelstepremainder[ chan ] &= 65536-1;
        //
        // 		// Check whether we are done.
        // 		if (channels[ chan ] >= channelsend[ chan ])
        // 		    channels[ chan ] = 0;
        // 	    }
        // 	}
        //
        // 	// Clamp to range. Left hardware channel.
        // 	// Has been char instead of short.
        // 	// if (dl > 127) *leftout = 127;
        // 	// else if (dl < -128) *leftout = -128;
        // 	// else *leftout = dl;
        //
        // 	if (dl > 0x7fff)
        // 	    *leftout = 0x7fff;
        // 	else if (dl < -0x8000)
        // 	    *leftout = -0x8000;
        // 	else
        // 	    *leftout = dl;
        //
        // 	// Same for right hardware channel.
        // 	if (dr > 0x7fff)
        // 	    *rightout = 0x7fff;
        // 	else if (dr < -0x8000)
        // 	    *rightout = -0x8000;
        // 	else
        // 	    *rightout = dr;
        //
        // 	// Increment current pointers in mixbuffer.
        // 	leftout += step;
        // 	rightout += step;
        //     }
        todo!("while statement not yet translated");
    }
}

pub unsafe extern "C" fn I_SubmitSound() {
    unsafe {
        write(audio_fd, mixbuffer, (SAMPLECOUNT * BUFMUL));
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn I_UpdateSoundParams(
    mut handle: std::ffi::c_int,
    mut vol: std::ffi::c_int,
    mut sep: std::ffi::c_int,
    mut pitch: std::ffi::c_int,
) {
    unsafe {
        handle = vol = sep = pitch = 0;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn I_ShutdownSound() {
    unsafe {
        // TODO: if statement not yet translated:
        //   if (sndserver)
        //   {
        //     // Send a "quit" command.
        //     fprintf(sndserver, "q\n");
        //     fflush(sndserver);
        //   }
        todo!("if statement not yet translated");
        return;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn I_InitSound() {
    unsafe {
        let mut buffer: [std::ffi::c_char; (256) as usize] = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //   if (getenv("DOOMWADDIR"))
        //     sprintf(buffer, "%s/%s",
        // 	    getenv("DOOMWADDIR"),
        // 	    sndserver_filename);
        //   else
        //     sprintf(buffer, "%s", sndserver_filename);
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //   // start sound process
        //   if ( !access(buffer, X_OK) )
        //   {
        //     strcat(buffer, " -quiet");
        //     sndserver = popen(buffer, "w");
        //   }
        //   else
        //     fprintf(stderr, "Could not start sound server [%s]\n", buffer);
        todo!("if statement not yet translated");
    }
}

pub unsafe extern "C" fn I_InitMusic() {
    unsafe {
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn I_ShutdownMusic() {
    unsafe {
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

static mut looping: std::ffi::c_int = unsafe { 0 };

static mut musicdies: std::ffi::c_int = unsafe { (-(1)) };

pub unsafe extern "C" fn I_PlaySong(mut handle: std::ffi::c_int, mut looping: std::ffi::c_int) {
    unsafe {
        handle = looping = 0;
        musicdies = (gametic + (TICRATE * 30));
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn I_PauseSong(mut handle: std::ffi::c_int) {
    unsafe {
        handle = 0;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn I_ResumeSong(mut handle: std::ffi::c_int) {
    unsafe {
        handle = 0;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn I_StopSong(mut handle: std::ffi::c_int) {
    unsafe {
        handle = 0;
        looping = 0;
        musicdies = 0;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn I_UnRegisterSong(mut handle: std::ffi::c_int) {
    unsafe {
        handle = 0;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn I_RegisterSong(mut data: *mut std::ffi::c_void) -> std::ffi::c_int {
    unsafe {
        data = NULL;
        return 1;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn I_QrySongPlaying(mut handle: std::ffi::c_int) -> std::ffi::c_int {
    unsafe {
        handle = 0;
        return (looping || (musicdies > gametic));
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub type tSigSet = std::ffi::c_int;

static mut itimer: std::ffi::c_int = unsafe { libc::ITIMER_REAL };

static mut sig: std::ffi::c_int = unsafe { libc::SIGALRM };

pub unsafe extern "C" fn I_HandleSoundTimer(mut ignore: std::ffi::c_int) {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //   // Debug.
        //   //fprintf( stderr, "%c", '+' ); fflush( stderr );
        //
        //   // Feed sound device if necesary.
        //   if ( flag )
        //   {
        //     // See I_SubmitSound().
        //     // Write it to DSP device.
        //     write(audio_fd, mixbuffer, SAMPLECOUNT*BUFMUL);
        //
        //     // Reset flag counter.
        //     flag = 0;
        //   }
        //   else
        //     return;
        todo!("if statement not yet translated");
        ignore = 0;
        return;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn I_SoundSetTimer(mut duration_of_tick: std::ffi::c_int) -> std::ffi::c_int {
    unsafe {
        let mut value: itimerval = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut ovalue: itimerval = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut act: sigaction = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut oact: sigaction = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut res: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        act.sa_handler = I_HandleSoundTimer;
        act.sa_flags = SA_RESTART;
        sigaction(
            sig,
            (&(act) as *const _ as *mut _),
            (&(oact) as *const _ as *mut _),
        );
        value.it_interval.tv_sec = 0;
        value.it_interval.tv_usec = duration_of_tick;
        value.it_value.tv_sec = 0;
        value.it_value.tv_usec = duration_of_tick;
        res = setitimer(
            itimer,
            (&(value) as *const _ as *mut _),
            (&(ovalue) as *const _ as *mut _),
        );
        // TODO: if statement not yet translated:
        //
        //
        //   // Debug.
        //   if ( res == -1 )
        //     fprintf( stderr, "I_SoundSetTimer: interrupt n.a.\n");
        todo!("if statement not yet translated");
        return res;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn I_SoundDelTimer() {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //   // Debug.
        //   if ( I_SoundSetTimer( 0 ) == -1)
        //     fprintf( stderr, "I_SoundDelTimer: failed to remove interrupt. Doh!\n");
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}
