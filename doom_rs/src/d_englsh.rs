pub const D_DEVSTR: *const std::ffi::c_char = (c"Development mode ON.\n").as_ptr();

pub const D_CDROM: *const std::ffi::c_char =
    (c"CD-ROM Version: default.cfg from c:\\doomdata\n").as_ptr();

pub const PRESSKEY: *const std::ffi::c_char = (c"press a key.").as_ptr();

pub const PRESSYN: *const std::ffi::c_char = (c"press y or n.").as_ptr();

pub const QUITMSG: *const std::ffi::c_char =
    (c"are you sure you want to\nquit this great game?").as_ptr();

pub const LOADNET: *const std::ffi::c_char =
    (c"you can't do load while in a net game!\n\n").as_ptr();

pub const QLOADNET: *const std::ffi::c_char =
    (c"you can't quickload during a netgame!\n\n").as_ptr();

pub const QSAVESPOT: *const std::ffi::c_char =
    (c"you haven't picked a quicksave slot yet!\n\n").as_ptr();

pub const SAVEDEAD: *const std::ffi::c_char =
    (c"you can't save if you aren't playing!\n\n").as_ptr();

pub const QSPROMPT: *const std::ffi::c_char =
    (c"quicksave over your game named\n\n'%s'?\n\n").as_ptr();

pub const QLPROMPT: *const std::ffi::c_char =
    (c"do you want to quickload the game named\n\n'%s'?\n\n").as_ptr();

pub const NEWGAME: *const std::ffi::c_char = (c"you can't start a new game\n").as_ptr();

pub const NIGHTMARE: *const std::ffi::c_char = (c"are you sure? this skill level\n").as_ptr();

pub const SWSTRING: *const std::ffi::c_char =
    (c"this is the shareware version of doom.\n\n").as_ptr();

pub const MSGOFF: *const std::ffi::c_char = (c"Messages OFF").as_ptr();

pub const MSGON: *const std::ffi::c_char = (c"Messages ON").as_ptr();

pub const NETEND: *const std::ffi::c_char = (c"you can't end a netgame!\n\n").as_ptr();

pub const ENDGAME: *const std::ffi::c_char =
    (c"are you sure you want to end the game?\n\n").as_ptr();

pub const DOSY: *const std::ffi::c_char = (c"(press y to quit)").as_ptr();

pub const DETAILHI: *const std::ffi::c_char = (c"High detail").as_ptr();

pub const DETAILLO: *const std::ffi::c_char = (c"Low detail").as_ptr();

pub const GAMMALVL0: *const std::ffi::c_char = (c"Gamma correction OFF").as_ptr();

pub const GAMMALVL1: *const std::ffi::c_char = (c"Gamma correction level 1").as_ptr();

pub const GAMMALVL2: *const std::ffi::c_char = (c"Gamma correction level 2").as_ptr();

pub const GAMMALVL3: *const std::ffi::c_char = (c"Gamma correction level 3").as_ptr();

pub const GAMMALVL4: *const std::ffi::c_char = (c"Gamma correction level 4").as_ptr();

pub const EMPTYSTRING: *const std::ffi::c_char = (c"empty slot").as_ptr();

pub const GOTARMOR: *const std::ffi::c_char = (c"Picked up the armor.").as_ptr();

pub const GOTMEGA: *const std::ffi::c_char = (c"Picked up the MegaArmor!").as_ptr();

pub const GOTHTHBONUS: *const std::ffi::c_char = (c"Picked up a health bonus.").as_ptr();

pub const GOTARMBONUS: *const std::ffi::c_char = (c"Picked up an armor bonus.").as_ptr();

pub const GOTSTIM: *const std::ffi::c_char = (c"Picked up a stimpack.").as_ptr();

pub const GOTMEDINEED: *const std::ffi::c_char =
    (c"Picked up a medikit that you REALLY need!").as_ptr();

pub const GOTMEDIKIT: *const std::ffi::c_char = (c"Picked up a medikit.").as_ptr();

pub const GOTSUPER: *const std::ffi::c_char = (c"Supercharge!").as_ptr();

pub const GOTBLUECARD: *const std::ffi::c_char = (c"Picked up a blue keycard.").as_ptr();

pub const GOTYELWCARD: *const std::ffi::c_char = (c"Picked up a yellow keycard.").as_ptr();

pub const GOTREDCARD: *const std::ffi::c_char = (c"Picked up a red keycard.").as_ptr();

pub const GOTBLUESKUL: *const std::ffi::c_char = (c"Picked up a blue skull key.").as_ptr();

pub const GOTYELWSKUL: *const std::ffi::c_char = (c"Picked up a yellow skull key.").as_ptr();

pub const GOTREDSKULL: *const std::ffi::c_char = (c"Picked up a red skull key.").as_ptr();

pub const GOTINVUL: *const std::ffi::c_char = (c"Invulnerability!").as_ptr();

pub const GOTBERSERK: *const std::ffi::c_char = (c"Berserk!").as_ptr();

pub const GOTINVIS: *const std::ffi::c_char = (c"Partial Invisibility").as_ptr();

pub const GOTSUIT: *const std::ffi::c_char = (c"Radiation Shielding Suit").as_ptr();

pub const GOTMAP: *const std::ffi::c_char = (c"Computer Area Map").as_ptr();

pub const GOTVISOR: *const std::ffi::c_char = (c"Light Amplification Visor").as_ptr();

pub const GOTMSPHERE: *const std::ffi::c_char = (c"MegaSphere!").as_ptr();

pub const GOTCLIP: *const std::ffi::c_char = (c"Picked up a clip.").as_ptr();

pub const GOTCLIPBOX: *const std::ffi::c_char = (c"Picked up a box of bullets.").as_ptr();

pub const GOTROCKET: *const std::ffi::c_char = (c"Picked up a rocket.").as_ptr();

pub const GOTROCKBOX: *const std::ffi::c_char = (c"Picked up a box of rockets.").as_ptr();

pub const GOTCELL: *const std::ffi::c_char = (c"Picked up an energy cell.").as_ptr();

pub const GOTCELLBOX: *const std::ffi::c_char = (c"Picked up an energy cell pack.").as_ptr();

pub const GOTSHELLS: *const std::ffi::c_char = (c"Picked up 4 shotgun shells.").as_ptr();

pub const GOTSHELLBOX: *const std::ffi::c_char = (c"Picked up a box of shotgun shells.").as_ptr();

pub const GOTBACKPACK: *const std::ffi::c_char = (c"Picked up a backpack full of ammo!").as_ptr();

pub const GOTBFG9000: *const std::ffi::c_char = (c"You got the BFG9000!  Oh, yes.").as_ptr();

pub const GOTCHAINGUN: *const std::ffi::c_char = (c"You got the chaingun!").as_ptr();

pub const GOTCHAINSAW: *const std::ffi::c_char = (c"A chainsaw!  Find some meat!").as_ptr();

pub const GOTLAUNCHER: *const std::ffi::c_char = (c"You got the rocket launcher!").as_ptr();

pub const GOTPLASMA: *const std::ffi::c_char = (c"You got the plasma gun!").as_ptr();

pub const GOTSHOTGUN: *const std::ffi::c_char = (c"You got the shotgun!").as_ptr();

pub const GOTSHOTGUN2: *const std::ffi::c_char = (c"You got the super shotgun!").as_ptr();

pub const PD_BLUEO: *const std::ffi::c_char =
    (c"You need a blue key to activate this object").as_ptr();

pub const PD_REDO: *const std::ffi::c_char =
    (c"You need a red key to activate this object").as_ptr();

pub const PD_YELLOWO: *const std::ffi::c_char =
    (c"You need a yellow key to activate this object").as_ptr();

pub const PD_BLUEK: *const std::ffi::c_char = (c"You need a blue key to open this door").as_ptr();

pub const PD_REDK: *const std::ffi::c_char = (c"You need a red key to open this door").as_ptr();

pub const PD_YELLOWK: *const std::ffi::c_char =
    (c"You need a yellow key to open this door").as_ptr();

pub const GGSAVED: *const std::ffi::c_char = (c"game saved.").as_ptr();

pub const HUSTR_MSGU: *const std::ffi::c_char = (c"[Message unsent]").as_ptr();

pub const HUSTR_E1M1: *const std::ffi::c_char = (c"E1M1: Hangar").as_ptr();

pub const HUSTR_E1M2: *const std::ffi::c_char = (c"E1M2: Nuclear Plant").as_ptr();

pub const HUSTR_E1M3: *const std::ffi::c_char = (c"E1M3: Toxin Refinery").as_ptr();

pub const HUSTR_E1M4: *const std::ffi::c_char = (c"E1M4: Command Control").as_ptr();

pub const HUSTR_E1M5: *const std::ffi::c_char = (c"E1M5: Phobos Lab").as_ptr();

pub const HUSTR_E1M6: *const std::ffi::c_char = (c"E1M6: Central Processing").as_ptr();

pub const HUSTR_E1M7: *const std::ffi::c_char = (c"E1M7: Computer Station").as_ptr();

pub const HUSTR_E1M8: *const std::ffi::c_char = (c"E1M8: Phobos Anomaly").as_ptr();

pub const HUSTR_E1M9: *const std::ffi::c_char = (c"E1M9: Military Base").as_ptr();

pub const HUSTR_E2M1: *const std::ffi::c_char = (c"E2M1: Deimos Anomaly").as_ptr();

pub const HUSTR_E2M2: *const std::ffi::c_char = (c"E2M2: Containment Area").as_ptr();

pub const HUSTR_E2M3: *const std::ffi::c_char = (c"E2M3: Refinery").as_ptr();

pub const HUSTR_E2M4: *const std::ffi::c_char = (c"E2M4: Deimos Lab").as_ptr();

pub const HUSTR_E2M5: *const std::ffi::c_char = (c"E2M5: Command Center").as_ptr();

pub const HUSTR_E2M6: *const std::ffi::c_char = (c"E2M6: Halls of the Damned").as_ptr();

pub const HUSTR_E2M7: *const std::ffi::c_char = (c"E2M7: Spawning Vats").as_ptr();

pub const HUSTR_E2M8: *const std::ffi::c_char = (c"E2M8: Tower of Babel").as_ptr();

pub const HUSTR_E2M9: *const std::ffi::c_char = (c"E2M9: Fortress of Mystery").as_ptr();

pub const HUSTR_E3M1: *const std::ffi::c_char = (c"E3M1: Hell Keep").as_ptr();

pub const HUSTR_E3M2: *const std::ffi::c_char = (c"E3M2: Slough of Despair").as_ptr();

pub const HUSTR_E3M3: *const std::ffi::c_char = (c"E3M3: Pandemonium").as_ptr();

pub const HUSTR_E3M4: *const std::ffi::c_char = (c"E3M4: House of Pain").as_ptr();

pub const HUSTR_E3M5: *const std::ffi::c_char = (c"E3M5: Unholy Cathedral").as_ptr();

pub const HUSTR_E3M6: *const std::ffi::c_char = (c"E3M6: Mt. Erebus").as_ptr();

pub const HUSTR_E3M7: *const std::ffi::c_char = (c"E3M7: Limbo").as_ptr();

pub const HUSTR_E3M8: *const std::ffi::c_char = (c"E3M8: Dis").as_ptr();

pub const HUSTR_E3M9: *const std::ffi::c_char = (c"E3M9: Warrens").as_ptr();

pub const HUSTR_E4M1: *const std::ffi::c_char = (c"E4M1: Hell Beneath").as_ptr();

pub const HUSTR_E4M2: *const std::ffi::c_char = (c"E4M2: Perfect Hatred").as_ptr();

pub const HUSTR_E4M3: *const std::ffi::c_char = (c"E4M3: Sever The Wicked").as_ptr();

pub const HUSTR_E4M4: *const std::ffi::c_char = (c"E4M4: Unruly Evil").as_ptr();

pub const HUSTR_E4M5: *const std::ffi::c_char = (c"E4M5: They Will Repent").as_ptr();

pub const HUSTR_E4M6: *const std::ffi::c_char = (c"E4M6: Against Thee Wickedly").as_ptr();

pub const HUSTR_E4M7: *const std::ffi::c_char = (c"E4M7: And Hell Followed").as_ptr();

pub const HUSTR_E4M8: *const std::ffi::c_char = (c"E4M8: Unto The Cruel").as_ptr();

pub const HUSTR_E4M9: *const std::ffi::c_char = (c"E4M9: Fear").as_ptr();

pub const HUSTR_1: *const std::ffi::c_char = (c"level 1: entryway").as_ptr();

pub const HUSTR_2: *const std::ffi::c_char = (c"level 2: underhalls").as_ptr();

pub const HUSTR_3: *const std::ffi::c_char = (c"level 3: the gantlet").as_ptr();

pub const HUSTR_4: *const std::ffi::c_char = (c"level 4: the focus").as_ptr();

pub const HUSTR_5: *const std::ffi::c_char = (c"level 5: the waste tunnels").as_ptr();

pub const HUSTR_6: *const std::ffi::c_char = (c"level 6: the crusher").as_ptr();

pub const HUSTR_7: *const std::ffi::c_char = (c"level 7: dead simple").as_ptr();

pub const HUSTR_8: *const std::ffi::c_char = (c"level 8: tricks and traps").as_ptr();

pub const HUSTR_9: *const std::ffi::c_char = (c"level 9: the pit").as_ptr();

pub const HUSTR_10: *const std::ffi::c_char = (c"level 10: refueling base").as_ptr();

pub const HUSTR_11: *const std::ffi::c_char = (c"level 11: 'o' of destruction!").as_ptr();

pub const HUSTR_12: *const std::ffi::c_char = (c"level 12: the factory").as_ptr();

pub const HUSTR_13: *const std::ffi::c_char = (c"level 13: downtown").as_ptr();

pub const HUSTR_14: *const std::ffi::c_char = (c"level 14: the inmost dens").as_ptr();

pub const HUSTR_15: *const std::ffi::c_char = (c"level 15: industrial zone").as_ptr();

pub const HUSTR_16: *const std::ffi::c_char = (c"level 16: suburbs").as_ptr();

pub const HUSTR_17: *const std::ffi::c_char = (c"level 17: tenements").as_ptr();

pub const HUSTR_18: *const std::ffi::c_char = (c"level 18: the courtyard").as_ptr();

pub const HUSTR_19: *const std::ffi::c_char = (c"level 19: the citadel").as_ptr();

pub const HUSTR_20: *const std::ffi::c_char = (c"level 20: gotcha!").as_ptr();

pub const HUSTR_21: *const std::ffi::c_char = (c"level 21: nirvana").as_ptr();

pub const HUSTR_22: *const std::ffi::c_char = (c"level 22: the catacombs").as_ptr();

pub const HUSTR_23: *const std::ffi::c_char = (c"level 23: barrels o' fun").as_ptr();

pub const HUSTR_24: *const std::ffi::c_char = (c"level 24: the chasm").as_ptr();

pub const HUSTR_25: *const std::ffi::c_char = (c"level 25: bloodfalls").as_ptr();

pub const HUSTR_26: *const std::ffi::c_char = (c"level 26: the abandoned mines").as_ptr();

pub const HUSTR_27: *const std::ffi::c_char = (c"level 27: monster condo").as_ptr();

pub const HUSTR_28: *const std::ffi::c_char = (c"level 28: the spirit world").as_ptr();

pub const HUSTR_29: *const std::ffi::c_char = (c"level 29: the living end").as_ptr();

pub const HUSTR_30: *const std::ffi::c_char = (c"level 30: icon of sin").as_ptr();

pub const HUSTR_31: *const std::ffi::c_char = (c"level 31: wolfenstein").as_ptr();

pub const HUSTR_32: *const std::ffi::c_char = (c"level 32: grosse").as_ptr();

pub const PHUSTR_1: *const std::ffi::c_char = (c"level 1: congo").as_ptr();

pub const PHUSTR_2: *const std::ffi::c_char = (c"level 2: well of souls").as_ptr();

pub const PHUSTR_3: *const std::ffi::c_char = (c"level 3: aztec").as_ptr();

pub const PHUSTR_4: *const std::ffi::c_char = (c"level 4: caged").as_ptr();

pub const PHUSTR_5: *const std::ffi::c_char = (c"level 5: ghost town").as_ptr();

pub const PHUSTR_6: *const std::ffi::c_char = (c"level 6: baron's lair").as_ptr();

pub const PHUSTR_7: *const std::ffi::c_char = (c"level 7: caughtyard").as_ptr();

pub const PHUSTR_8: *const std::ffi::c_char = (c"level 8: realm").as_ptr();

pub const PHUSTR_9: *const std::ffi::c_char = (c"level 9: abattoire").as_ptr();

pub const PHUSTR_10: *const std::ffi::c_char = (c"level 10: onslaught").as_ptr();

pub const PHUSTR_11: *const std::ffi::c_char = (c"level 11: hunted").as_ptr();

pub const PHUSTR_12: *const std::ffi::c_char = (c"level 12: speed").as_ptr();

pub const PHUSTR_13: *const std::ffi::c_char = (c"level 13: the crypt").as_ptr();

pub const PHUSTR_14: *const std::ffi::c_char = (c"level 14: genesis").as_ptr();

pub const PHUSTR_15: *const std::ffi::c_char = (c"level 15: the twilight").as_ptr();

pub const PHUSTR_16: *const std::ffi::c_char = (c"level 16: the omen").as_ptr();

pub const PHUSTR_17: *const std::ffi::c_char = (c"level 17: compound").as_ptr();

pub const PHUSTR_18: *const std::ffi::c_char = (c"level 18: neurosphere").as_ptr();

pub const PHUSTR_19: *const std::ffi::c_char = (c"level 19: nme").as_ptr();

pub const PHUSTR_20: *const std::ffi::c_char = (c"level 20: the death domain").as_ptr();

pub const PHUSTR_21: *const std::ffi::c_char = (c"level 21: slayer").as_ptr();

pub const PHUSTR_22: *const std::ffi::c_char = (c"level 22: impossible mission").as_ptr();

pub const PHUSTR_23: *const std::ffi::c_char = (c"level 23: tombstone").as_ptr();

pub const PHUSTR_24: *const std::ffi::c_char = (c"level 24: the final frontier").as_ptr();

pub const PHUSTR_25: *const std::ffi::c_char = (c"level 25: the temple of darkness").as_ptr();

pub const PHUSTR_26: *const std::ffi::c_char = (c"level 26: bunker").as_ptr();

pub const PHUSTR_27: *const std::ffi::c_char = (c"level 27: anti-christ").as_ptr();

pub const PHUSTR_28: *const std::ffi::c_char = (c"level 28: the sewers").as_ptr();

pub const PHUSTR_29: *const std::ffi::c_char = (c"level 29: odyssey of noises").as_ptr();

pub const PHUSTR_30: *const std::ffi::c_char = (c"level 30: the gateway of hell").as_ptr();

pub const PHUSTR_31: *const std::ffi::c_char = (c"level 31: cyberden").as_ptr();

pub const PHUSTR_32: *const std::ffi::c_char = (c"level 32: go 2 it").as_ptr();

pub const THUSTR_1: *const std::ffi::c_char = (c"level 1: system control").as_ptr();

pub const THUSTR_2: *const std::ffi::c_char = (c"level 2: human bbq").as_ptr();

pub const THUSTR_3: *const std::ffi::c_char = (c"level 3: power control").as_ptr();

pub const THUSTR_4: *const std::ffi::c_char = (c"level 4: wormhole").as_ptr();

pub const THUSTR_5: *const std::ffi::c_char = (c"level 5: hanger").as_ptr();

pub const THUSTR_6: *const std::ffi::c_char = (c"level 6: open season").as_ptr();

pub const THUSTR_7: *const std::ffi::c_char = (c"level 7: prison").as_ptr();

pub const THUSTR_8: *const std::ffi::c_char = (c"level 8: metal").as_ptr();

pub const THUSTR_9: *const std::ffi::c_char = (c"level 9: stronghold").as_ptr();

pub const THUSTR_10: *const std::ffi::c_char = (c"level 10: redemption").as_ptr();

pub const THUSTR_11: *const std::ffi::c_char = (c"level 11: storage facility").as_ptr();

pub const THUSTR_12: *const std::ffi::c_char = (c"level 12: crater").as_ptr();

pub const THUSTR_13: *const std::ffi::c_char = (c"level 13: nukage processing").as_ptr();

pub const THUSTR_14: *const std::ffi::c_char = (c"level 14: steel works").as_ptr();

pub const THUSTR_15: *const std::ffi::c_char = (c"level 15: dead zone").as_ptr();

pub const THUSTR_16: *const std::ffi::c_char = (c"level 16: deepest reaches").as_ptr();

pub const THUSTR_17: *const std::ffi::c_char = (c"level 17: processing area").as_ptr();

pub const THUSTR_18: *const std::ffi::c_char = (c"level 18: mill").as_ptr();

pub const THUSTR_19: *const std::ffi::c_char = (c"level 19: shipping/respawning").as_ptr();

pub const THUSTR_20: *const std::ffi::c_char = (c"level 20: central processing").as_ptr();

pub const THUSTR_21: *const std::ffi::c_char = (c"level 21: administration center").as_ptr();

pub const THUSTR_22: *const std::ffi::c_char = (c"level 22: habitat").as_ptr();

pub const THUSTR_23: *const std::ffi::c_char = (c"level 23: lunar mining project").as_ptr();

pub const THUSTR_24: *const std::ffi::c_char = (c"level 24: quarry").as_ptr();

pub const THUSTR_25: *const std::ffi::c_char = (c"level 25: baron's den").as_ptr();

pub const THUSTR_26: *const std::ffi::c_char = (c"level 26: ballistyx").as_ptr();

pub const THUSTR_27: *const std::ffi::c_char = (c"level 27: mount pain").as_ptr();

pub const THUSTR_28: *const std::ffi::c_char = (c"level 28: heck").as_ptr();

pub const THUSTR_29: *const std::ffi::c_char = (c"level 29: river styx").as_ptr();

pub const THUSTR_30: *const std::ffi::c_char = (c"level 30: last call").as_ptr();

pub const THUSTR_31: *const std::ffi::c_char = (c"level 31: pharaoh").as_ptr();

pub const THUSTR_32: *const std::ffi::c_char = (c"level 32: caribbean").as_ptr();

pub const HUSTR_CHATMACRO1: *const std::ffi::c_char = (c"I'm ready to kick butt!").as_ptr();

pub const HUSTR_CHATMACRO2: *const std::ffi::c_char = (c"I'm OK.").as_ptr();

pub const HUSTR_CHATMACRO3: *const std::ffi::c_char = (c"I'm not looking too good!").as_ptr();

pub const HUSTR_CHATMACRO4: *const std::ffi::c_char = (c"Help!").as_ptr();

pub const HUSTR_CHATMACRO5: *const std::ffi::c_char = (c"You suck!").as_ptr();

pub const HUSTR_CHATMACRO6: *const std::ffi::c_char = (c"Next time, scumbag...").as_ptr();

pub const HUSTR_CHATMACRO7: *const std::ffi::c_char = (c"Come here!").as_ptr();

pub const HUSTR_CHATMACRO8: *const std::ffi::c_char = (c"I'll take care of it.").as_ptr();

pub const HUSTR_CHATMACRO9: *const std::ffi::c_char = (c"Yes").as_ptr();

pub const HUSTR_CHATMACRO0: *const std::ffi::c_char = (c"No").as_ptr();

pub const HUSTR_TALKTOSELF1: *const std::ffi::c_char = (c"You mumble to yourself").as_ptr();

pub const HUSTR_TALKTOSELF2: *const std::ffi::c_char = (c"Who's there?").as_ptr();

pub const HUSTR_TALKTOSELF3: *const std::ffi::c_char = (c"You scare yourself").as_ptr();

pub const HUSTR_TALKTOSELF4: *const std::ffi::c_char = (c"You start to rave").as_ptr();

pub const HUSTR_TALKTOSELF5: *const std::ffi::c_char = (c"You've lost it...").as_ptr();

pub const HUSTR_MESSAGESENT: *const std::ffi::c_char = (c"[Message Sent]").as_ptr();

pub const HUSTR_PLRGREEN: *const std::ffi::c_char = (c"Green: ").as_ptr();

pub const HUSTR_PLRINDIGO: *const std::ffi::c_char = (c"Indigo: ").as_ptr();

pub const HUSTR_PLRBROWN: *const std::ffi::c_char = (c"Brown: ").as_ptr();

pub const HUSTR_PLRRED: *const std::ffi::c_char = (c"Red: ").as_ptr();

pub const HUSTR_KEYGREEN: std::ffi::c_int = (b'g' as std::ffi::c_int);

pub const HUSTR_KEYINDIGO: std::ffi::c_int = (b'i' as std::ffi::c_int);

pub const HUSTR_KEYBROWN: std::ffi::c_int = (b'b' as std::ffi::c_int);

pub const HUSTR_KEYRED: std::ffi::c_int = (b'r' as std::ffi::c_int);

pub const AMSTR_FOLLOWON: *const std::ffi::c_char = (c"Follow Mode ON").as_ptr();

pub const AMSTR_FOLLOWOFF: *const std::ffi::c_char = (c"Follow Mode OFF").as_ptr();

pub const AMSTR_GRIDON: *const std::ffi::c_char = (c"Grid ON").as_ptr();

pub const AMSTR_GRIDOFF: *const std::ffi::c_char = (c"Grid OFF").as_ptr();

pub const AMSTR_MARKEDSPOT: *const std::ffi::c_char = (c"Marked Spot").as_ptr();

pub const AMSTR_MARKSCLEARED: *const std::ffi::c_char = (c"All Marks Cleared").as_ptr();

pub const STSTR_MUS: *const std::ffi::c_char = (c"Music Change").as_ptr();

pub const STSTR_NOMUS: *const std::ffi::c_char = (c"IMPOSSIBLE SELECTION").as_ptr();

pub const STSTR_DQDON: *const std::ffi::c_char = (c"Degreelessness Mode On").as_ptr();

pub const STSTR_DQDOFF: *const std::ffi::c_char = (c"Degreelessness Mode Off").as_ptr();

pub const STSTR_KFAADDED: *const std::ffi::c_char = (c"Very Happy Ammo Added").as_ptr();

pub const STSTR_FAADDED: *const std::ffi::c_char = (c"Ammo (no keys) Added").as_ptr();

pub const STSTR_NCON: *const std::ffi::c_char = (c"No Clipping Mode ON").as_ptr();

pub const STSTR_NCOFF: *const std::ffi::c_char = (c"No Clipping Mode OFF").as_ptr();

pub const STSTR_BEHOLD: *const std::ffi::c_char =
    (c"inVuln, Str, Inviso, Rad, Allmap, or Lite-amp").as_ptr();

pub const STSTR_BEHOLDX: *const std::ffi::c_char = (c"Power-up Toggled").as_ptr();

pub const STSTR_CHOPPERS: *const std::ffi::c_char = (c"... doesn't suck - GM").as_ptr();

pub const STSTR_CLEV: *const std::ffi::c_char = (c"Changing Level...").as_ptr();

pub const E1TEXT: *const std::ffi::c_char = (c"Once you beat the big badasses and\n").as_ptr();

pub const E2TEXT: *const std::ffi::c_char = (c"You've done it! The hideous cyber-\n").as_ptr();

pub const E3TEXT: *const std::ffi::c_char = (c"The loathsome spiderdemon that\n").as_ptr();

pub const E4TEXT: *const std::ffi::c_char =
    (c"the spider mastermind must have sent forth\n").as_ptr();

pub const C1TEXT: *const std::ffi::c_char =
    (c"YOU HAVE ENTERED DEEPLY INTO THE INFESTED\n").as_ptr();

pub const C2TEXT: *const std::ffi::c_char = (c"YOU HAVE WON! YOUR VICTORY HAS ENABLED\n").as_ptr();

pub const C3TEXT: *const std::ffi::c_char =
    (c"YOU ARE AT THE CORRUPT HEART OF THE CITY,\n").as_ptr();

pub const C4TEXT: *const std::ffi::c_char = (c"THE HORRENDOUS VISAGE OF THE BIGGEST\n").as_ptr();

pub const C5TEXT: *const std::ffi::c_char =
    (c"CONGRATULATIONS, YOU'VE FOUND THE SECRET\n").as_ptr();

pub const C6TEXT: *const std::ffi::c_char = (c"CONGRATULATIONS, YOU'VE FOUND THE\n").as_ptr();

pub const P1TEXT: *const std::ffi::c_char =
    (c"You gloat over the steaming carcass of the\n").as_ptr();

pub const P2TEXT: *const std::ffi::c_char =
    (c"Even the deadly Arch-Vile labyrinth could\n").as_ptr();

pub const P3TEXT: *const std::ffi::c_char =
    (c"You've bashed and battered your way into\n").as_ptr();

pub const P4TEXT: *const std::ffi::c_char =
    (c"The Gatekeeper's evil face is splattered\n").as_ptr();

pub const P5TEXT: *const std::ffi::c_char =
    (c"You've found the second-hardest level we\n").as_ptr();

pub const P6TEXT: *const std::ffi::c_char =
    (c"Betcha wondered just what WAS the hardest\n").as_ptr();

pub const T1TEXT: *const std::ffi::c_char =
    (c"You've fought your way out of the infested\n").as_ptr();

pub const T2TEXT: *const std::ffi::c_char =
    (c"You hear the grinding of heavy machinery\n").as_ptr();

pub const T3TEXT: *const std::ffi::c_char = (c"The vista opening ahead looks real damn\n").as_ptr();

pub const T4TEXT: *const std::ffi::c_char =
    (c"Suddenly, all is silent, from one horizon\n").as_ptr();

pub const T5TEXT: *const std::ffi::c_char = (c"What now? Looks totally different. Kind\n").as_ptr();

pub const T6TEXT: *const std::ffi::c_char = (c"Time for a vacation. You've burst the\n").as_ptr();

pub const CC_ZOMBIE: *const std::ffi::c_char = (c"ZOMBIEMAN").as_ptr();

pub const CC_SHOTGUN: *const std::ffi::c_char = (c"SHOTGUN GUY").as_ptr();

pub const CC_HEAVY: *const std::ffi::c_char = (c"HEAVY WEAPON DUDE").as_ptr();

pub const CC_IMP: *const std::ffi::c_char = (c"IMP").as_ptr();

pub const CC_DEMON: *const std::ffi::c_char = (c"DEMON").as_ptr();

pub const CC_LOST: *const std::ffi::c_char = (c"LOST SOUL").as_ptr();

pub const CC_CACO: *const std::ffi::c_char = (c"CACODEMON").as_ptr();

pub const CC_HELL: *const std::ffi::c_char = (c"HELL KNIGHT").as_ptr();

pub const CC_BARON: *const std::ffi::c_char = (c"BARON OF HELL").as_ptr();

pub const CC_ARACH: *const std::ffi::c_char = (c"ARACHNOTRON").as_ptr();

pub const CC_PAIN: *const std::ffi::c_char = (c"PAIN ELEMENTAL").as_ptr();

pub const CC_REVEN: *const std::ffi::c_char = (c"REVENANT").as_ptr();

pub const CC_MANCU: *const std::ffi::c_char = (c"MANCUBUS").as_ptr();

pub const CC_ARCH: *const std::ffi::c_char = (c"ARCH-VILE").as_ptr();

pub const CC_SPIDER: *const std::ffi::c_char = (c"THE SPIDER MASTERMIND").as_ptr();

pub const CC_CYBER: *const std::ffi::c_char = (c"THE CYBERDEMON").as_ptr();

pub const CC_HERO: *const std::ffi::c_char = (c"OUR HERO").as_ptr();
