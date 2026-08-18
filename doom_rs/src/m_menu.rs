use crate::d_englsh::*;
use crate::d_event::*;
use crate::d_items::*;
use crate::d_main::*;
use crate::d_net::*;
use crate::d_player::*;
use crate::d_think::*;
use crate::d_ticcmd::*;
use crate::doomdata::*;
use crate::doomdef::*;
use crate::doomstat::*;
use crate::doomtype::*;
use crate::dstrings::*;
use crate::g_game::*;
use crate::hu_stuff::*;
use crate::i_system::*;
use crate::i_video::*;
use crate::info::*;
use crate::m_argv::*;
use crate::m_fixed::*;
use crate::m_swap::*;
use crate::p_mobj::*;
use crate::p_pspr::*;
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
use crate::s_sound::*;
use crate::sounds::*;
use crate::tables::*;
use crate::v_video::*;
use crate::w_wad::*;
use crate::z_zone::*;

static mut rcsid: [std::ffi::c_char; 49] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        109 as std::ffi::c_char,
        95 as std::ffi::c_char,
        109 as std::ffi::c_char,
        101 as std::ffi::c_char,
        110 as std::ffi::c_char,
        117 as std::ffi::c_char,
        46 as std::ffi::c_char,
        99 as std::ffi::c_char,
        44 as std::ffi::c_char,
        118 as std::ffi::c_char,
        32 as std::ffi::c_char,
        49 as std::ffi::c_char,
        46 as std::ffi::c_char,
        55 as std::ffi::c_char,
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

unsafe extern "C" {
    pub static mut hu_font: [*mut patch_t; (HU_FONTSIZE) as usize];
}

unsafe extern "C" {
    pub static mut message_dontfuckwithme: boolean;
}

unsafe extern "C" {
    pub static mut chat_on: boolean;
}

pub static mut mouseSensitivity: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut showMessages: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut detailLevel: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut screenblocks: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut screenSize: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut quickSaveSlot: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut messageToPrint: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut messageString: *mut std::ffi::c_char = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut messx: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut messy: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut messageLastMenuActive: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut messageNeedsInput: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut messageRoutine: Option<unsafe extern "C" fn(std::ffi::c_int)> =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub const SAVESTRINGSIZE: std::ffi::c_int = 24;

pub static mut gammamsg: [[std::ffi::c_char; (26) as usize]; (5) as usize] =
    unsafe { [GAMMALVL0, GAMMALVL1, GAMMALVL2, GAMMALVL3, GAMMALVL4] };

pub static mut saveStringEnter: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut saveSlot: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut saveCharIndex: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut saveOldString: [std::ffi::c_char; (SAVESTRINGSIZE) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut inhelpscreens: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut menuactive: boolean = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub const SKULLXOFF: std::ffi::c_int = (-(32));

pub const LINEHEIGHT: std::ffi::c_int = 16;

unsafe extern "C" {
    pub static mut sendpause: boolean;
}

pub static mut savegamestrings: [[std::ffi::c_char; (SAVESTRINGSIZE) as usize]; (10) as usize] =
    unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut endstring: [std::ffi::c_char; (160) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

#[repr(C)]
#[derive(Copy, Clone)]
pub struct menuitem_t {
    pub status: std::ffi::c_short,
    pub name: [std::ffi::c_char; (10) as usize],
    pub routine: Option<unsafe extern "C" fn(std::ffi::c_int)>,
    pub alphaKey: std::ffi::c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct menu_t {
    pub numitems: std::ffi::c_short,
    pub prevMenu: *mut menu_s,
    pub menuitems: *mut menuitem_t,
    pub routine: Option<unsafe extern "C" fn()>,
    pub x: std::ffi::c_short,
    pub y: std::ffi::c_short,
    pub lastOn: std::ffi::c_short,
}

pub type menu_s = menu_t;

pub static mut itemOn: std::ffi::c_short = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut skullAnimCounter: std::ffi::c_short = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut whichSkull: std::ffi::c_short = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut skullName: [[std::ffi::c_char; (9) as usize]; (2) as usize] =
    unsafe { [(c"M_SKULL1").as_ptr(), (c"M_SKULL2").as_ptr()] };

pub static mut currentMenu: *mut menu_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

unsafe extern "C" {
    pub fn M_StartGame(choice: std::ffi::c_int);
}

pub const newgame: std::ffi::c_int = 0;
pub const options: std::ffi::c_int = newgame + 1;
pub const loadgame: std::ffi::c_int = options + 1;
pub const savegame: std::ffi::c_int = loadgame + 1;
pub const readthis: std::ffi::c_int = savegame + 1;
pub const quitdoom: std::ffi::c_int = readthis + 1;
pub const main_end: std::ffi::c_int = quitdoom + 1;

pub type main_e = std::ffi::c_int;

pub static mut MainMenu: *mut menuitem_t /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut MainDef: menu_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub const ep1: std::ffi::c_int = 0;
pub const ep2: std::ffi::c_int = ep1 + 1;
pub const ep3: std::ffi::c_int = ep2 + 1;
pub const ep4: std::ffi::c_int = ep3 + 1;
pub const ep_end: std::ffi::c_int = ep4 + 1;

pub type episodes_e = std::ffi::c_int;

pub static mut EpisodeMenu: *mut menuitem_t /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut EpiDef: menu_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub const killthings: std::ffi::c_int = 0;
pub const toorough: std::ffi::c_int = killthings + 1;
pub const hurtme: std::ffi::c_int = toorough + 1;
pub const violence: std::ffi::c_int = hurtme + 1;
pub const nightmare: std::ffi::c_int = violence + 1;
pub const newg_end: std::ffi::c_int = nightmare + 1;

pub type newgame_e = std::ffi::c_int;

pub static mut NewGameMenu: *mut menuitem_t /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut NewDef: menu_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub const endgame: std::ffi::c_int = 0;
pub const messages: std::ffi::c_int = endgame + 1;
pub const detail: std::ffi::c_int = messages + 1;
pub const scrnsize: std::ffi::c_int = detail + 1;
pub const option_empty1: std::ffi::c_int = scrnsize + 1;
pub const mousesens: std::ffi::c_int = option_empty1 + 1;
pub const option_empty2: std::ffi::c_int = mousesens + 1;
pub const soundvol: std::ffi::c_int = option_empty2 + 1;
pub const opt_end: std::ffi::c_int = soundvol + 1;

pub type options_e = std::ffi::c_int;

pub static mut OptionsMenu: *mut menuitem_t /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut OptionsDef: menu_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub const rdthsempty1: std::ffi::c_int = 0;
pub const read1_end: std::ffi::c_int = rdthsempty1 + 1;

pub type read_e = std::ffi::c_int;

pub static mut ReadMenu1: *mut menuitem_t /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut ReadDef1: menu_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub const rdthsempty2: std::ffi::c_int = 0;
pub const read2_end: std::ffi::c_int = rdthsempty2 + 1;

pub type read_e2 = std::ffi::c_int;

pub static mut ReadMenu2: *mut menuitem_t /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut ReadDef2: menu_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub const sfx_vol: std::ffi::c_int = 0;
pub const sfx_empty1: std::ffi::c_int = sfx_vol + 1;
pub const music_vol: std::ffi::c_int = sfx_empty1 + 1;
pub const sfx_empty2: std::ffi::c_int = music_vol + 1;
pub const sound_end: std::ffi::c_int = sfx_empty2 + 1;

pub type sound_e = std::ffi::c_int;

pub static mut SoundMenu: *mut menuitem_t /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut SoundDef: menu_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub const load1: std::ffi::c_int = 0;
pub const load2: std::ffi::c_int = load1 + 1;
pub const load3: std::ffi::c_int = load2 + 1;
pub const load4: std::ffi::c_int = load3 + 1;
pub const load5: std::ffi::c_int = load4 + 1;
pub const load6: std::ffi::c_int = load5 + 1;
pub const load_end: std::ffi::c_int = load6 + 1;

pub type load_e = std::ffi::c_int;

pub static mut LoadMenu: *mut menuitem_t /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut LoadDef: menu_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut SaveMenu: *mut menuitem_t /* TODO: was unsized array */ = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub static mut SaveDef: menu_t = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn M_ReadSaveStrings() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_DrawLoad() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_DrawSaveLoadBorder(x: std::ffi::c_int, y: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_LoadSelect(choice: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_LoadGame(choice: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_DrawSave() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_DoSave(slot: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_SaveSelect(choice: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_SaveGame(choice: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub static mut tempstring: [std::ffi::c_char; (80) as usize] = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn M_QuickSaveResponse(ch: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_QuickSave() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_QuickLoadResponse(ch: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_QuickLoad() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_DrawReadThis1() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_DrawReadThis2() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_DrawSound() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_Sound(choice: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_SfxVol(choice: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_MusicVol(choice: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_DrawMainMenu() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_DrawNewGame() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_NewGame(choice: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub static mut epi: std::ffi::c_int = unsafe { std::mem::zeroed() }; // TODO: initializer not yet translated

pub unsafe extern "C" fn M_DrawEpisode() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_VerifyNightmare(ch: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_ChooseSkill(choice: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_Episode(choice: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub static mut detailNames: [[std::ffi::c_char; (9) as usize]; (2) as usize] =
    unsafe { [(c"M_GDHIGH").as_ptr(), (c"M_GDLOW").as_ptr()] };

pub static mut msgNames: [[std::ffi::c_char; (9) as usize]; (2) as usize] =
    unsafe { [(c"M_MSGOFF").as_ptr(), (c"M_MSGON").as_ptr()] };

pub unsafe extern "C" fn M_DrawOptions() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_Options(choice: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_ChangeMessages(choice: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_EndGameResponse(ch: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_EndGame(choice: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_ReadThis(choice: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_ReadThis2(choice: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_FinishReadThis(choice: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub static mut quitsounds: [std::ffi::c_int; (8) as usize] = unsafe {
    [
        sfx_pldeth, sfx_dmpain, sfx_popain, sfx_slop, sfx_telept, sfx_posit1, sfx_posit3,
        sfx_sgtatk,
    ]
};

pub static mut quitsounds2: [std::ffi::c_int; (8) as usize] = unsafe {
    [
        sfx_vilact, sfx_getpow, sfx_boscub, sfx_slop, sfx_skeswg, sfx_kntdth, sfx_bspact,
        sfx_sgtatk,
    ]
};

pub unsafe extern "C" fn M_QuitResponse(ch: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_QuitDOOM(choice: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_ChangeSensitivity(choice: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_ChangeDetail(choice: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_SizeDisplay(choice: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_DrawThermo(
    x: std::ffi::c_int,
    y: std::ffi::c_int,
    thermWidth: std::ffi::c_int,
    thermDot: std::ffi::c_int,
) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_DrawEmptyCell(menu: *mut menu_t, item: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_DrawSelCell(menu: *mut menu_t, item: std::ffi::c_int) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_StartMessage(
    string: *mut std::ffi::c_char,
    routine: *mut std::ffi::c_void,
    input: boolean,
) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_StopMessage() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_StringWidth(string: *mut std::ffi::c_char) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_StringHeight(string: *mut std::ffi::c_char) -> std::ffi::c_int {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_WriteText(
    x: std::ffi::c_int,
    y: std::ffi::c_int,
    string: *mut std::ffi::c_char,
) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_Responder(ev: *mut event_t) -> boolean {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_StartControlPanel() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_Drawer() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_ClearMenus() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_SetupNextMenu(menudef: *mut menu_t) {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_Ticker() {
    todo!("body not yet translated")
}

pub unsafe extern "C" fn M_Init() {
    todo!("body not yet translated")
}
