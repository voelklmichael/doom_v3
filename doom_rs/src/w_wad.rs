use crate::d_event::*;
use crate::d_ticcmd::*;
use crate::doomtype::*;
use crate::i_system::*;
use crate::m_swap::*;
use crate::z_zone::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wadinfo_t {
    pub identification: [std::ffi::c_char; (4) as usize],
    pub numlumps: std::ffi::c_int,
    pub infotableofs: std::ffi::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct filelump_t {
    pub filepos: std::ffi::c_int,
    pub size: std::ffi::c_int,
    pub name: [std::ffi::c_char; (8) as usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lumpinfo_t {
    pub name: [std::ffi::c_char; (8) as usize],
    pub handle: std::ffi::c_int,
    pub position: std::ffi::c_int,
    pub size: std::ffi::c_int,
}

static mut rcsid: [std::ffi::c_char; 48] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        119 as std::ffi::c_char,
        95 as std::ffi::c_char,
        119 as std::ffi::c_char,
        97 as std::ffi::c_char,
        100 as std::ffi::c_char,
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
        49 as std::ffi::c_char,
        54 as std::ffi::c_char,
        58 as std::ffi::c_char,
        52 as std::ffi::c_char,
        55 as std::ffi::c_char,
        58 as std::ffi::c_char,
        53 as std::ffi::c_char,
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

pub const O_BINARY: std::ffi::c_int = 0;

pub static mut lumpinfo: *mut lumpinfo_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut numlumps: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut lumpcache: *mut *mut std::ffi::c_void = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

/* TODO: unparsed macro value, references an identifier with no known definition anywhere in this module's visible corpus (likely dead code never expanded in the original C):
#define strcmpi strcasecmp
*/

pub unsafe extern "C" fn strupr(mut s: *mut std::ffi::c_char) {
    unsafe {
        // TODO: while statement not yet translated:
        //
        //     while (*s) { *s = toupper(*s); s++; }
        todo!("while statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn filelength(mut handle: std::ffi::c_int) -> std::ffi::c_int {
    unsafe {
        let mut fileinfo: stat = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (fstat (handle,&fileinfo) == -1)
        // 	I_Error ("Error fstating");
        todo!("if statement not yet translated");
        return fileinfo.st_size;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn ExtractFileBase(
    mut path: *mut std::ffi::c_char,
    mut dest: *mut std::ffi::c_char,
) {
    unsafe {
        let mut src: *mut std::ffi::c_char = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut length: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        src = ((path + strlen(path)) - 1);
        // TODO: while statement not yet translated:
        //
        //
        //     // back up until a \ or the start
        //     while (src != path
        // 	   && *(src-1) != '\\'
        // 	   && *(src-1) != '/')
        //     {
        // 	src--;
        //     }
        todo!("while statement not yet translated");
        memset(dest, 0, 8);
        length = 0;
        // TODO: while statement not yet translated:
        //
        //
        //     while (*src && *src != '.')
        //     {
        // 	if (++length == 9)
        // 	    I_Error ("Filename base of %s >8 chars",path);
        //
        // 	*dest++ = toupper((int)*src++);
        //     }
        todo!("while statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub static mut reloadlump: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut reloadname: *mut std::ffi::c_char = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn W_AddFile(mut filename: *mut std::ffi::c_char) {
    unsafe {
        let mut header: wadinfo_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut lump_p: *mut lumpinfo_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut i: std::ffi::c_uint = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut handle: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut length: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut startlump: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut fileinfo: *mut filelump_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut singleinfo: filelump_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut storehandle: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     // open the file and add to directory
        //
        //     // handle reload indicator.
        //     if (filename[0] == '~')
        //     {
        // 	filename++;
        // 	reloadname = filename;
        // 	reloadlump = numlumps;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if ( (handle = open (filename,O_RDONLY | O_BINARY)) == -1)
        //     {
        // 	printf (" couldn't open %s\n",filename);
        // 	return;
        //     }
        todo!("if statement not yet translated");
        printf((c" adding %s\n").as_ptr(), filename);
        startlump = numlumps;
        // TODO: if statement not yet translated:
        //
        //
        //     if (strcmpi (filename+strlen(filename)-3 , "wad" ) )
        //     {
        // 	// single lump file
        // 	fileinfo = &singleinfo;
        // 	singleinfo.filepos = 0;
        // 	singleinfo.size = LONG(filelength(handle));
        // 	ExtractFileBase (filename, singleinfo.name);
        // 	numlumps++;
        //     }
        //     else
        //     {
        // 	// WAD file
        // 	read (handle, &header, sizeof(header));
        // 	if (strncmp(header.identification,"IWAD",4))
        // 	{
        // 	    // Homebrew levels?
        // 	    if (strncmp(header.identification,"PWAD",4))
        // 	    {
        // 		I_Error ("Wad file %s doesn't have IWAD "
        // 			 "or PWAD id\n", filename);
        // 	    }
        //
        // 	    // ???modifiedgame = true;
        // 	}
        // 	header.numlumps = LONG(header.numlumps);
        // 	header.infotableofs = LONG(header.infotableofs);
        // 	length = header.numlumps*sizeof(filelump_t);
        // 	fileinfo = alloca (length);
        // 	lseek (handle, header.infotableofs, SEEK_SET);
        // 	read (handle, fileinfo, length);
        // 	numlumps += header.numlumps;
        //     }
        todo!("if statement not yet translated");
        lumpinfo = realloc(lumpinfo, (numlumps * std::mem::size_of::<lumpinfo_t>()));
        // TODO: if statement not yet translated:
        //
        //
        //     if (!lumpinfo)
        // 	I_Error ("Couldn't realloc lumpinfo");
        todo!("if statement not yet translated");
        lump_p = (&(lumpinfo[(startlump) as usize]) as *const _ as *mut _);
        storehandle = (if (reloadname) != 0 { (-(1)) } else { handle });
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=startlump ; i<numlumps ; i++,lump_p++, fileinfo++)
        //     {
        // 	lump_p->handle = storehandle;
        // 	lump_p->position = LONG(fileinfo->filepos);
        // 	lump_p->size = LONG(fileinfo->size);
        // 	strncpy (lump_p->name, fileinfo->name, 8);
        //     }
        todo!("for statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (reloadname)
        // 	close (handle);
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn W_Reload() {
    unsafe {
        let mut header: wadinfo_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut lumpcount: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut lump_p: *mut lumpinfo_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut i: std::ffi::c_uint = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut handle: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut length: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut fileinfo: *mut filelump_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (!reloadname)
        // 	return;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if ( (handle = open (reloadname,O_RDONLY | O_BINARY)) == -1)
        // 	I_Error ("W_Reload: couldn't open %s",reloadname);
        todo!("if statement not yet translated");
        read(
            handle,
            (&(header) as *const _ as *mut _),
            std::mem::size_of_val(&(header)),
        );
        lumpcount = LONG(header.numlumps);
        header.infotableofs = LONG(header.infotableofs);
        length = (lumpcount * std::mem::size_of::<filelump_t>());
        fileinfo = alloca(length);
        lseek(handle, header.infotableofs, SEEK_SET);
        read(handle, fileinfo, length);
        lump_p = (&(lumpinfo[(reloadlump) as usize]) as *const _ as *mut _);
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=reloadlump ;
        // 	 i<reloadlump+lumpcount ;
        // 	 i++,lump_p++, fileinfo++)
        //     {
        // 	if (lumpcache[i])
        // 	    Z_Free (lumpcache[i]);
        //
        // 	lump_p->position = LONG(fileinfo->filepos);
        // 	lump_p->size = LONG(fileinfo->size);
        //     }
        todo!("for statement not yet translated");
        close(handle);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn W_InitMultipleFiles(mut filenames: *mut *mut std::ffi::c_char) {
    unsafe {
        let mut size: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        numlumps = 0;
        lumpinfo = malloc(1);
        // TODO: for statement not yet translated:
        //
        //
        //     for ( ; *filenames ; filenames++)
        // 	W_AddFile (*filenames);
        todo!("for statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (!numlumps)
        // 	I_Error ("W_InitFiles: no files found");
        todo!("if statement not yet translated");
        size = (numlumps * std::mem::size_of_val(&(*(lumpcache))));
        lumpcache = malloc(size);
        // TODO: if statement not yet translated:
        //
        //
        //     if (!lumpcache)
        // 	I_Error ("Couldn't allocate lumpcache");
        todo!("if statement not yet translated");
        memset(lumpcache, 0, size);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn W_InitFile(mut filename: *mut std::ffi::c_char) {
    unsafe {
        let mut names: [*mut std::ffi::c_char; (2) as usize] = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        names[(0) as usize] = filename;
        names[(1) as usize] = NULL;
        W_InitMultipleFiles(names);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn W_NumLumps() -> std::ffi::c_int {
    unsafe {
        return numlumps;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn W_CheckNumForName(mut name: *mut std::ffi::c_char) -> std::ffi::c_int {
    unsafe {
        let mut name8: () = (); // TODO: unparsed local type, needs manual translation
        let mut v1: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut v2: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut lump_p: *mut lumpinfo_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        strncpy(name8.s, name, 8);
        name8.s[(8) as usize] = 0;
        strupr(name8.s);
        v1 = name8.x[(0) as usize];
        v2 = name8.x[(1) as usize];
        lump_p = (lumpinfo + numlumps);
        // TODO: while statement not yet translated:
        //
        //
        //     while (lump_p-- != lumpinfo)
        //     {
        // 	if ( *(int *)lump_p->name == v1
        // 	     && *(int *)&lump_p->name[4] == v2)
        // 	{
        // 	    return lump_p - lumpinfo;
        // 	}
        //     }
        todo!("while statement not yet translated");
        return (-(1));
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn W_GetNumForName(mut name: *mut std::ffi::c_char) -> std::ffi::c_int {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        i = W_CheckNumForName(name);
        // TODO: if statement not yet translated:
        //
        //
        //     if (i == -1)
        //       I_Error ("W_GetNumForName: %s not found!", name);
        todo!("if statement not yet translated");
        return i;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn W_LumpLength(mut lump: std::ffi::c_int) -> std::ffi::c_int {
    unsafe {
        // TODO: if statement not yet translated:
        //
        //     if (lump >= numlumps)
        // 	I_Error ("W_LumpLength: %i >= numlumps",lump);
        todo!("if statement not yet translated");
        return lumpinfo[(lump) as usize].size;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn W_ReadLump(mut lump: std::ffi::c_int, mut dest: *mut std::ffi::c_void) {
    unsafe {
        let mut c: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut l: *mut lumpinfo_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut handle: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if (lump >= numlumps)
        // 	I_Error ("W_ReadLump: %i >= numlumps",lump);
        todo!("if statement not yet translated");
        l = (lumpinfo + lump);
        // TODO: if statement not yet translated:
        //
        //
        //     // ??? I_BeginRead ();
        //
        //     if (l->handle == -1)
        //     {
        // 	// reloadable file, so use open / read / close
        // 	if ( (handle = open (reloadname,O_RDONLY | O_BINARY)) == -1)
        // 	    I_Error ("W_ReadLump: couldn't open %s",reloadname);
        //     }
        //     else
        // 	handle = l->handle;
        todo!("if statement not yet translated");
        lseek(handle, (*l).position, SEEK_SET);
        c = read(handle, dest, (*l).size);
        // TODO: if statement not yet translated:
        //
        //
        //     if (c < l->size)
        // 	I_Error ("W_ReadLump: only read %i of %i on lump %i",
        // 		 c,l->size,lump);
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (l->handle == -1)
        // 	close (handle);
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        //     // ??? I_EndRead ();
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn W_CacheLumpNum(
    mut lump: std::ffi::c_int,
    mut tag: std::ffi::c_int,
) -> *mut std::ffi::c_void {
    unsafe {
        let mut ptr: *mut byte = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: if statement not yet translated:
        //
        //
        //     if ((unsigned)lump >= numlumps)
        // 	I_Error ("W_CacheLumpNum: %i >= numlumps",lump);
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (!lumpcache[lump])
        //     {
        // 	// read the lump in
        //
        // 	//printf ("cache miss on lump %i\n",lump);
        // 	ptr = Z_Malloc (W_LumpLength (lump), tag, &lumpcache[lump]);
        // 	W_ReadLump (lump, lumpcache[lump]);
        //     }
        //     else
        //     {
        // 	//printf ("cache hit on lump %i\n",lump);
        // 	Z_ChangeTag (lumpcache[lump],tag);
        //     }
        todo!("if statement not yet translated");
        return lumpcache[(lump) as usize];
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn W_CacheLumpName(
    mut name: *mut std::ffi::c_char,
    mut tag: std::ffi::c_int,
) -> *mut std::ffi::c_void {
    unsafe {
        return W_CacheLumpNum(W_GetNumForName(name), tag);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub static mut info: [[std::ffi::c_int; (10) as usize]; (2500) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut profilecount: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn W_Profile() {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut block: *mut memblock_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut ptr: *mut std::ffi::c_void = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut ch: std::ffi::c_char = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        (FILE * f);
        let mut j: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut name: [std::ffi::c_char; (9) as usize] = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: for statement not yet translated:
        //
        //
        //
        //     for (i=0 ; i<numlumps ; i++)
        //     {
        // 	ptr = lumpcache[i];
        // 	if (!ptr)
        // 	{
        // 	    ch = ' ';
        // 	    continue;
        // 	}
        // 	else
        // 	{
        // 	    block = (memblock_t *) ( (byte *)ptr - sizeof(memblock_t));
        // 	    if (block->tag < PU_PURGELEVEL)
        // 		ch = 'S';
        // 	    else
        // 		ch = 'P';
        // 	}
        // 	info[i][profilecount] = ch;
        //     }
        todo!("for statement not yet translated");
        {
            let __macro_tmp = profilecount;
            profilecount += 1;
            __macro_tmp
        };
        f = fopen((c"waddump.txt").as_ptr(), (c"w").as_ptr());
        name[(8) as usize] = 0;
        // TODO: for statement not yet translated:
        //
        //
        //     for (i=0 ; i<numlumps ; i++)
        //     {
        // 	memcpy (name,lumpinfo[i].name,8);
        //
        // 	for (j=0 ; j<8 ; j++)
        // 	    if (!name[j])
        // 		break;
        //
        // 	for ( ; j<8 ; j++)
        // 	    name[j] = ' ';
        //
        // 	fprintf (f,"%s ",name);
        //
        // 	for (j=0 ; j<profilecount ; j++)
        // 	    fprintf (f,"    %c",info[i][j]);
        //
        // 	fprintf (f,"\n");
        //     }
        todo!("for statement not yet translated");
        fclose(f);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}
