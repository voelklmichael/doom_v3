pub const D_DEVSTR: *const std::ffi::c_char = (c"MODE DEVELOPPEMENT ON.\n").as_ptr();

pub const D_CDROM: *const std::ffi::c_char =
    (c"VERSION CD-ROM: DEFAULT.CFG DANS C:\\DOOMDATA\n").as_ptr();

pub const PRESSKEY: *const std::ffi::c_char = (c"APPUYEZ SUR UNE TOUCHE.").as_ptr();

pub const PRESSYN: *const std::ffi::c_char = (c"APPUYEZ SUR Y OU N").as_ptr();

pub const QUITMSG: *const std::ffi::c_char =
    (c"VOUS VOULEZ VRAIMENT\nQUITTER CE SUPER JEU?").as_ptr();

pub const LOADNET: *const std::ffi::c_char =
    (c"VOUS NE POUVEZ PAS CHARGER\nUN JEU EN RESEAU!\n\n").as_ptr();

pub const QLOADNET: *const std::ffi::c_char =
    (c"CHARGEMENT RAPIDE INTERDIT EN RESEAU!\n\n").as_ptr();

pub const QSAVESPOT: *const std::ffi::c_char =
    (c"VOUS N'AVEZ PAS CHOISI UN EMPLACEMENT!\n\n").as_ptr();

pub const SAVEDEAD: *const std::ffi::c_char =
    (c"VOUS NE POUVEZ PAS SAUVER SI VOUS NE JOUEZ ").as_ptr();

pub const QSPROMPT: *const std::ffi::c_char =
    (c"SAUVEGARDE RAPIDE DANS LE FICHIER \n\n'%s'?\n\n").as_ptr();

pub const QLPROMPT: *const std::ffi::c_char = (c"VOULEZ-VOUS CHARGER LA SAUVEGARDE").as_ptr();

pub const NEWGAME: *const std::ffi::c_char = (c"VOUS NE POUVEZ PAS LANCER\n").as_ptr();

pub const NIGHTMARE: *const std::ffi::c_char = (c"VOUS CONFIRMEZ? CE NIVEAU EST\n").as_ptr();

pub const SWSTRING: *const std::ffi::c_char =
    (c"CECI EST UNE VERSION SHAREWARE DE DOOM.\n\n").as_ptr();

pub const MSGOFF: *const std::ffi::c_char = (c"MESSAGES OFF").as_ptr();

pub const MSGON: *const std::ffi::c_char = (c"MESSAGES ON").as_ptr();

pub const NETEND: *const std::ffi::c_char =
    (c"VOUS NE POUVEZ PAS METTRE FIN A UN JEU SUR ").as_ptr();

pub const ENDGAME: *const std::ffi::c_char =
    (c"VOUS VOULEZ VRAIMENT METTRE FIN AU JEU?\n\n").as_ptr();

pub const DOSY: *const std::ffi::c_char = (c"(APPUYEZ SUR Y POUR REVENIR AU OS.)").as_ptr();

pub const DETAILHI: *const std::ffi::c_char = (c"GRAPHISMES MAXIMUM ").as_ptr();

pub const DETAILLO: *const std::ffi::c_char = (c"GRAPHISMES MINIMUM ").as_ptr();

pub const GAMMALVL0: *const std::ffi::c_char = (c"CORRECTION GAMMA OFF").as_ptr();

pub const GAMMALVL1: *const std::ffi::c_char = (c"CORRECTION GAMMA NIVEAU 1").as_ptr();

pub const GAMMALVL2: *const std::ffi::c_char = (c"CORRECTION GAMMA NIVEAU 2").as_ptr();

pub const GAMMALVL3: *const std::ffi::c_char = (c"CORRECTION GAMMA NIVEAU 3").as_ptr();

pub const GAMMALVL4: *const std::ffi::c_char = (c"CORRECTION GAMMA NIVEAU 4").as_ptr();

pub const EMPTYSTRING: *const std::ffi::c_char = (c"EMPLACEMENT VIDE").as_ptr();

pub const GOTARMOR: *const std::ffi::c_char = (c"ARMURE RECUPEREE.").as_ptr();

pub const GOTMEGA: *const std::ffi::c_char = (c"MEGA-ARMURE RECUPEREE!").as_ptr();

pub const GOTHTHBONUS: *const std::ffi::c_char = (c"BONUS DE SANTE RECUPERE.").as_ptr();

pub const GOTARMBONUS: *const std::ffi::c_char = (c"BONUS D'ARMURE RECUPERE.").as_ptr();

pub const GOTSTIM: *const std::ffi::c_char = (c"STIMPACK RECUPERE.").as_ptr();

pub const GOTMEDINEED: *const std::ffi::c_char =
    (c"MEDIKIT RECUPERE. VOUS EN AVEZ VRAIMENT BESOIN!").as_ptr();

pub const GOTMEDIKIT: *const std::ffi::c_char = (c"MEDIKIT RECUPERE.").as_ptr();

pub const GOTSUPER: *const std::ffi::c_char = (c"SUPERCHARGE!").as_ptr();

pub const GOTBLUECARD: *const std::ffi::c_char = (c"CARTE MAGNETIQUE BLEUE RECUPEREE.").as_ptr();

pub const GOTYELWCARD: *const std::ffi::c_char = (c"CARTE MAGNETIQUE JAUNE RECUPEREE.").as_ptr();

pub const GOTREDCARD: *const std::ffi::c_char = (c"CARTE MAGNETIQUE ROUGE RECUPEREE.").as_ptr();

pub const GOTBLUESKUL: *const std::ffi::c_char = (c"CLEF CRANE BLEUE RECUPEREE.").as_ptr();

pub const GOTYELWSKUL: *const std::ffi::c_char = (c"CLEF CRANE JAUNE RECUPEREE.").as_ptr();

pub const GOTREDSKULL: *const std::ffi::c_char = (c"CLEF CRANE ROUGE RECUPEREE.").as_ptr();

pub const GOTINVUL: *const std::ffi::c_char = (c"INVULNERABILITE!").as_ptr();

pub const GOTBERSERK: *const std::ffi::c_char = (c"BERSERK!").as_ptr();

pub const GOTINVIS: *const std::ffi::c_char = (c"INVISIBILITE PARTIELLE ").as_ptr();

pub const GOTSUIT: *const std::ffi::c_char = (c"COMBINAISON ANTI-RADIATIONS ").as_ptr();

pub const GOTMAP: *const std::ffi::c_char = (c"CARTE INFORMATIQUE ").as_ptr();

pub const GOTVISOR: *const std::ffi::c_char = (c"VISEUR A AMPLIFICATION DE LUMIERE ").as_ptr();

pub const GOTMSPHERE: *const std::ffi::c_char = (c"MEGASPHERE!").as_ptr();

pub const GOTCLIP: *const std::ffi::c_char = (c"CHARGEUR RECUPERE.").as_ptr();

pub const GOTCLIPBOX: *const std::ffi::c_char = (c"BOITE DE BALLES RECUPEREE.").as_ptr();

pub const GOTROCKET: *const std::ffi::c_char = (c"ROQUETTE RECUPEREE.").as_ptr();

pub const GOTROCKBOX: *const std::ffi::c_char = (c"CAISSE DE ROQUETTES RECUPEREE.").as_ptr();

pub const GOTCELL: *const std::ffi::c_char = (c"CELLULE D'ENERGIE RECUPEREE.").as_ptr();

pub const GOTCELLBOX: *const std::ffi::c_char = (c"PACK DE CELLULES D'ENERGIE RECUPERE.").as_ptr();

pub const GOTSHELLS: *const std::ffi::c_char = (c"4 CARTOUCHES RECUPEREES.").as_ptr();

pub const GOTSHELLBOX: *const std::ffi::c_char = (c"BOITE DE CARTOUCHES RECUPEREE.").as_ptr();

pub const GOTBACKPACK: *const std::ffi::c_char = (c"SAC PLEIN DE MUNITIONS RECUPERE!").as_ptr();

pub const GOTBFG9000: *const std::ffi::c_char = (c"VOUS AVEZ UN BFG9000!  OH, OUI!").as_ptr();

pub const GOTCHAINGUN: *const std::ffi::c_char = (c"VOUS AVEZ LA MITRAILLEUSE!").as_ptr();

pub const GOTCHAINSAW: *const std::ffi::c_char = (c"UNE TRONCONNEUSE!").as_ptr();

pub const GOTLAUNCHER: *const std::ffi::c_char = (c"VOUS AVEZ UN LANCE-ROQUETTES!").as_ptr();

pub const GOTPLASMA: *const std::ffi::c_char = (c"VOUS AVEZ UN FUSIL A PLASMA!").as_ptr();

pub const GOTSHOTGUN: *const std::ffi::c_char = (c"VOUS AVEZ UN FUSIL!").as_ptr();

pub const GOTSHOTGUN2: *const std::ffi::c_char = (c"VOUS AVEZ UN SUPER FUSIL!").as_ptr();

pub const PD_BLUEO: *const std::ffi::c_char = (c"IL VOUS FAUT UNE CLEF BLEUE").as_ptr();

pub const PD_REDO: *const std::ffi::c_char = (c"IL VOUS FAUT UNE CLEF ROUGE").as_ptr();

pub const PD_YELLOWO: *const std::ffi::c_char = (c"IL VOUS FAUT UNE CLEF JAUNE").as_ptr();

pub const PD_BLUEK: std::ffi::c_int = PD_BLUEO;

pub const PD_REDK: std::ffi::c_int = PD_REDO;

pub const PD_YELLOWK: std::ffi::c_int = PD_YELLOWO;

pub const GGSAVED: *const std::ffi::c_char = (c"JEU SAUVEGARDE.").as_ptr();

pub const HUSTR_MSGU: *const std::ffi::c_char = (c"[MESSAGE NON ENVOYE]").as_ptr();

pub const HUSTR_E1M1: *const std::ffi::c_char = (c"E1M1: HANGAR").as_ptr();

pub const HUSTR_E1M2: *const std::ffi::c_char = (c"E1M2: USINE NUCLEAIRE ").as_ptr();

pub const HUSTR_E1M3: *const std::ffi::c_char = (c"E1M3: RAFFINERIE DE TOXINES ").as_ptr();

pub const HUSTR_E1M4: *const std::ffi::c_char = (c"E1M4: CENTRE DE CONTROLE ").as_ptr();

pub const HUSTR_E1M5: *const std::ffi::c_char = (c"E1M5: LABORATOIRE PHOBOS ").as_ptr();

pub const HUSTR_E1M6: *const std::ffi::c_char = (c"E1M6: TRAITEMENT CENTRAL ").as_ptr();

pub const HUSTR_E1M7: *const std::ffi::c_char = (c"E1M7: CENTRE INFORMATIQUE ").as_ptr();

pub const HUSTR_E1M8: *const std::ffi::c_char = (c"E1M8: ANOMALIE PHOBOS ").as_ptr();

pub const HUSTR_E1M9: *const std::ffi::c_char = (c"E1M9: BASE MILITAIRE ").as_ptr();

pub const HUSTR_E2M1: *const std::ffi::c_char = (c"E2M1: ANOMALIE DEIMOS ").as_ptr();

pub const HUSTR_E2M2: *const std::ffi::c_char = (c"E2M2: ZONE DE CONFINEMENT ").as_ptr();

pub const HUSTR_E2M3: *const std::ffi::c_char = (c"E2M3: RAFFINERIE").as_ptr();

pub const HUSTR_E2M4: *const std::ffi::c_char = (c"E2M4: LABORATOIRE DEIMOS ").as_ptr();

pub const HUSTR_E2M5: *const std::ffi::c_char = (c"E2M5: CENTRE DE CONTROLE ").as_ptr();

pub const HUSTR_E2M6: *const std::ffi::c_char = (c"E2M6: HALLS DES DAMNES ").as_ptr();

pub const HUSTR_E2M7: *const std::ffi::c_char = (c"E2M7: CUVES DE REPRODUCTION ").as_ptr();

pub const HUSTR_E2M8: *const std::ffi::c_char = (c"E2M8: TOUR DE BABEL ").as_ptr();

pub const HUSTR_E2M9: *const std::ffi::c_char = (c"E2M9: FORTERESSE DU MYSTERE ").as_ptr();

pub const HUSTR_E3M1: *const std::ffi::c_char = (c"E3M1: DONJON DE L'ENFER ").as_ptr();

pub const HUSTR_E3M2: *const std::ffi::c_char = (c"E3M2: BOURBIER DU DESESPOIR ").as_ptr();

pub const HUSTR_E3M3: *const std::ffi::c_char = (c"E3M3: PANDEMONIUM").as_ptr();

pub const HUSTR_E3M4: *const std::ffi::c_char = (c"E3M4: MAISON DE LA DOULEUR ").as_ptr();

pub const HUSTR_E3M5: *const std::ffi::c_char = (c"E3M5: CATHEDRALE PROFANE ").as_ptr();

pub const HUSTR_E3M6: *const std::ffi::c_char = (c"E3M6: MONT EREBUS").as_ptr();

pub const HUSTR_E3M7: *const std::ffi::c_char = (c"E3M7: LIMBES").as_ptr();

pub const HUSTR_E3M8: *const std::ffi::c_char = (c"E3M8: DIS").as_ptr();

pub const HUSTR_E3M9: *const std::ffi::c_char = (c"E3M9: CLAPIERS").as_ptr();

pub const HUSTR_1: *const std::ffi::c_char = (c"NIVEAU 1: ENTREE ").as_ptr();

pub const HUSTR_2: *const std::ffi::c_char = (c"NIVEAU 2: HALLS SOUTERRAINS ").as_ptr();

pub const HUSTR_3: *const std::ffi::c_char = (c"NIVEAU 3: LE FEU NOURRI ").as_ptr();

pub const HUSTR_4: *const std::ffi::c_char = (c"NIVEAU 4: LE FOYER ").as_ptr();

pub const HUSTR_5: *const std::ffi::c_char = (c"NIVEAU 5: LES EGOUTS ").as_ptr();

pub const HUSTR_6: *const std::ffi::c_char = (c"NIVEAU 6: LE BROYEUR ").as_ptr();

pub const HUSTR_7: *const std::ffi::c_char = (c"NIVEAU 7: L'HERBE DE LA MORT").as_ptr();

pub const HUSTR_8: *const std::ffi::c_char = (c"NIVEAU 8: RUSES ET PIEGES ").as_ptr();

pub const HUSTR_9: *const std::ffi::c_char = (c"NIVEAU 9: LE PUITS ").as_ptr();

pub const HUSTR_10: *const std::ffi::c_char = (c"NIVEAU 10: BASE DE RAVITAILLEMENT ").as_ptr();

pub const HUSTR_11: *const std::ffi::c_char = (c"NIVEAU 11: LE CERCLE DE LA MORT!").as_ptr();

pub const HUSTR_12: *const std::ffi::c_char = (c"NIVEAU 12: L'USINE ").as_ptr();

pub const HUSTR_13: *const std::ffi::c_char = (c"NIVEAU 13: LE CENTRE VILLE").as_ptr();

pub const HUSTR_14: *const std::ffi::c_char = (c"NIVEAU 14: LES ANTRES PROFONDES ").as_ptr();

pub const HUSTR_15: *const std::ffi::c_char = (c"NIVEAU 15: LA ZONE INDUSTRIELLE ").as_ptr();

pub const HUSTR_16: *const std::ffi::c_char = (c"NIVEAU 16: LA BANLIEUE").as_ptr();

pub const HUSTR_17: *const std::ffi::c_char = (c"NIVEAU 17: LES IMMEUBLES").as_ptr();

pub const HUSTR_18: *const std::ffi::c_char = (c"NIVEAU 18: LA COUR ").as_ptr();

pub const HUSTR_19: *const std::ffi::c_char = (c"NIVEAU 19: LA CITADELLE ").as_ptr();

pub const HUSTR_20: *const std::ffi::c_char = (c"NIVEAU 20: JE T'AI EU!").as_ptr();

pub const HUSTR_21: *const std::ffi::c_char = (c"NIVEAU 21: LE NIRVANA").as_ptr();

pub const HUSTR_22: *const std::ffi::c_char = (c"NIVEAU 22: LES CATACOMBES ").as_ptr();

pub const HUSTR_23: *const std::ffi::c_char = (c"NIVEAU 23: LA GRANDE FETE ").as_ptr();

pub const HUSTR_24: *const std::ffi::c_char = (c"NIVEAU 24: LE GOUFFRE ").as_ptr();

pub const HUSTR_25: *const std::ffi::c_char = (c"NIVEAU 25: LES CHUTES DE SANG").as_ptr();

pub const HUSTR_26: *const std::ffi::c_char = (c"NIVEAU 26: LES MINES ABANDONNEES ").as_ptr();

pub const HUSTR_27: *const std::ffi::c_char = (c"NIVEAU 27: CHEZ LES MONSTRES ").as_ptr();

pub const HUSTR_28: *const std::ffi::c_char = (c"NIVEAU 28: LE MONDE DE L'ESPRIT ").as_ptr();

pub const HUSTR_29: *const std::ffi::c_char = (c"NIVEAU 29: LA LIMITE ").as_ptr();

pub const HUSTR_30: *const std::ffi::c_char = (c"NIVEAU 30: L'ICONE DU PECHE ").as_ptr();

pub const HUSTR_31: *const std::ffi::c_char = (c"NIVEAU 31: WOLFENSTEIN").as_ptr();

pub const HUSTR_32: *const std::ffi::c_char = (c"NIVEAU 32: LE MASSACRE").as_ptr();

pub const HUSTR_CHATMACRO1: *const std::ffi::c_char =
    (c"JE SUIS PRET A LEUR EN FAIRE BAVER!").as_ptr();

pub const HUSTR_CHATMACRO2: *const std::ffi::c_char = (c"JE VAIS BIEN.").as_ptr();

pub const HUSTR_CHATMACRO3: *const std::ffi::c_char = (c"JE N'AI PAS L'AIR EN FORME!").as_ptr();

pub const HUSTR_CHATMACRO4: *const std::ffi::c_char = (c"AU SECOURS!").as_ptr();

pub const HUSTR_CHATMACRO5: *const std::ffi::c_char = (c"TU CRAINS!").as_ptr();

pub const HUSTR_CHATMACRO6: *const std::ffi::c_char = (c"LA PROCHAINE FOIS, MINABLE...").as_ptr();

pub const HUSTR_CHATMACRO7: *const std::ffi::c_char = (c"VIENS ICI!").as_ptr();

pub const HUSTR_CHATMACRO8: *const std::ffi::c_char = (c"JE VAIS M'EN OCCUPER.").as_ptr();

pub const HUSTR_CHATMACRO9: *const std::ffi::c_char = (c"OUI").as_ptr();

pub const HUSTR_CHATMACRO0: *const std::ffi::c_char = (c"NON").as_ptr();

pub const HUSTR_TALKTOSELF1: *const std::ffi::c_char = (c"VOUS PARLEZ TOUT SEUL ").as_ptr();

pub const HUSTR_TALKTOSELF2: *const std::ffi::c_char = (c"QUI EST LA?").as_ptr();

pub const HUSTR_TALKTOSELF3: *const std::ffi::c_char = (c"VOUS VOUS FAITES PEUR ").as_ptr();

pub const HUSTR_TALKTOSELF4: *const std::ffi::c_char = (c"VOUS COMMENCEZ A DELIRER ").as_ptr();

pub const HUSTR_TALKTOSELF5: *const std::ffi::c_char = (c"VOUS ETES LARGUE...").as_ptr();

pub const HUSTR_MESSAGESENT: *const std::ffi::c_char = (c"[MESSAGE ENVOYE]").as_ptr();

pub const HUSTR_PLRGREEN: *const std::ffi::c_char = (c"VERT: ").as_ptr();

pub const HUSTR_PLRINDIGO: *const std::ffi::c_char = (c"INDIGO: ").as_ptr();

pub const HUSTR_PLRBROWN: *const std::ffi::c_char = (c"BRUN: ").as_ptr();

pub const HUSTR_PLRRED: *const std::ffi::c_char = (c"ROUGE: ").as_ptr();

pub const HUSTR_KEYGREEN: std::ffi::c_int = (b'g' as std::ffi::c_int);

pub const HUSTR_KEYINDIGO: std::ffi::c_int = (b'i' as std::ffi::c_int);

pub const HUSTR_KEYBROWN: std::ffi::c_int = (b'b' as std::ffi::c_int);

pub const HUSTR_KEYRED: std::ffi::c_int = (b'r' as std::ffi::c_int);

pub const AMSTR_FOLLOWON: *const std::ffi::c_char = (c"MODE POURSUITE ON").as_ptr();

pub const AMSTR_FOLLOWOFF: *const std::ffi::c_char = (c"MODE POURSUITE OFF").as_ptr();

pub const AMSTR_GRIDON: *const std::ffi::c_char = (c"GRILLE ON").as_ptr();

pub const AMSTR_GRIDOFF: *const std::ffi::c_char = (c"GRILLE OFF").as_ptr();

pub const AMSTR_MARKEDSPOT: *const std::ffi::c_char = (c"REPERE MARQUE ").as_ptr();

pub const AMSTR_MARKSCLEARED: *const std::ffi::c_char = (c"REPERES EFFACES ").as_ptr();

pub const STSTR_MUS: *const std::ffi::c_char = (c"CHANGEMENT DE MUSIQUE ").as_ptr();

pub const STSTR_NOMUS: *const std::ffi::c_char = (c"IMPOSSIBLE SELECTION").as_ptr();

pub const STSTR_DQDON: *const std::ffi::c_char = (c"INVULNERABILITE ON ").as_ptr();

pub const STSTR_DQDOFF: *const std::ffi::c_char = (c"INVULNERABILITE OFF").as_ptr();

pub const STSTR_KFAADDED: *const std::ffi::c_char = (c"ARMEMENT MAXIMUM! ").as_ptr();

pub const STSTR_FAADDED: *const std::ffi::c_char = (c"ARMES (SAUF CLEFS) AJOUTEES").as_ptr();

pub const STSTR_NCON: *const std::ffi::c_char = (c"BARRIERES ON").as_ptr();

pub const STSTR_NCOFF: *const std::ffi::c_char = (c"BARRIERES OFF").as_ptr();

pub const STSTR_BEHOLD: *const std::ffi::c_char =
    (c" inVuln, Str, Inviso, Rad, Allmap, or Lite-amp").as_ptr();

pub const STSTR_BEHOLDX: *const std::ffi::c_char = (c"AMELIORATION ACTIVEE").as_ptr();

pub const STSTR_CHOPPERS: *const std::ffi::c_char = (c"... DOESN'T SUCK - GM").as_ptr();

pub const STSTR_CLEV: *const std::ffi::c_char = (c"CHANGEMENT DE NIVEAU...").as_ptr();

pub const E1TEXT: *const std::ffi::c_char = (c"APRES AVOIR VAINCU LES GROS MECHANTS\n").as_ptr();

pub const E2TEXT: *const std::ffi::c_char = (c"VOUS AVEZ REUSSI. L'INFAME DEMON\n").as_ptr();

pub const E3TEXT: *const std::ffi::c_char = (c"LE DEMON ARACHNEEN ET REPUGNANT\n").as_ptr();

pub const C1TEXT: *const std::ffi::c_char =
    (c"VOUS ETES AU PLUS PROFOND DE L'ASTROPORT\n").as_ptr();

pub const C2TEXT: *const std::ffi::c_char =
    (c"VOUS AVEZ GAGNE! VOTRE VICTOIRE A PERMIS\n").as_ptr();

pub const C3TEXT: *const std::ffi::c_char =
    (c"VOUS ETES AU COEUR DE LA CITE CORROMPUE,\n").as_ptr();

pub const C4TEXT: *const std::ffi::c_char = (c"LE VISAGE HORRIBLE D'UN DEMON D'UNE\n").as_ptr();

pub const C5TEXT: *const std::ffi::c_char = (c"FELICITATIONS! VOUS AVEZ TROUVE LE\n").as_ptr();

pub const C6TEXT: *const std::ffi::c_char = (c"FELICITATIONS! VOUS AVEZ DECOUVERT\n").as_ptr();

pub const CC_ZOMBIE: *const std::ffi::c_char = (c"ZOMBIE").as_ptr();

pub const CC_SHOTGUN: *const std::ffi::c_char = (c"TYPE AU FUSIL").as_ptr();

pub const CC_HEAVY: *const std::ffi::c_char = (c"MEC SUPER-ARME").as_ptr();

pub const CC_IMP: *const std::ffi::c_char = (c"DIABLOTIN").as_ptr();

pub const CC_DEMON: *const std::ffi::c_char = (c"DEMON").as_ptr();

pub const CC_LOST: *const std::ffi::c_char = (c"AME PERDUE").as_ptr();

pub const CC_CACO: *const std::ffi::c_char = (c"CACODEMON").as_ptr();

pub const CC_HELL: *const std::ffi::c_char = (c"CHEVALIER DE L'ENFER").as_ptr();

pub const CC_BARON: *const std::ffi::c_char = (c"BARON DE L'ENFER").as_ptr();

pub const CC_ARACH: *const std::ffi::c_char = (c"ARACHNOTRON").as_ptr();

pub const CC_PAIN: *const std::ffi::c_char = (c"ELEMENTAIRE DE LA DOULEUR").as_ptr();

pub const CC_REVEN: *const std::ffi::c_char = (c"REVENANT").as_ptr();

pub const CC_MANCU: *const std::ffi::c_char = (c"MANCUBUS").as_ptr();

pub const CC_ARCH: *const std::ffi::c_char = (c"ARCHI-INFAME").as_ptr();

pub const CC_SPIDER: *const std::ffi::c_char = (c"L'ARAIGNEE CERVEAU").as_ptr();

pub const CC_CYBER: *const std::ffi::c_char = (c"LE CYBERDEMON").as_ptr();

pub const CC_HERO: *const std::ffi::c_char = (c"NOTRE HEROS").as_ptr();
