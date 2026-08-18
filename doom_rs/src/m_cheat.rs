pub unsafe extern "C" fn SCRAMBLE(a: std::ffi::c_int) -> std::ffi::c_int {
    ((((((((((a) & 1) << 7) + (((a) & 2) << 5)) + ((a) & 4)) + (((a) & 8) << 1))
        + (((a) & 16) >> 1))
        + ((a) & 32))
        + (((a) & 64) >> 5))
        + (((a) & 128) >> 7))
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cheatseq_t {
    pub sequence: *mut std::ffi::c_uchar,
    pub p: *mut std::ffi::c_uchar,
}

static mut rcsid: [std::ffi::c_char; 50] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        109 as std::ffi::c_char,
        95 as std::ffi::c_char,
        99 as std::ffi::c_char,
        104 as std::ffi::c_char,
        101 as std::ffi::c_char,
        97 as std::ffi::c_char,
        116 as std::ffi::c_char,
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
        49 as std::ffi::c_char,
        58 as std::ffi::c_char,
        50 as std::ffi::c_char,
        52 as std::ffi::c_char,
        58 as std::ffi::c_char,
        51 as std::ffi::c_char,
        52 as std::ffi::c_char,
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

static mut firsttime: std::ffi::c_int = unsafe { 1 };

static mut cheat_xlate_table: [std::ffi::c_uchar; (256) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn cht_CheckCheat(
    mut cht: *mut cheatseq_t,
    mut key: std::ffi::c_char,
) -> std::ffi::c_int {
    unsafe {
        let mut i: std::ffi::c_int = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut rc: std::ffi::c_int = unsafe { 0 };
        // TODO: if statement not yet translated:
        //
        //
        //     if (firsttime)
        //     {
        // 	firsttime = 0;
        // 	for (i=0;i<256;i++) cheat_xlate_table[i] = SCRAMBLE(i);
        //     }
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (!cht->p)
        // 	cht->p = cht->sequence; // initialize if first time
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //     if (*cht->p == 0)
        // 	*(cht->p++) = key;
        //     else if
        // 	(cheat_xlate_table[(unsigned char)key] == *cht->p) cht->p++;
        //     else
        // 	cht->p = cht->sequence;
        todo!("if statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (*cht->p == 1)
        // 	cht->p++;
        //     else if (*cht->p == 0xff) // end of sequence character
        //     {
        // 	cht->p = cht->sequence;
        // 	rc = 1;
        //     }
        todo!("if statement not yet translated");
        return rc;
        // TODO: statement not yet translated:
        //
        todo!("statement not yet translated");
    }
    todo!("fell off the end of a non-void C function")
}

pub unsafe extern "C" fn cht_GetParam(mut cht: *mut cheatseq_t, mut buffer: *mut std::ffi::c_char) {
    unsafe {
        let mut p: *mut std::ffi::c_uchar = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        let mut c: std::ffi::c_uchar = unsafe {
            std::mem::zeroed() /* TODO: initializer not yet translated */
        };
        p = (*cht).sequence;
        // TODO: while statement not yet translated:
        //
        //     while (*(p++) != 1);
        todo!("while statement not yet translated");
        // TODO: do-while statement not yet translated:
        //
        //
        //     do
        //     {
        // 	c = *p;
        // 	*(buffer++) = c;
        // 	*(p++) = 0;
        //     }
        //     while (c && *p!=0xff );
        todo!("do-while statement not yet translated");
        // TODO: if statement not yet translated:
        //
        //
        //     if (*p==0xff)
        // 	*buffer = 0;
        todo!("if statement not yet translated");
        // TODO: statement not yet translated:
        //
        //
        todo!("statement not yet translated");
    }
}
