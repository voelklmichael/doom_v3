use crate::d_event::*;
use crate::d_ticcmd::*;
use crate::doomdef::*;
use crate::doomtype::*;
use crate::i_system::*;

pub const PU_STATIC: std::ffi::c_int = 1;

pub const PU_SOUND: std::ffi::c_int = 2;

pub const PU_MUSIC: std::ffi::c_int = 3;

pub const PU_DAVE: std::ffi::c_int = 4;

pub const PU_LEVEL: std::ffi::c_int = 50;

pub const PU_LEVSPEC: std::ffi::c_int = 51;

pub const PU_PURGELEVEL: std::ffi::c_int = 100;

pub const PU_CACHE: std::ffi::c_int = 101;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct memblock_t {
    pub size: std::ffi::c_int,
    pub user: *mut *mut std::ffi::c_void,
    pub tag: std::ffi::c_int,
    pub id: std::ffi::c_int,
    pub next: *mut memblock_s,
    pub prev: *mut memblock_s,
}

pub type memblock_s = memblock_t;

/* TODO: statement-shaped macro body, needs manual translation:
#define Z_ChangeTag(...) \
{ \
      if (( (memblock_t *)( (byte *)(p) - sizeof(memblock_t)))->id!=0x1d4a11) \
      I_Error("Z_CT at "__FILE__":%i",__LINE__); \
      Z_ChangeTag2(p,t); \
};
*/

static mut rcsid: [std::ffi::c_char; 49] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        122 as std::ffi::c_char,
        95 as std::ffi::c_char,
        122 as std::ffi::c_char,
        111 as std::ffi::c_char,
        110 as std::ffi::c_char,
        101 as std::ffi::c_char,
        46 as std::ffi::c_char,
        99 as std::ffi::c_char,
        44 as std::ffi::c_char,
        118 as std::ffi::c_char,
        32 as std::ffi::c_char,
        49 as std::ffi::c_char,
        46 as std::ffi::c_char,
        52 as std::ffi::c_char,
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
        56 as std::ffi::c_char,
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

pub const ZONEID: std::ffi::c_int = 0x1d4a11;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct memzone_t {
    pub size: std::ffi::c_int,
    pub blocklist: memblock_t,
    pub rover: *mut memblock_t,
}

pub static mut mainzone: *mut memzone_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn Z_ClearZone(mut zone: *mut memzone_t) {
    unsafe {
        let mut block: *mut memblock_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        (*zone).blocklist.next = (*zone).blocklist.prev =
            block = ((((zone) as *mut byte) + std::mem::size_of::<memzone_t>()) as *mut memblock_t);
        (*zone).blocklist.user = ((zone) as *mut std::ffi::c_void);
        (*zone).blocklist.tag = PU_STATIC;
        (*zone).rover = block;
        (*block).prev = (*block).next = (&((*zone).blocklist) as *const _ as *mut _);
        (*block).user = NULL;
        (*block).size = ((*zone).size - std::mem::size_of::<memzone_t>());
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn Z_Init() {
    unsafe {
        let mut block: *mut memblock_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut size: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        mainzone = ((I_ZoneBase((&(size) as *const _ as *mut _))) as *mut memzone_t);
        (*mainzone).size = size;
        (*mainzone).blocklist.next = (*mainzone).blocklist.prev = block =
            ((((mainzone) as *mut byte) + std::mem::size_of::<memzone_t>()) as *mut memblock_t);
        (*mainzone).blocklist.user = ((mainzone) as *mut std::ffi::c_void);
        (*mainzone).blocklist.tag = PU_STATIC;
        (*mainzone).rover = block;
        (*block).prev = (*block).next = (&((*mainzone).blocklist) as *const _ as *mut _);
        (*block).user = NULL;
        (*block).size = ((*mainzone).size - std::mem::size_of::<memzone_t>());
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn Z_Free(mut ptr: *mut std::ffi::c_void) {
    unsafe {
        let mut block: *mut memblock_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut other: *mut memblock_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        block = ((((ptr) as *mut byte) - std::mem::size_of::<memblock_t>()) as *mut memblock_t);
        // TODO: if statement not yet translated:
        //
        //
        //     if (block->id != ZONEID)
        // 	I_Error ("Z_Free: freed a pointer without ZONEID");
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (block->user > (void **)0x100)
        //     {
        // 	// smaller values are not pointers
        // 	// Note: OS-dependend?
        //
        // 	// clear the user's mark
        // 	*block->user = 0;
        //     }
        todo!("if statement not yet translated");
        (*block).user = NULL;
        (*block).tag = 0;
        (*block).id = 0;
        other = (*block).prev;
        // TODO: if statement not yet translated:
        //
        //
        //     if (!other->user)
        //     {
        // 	// merge with previous free block
        // 	other->size += block->size;
        // 	other->next = block->next;
        // 	other->next->prev = other;
        //
        // 	if (block == mainzone->rover)
        // 	    mainzone->rover = other;
        //
        // 	block = other;
        //     }
        todo!("if statement not yet translated");
        other = (*block).next;
        // TODO: if statement not yet translated:
        //
        //     if (!other->user)
        //     {
        // 	// merge the next free block onto the end
        // 	block->size += other->size;
        // 	block->next = other->next;
        // 	block->next->prev = block;
        //
        // 	if (other == mainzone->rover)
        // 	    mainzone->rover = block;
        //     }
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub const MINFRAGMENT: std::ffi::c_int = 64;

pub unsafe extern "C" fn Z_Malloc(
    mut size: std::ffi::c_int,
    mut tag: std::ffi::c_int,
    mut user: *mut std::ffi::c_void,
) -> *mut std::ffi::c_void {
    unsafe {
        let mut extra: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut start: *mut memblock_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut rover: *mut memblock_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut newblock: *mut memblock_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut base: *mut memblock_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        size = ((size + 3) & (!(3)));
        size += std::mem::size_of::<memblock_t>();
        base = (*mainzone).rover;
        // TODO: if statement not yet translated:
        //
        //
        //     if (!base->prev->user)
        // 	base = base->prev;
        todo!("if statement not yet translated");
        rover = base;
        start = (*base).prev;
        // TODO: do-while statement not yet translated:
        //
        //
        //     do
        //     {
        // 	if (rover == start)
        // 	{
        // 	    // scanned all the way around the list
        // 	    I_Error ("Z_Malloc: failed on allocation of %i bytes", size);
        // 	}
        //
        // 	if (rover->user)
        // 	{
        // 	    if (rover->tag < PU_PURGELEVEL)
        // 	    {
        // 		// hit a block that can't be purged,
        // 		//  so move base past it
        // 		base = rover = rover->next;
        // 	    }
        // 	    else
        // 	    {
        // 		// free the rover block (adding the size to base)
        //
        // 		// the rover can be the base block
        // 		base = base->prev;
        // 		Z_Free ((byte *)rover+sizeof(memblock_t));
        // 		base = base->next;
        // 		rover = base->next;
        // 	    }
        // 	}
        // 	else
        // 	    rover = rover->next;
        //     } while (base->user || base->size < size);
        todo!("do-while statement not yet translated");
        extra = ((*base).size - size);
        // TODO: if statement not yet translated:
        //
        //
        //     if (extra >  MINFRAGMENT)
        //     {
        // 	// there will be a free fragment after the allocated block
        // 	newblock = (memblock_t *) ((byte *)base + size );
        // 	newblock->size = extra;
        //
        // 	// NULL indicates free block.
        // 	newblock->user = NULL;
        // 	newblock->tag = 0;
        // 	newblock->prev = base;
        // 	newblock->next = base->next;
        // 	newblock->next->prev = newblock;
        //
        // 	base->next = newblock;
        // 	base->size = size;
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (user)
        //     {
        // 	// mark as an in use block
        // 	base->user = user;
        // 	*(void **)user = (void *) ((byte *)base + sizeof(memblock_t));
        //     }
        //     else
        //     {
        // 	if (tag >= PU_PURGELEVEL)
        // 	    I_Error ("Z_Malloc: an owner is required for purgable blocks");
        //
        // 	// mark as in use, but unowned
        // 	base->user = (void *)2;
        //     }
        todo!("if statement not yet translated");
        (*base).tag = tag;
        (*mainzone).rover = (*base).next;
        (*base).id = ZONEID;
        return ((((base) as *mut byte) + std::mem::size_of::<memblock_t>())
            as *mut std::ffi::c_void);
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn Z_FreeTags(mut lowtag: std::ffi::c_int, mut hightag: std::ffi::c_int) {
    unsafe {
        let mut block: *mut memblock_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut next: *mut memblock_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: for statement not yet translated:
        //
        //
        //     for (block = mainzone->blocklist.next ;
        // 	 block != &mainzone->blocklist ;
        // 	 block = next)
        //     {
        // 	// get link before freeing
        // 	next = block->next;
        //
        // 	// free block?
        // 	if (!block->user)
        // 	    continue;
        //
        // 	if (block->tag >= lowtag && block->tag <= hightag)
        // 	    Z_Free ( (byte *)block+sizeof(memblock_t));
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn Z_DumpHeap(mut lowtag: std::ffi::c_int, mut hightag: std::ffi::c_int) {
    unsafe {
        let mut block: *mut memblock_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        printf(
            (c"zone size: %i  location: %p\n").as_ptr(),
            (*mainzone).size,
            mainzone,
        );
        printf((c"tag range: %i to %i\n").as_ptr(), lowtag, hightag);
        // TODO: for statement not yet translated:
        //
        //
        //     for (block = mainzone->blocklist.next ; ; block = block->next)
        //     {
        // 	if (block->tag >= lowtag && block->tag <= hightag)
        // 	    printf ("block:%p    size:%7i    user:%p    tag:%3i\n",
        // 		    block, block->size, block->user, block->tag);
        //
        // 	if (block->next == &mainzone->blocklist)
        // 	{
        // 	    // all blocks have been hit
        // 	    break;
        // 	}
        //
        // 	if ( (byte *)block + block->size != (byte *)block->next)
        // 	    printf ("ERROR: block size does not touch the next block\n");
        //
        // 	if ( block->next->prev != block)
        // 	    printf ("ERROR: next block doesn't have proper back link\n");
        //
        // 	if (!block->user && !block->next->user)
        // 	    printf ("ERROR: two consecutive free blocks\n");
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn Z_FileDumpHeap(mut f: *mut libc::FILE) {
    unsafe {
        let mut block: *mut memblock_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        fprintf(
            f,
            (c"zone size: %i  location: %p\n").as_ptr(),
            (*mainzone).size,
            mainzone,
        );
        // TODO: for statement not yet translated:
        //
        //
        //     for (block = mainzone->blocklist.next ; ; block = block->next)
        //     {
        // 	fprintf (f,"block:%p    size:%7i    user:%p    tag:%3i\n",
        // 		 block, block->size, block->user, block->tag);
        //
        // 	if (block->next == &mainzone->blocklist)
        // 	{
        // 	    // all blocks have been hit
        // 	    break;
        // 	}
        //
        // 	if ( (byte *)block + block->size != (byte *)block->next)
        // 	    fprintf (f,"ERROR: block size does not touch the next block\n");
        //
        // 	if ( block->next->prev != block)
        // 	    fprintf (f,"ERROR: next block doesn't have proper back link\n");
        //
        // 	if (!block->user && !block->next->user)
        // 	    fprintf (f,"ERROR: two consecutive free blocks\n");
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn Z_CheckHeap() {
    unsafe {
        let mut block: *mut memblock_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        // TODO: for statement not yet translated:
        //
        //
        //     for (block = mainzone->blocklist.next ; ; block = block->next)
        //     {
        // 	if (block->next == &mainzone->blocklist)
        // 	{
        // 	    // all blocks have been hit
        // 	    break;
        // 	}
        //
        // 	if ( (byte *)block + block->size != (byte *)block->next)
        // 	    I_Error ("Z_CheckHeap: block size does not touch the next block\n");
        //
        // 	if ( block->next->prev != block)
        // 	    I_Error ("Z_CheckHeap: next block doesn't have proper back link\n");
        //
        // 	if (!block->user && !block->next->user)
        // 	    I_Error ("Z_CheckHeap: two consecutive free blocks\n");
        //     }
        todo!("for statement not yet translated");
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn Z_ChangeTag2(mut ptr: *mut std::ffi::c_void, mut tag: std::ffi::c_int) {
    unsafe {
        let mut block: *mut memblock_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        block = ((((ptr) as *mut byte) - std::mem::size_of::<memblock_t>()) as *mut memblock_t);
        // TODO: if statement not yet translated:
        //
        //
        //     if (block->id != ZONEID)
        // 	I_Error ("Z_ChangeTag: freed a pointer without ZONEID");
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (tag >= PU_PURGELEVEL && (unsigned)block->user < 0x100)
        // 	I_Error ("Z_ChangeTag: an owner is required for purgable blocks");
        todo!("if statement not yet translated");
        (*block).tag = tag;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
}

pub unsafe extern "C" fn Z_FreeMemory() -> std::ffi::c_int {
    unsafe {
        let mut block: *mut memblock_t = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut free: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        free = 0;
        // TODO: for statement not yet translated:
        //
        //
        //     for (block = mainzone->blocklist.next ;
        // 	 block != &mainzone->blocklist;
        // 	 block = block->next)
        //     {
        // 	if (!block->user || block->tag >= PU_PURGELEVEL)
        // 	    free += block->size;
        //     }
        todo!("for statement not yet translated");
        return free;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}
