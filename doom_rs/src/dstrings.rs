use crate::d_englsh::*;

pub const SAVEGAMENAME: *const std::ffi::c_char = (c"doomsav").as_ptr();

pub const DEVMAPS: *const std::ffi::c_char = (c"devmaps").as_ptr();

pub const DEVDATA: *const std::ffi::c_char = (c"devdata").as_ptr();

pub const NUM_QUITMESSAGES: std::ffi::c_int = 22;

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

pub static mut endmsg: [*mut std::ffi::c_char; (NUM_QUITMESSAGES + 1) as usize] = unsafe {
    [
        QUITMSG as *mut std::ffi::c_char,
        (c"please don't leave, there's more\ndemons to toast!").as_ptr() as *mut std::ffi::c_char,
        (c"let's beat it -- this is turning\ninto a bloodbath!").as_ptr() as *mut std::ffi::c_char,
        (c"i wouldn't leave if i were you.\ndos is much worse.").as_ptr() as *mut std::ffi::c_char,
        (c"you're trying to say you like dos\nbetter than me, right?").as_ptr()
            as *mut std::ffi::c_char,
        (c"don't leave yet -- there's a\ndemon around that corner!").as_ptr()
            as *mut std::ffi::c_char,
        (c"ya know, next time you come in here\ni'm gonna toast ya.").as_ptr()
            as *mut std::ffi::c_char,
        (c"go ahead and leave. see if i care.").as_ptr() as *mut std::ffi::c_char,
        (c"don't go now, there's a \ndimensional shambler waiting\nat the dos prompt!").as_ptr()
            as *mut std::ffi::c_char,
        (c"get outta here and go back\nto your boring programs.").as_ptr() as *mut std::ffi::c_char,
        (c"if i were your boss, i'd \n deathmatch ya in a minute!").as_ptr()
            as *mut std::ffi::c_char,
        (c"look, bud. you leave now\nand you forfeit your body count!").as_ptr()
            as *mut std::ffi::c_char,
        (c"just leave. when you come\nback, i'll be waiting with a bat.").as_ptr()
            as *mut std::ffi::c_char,
        (c"you're lucky i don't smack\nyou for thinking about leaving.").as_ptr()
            as *mut std::ffi::c_char,
        (c"you quit and i'll jizz\nin your cystholes!").as_ptr() as *mut std::ffi::c_char,
        (c"if you leave, i'll make\nthe lord drink my jizz.").as_ptr() as *mut std::ffi::c_char,
        (c"hey, ron! can we say\n'fuck' in the game?").as_ptr() as *mut std::ffi::c_char,
        (c"i'd leave: this is just\nmore monsters and levels.\nwhat a load.").as_ptr()
            as *mut std::ffi::c_char,
        (c"suck it down, asshole!\nyou're a fucking wimp!").as_ptr() as *mut std::ffi::c_char,
        (c"don't quit now! we're \nstill spending your money!").as_ptr() as *mut std::ffi::c_char,
        (c"THIS IS NO MESSAGE!\nPage intentionally left blank.").as_ptr() as *mut std::ffi::c_char,
    ]
};
