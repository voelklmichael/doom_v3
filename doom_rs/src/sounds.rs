use crate::doomtype::*;

pub type sfxinfo_t = sfxinfo_struct;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfxinfo_struct {
    pub name: *mut std::ffi::c_char,
    pub singularity: std::ffi::c_int,
    pub priority: std::ffi::c_int,
    pub link: *mut sfxinfo_t,
    pub pitch: std::ffi::c_int,
    pub volume: std::ffi::c_int,
    pub data: *mut std::ffi::c_void,
    pub usefulness: std::ffi::c_int,
    pub lumpnum: std::ffi::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct musicinfo_t {
    pub name: *mut std::ffi::c_char,
    pub lumpnum: std::ffi::c_int,
    pub data: *mut std::ffi::c_void,
    pub handle: std::ffi::c_int,
}

pub const mus_None: std::ffi::c_int = 0;
pub const mus_e1m1: std::ffi::c_int = mus_None + 1;
pub const mus_e1m2: std::ffi::c_int = mus_e1m1 + 1;
pub const mus_e1m3: std::ffi::c_int = mus_e1m2 + 1;
pub const mus_e1m4: std::ffi::c_int = mus_e1m3 + 1;
pub const mus_e1m5: std::ffi::c_int = mus_e1m4 + 1;
pub const mus_e1m6: std::ffi::c_int = mus_e1m5 + 1;
pub const mus_e1m7: std::ffi::c_int = mus_e1m6 + 1;
pub const mus_e1m8: std::ffi::c_int = mus_e1m7 + 1;
pub const mus_e1m9: std::ffi::c_int = mus_e1m8 + 1;
pub const mus_e2m1: std::ffi::c_int = mus_e1m9 + 1;
pub const mus_e2m2: std::ffi::c_int = mus_e2m1 + 1;
pub const mus_e2m3: std::ffi::c_int = mus_e2m2 + 1;
pub const mus_e2m4: std::ffi::c_int = mus_e2m3 + 1;
pub const mus_e2m5: std::ffi::c_int = mus_e2m4 + 1;
pub const mus_e2m6: std::ffi::c_int = mus_e2m5 + 1;
pub const mus_e2m7: std::ffi::c_int = mus_e2m6 + 1;
pub const mus_e2m8: std::ffi::c_int = mus_e2m7 + 1;
pub const mus_e2m9: std::ffi::c_int = mus_e2m8 + 1;
pub const mus_e3m1: std::ffi::c_int = mus_e2m9 + 1;
pub const mus_e3m2: std::ffi::c_int = mus_e3m1 + 1;
pub const mus_e3m3: std::ffi::c_int = mus_e3m2 + 1;
pub const mus_e3m4: std::ffi::c_int = mus_e3m3 + 1;
pub const mus_e3m5: std::ffi::c_int = mus_e3m4 + 1;
pub const mus_e3m6: std::ffi::c_int = mus_e3m5 + 1;
pub const mus_e3m7: std::ffi::c_int = mus_e3m6 + 1;
pub const mus_e3m8: std::ffi::c_int = mus_e3m7 + 1;
pub const mus_e3m9: std::ffi::c_int = mus_e3m8 + 1;
pub const mus_inter: std::ffi::c_int = mus_e3m9 + 1;
pub const mus_intro: std::ffi::c_int = mus_inter + 1;
pub const mus_bunny: std::ffi::c_int = mus_intro + 1;
pub const mus_victor: std::ffi::c_int = mus_bunny + 1;
pub const mus_introa: std::ffi::c_int = mus_victor + 1;
pub const mus_runnin: std::ffi::c_int = mus_introa + 1;
pub const mus_stalks: std::ffi::c_int = mus_runnin + 1;
pub const mus_countd: std::ffi::c_int = mus_stalks + 1;
pub const mus_betwee: std::ffi::c_int = mus_countd + 1;
pub const mus_doom: std::ffi::c_int = mus_betwee + 1;
pub const mus_the_da: std::ffi::c_int = mus_doom + 1;
pub const mus_shawn: std::ffi::c_int = mus_the_da + 1;
pub const mus_ddtblu: std::ffi::c_int = mus_shawn + 1;
pub const mus_in_cit: std::ffi::c_int = mus_ddtblu + 1;
pub const mus_dead: std::ffi::c_int = mus_in_cit + 1;
pub const mus_stlks2: std::ffi::c_int = mus_dead + 1;
pub const mus_theda2: std::ffi::c_int = mus_stlks2 + 1;
pub const mus_doom2: std::ffi::c_int = mus_theda2 + 1;
pub const mus_ddtbl2: std::ffi::c_int = mus_doom2 + 1;
pub const mus_runni2: std::ffi::c_int = mus_ddtbl2 + 1;
pub const mus_dead2: std::ffi::c_int = mus_runni2 + 1;
pub const mus_stlks3: std::ffi::c_int = mus_dead2 + 1;
pub const mus_romero: std::ffi::c_int = mus_stlks3 + 1;
pub const mus_shawn2: std::ffi::c_int = mus_romero + 1;
pub const mus_messag: std::ffi::c_int = mus_shawn2 + 1;
pub const mus_count2: std::ffi::c_int = mus_messag + 1;
pub const mus_ddtbl3: std::ffi::c_int = mus_count2 + 1;
pub const mus_ampie: std::ffi::c_int = mus_ddtbl3 + 1;
pub const mus_theda3: std::ffi::c_int = mus_ampie + 1;
pub const mus_adrian: std::ffi::c_int = mus_theda3 + 1;
pub const mus_messg2: std::ffi::c_int = mus_adrian + 1;
pub const mus_romer2: std::ffi::c_int = mus_messg2 + 1;
pub const mus_tense: std::ffi::c_int = mus_romer2 + 1;
pub const mus_shawn3: std::ffi::c_int = mus_tense + 1;
pub const mus_openin: std::ffi::c_int = mus_shawn3 + 1;
pub const mus_evil: std::ffi::c_int = mus_openin + 1;
pub const mus_ultima: std::ffi::c_int = mus_evil + 1;
pub const mus_read_m: std::ffi::c_int = mus_ultima + 1;
pub const mus_dm2ttl: std::ffi::c_int = mus_read_m + 1;
pub const mus_dm2int: std::ffi::c_int = mus_dm2ttl + 1;
pub const NUMMUSIC: std::ffi::c_int = mus_dm2int + 1;

pub type musicenum_t = std::ffi::c_int;

pub const sfx_None: std::ffi::c_int = 0;
pub const sfx_pistol: std::ffi::c_int = sfx_None + 1;
pub const sfx_shotgn: std::ffi::c_int = sfx_pistol + 1;
pub const sfx_sgcock: std::ffi::c_int = sfx_shotgn + 1;
pub const sfx_dshtgn: std::ffi::c_int = sfx_sgcock + 1;
pub const sfx_dbopn: std::ffi::c_int = sfx_dshtgn + 1;
pub const sfx_dbcls: std::ffi::c_int = sfx_dbopn + 1;
pub const sfx_dbload: std::ffi::c_int = sfx_dbcls + 1;
pub const sfx_plasma: std::ffi::c_int = sfx_dbload + 1;
pub const sfx_bfg: std::ffi::c_int = sfx_plasma + 1;
pub const sfx_sawup: std::ffi::c_int = sfx_bfg + 1;
pub const sfx_sawidl: std::ffi::c_int = sfx_sawup + 1;
pub const sfx_sawful: std::ffi::c_int = sfx_sawidl + 1;
pub const sfx_sawhit: std::ffi::c_int = sfx_sawful + 1;
pub const sfx_rlaunc: std::ffi::c_int = sfx_sawhit + 1;
pub const sfx_rxplod: std::ffi::c_int = sfx_rlaunc + 1;
pub const sfx_firsht: std::ffi::c_int = sfx_rxplod + 1;
pub const sfx_firxpl: std::ffi::c_int = sfx_firsht + 1;
pub const sfx_pstart: std::ffi::c_int = sfx_firxpl + 1;
pub const sfx_pstop: std::ffi::c_int = sfx_pstart + 1;
pub const sfx_doropn: std::ffi::c_int = sfx_pstop + 1;
pub const sfx_dorcls: std::ffi::c_int = sfx_doropn + 1;
pub const sfx_stnmov: std::ffi::c_int = sfx_dorcls + 1;
pub const sfx_swtchn: std::ffi::c_int = sfx_stnmov + 1;
pub const sfx_swtchx: std::ffi::c_int = sfx_swtchn + 1;
pub const sfx_plpain: std::ffi::c_int = sfx_swtchx + 1;
pub const sfx_dmpain: std::ffi::c_int = sfx_plpain + 1;
pub const sfx_popain: std::ffi::c_int = sfx_dmpain + 1;
pub const sfx_vipain: std::ffi::c_int = sfx_popain + 1;
pub const sfx_mnpain: std::ffi::c_int = sfx_vipain + 1;
pub const sfx_pepain: std::ffi::c_int = sfx_mnpain + 1;
pub const sfx_slop: std::ffi::c_int = sfx_pepain + 1;
pub const sfx_itemup: std::ffi::c_int = sfx_slop + 1;
pub const sfx_wpnup: std::ffi::c_int = sfx_itemup + 1;
pub const sfx_oof: std::ffi::c_int = sfx_wpnup + 1;
pub const sfx_telept: std::ffi::c_int = sfx_oof + 1;
pub const sfx_posit1: std::ffi::c_int = sfx_telept + 1;
pub const sfx_posit2: std::ffi::c_int = sfx_posit1 + 1;
pub const sfx_posit3: std::ffi::c_int = sfx_posit2 + 1;
pub const sfx_bgsit1: std::ffi::c_int = sfx_posit3 + 1;
pub const sfx_bgsit2: std::ffi::c_int = sfx_bgsit1 + 1;
pub const sfx_sgtsit: std::ffi::c_int = sfx_bgsit2 + 1;
pub const sfx_cacsit: std::ffi::c_int = sfx_sgtsit + 1;
pub const sfx_brssit: std::ffi::c_int = sfx_cacsit + 1;
pub const sfx_cybsit: std::ffi::c_int = sfx_brssit + 1;
pub const sfx_spisit: std::ffi::c_int = sfx_cybsit + 1;
pub const sfx_bspsit: std::ffi::c_int = sfx_spisit + 1;
pub const sfx_kntsit: std::ffi::c_int = sfx_bspsit + 1;
pub const sfx_vilsit: std::ffi::c_int = sfx_kntsit + 1;
pub const sfx_mansit: std::ffi::c_int = sfx_vilsit + 1;
pub const sfx_pesit: std::ffi::c_int = sfx_mansit + 1;
pub const sfx_sklatk: std::ffi::c_int = sfx_pesit + 1;
pub const sfx_sgtatk: std::ffi::c_int = sfx_sklatk + 1;
pub const sfx_skepch: std::ffi::c_int = sfx_sgtatk + 1;
pub const sfx_vilatk: std::ffi::c_int = sfx_skepch + 1;
pub const sfx_claw: std::ffi::c_int = sfx_vilatk + 1;
pub const sfx_skeswg: std::ffi::c_int = sfx_claw + 1;
pub const sfx_pldeth: std::ffi::c_int = sfx_skeswg + 1;
pub const sfx_pdiehi: std::ffi::c_int = sfx_pldeth + 1;
pub const sfx_podth1: std::ffi::c_int = sfx_pdiehi + 1;
pub const sfx_podth2: std::ffi::c_int = sfx_podth1 + 1;
pub const sfx_podth3: std::ffi::c_int = sfx_podth2 + 1;
pub const sfx_bgdth1: std::ffi::c_int = sfx_podth3 + 1;
pub const sfx_bgdth2: std::ffi::c_int = sfx_bgdth1 + 1;
pub const sfx_sgtdth: std::ffi::c_int = sfx_bgdth2 + 1;
pub const sfx_cacdth: std::ffi::c_int = sfx_sgtdth + 1;
pub const sfx_skldth: std::ffi::c_int = sfx_cacdth + 1;
pub const sfx_brsdth: std::ffi::c_int = sfx_skldth + 1;
pub const sfx_cybdth: std::ffi::c_int = sfx_brsdth + 1;
pub const sfx_spidth: std::ffi::c_int = sfx_cybdth + 1;
pub const sfx_bspdth: std::ffi::c_int = sfx_spidth + 1;
pub const sfx_vildth: std::ffi::c_int = sfx_bspdth + 1;
pub const sfx_kntdth: std::ffi::c_int = sfx_vildth + 1;
pub const sfx_pedth: std::ffi::c_int = sfx_kntdth + 1;
pub const sfx_skedth: std::ffi::c_int = sfx_pedth + 1;
pub const sfx_posact: std::ffi::c_int = sfx_skedth + 1;
pub const sfx_bgact: std::ffi::c_int = sfx_posact + 1;
pub const sfx_dmact: std::ffi::c_int = sfx_bgact + 1;
pub const sfx_bspact: std::ffi::c_int = sfx_dmact + 1;
pub const sfx_bspwlk: std::ffi::c_int = sfx_bspact + 1;
pub const sfx_vilact: std::ffi::c_int = sfx_bspwlk + 1;
pub const sfx_noway: std::ffi::c_int = sfx_vilact + 1;
pub const sfx_barexp: std::ffi::c_int = sfx_noway + 1;
pub const sfx_punch: std::ffi::c_int = sfx_barexp + 1;
pub const sfx_hoof: std::ffi::c_int = sfx_punch + 1;
pub const sfx_metal: std::ffi::c_int = sfx_hoof + 1;
pub const sfx_chgun: std::ffi::c_int = sfx_metal + 1;
pub const sfx_tink: std::ffi::c_int = sfx_chgun + 1;
pub const sfx_bdopn: std::ffi::c_int = sfx_tink + 1;
pub const sfx_bdcls: std::ffi::c_int = sfx_bdopn + 1;
pub const sfx_itmbk: std::ffi::c_int = sfx_bdcls + 1;
pub const sfx_flame: std::ffi::c_int = sfx_itmbk + 1;
pub const sfx_flamst: std::ffi::c_int = sfx_flame + 1;
pub const sfx_getpow: std::ffi::c_int = sfx_flamst + 1;
pub const sfx_bospit: std::ffi::c_int = sfx_getpow + 1;
pub const sfx_boscub: std::ffi::c_int = sfx_bospit + 1;
pub const sfx_bossit: std::ffi::c_int = sfx_boscub + 1;
pub const sfx_bospn: std::ffi::c_int = sfx_bossit + 1;
pub const sfx_bosdth: std::ffi::c_int = sfx_bospn + 1;
pub const sfx_manatk: std::ffi::c_int = sfx_bosdth + 1;
pub const sfx_mandth: std::ffi::c_int = sfx_manatk + 1;
pub const sfx_sssit: std::ffi::c_int = sfx_mandth + 1;
pub const sfx_ssdth: std::ffi::c_int = sfx_sssit + 1;
pub const sfx_keenpn: std::ffi::c_int = sfx_ssdth + 1;
pub const sfx_keendt: std::ffi::c_int = sfx_keenpn + 1;
pub const sfx_skeact: std::ffi::c_int = sfx_keendt + 1;
pub const sfx_skesit: std::ffi::c_int = sfx_skeact + 1;
pub const sfx_skeatk: std::ffi::c_int = sfx_skesit + 1;
pub const sfx_radio: std::ffi::c_int = sfx_skeatk + 1;
pub const NUMSFX: std::ffi::c_int = sfx_radio + 1;

pub type sfxenum_t = std::ffi::c_int;

static mut rcsid: [std::ffi::c_char; 49] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        115 as std::ffi::c_char,
        111 as std::ffi::c_char,
        117 as std::ffi::c_char,
        110 as std::ffi::c_char,
        100 as std::ffi::c_char,
        115 as std::ffi::c_char,
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
        49 as std::ffi::c_char,
        47 as std::ffi::c_char,
        50 as std::ffi::c_char,
        57 as std::ffi::c_char,
        32 as std::ffi::c_char,
        50 as std::ffi::c_char,
        50 as std::ffi::c_char,
        58 as std::ffi::c_char,
        52 as std::ffi::c_char,
        48 as std::ffi::c_char,
        58 as std::ffi::c_char,
        52 as std::ffi::c_char,
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

pub static mut S_music: [musicinfo_t; 68] = unsafe {
    [
        musicinfo_t {
            name: std::ptr::null_mut(),
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"e1m1").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"e1m2").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"e1m3").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"e1m4").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"e1m5").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"e1m6").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"e1m7").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"e1m8").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"e1m9").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"e2m1").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"e2m2").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"e2m3").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"e2m4").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"e2m5").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"e2m6").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"e2m7").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"e2m8").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"e2m9").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"e3m1").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"e3m2").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"e3m3").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"e3m4").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"e3m5").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"e3m6").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"e3m7").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"e3m8").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"e3m9").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"inter").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"intro").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"bunny").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"victor").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"introa").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"runnin").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"stalks").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"countd").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"betwee").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"doom").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"the_da").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"shawn").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"ddtblu").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"in_cit").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"dead").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"stlks2").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"theda2").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"doom2").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"ddtbl2").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"runni2").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"dead2").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"stlks3").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"romero").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"shawn2").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"messag").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"count2").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"ddtbl3").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"ampie").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"theda3").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"adrian").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"messg2").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"romer2").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"tense").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"shawn3").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"openin").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"evil").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"ultima").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"read_m").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"dm2ttl").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
        musicinfo_t {
            name: (c"dm2int").as_ptr() as *mut std::ffi::c_char,
            lumpnum: 0,
            ..ZEROED_musicinfo_t
        },
    ]
};

pub static mut S_sfx: *mut sfxinfo_t /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

const ZEROED_musicinfo_t: musicinfo_t = unsafe { std::mem::zeroed() };
