pub const VERSION: std::ffi::c_int = 110;

pub const shareware: std::ffi::c_int = 0;
pub const registered: std::ffi::c_int = shareware + 1;
pub const commercial: std::ffi::c_int = registered + 1;
pub const retail: std::ffi::c_int = commercial + 1;
pub const indetermined: std::ffi::c_int = retail + 1;

pub type GameMode_t = std::ffi::c_int;

pub const doom: std::ffi::c_int = 0;
pub const doom2: std::ffi::c_int = doom + 1;
pub const pack_tnt: std::ffi::c_int = doom2 + 1;
pub const pack_plut: std::ffi::c_int = pack_tnt + 1;
pub const none: std::ffi::c_int = pack_plut + 1;

pub type GameMission_t = std::ffi::c_int;

pub const english: std::ffi::c_int = 0;
pub const french: std::ffi::c_int = english + 1;
pub const german: std::ffi::c_int = french + 1;
pub const unknown: std::ffi::c_int = german + 1;

pub type Language_t = std::ffi::c_int;

pub const GS_LEVEL: std::ffi::c_int = 0;
pub const GS_INTERMISSION: std::ffi::c_int = GS_LEVEL + 1;
pub const GS_FINALE: std::ffi::c_int = GS_INTERMISSION + 1;
pub const GS_DEMOSCREEN: std::ffi::c_int = GS_FINALE + 1;

pub type gamestate_t = std::ffi::c_int;

pub const sk_baby: std::ffi::c_int = 0;
pub const sk_easy: std::ffi::c_int = sk_baby + 1;
pub const sk_medium: std::ffi::c_int = sk_easy + 1;
pub const sk_hard: std::ffi::c_int = sk_medium + 1;
pub const sk_nightmare: std::ffi::c_int = sk_hard + 1;

pub type skill_t = std::ffi::c_int;

pub const it_bluecard: std::ffi::c_int = 0;
pub const it_yellowcard: std::ffi::c_int = it_bluecard + 1;
pub const it_redcard: std::ffi::c_int = it_yellowcard + 1;
pub const it_blueskull: std::ffi::c_int = it_redcard + 1;
pub const it_yellowskull: std::ffi::c_int = it_blueskull + 1;
pub const it_redskull: std::ffi::c_int = it_yellowskull + 1;
pub const NUMCARDS: std::ffi::c_int = it_redskull + 1;

pub type card_t = std::ffi::c_int;

pub const wp_fist: std::ffi::c_int = 0;
pub const wp_pistol: std::ffi::c_int = wp_fist + 1;
pub const wp_shotgun: std::ffi::c_int = wp_pistol + 1;
pub const wp_chaingun: std::ffi::c_int = wp_shotgun + 1;
pub const wp_missile: std::ffi::c_int = wp_chaingun + 1;
pub const wp_plasma: std::ffi::c_int = wp_missile + 1;
pub const wp_bfg: std::ffi::c_int = wp_plasma + 1;
pub const wp_chainsaw: std::ffi::c_int = wp_bfg + 1;
pub const wp_supershotgun: std::ffi::c_int = wp_chainsaw + 1;
pub const NUMWEAPONS: std::ffi::c_int = wp_supershotgun + 1;
pub const wp_nochange: std::ffi::c_int = NUMWEAPONS + 1;

pub type weapontype_t = std::ffi::c_int;

pub const am_clip: std::ffi::c_int = 0;
pub const am_shell: std::ffi::c_int = am_clip + 1;
pub const am_cell: std::ffi::c_int = am_shell + 1;
pub const am_misl: std::ffi::c_int = am_cell + 1;
pub const NUMAMMO: std::ffi::c_int = am_misl + 1;
pub const am_noammo: std::ffi::c_int = NUMAMMO + 1;

pub type ammotype_t = std::ffi::c_int;

pub const pw_invulnerability: std::ffi::c_int = 0;
pub const pw_strength: std::ffi::c_int = pw_invulnerability + 1;
pub const pw_invisibility: std::ffi::c_int = pw_strength + 1;
pub const pw_ironfeet: std::ffi::c_int = pw_invisibility + 1;
pub const pw_allmap: std::ffi::c_int = pw_ironfeet + 1;
pub const pw_infrared: std::ffi::c_int = pw_allmap + 1;
pub const NUMPOWERS: std::ffi::c_int = pw_infrared + 1;

pub type powertype_t = std::ffi::c_int;

pub const INVULNTICS: std::ffi::c_int = (30 * TICRATE);
pub const INVISTICS: std::ffi::c_int = (60 * TICRATE);
pub const INFRATICS: std::ffi::c_int = (120 * TICRATE);
pub const IRONTICS: std::ffi::c_int = (60 * TICRATE);

pub type powerduration_t = std::ffi::c_int;

static mut rcsid: *mut std::ffi::c_char /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

/* TODO: unparsed C construct, needs manual translation:

// Location for any defines turned variables.

// None.



*/
