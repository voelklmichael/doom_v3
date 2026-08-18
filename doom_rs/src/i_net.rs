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
    unsafe {
        let mut s: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        s = socket(PF_INET, SOCK_DGRAM, IPPROTO_UDP);
        // TODO: if statement not yet translated:
        //
        //     if (s<0)
        // 	I_Error ("can't create socket: %s",strerror(errno));
        todo!("if statement not yet translated");
        return s;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn BindToLocalPort(mut s: std::ffi::c_int, mut port: std::ffi::c_int) {
    unsafe {
        let mut v: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut address: libc::sockaddr_in = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        memset(
            (&(address) as *const _ as *mut _),
            0,
            std::mem::size_of_val(&(address)),
        );
        address.sin_family = AF_INET;
        address.sin_addr.s_addr = INADDR_ANY;
        address.sin_port = port;
        v = bind(
            s,
            ((&(address) as *const _ as *mut _) as *mut std::ffi::c_void),
            std::mem::size_of_val(&(address)),
        );
        // TODO: if statement not yet translated:
        //
        //     if (v == -1)
        // 	I_Error ("BindToPort: bind: %s", strerror(errno));
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn PacketSend() {
    unsafe {
        let mut c: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut sw: doomdata_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        sw.checksum = htonl((*netbuffer).checksum);
        sw.player = (*netbuffer).player;
        sw.retransmitfrom = (*netbuffer).retransmitfrom;
        sw.starttic = (*netbuffer).starttic;
        sw.numtics = (*netbuffer).numtics;
        // TODO: for statement not yet translated:
        //
        //     for (c=0 ; c< netbuffer->numtics ; c++)
        //     {
        // 	sw.cmds[c].forwardmove = netbuffer->cmds[c].forwardmove;
        // 	sw.cmds[c].sidemove = netbuffer->cmds[c].sidemove;
        // 	sw.cmds[c].angleturn = htons(netbuffer->cmds[c].angleturn);
        // 	sw.cmds[c].consistancy = htons(netbuffer->cmds[c].consistancy);
        // 	sw.cmds[c].chatchar = netbuffer->cmds[c].chatchar;
        // 	sw.cmds[c].buttons = netbuffer->cmds[c].buttons;
        //     }
        todo!("for statement not yet translated");
        c = sendto(
            sendsocket,
            (&(sw) as *const _ as *mut _),
            (*doomcom).datalength,
            0,
            ((&(sendaddress[((*doomcom).remotenode) as usize]) as *const _ as *mut _)
                as *mut std::ffi::c_void),
            std::mem::size_of_val(&(sendaddress[((*doomcom).remotenode) as usize])),
        );
        // TODO: statement not yet translated:
        //
        //
        //     //	if (c == -1)
        //     //		I_Error ("SendPacket error: %s",strerror(errno));
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn PacketGet() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut c: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut fromaddress: libc::sockaddr_in = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut fromlen: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut sw: doomdata_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        fromlen = std::mem::size_of_val(&(fromaddress));
        c = recvfrom(
            insocket,
            (&(sw) as *const _ as *mut _),
            std::mem::size_of_val(&(sw)),
            0,
            ((&(fromaddress) as *const _ as *mut _) as *mut sockaddr),
            (&(fromlen) as *const _ as *mut _),
        );
        // TODO: if statement not yet translated:
        //
        //     if (c == -1 )
        //     {
        // 	if (errno != EWOULDBLOCK)
        // 	    I_Error ("GetPacket: %s",strerror(errno));
        // 	doomcom->remotenode = -1;		// no packet
        // 	return;
        //     }
        todo!("if statement not yet translated");
        {
            static mut first: std::ffi::c_int = unsafe { 1 };
            // TODO: if statement not yet translated:
            //
            // 	if (first)
            // 	    printf("len=%d:p=[0x%x 0x%x] \n", c, *(int*)&sw, *((int*)&sw+1));
            todo!("if statement not yet translated");
            first = 0;
            // TODO: statement not yet translated:
            //
            //
            todo!("statement not yet translated");
        }
        // TODO: for statement not yet translated:
        //
        //
        //     // find remote node number
        //     for (i=0 ; i<doomcom->numnodes ; i++)
        // 	if ( fromaddress.sin_addr.s_addr == sendaddress[i].sin_addr.s_addr )
        // 	    break;
        todo!("for statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (i == doomcom->numnodes)
        //     {
        // 	// packet is not from one of the players (new game broadcast)
        // 	doomcom->remotenode = -1;		// no packet
        // 	return;
        //     }
        todo!("if statement not yet translated");
        (*doomcom).remotenode = i;
        (*doomcom).datalength = c;
        (*netbuffer).checksum = ntohl(sw.checksum);
        (*netbuffer).player = sw.player;
        (*netbuffer).retransmitfrom = sw.retransmitfrom;
        (*netbuffer).starttic = sw.starttic;
        (*netbuffer).numtics = sw.numtics;
        // TODO: for statement not yet translated:
        //
        //
        //     for (c=0 ; c< netbuffer->numtics ; c++)
        //     {
        // 	netbuffer->cmds[c].forwardmove = sw.cmds[c].forwardmove;
        // 	netbuffer->cmds[c].sidemove = sw.cmds[c].sidemove;
        // 	netbuffer->cmds[c].angleturn = ntohs(sw.cmds[c].angleturn);
        // 	netbuffer->cmds[c].consistancy = ntohs(sw.cmds[c].consistancy);
        // 	netbuffer->cmds[c].chatchar = sw.cmds[c].chatchar;
        // 	netbuffer->cmds[c].buttons = sw.cmds[c].buttons;
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn GetLocalAddress() -> std::ffi::c_int {
    unsafe {
        let mut hostname: [std::ffi::c_char; (1024) as usize] = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut hostentry: *mut hostent = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut v: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        v = gethostname(hostname, std::mem::size_of_val(&(hostname)));
        // TODO: if statement not yet translated:
        //
        //     if (v == -1)
        // 	I_Error ("GetLocalAddress : gethostname: errno %d",errno);
        todo!("if statement not yet translated");
        hostentry = gethostbyname(hostname);
        // TODO: if statement not yet translated:
        //
        //     if (!hostentry)
        // 	I_Error ("GetLocalAddress : gethostbyname: couldn't get local host");
        todo!("if statement not yet translated");
        return (*(((*hostentry).h_addr_list[(0) as usize]) as *mut std::ffi::c_int));
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn I_InitNetwork() {
    unsafe {
        let mut trueval: boolean = unsafe { true_ };
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut p: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut hostentry: *mut hostent = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        doomcom = malloc(std::mem::size_of_val(&(*(doomcom))));
        memset(doomcom, 0, std::mem::size_of_val(&(*(doomcom))));
        i = M_CheckParm((c"-dup").as_ptr());
        // TODO: if statement not yet translated:
        //
        //     if (i && i< myargc-1)
        //     {
        // 	doomcom->ticdup = myargv[i+1][0]-'0';
        // 	if (doomcom->ticdup < 1)
        // 	    doomcom->ticdup = 1;
        // 	if (doomcom->ticdup > 9)
        // 	    doomcom->ticdup = 9;
        //     }
        //     else
        // 	doomcom-> ticdup = 1;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (M_CheckParm ("-extratic"))
        // 	doomcom-> extratics = 1;
        //     else
        // 	doomcom-> extratics = 0;
        todo!("if statement not yet translated");
        p = M_CheckParm((c"-port").as_ptr());
        // TODO: if statement not yet translated:
        //
        //     if (p && p<myargc-1)
        //     {
        // 	DOOMPORT = atoi (myargv[p+1]);
        // 	printf ("using alternate port %i\n",DOOMPORT);
        //     }
        todo!("if statement not yet translated");
        i = M_CheckParm((c"-net").as_ptr());
        // TODO: if statement not yet translated:
        //
        //     if (!i)
        //     {
        // 	// single player game
        // 	netgame = false;
        // 	doomcom->id = DOOMCOM_ID;
        // 	doomcom->numplayers = doomcom->numnodes = 1;
        // 	doomcom->deathmatch = false;
        // 	doomcom->consoleplayer = 0;
        // 	return;
        //     }
        todo!("if statement not yet translated");
        netsend = PacketSend;
        netget = PacketGet;
        netgame = true_;
        (*doomcom).consoleplayer =
            (myargv[(i + 1) as usize][(0) as usize] - (b'1' as std::ffi::c_int));
        (*doomcom).numnodes = 1;
        {
            let __macro_tmp = i;
            i += 1;
            __macro_tmp
        };
        // TODO: while statement not yet translated:
        //
        //     while (++i < myargc && myargv[i][0] != '-')
        //     {
        // 	sendaddress[doomcom->numnodes].sin_family = AF_INET;
        // 	sendaddress[doomcom->numnodes].sin_port = htons(DOOMPORT);
        // 	if (myargv[i][0] == '.')
        // 	{
        // 	    sendaddress[doomcom->numnodes].sin_addr.s_addr
        // 		= inet_addr (myargv[i]+1);
        // 	}
        // 	else
        // 	{
        // 	    hostentry = gethostbyname (myargv[i]);
        // 	    if (!hostentry)
        // 		I_Error ("gethostbyname: couldn't find %s", myargv[i]);
        // 	    sendaddress[doomcom->numnodes].sin_addr.s_addr
        // 		= *(int *)hostentry->h_addr_list[0];
        // 	}
        // 	doomcom->numnodes++;
        //     }
        todo!("while statement not yet translated");
        (*doomcom).id = DOOMCOM_ID;
        (*doomcom).numplayers = (*doomcom).numnodes;
        insocket = UDPsocket();
        BindToLocalPort(insocket, htons(DOOMPORT));
        ioctl(insocket, FIONBIO, (&(trueval) as *const _ as *mut _));
        sendsocket = UDPsocket();
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn I_NetCmd() {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (doomcom->command == CMD_SEND)
        //     {
        // 	netsend ();
        //     }
        //     else if (doomcom->command == CMD_GET)
        //     {
        // 	netget ();
        //     }
        //     else
        // 	I_Error ("Bad net cmd: %i\n",doomcom->command);
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}
