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
    unsafe {
        return ((&((*((0) as *mut doomdata_t)).cmds[((*netbuffer).numtics) as usize]) as *const _
            as *mut _) as std::ffi::c_int);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn NetbufferChecksum() -> std::ffi::c_uint {
    unsafe {
        let mut c: std::ffi::c_uint = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut l: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        c = 0x1234567;
        return 0;
        l = ((NetbufferSize()
            - ((&((*((0) as *mut doomdata_t)).retransmitfrom) as *const _ as *mut _)
                as std::ffi::c_int))
            / 4);
        // TODO: for statement not yet translated:
        //
        //     for (i=0 ; i<l ; i++)
        // 	c += ((unsigned *)&netbuffer->retransmitfrom)[i] * (i+1);
        todo!("for statement not yet translated");
        return (c & NCMD_CHECKSUM);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn ExpandTics(mut low: std::ffi::c_int) -> std::ffi::c_int {
    unsafe {
        let mut delta: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        delta = (low - (maketic & 0xff));
        // TODO: if statement not yet translated:
        //
        //
        //     if (delta >= -64 && delta <= 64)
        // 	return (maketic&~0xff) + low;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (delta > 64)
        // 	return (maketic&~0xff) - 256 + low;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (delta < -64)
        // 	return (maketic&~0xff) + 256 + low;
        todo!("if statement not yet translated");
        I_Error(
            (c"ExpandTics: strange value %i at maketic %i").as_ptr(),
            low,
            maketic,
        );
        return 0;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn HSendPacket(mut node: std::ffi::c_int, mut flags: std::ffi::c_int) {
    unsafe {
        (*netbuffer).checksum = (NetbufferChecksum() | flags);
        // TODO: if statement not yet translated:
        //
        //
        //     if (!node)
        //     {
        // 	reboundstore = *netbuffer;
        // 	reboundpacket = true;
        // 	return;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (demoplayback)
        // 	return;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (!netgame)
        // 	I_Error ("Tried to transmit to another node");
        todo!("if statement not yet translated");
        (*doomcom).command = CMD_SEND;
        (*doomcom).remotenode = node;
        (*doomcom).datalength = NetbufferSize();
        // TODO: if statement not yet translated:
        //
        //
        //     if (debugfile)
        //     {
        // 	int		i;
        // 	int		realretrans;
        // 	if (netbuffer->checksum & NCMD_RETRANSMIT)
        // 	    realretrans = ExpandTics (netbuffer->retransmitfrom);
        // 	else
        // 	    realretrans = -1;
        //
        // 	fprintf (debugfile,"send (%i + %i, R %i) [%i] ",
        // 		 ExpandTics(netbuffer->starttic),
        // 		 netbuffer->numtics, realretrans, doomcom->datalength);
        //
        // 	for (i=0 ; i<doomcom->datalength ; i++)
        // 	    fprintf (debugfile,"%i ",((byte *)netbuffer)[i]);
        //
        // 	fprintf (debugfile,"\n");
        //     }
        todo!("if statement not yet translated");
        I_NetCmd();
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn HGetPacket() -> boolean {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (reboundpacket)
        //     {
        // 	*netbuffer = reboundstore;
        // 	doomcom->remotenode = 0;
        // 	reboundpacket = false;
        // 	return true;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (!netgame)
        // 	return false;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (demoplayback)
        // 	return false;
        todo!("if statement not yet translated");
        (*doomcom).command = CMD_GET;
        I_NetCmd();
        // TODO: if statement not yet translated:
        //
        //
        //     if (doomcom->remotenode == -1)
        // 	return false;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (doomcom->datalength != NetbufferSize ())
        //     {
        // 	if (debugfile)
        // 	    fprintf (debugfile,"bad packet length %i\n",doomcom->datalength);
        // 	return false;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (NetbufferChecksum () != (netbuffer->checksum&NCMD_CHECKSUM) )
        //     {
        // 	if (debugfile)
        // 	    fprintf (debugfile,"bad packet checksum\n");
        // 	return false;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (debugfile)
        //     {
        // 	int		realretrans;
        // 	int	i;
        //
        // 	if (netbuffer->checksum & NCMD_SETUP)
        // 	    fprintf (debugfile,"setup packet\n");
        // 	else
        // 	{
        // 	    if (netbuffer->checksum & NCMD_RETRANSMIT)
        // 		realretrans = ExpandTics (netbuffer->retransmitfrom);
        // 	    else
        // 		realretrans = -1;
        //
        // 	    fprintf (debugfile,"get %i = (%i + %i, R %i)[%i] ",
        // 		     doomcom->remotenode,
        // 		     ExpandTics(netbuffer->starttic),
        // 		     netbuffer->numtics, realretrans, doomcom->datalength);
        //
        // 	    for (i=0 ; i<doomcom->datalength ; i++)
        // 		fprintf (debugfile,"%i ",((byte *)netbuffer)[i]);
        // 	    fprintf (debugfile,"\n");
        // 	}
        //     }
        todo!("if statement not yet translated");
        return true_;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub static mut exitmsg: [std::ffi::c_char; (80) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn GetPackets() {
    unsafe {
        let mut netconsole: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut netnode: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut src: *mut ticcmd_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut dest: *mut ticcmd_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut realend: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut realstart: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: while statement not yet translated:
        //
        //
        //     while ( HGetPacket() )
        //     {
        // 	if (netbuffer->checksum & NCMD_SETUP)
        // 	    continue;		// extra setup packet
        //
        // 	netconsole = netbuffer->player & ~PL_DRONE;
        // 	netnode = doomcom->remotenode;
        //
        // 	// to save bytes, only the low byte of tic numbers are sent
        // 	// Figure out what the rest of the bytes are
        // 	realstart = ExpandTics (netbuffer->starttic);
        // 	realend = (realstart+netbuffer->numtics);
        //
        // 	// check for exiting the game
        // 	if (netbuffer->checksum & NCMD_EXIT)
        // 	{
        // 	    if (!nodeingame[netnode])
        // 		continue;
        // 	    nodeingame[netnode] = false;
        // 	    playeringame[netconsole] = false;
        // 	    strcpy (exitmsg, "Player 1 left the game");
        // 	    exitmsg[7] += netconsole;
        // 	    players[consoleplayer].message = exitmsg;
        // 	    if (demorecording)
        // 		G_CheckDemoStatus ();
        // 	    continue;
        // 	}
        //
        // 	// check for a remote game kill
        // 	if (netbuffer->checksum & NCMD_KILL)
        // 	    I_Error ("Killed by network driver");
        //
        // 	nodeforplayer[netconsole] = netnode;
        //
        // 	// check for retransmit request
        // 	if ( resendcount[netnode] <= 0
        // 	     && (netbuffer->checksum & NCMD_RETRANSMIT) )
        // 	{
        // 	    resendto[netnode] = ExpandTics(netbuffer->retransmitfrom);
        // 	    if (debugfile)
        // 		fprintf (debugfile,"retransmit from %i\n", resendto[netnode]);
        // 	    resendcount[netnode] = RESENDCOUNT;
        // 	}
        // 	else
        // 	    resendcount[netnode]--;
        //
        // 	// check for out of order / duplicated packet
        // 	if (realend == nettics[netnode])
        // 	    continue;
        //
        // 	if (realend < nettics[netnode])
        // 	{
        // 	    if (debugfile)
        // 		fprintf (debugfile,
        // 			 "out of order packet (%i + %i)\n" ,
        // 			 realstart,netbuffer->numtics);
        // 	    continue;
        // 	}
        //
        // 	// check for a missed packet
        // 	if (realstart > nettics[netnode])
        // 	{
        // 	    // stop processing until the other system resends the missed tics
        // 	    if (debugfile)
        // 		fprintf (debugfile,
        // 			 "missed tics from %i (%i - %i)\n",
        // 			 netnode, realstart, nettics[netnode]);
        // 	    remoteresend[netnode] = true;
        // 	    continue;
        // 	}
        //
        // 	// update command store from the packet
        //         {
        // 	    int		start;
        //
        // 	    remoteresend[netnode] = false;
        //
        // 	    start = nettics[netnode] - realstart;
        // 	    src = &netbuffer->cmds[start];
        //
        // 	    while (nettics[netnode] < realend)
        // 	    {
        // 		dest = &netcmds[netconsole][nettics[netnode]%BACKUPTICS];
        // 		nettics[netnode]++;
        // 		*dest = *src;
        // 		src++;
        // 	    }
        // 	}
        //     }
        todo!("while statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub static mut gametime: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn NetUpdate() {
    unsafe {
        let mut nowtime: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut newtics: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut j: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut realstart: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut gameticdiv: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        nowtime = (I_GetTime() / ticdup);
        newtics = (nowtime - gametime);
        gametime = nowtime;
        // TODO: if statement not yet translated:
        //
        //
        //     if (newtics <= 0) 	// nothing new to update
        // 	goto listen;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (skiptics <= newtics)
        //     {
        // 	newtics -= skiptics;
        // 	skiptics = 0;
        //     }
        //     else
        //     {
        // 	skiptics -= newtics;
        // 	newtics = 0;
        //     }
        todo!("if statement not yet translated");
        (*netbuffer).player = consoleplayer;
        gameticdiv = (gametic / ticdup);
        // TODO: for statement not yet translated:
        //
        //     for (i=0 ; i<newtics ; i++)
        //     {
        // 	I_StartTic ();
        // 	D_ProcessEvents ();
        // 	if (maketic - gameticdiv >= BACKUPTICS/2-1)
        // 	    break;          // can't hold any more
        //
        // 	//printf ("mk:%i ",maketic);
        // 	G_BuildTiccmd (&localcmds[maketic%BACKUPTICS]);
        // 	maketic++;
        //     }
        todo!("for statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //
        //     if (singletics)
        // 	return;         // singletic update is syncronous
        todo!("if statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //     // send the packet to the other nodes
        //     for (i=0 ; i<doomcom->numnodes ; i++)
        // 	if (nodeingame[i])
        // 	{
        // 	    netbuffer->starttic = realstart = resendto[i];
        // 	    netbuffer->numtics = maketic - realstart;
        // 	    if (netbuffer->numtics > BACKUPTICS)
        // 		I_Error ("NetUpdate: netbuffer->numtics > BACKUPTICS");
        //
        // 	    resendto[i] = maketic - doomcom->extratics;
        //
        // 	    for (j=0 ; j< netbuffer->numtics ; j++)
        // 		netbuffer->cmds[j] =
        // 		    localcmds[(realstart+j)%BACKUPTICS];
        //
        // 	    if (remoteresend[i])
        // 	    {
        // 		netbuffer->retransmitfrom = nettics[i];
        // 		HSendPacket (i, NCMD_RETRANSMIT);
        // 	    }
        // 	    else
        // 	    {
        // 		netbuffer->retransmitfrom = 0;
        // 		HSendPacket (i, 0);
        // 	    }
        // 	}
        todo!("for statement not yet translated");
        // C label listen: (goto targets are not translated)
        GetPackets();
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn CheckAbort() {
    unsafe {
        let mut ev: *mut event_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut stoptic: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        stoptic = (I_GetTime() + 2);
        // TODO: while statement not yet translated:
        //
        //     while (I_GetTime() < stoptic)
        // 	I_StartTic ();
        todo!("while statement not yet translated");
        I_StartTic();
        // TODO: for statement not yet translated:
        //
        //     for ( ; eventtail != eventhead
        // 	      ; eventtail = (++eventtail)&(MAXEVENTS-1) )
        //     {
        // 	ev = &events[eventtail];
        // 	if (ev->type == ev_keydown && ev->data1 == KEY_ESCAPE)
        // 	    I_Error ("Network game synchronization aborted.");
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn D_ArbitrateNetStart() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut gotinfo: [boolean; (MAXNETNODES) as usize] = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        autostart = true_;
        memset(gotinfo, 0, std::mem::size_of_val(&(gotinfo)));
        // TODO: if statement not yet translated:
        //
        //
        //     if (doomcom->consoleplayer)
        //     {
        // 	// listen for setup info from key player
        // 	printf ("listening for network start info...\n");
        // 	while (1)
        // 	{
        // 	    CheckAbort ();
        // 	    if (!HGetPacket ())
        // 		continue;
        // 	    if (netbuffer->checksum & NCMD_SETUP)
        // 	    {
        // 		if (netbuffer->player != VERSION)
        // 		    I_Error ("Different DOOM versions cannot play a net game!");
        // 		startskill = netbuffer->retransmitfrom & 15;
        // 		deathmatch = (netbuffer->retransmitfrom & 0xc0) >> 6;
        // 		nomonsters = (netbuffer->retransmitfrom & 0x20) > 0;
        // 		respawnparm = (netbuffer->retransmitfrom & 0x10) > 0;
        // 		startmap = netbuffer->starttic & 0x3f;
        // 		startepisode = netbuffer->starttic >> 6;
        // 		return;
        // 	    }
        // 	}
        //     }
        //     else
        //     {
        // 	// key player, send the setup info
        // 	printf ("sending network start info...\n");
        // 	do
        // 	{
        // 	    CheckAbort ();
        // 	    for (i=0 ; i<doomcom->numnodes ; i++)
        // 	    {
        // 		netbuffer->retransmitfrom = startskill;
        // 		if (deathmatch)
        // 		    netbuffer->retransmitfrom |= (deathmatch<<6);
        // 		if (nomonsters)
        // 		    netbuffer->retransmitfrom |= 0x20;
        // 		if (respawnparm)
        // 		    netbuffer->retransmitfrom |= 0x10;
        // 		netbuffer->starttic = startepisode * 64 + startmap;
        // 		netbuffer->player = VERSION;
        // 		netbuffer->numtics = 0;
        // 		HSendPacket (i, NCMD_SETUP);
        // 	    }
        //
        // #if 1
        // 	    for(i = 10 ; i  &&  HGetPacket(); --i)
        // 	    {
        // 		if((netbuffer->player&0x7f) < MAXNETNODES)
        // 		    gotinfo[netbuffer->player&0x7f] = true;
        // 	    }
        // #else
        // 	    while (HGetPacket ())
        // 	    {
        // 		gotinfo[netbuffer->player&0x7f] = true;
        // 	    }
        // #endif
        //
        // 	    for (i=1 ; i<doomcom->numnodes ; i++)
        // 		if (!gotinfo[i])
        // 		    break;
        // 	} while (i < doomcom->numnodes);
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

unsafe extern "C" {
    pub static mut viewangleoffset: std::ffi::c_int;
}

pub unsafe extern "C" fn D_CheckNetGame() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<MAXNETNODES ; i++)
        //     {
        // 	nodeingame[i] = false;
        //        	nettics[i] = 0;
        // 	remoteresend[i] = false;	// set when local needs tics
        // 	resendto[i] = 0;		// which tic to start sending
        //     }
        todo!("for statement not yet translated");
        I_InitNetwork();
        // TODO: if statement not yet translated:
        //
        //     if (doomcom->id != DOOMCOM_ID)
        // 	I_Error ("Doomcom buffer invalid!");
        todo!("if statement not yet translated");
        netbuffer = (&((*doomcom).data) as *const _ as *mut _);
        consoleplayer = displayplayer = (*doomcom).consoleplayer;
        // TODO: if statement not yet translated:
        //
        //     if (netgame)
        // 	D_ArbitrateNetStart ();
        todo!("if statement not yet translated");
        printf(
            (c"startskill %i  deathmatch: %i  startmap: %i  startepisode: %i\n").as_ptr(),
            startskill,
            deathmatch,
            startmap,
            startepisode,
        );
        ticdup = (*doomcom).ticdup;
        maxsend = ((BACKUPTICS / (2 * ticdup)) - 1);
        // TODO: if statement not yet translated:
        //
        //     if (maxsend<1)
        // 	maxsend = 1;
        todo!("if statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<doomcom->numplayers ; i++)
        // 	playeringame[i] = true;
        todo!("for statement not yet translated");
        // TODO: for statement not yet translated:
        //
        //     for (i=0 ; i<doomcom->numnodes ; i++)
        // 	nodeingame[i] = true;
        todo!("for statement not yet translated");
        printf(
            (c"player %i of %i (%i nodes)\n").as_ptr(),
            (consoleplayer + 1),
            (*doomcom).numplayers,
            (*doomcom).numnodes,
        );
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn D_QuitNetGame() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut j: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (debugfile)
        // 	fclose (debugfile);
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (!netgame || !usergame || consoleplayer == -1 || demoplayback)
        // 	return;
        todo!("if statement not yet translated");
        (*netbuffer).player = consoleplayer;
        (*netbuffer).numtics = 0;
        // TODO: for statement not yet translated:
        //
        //     for (i=0 ; i<4 ; i++)
        //     {
        // 	for (j=1 ; j<doomcom->numnodes ; j++)
        // 	    if (nodeingame[j])
        // 		HSendPacket (j, NCMD_EXIT);
        // 	I_WaitVBL (1);
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub static mut frametics: [std::ffi::c_int; (4) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut frameon: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut frameskip: [std::ffi::c_int; (4) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut oldnettics: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

unsafe extern "C" {
    pub static mut advancedemo: boolean;
}

pub unsafe extern "C" fn TryRunTics() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut lowtic: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut entertic: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        static mut oldentertics: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut realtics: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut availabletics: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut counts: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut numplaying: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        entertic = (I_GetTime() / ticdup);
        realtics = (entertic - oldentertics);
        oldentertics = entertic;
        NetUpdate();
        lowtic = std::ffi::c_int::MAX;
        numplaying = 0;
        // TODO: for statement not yet translated:
        //
        //     for (i=0 ; i<doomcom->numnodes ; i++)
        //     {
        // 	if (nodeingame[i])
        // 	{
        // 	    numplaying++;
        // 	    if (nettics[i] < lowtic)
        // 		lowtic = nettics[i];
        // 	}
        //     }
        todo!("for statement not yet translated");
        availabletics = (lowtic - (gametic / ticdup));
        // TODO: if statement not yet translated:
        //
        //
        //     // decide how many tics to run
        //     if (realtics < availabletics-1)
        // 	counts = realtics+1;
        //     else if (realtics < availabletics)
        // 	counts = realtics;
        //     else
        // 	counts = availabletics;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (counts < 1)
        // 	counts = 1;
        todo!("if statement not yet translated");
        {
            let __macro_tmp = frameon;
            frameon += 1;
            __macro_tmp
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (debugfile)
        // 	fprintf (debugfile,
        // 		 "=======real: %i  avail: %i  game: %i\n",
        // 		 realtics, availabletics,counts);
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (!demoplayback)
        //     {
        // 	// ideally nettics[0] should be 1 - 3 tics above lowtic
        // 	// if we are consistantly slower, speed up time
        // 	for (i=0 ; i<MAXPLAYERS ; i++)
        // 	    if (playeringame[i])
        // 		break;
        // 	if (consoleplayer == i)
        // 	{
        // 	    // the key player does not adapt
        // 	}
        // 	else
        // 	{
        // 	    if (nettics[0] <= nettics[nodeforplayer[i]])
        // 	    {
        // 		gametime--;
        // 		// printf ("-");
        // 	    }
        // 	    frameskip[frameon&3] = (oldnettics > nettics[nodeforplayer[i]]);
        // 	    oldnettics = nettics[0];
        // 	    if (frameskip[0] && frameskip[1] && frameskip[2] && frameskip[3])
        // 	    {
        // 		skiptics = 1;
        // 		// printf ("+");
        // 	    }
        // 	}
        //     }// demoplayback
        todo!("if statement not yet translated");
        // TODO: while statement not yet translated:
        //
        //     // wait for new tics if needed
        //     while (lowtic < gametic/ticdup + counts)
        //     {
        // 	NetUpdate ();
        // 	lowtic = MAXINT;
        //
        // 	for (i=0 ; i<doomcom->numnodes ; i++)
        // 	    if (nodeingame[i] && nettics[i] < lowtic)
        // 		lowtic = nettics[i];
        //
        // 	if (lowtic < gametic/ticdup)
        // 	    I_Error ("TryRunTics: lowtic < gametic");
        //
        // 	// don't stay in here forever -- give the menu a chance to work
        // 	if (I_GetTime ()/ticdup - entertic >= 20)
        // 	{
        // 	    M_Ticker ();
        // 	    return;
        // 	}
        //     }
        todo!("while statement not yet translated");
        // TODO: while statement not yet translated:
        //
        //
        //     // run the count * ticdup dics
        //     while (counts--)
        //     {
        // 	for (i=0 ; i<ticdup ; i++)
        // 	{
        // 	    if (gametic/ticdup > lowtic)
        // 		I_Error ("gametic>lowtic");
        // 	    if (advancedemo)
        // 		D_DoAdvanceDemo ();
        // 	    M_Ticker ();
        // 	    G_Ticker ();
        // 	    gametic++;
        //
        // 	    // modify command for duplicated tics
        // 	    if (i != ticdup-1)
        // 	    {
        // 		ticcmd_t	*cmd;
        // 		int			buf;
        // 		int			j;
        //
        // 		buf = (gametic/ticdup)%BACKUPTICS;
        // 		for (j=0 ; j<MAXPLAYERS ; j++)
        // 		{
        // 		    cmd = &netcmds[j][buf];
        // 		    cmd->chatchar = 0;
        // 		    if (cmd->buttons & BT_SPECIAL)
        // 			cmd->buttons = 0;
        // 		}
        // 	    }
        // 	}
        // 	NetUpdate ();	// check for new console commands
        //     }
        todo!("while statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}
