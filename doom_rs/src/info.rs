use crate::d_think::*;
use crate::doomdata::*;
use crate::doomdef::*;
use crate::doomtype::*;
use crate::m_fixed::*;
use crate::p_mobj::*;
use crate::sounds::*;
use crate::tables::*;

pub const SPR_TROO: std::ffi::c_int = 0;
pub const SPR_SHTG: std::ffi::c_int = SPR_TROO + 1;
pub const SPR_PUNG: std::ffi::c_int = SPR_SHTG + 1;
pub const SPR_PISG: std::ffi::c_int = SPR_PUNG + 1;
pub const SPR_PISF: std::ffi::c_int = SPR_PISG + 1;
pub const SPR_SHTF: std::ffi::c_int = SPR_PISF + 1;
pub const SPR_SHT2: std::ffi::c_int = SPR_SHTF + 1;
pub const SPR_CHGG: std::ffi::c_int = SPR_SHT2 + 1;
pub const SPR_CHGF: std::ffi::c_int = SPR_CHGG + 1;
pub const SPR_MISG: std::ffi::c_int = SPR_CHGF + 1;
pub const SPR_MISF: std::ffi::c_int = SPR_MISG + 1;
pub const SPR_SAWG: std::ffi::c_int = SPR_MISF + 1;
pub const SPR_PLSG: std::ffi::c_int = SPR_SAWG + 1;
pub const SPR_PLSF: std::ffi::c_int = SPR_PLSG + 1;
pub const SPR_BFGG: std::ffi::c_int = SPR_PLSF + 1;
pub const SPR_BFGF: std::ffi::c_int = SPR_BFGG + 1;
pub const SPR_BLUD: std::ffi::c_int = SPR_BFGF + 1;
pub const SPR_PUFF: std::ffi::c_int = SPR_BLUD + 1;
pub const SPR_BAL1: std::ffi::c_int = SPR_PUFF + 1;
pub const SPR_BAL2: std::ffi::c_int = SPR_BAL1 + 1;
pub const SPR_PLSS: std::ffi::c_int = SPR_BAL2 + 1;
pub const SPR_PLSE: std::ffi::c_int = SPR_PLSS + 1;
pub const SPR_MISL: std::ffi::c_int = SPR_PLSE + 1;
pub const SPR_BFS1: std::ffi::c_int = SPR_MISL + 1;
pub const SPR_BFE1: std::ffi::c_int = SPR_BFS1 + 1;
pub const SPR_BFE2: std::ffi::c_int = SPR_BFE1 + 1;
pub const SPR_TFOG: std::ffi::c_int = SPR_BFE2 + 1;
pub const SPR_IFOG: std::ffi::c_int = SPR_TFOG + 1;
pub const SPR_PLAY: std::ffi::c_int = SPR_IFOG + 1;
pub const SPR_POSS: std::ffi::c_int = SPR_PLAY + 1;
pub const SPR_SPOS: std::ffi::c_int = SPR_POSS + 1;
pub const SPR_VILE: std::ffi::c_int = SPR_SPOS + 1;
pub const SPR_FIRE: std::ffi::c_int = SPR_VILE + 1;
pub const SPR_FATB: std::ffi::c_int = SPR_FIRE + 1;
pub const SPR_FBXP: std::ffi::c_int = SPR_FATB + 1;
pub const SPR_SKEL: std::ffi::c_int = SPR_FBXP + 1;
pub const SPR_MANF: std::ffi::c_int = SPR_SKEL + 1;
pub const SPR_FATT: std::ffi::c_int = SPR_MANF + 1;
pub const SPR_CPOS: std::ffi::c_int = SPR_FATT + 1;
pub const SPR_SARG: std::ffi::c_int = SPR_CPOS + 1;
pub const SPR_HEAD: std::ffi::c_int = SPR_SARG + 1;
pub const SPR_BAL7: std::ffi::c_int = SPR_HEAD + 1;
pub const SPR_BOSS: std::ffi::c_int = SPR_BAL7 + 1;
pub const SPR_BOS2: std::ffi::c_int = SPR_BOSS + 1;
pub const SPR_SKUL: std::ffi::c_int = SPR_BOS2 + 1;
pub const SPR_SPID: std::ffi::c_int = SPR_SKUL + 1;
pub const SPR_BSPI: std::ffi::c_int = SPR_SPID + 1;
pub const SPR_APLS: std::ffi::c_int = SPR_BSPI + 1;
pub const SPR_APBX: std::ffi::c_int = SPR_APLS + 1;
pub const SPR_CYBR: std::ffi::c_int = SPR_APBX + 1;
pub const SPR_PAIN: std::ffi::c_int = SPR_CYBR + 1;
pub const SPR_SSWV: std::ffi::c_int = SPR_PAIN + 1;
pub const SPR_KEEN: std::ffi::c_int = SPR_SSWV + 1;
pub const SPR_BBRN: std::ffi::c_int = SPR_KEEN + 1;
pub const SPR_BOSF: std::ffi::c_int = SPR_BBRN + 1;
pub const SPR_ARM1: std::ffi::c_int = SPR_BOSF + 1;
pub const SPR_ARM2: std::ffi::c_int = SPR_ARM1 + 1;
pub const SPR_BAR1: std::ffi::c_int = SPR_ARM2 + 1;
pub const SPR_BEXP: std::ffi::c_int = SPR_BAR1 + 1;
pub const SPR_FCAN: std::ffi::c_int = SPR_BEXP + 1;
pub const SPR_BON1: std::ffi::c_int = SPR_FCAN + 1;
pub const SPR_BON2: std::ffi::c_int = SPR_BON1 + 1;
pub const SPR_BKEY: std::ffi::c_int = SPR_BON2 + 1;
pub const SPR_RKEY: std::ffi::c_int = SPR_BKEY + 1;
pub const SPR_YKEY: std::ffi::c_int = SPR_RKEY + 1;
pub const SPR_BSKU: std::ffi::c_int = SPR_YKEY + 1;
pub const SPR_RSKU: std::ffi::c_int = SPR_BSKU + 1;
pub const SPR_YSKU: std::ffi::c_int = SPR_RSKU + 1;
pub const SPR_STIM: std::ffi::c_int = SPR_YSKU + 1;
pub const SPR_MEDI: std::ffi::c_int = SPR_STIM + 1;
pub const SPR_SOUL: std::ffi::c_int = SPR_MEDI + 1;
pub const SPR_PINV: std::ffi::c_int = SPR_SOUL + 1;
pub const SPR_PSTR: std::ffi::c_int = SPR_PINV + 1;
pub const SPR_PINS: std::ffi::c_int = SPR_PSTR + 1;
pub const SPR_MEGA: std::ffi::c_int = SPR_PINS + 1;
pub const SPR_SUIT: std::ffi::c_int = SPR_MEGA + 1;
pub const SPR_PMAP: std::ffi::c_int = SPR_SUIT + 1;
pub const SPR_PVIS: std::ffi::c_int = SPR_PMAP + 1;
pub const SPR_CLIP: std::ffi::c_int = SPR_PVIS + 1;
pub const SPR_AMMO: std::ffi::c_int = SPR_CLIP + 1;
pub const SPR_ROCK: std::ffi::c_int = SPR_AMMO + 1;
pub const SPR_BROK: std::ffi::c_int = SPR_ROCK + 1;
pub const SPR_CELL: std::ffi::c_int = SPR_BROK + 1;
pub const SPR_CELP: std::ffi::c_int = SPR_CELL + 1;
pub const SPR_SHEL: std::ffi::c_int = SPR_CELP + 1;
pub const SPR_SBOX: std::ffi::c_int = SPR_SHEL + 1;
pub const SPR_BPAK: std::ffi::c_int = SPR_SBOX + 1;
pub const SPR_BFUG: std::ffi::c_int = SPR_BPAK + 1;
pub const SPR_MGUN: std::ffi::c_int = SPR_BFUG + 1;
pub const SPR_CSAW: std::ffi::c_int = SPR_MGUN + 1;
pub const SPR_LAUN: std::ffi::c_int = SPR_CSAW + 1;
pub const SPR_PLAS: std::ffi::c_int = SPR_LAUN + 1;
pub const SPR_SHOT: std::ffi::c_int = SPR_PLAS + 1;
pub const SPR_SGN2: std::ffi::c_int = SPR_SHOT + 1;
pub const SPR_COLU: std::ffi::c_int = SPR_SGN2 + 1;
pub const SPR_SMT2: std::ffi::c_int = SPR_COLU + 1;
pub const SPR_GOR1: std::ffi::c_int = SPR_SMT2 + 1;
pub const SPR_POL2: std::ffi::c_int = SPR_GOR1 + 1;
pub const SPR_POL5: std::ffi::c_int = SPR_POL2 + 1;
pub const SPR_POL4: std::ffi::c_int = SPR_POL5 + 1;
pub const SPR_POL3: std::ffi::c_int = SPR_POL4 + 1;
pub const SPR_POL1: std::ffi::c_int = SPR_POL3 + 1;
pub const SPR_POL6: std::ffi::c_int = SPR_POL1 + 1;
pub const SPR_GOR2: std::ffi::c_int = SPR_POL6 + 1;
pub const SPR_GOR3: std::ffi::c_int = SPR_GOR2 + 1;
pub const SPR_GOR4: std::ffi::c_int = SPR_GOR3 + 1;
pub const SPR_GOR5: std::ffi::c_int = SPR_GOR4 + 1;
pub const SPR_SMIT: std::ffi::c_int = SPR_GOR5 + 1;
pub const SPR_COL1: std::ffi::c_int = SPR_SMIT + 1;
pub const SPR_COL2: std::ffi::c_int = SPR_COL1 + 1;
pub const SPR_COL3: std::ffi::c_int = SPR_COL2 + 1;
pub const SPR_COL4: std::ffi::c_int = SPR_COL3 + 1;
pub const SPR_CAND: std::ffi::c_int = SPR_COL4 + 1;
pub const SPR_CBRA: std::ffi::c_int = SPR_CAND + 1;
pub const SPR_COL6: std::ffi::c_int = SPR_CBRA + 1;
pub const SPR_TRE1: std::ffi::c_int = SPR_COL6 + 1;
pub const SPR_TRE2: std::ffi::c_int = SPR_TRE1 + 1;
pub const SPR_ELEC: std::ffi::c_int = SPR_TRE2 + 1;
pub const SPR_CEYE: std::ffi::c_int = SPR_ELEC + 1;
pub const SPR_FSKU: std::ffi::c_int = SPR_CEYE + 1;
pub const SPR_COL5: std::ffi::c_int = SPR_FSKU + 1;
pub const SPR_TBLU: std::ffi::c_int = SPR_COL5 + 1;
pub const SPR_TGRN: std::ffi::c_int = SPR_TBLU + 1;
pub const SPR_TRED: std::ffi::c_int = SPR_TGRN + 1;
pub const SPR_SMBT: std::ffi::c_int = SPR_TRED + 1;
pub const SPR_SMGT: std::ffi::c_int = SPR_SMBT + 1;
pub const SPR_SMRT: std::ffi::c_int = SPR_SMGT + 1;
pub const SPR_HDB1: std::ffi::c_int = SPR_SMRT + 1;
pub const SPR_HDB2: std::ffi::c_int = SPR_HDB1 + 1;
pub const SPR_HDB3: std::ffi::c_int = SPR_HDB2 + 1;
pub const SPR_HDB4: std::ffi::c_int = SPR_HDB3 + 1;
pub const SPR_HDB5: std::ffi::c_int = SPR_HDB4 + 1;
pub const SPR_HDB6: std::ffi::c_int = SPR_HDB5 + 1;
pub const SPR_POB1: std::ffi::c_int = SPR_HDB6 + 1;
pub const SPR_POB2: std::ffi::c_int = SPR_POB1 + 1;
pub const SPR_BRS1: std::ffi::c_int = SPR_POB2 + 1;
pub const SPR_TLMP: std::ffi::c_int = SPR_BRS1 + 1;
pub const SPR_TLP2: std::ffi::c_int = SPR_TLMP + 1;
pub const NUMSPRITES: std::ffi::c_int = SPR_TLP2 + 1;

pub type spritenum_t = std::ffi::c_int;

pub const S_NULL: std::ffi::c_int = 0;
pub const S_LIGHTDONE: std::ffi::c_int = S_NULL + 1;
pub const S_PUNCH: std::ffi::c_int = S_LIGHTDONE + 1;
pub const S_PUNCHDOWN: std::ffi::c_int = S_PUNCH + 1;
pub const S_PUNCHUP: std::ffi::c_int = S_PUNCHDOWN + 1;
pub const S_PUNCH1: std::ffi::c_int = S_PUNCHUP + 1;
pub const S_PUNCH2: std::ffi::c_int = S_PUNCH1 + 1;
pub const S_PUNCH3: std::ffi::c_int = S_PUNCH2 + 1;
pub const S_PUNCH4: std::ffi::c_int = S_PUNCH3 + 1;
pub const S_PUNCH5: std::ffi::c_int = S_PUNCH4 + 1;
pub const S_PISTOL: std::ffi::c_int = S_PUNCH5 + 1;
pub const S_PISTOLDOWN: std::ffi::c_int = S_PISTOL + 1;
pub const S_PISTOLUP: std::ffi::c_int = S_PISTOLDOWN + 1;
pub const S_PISTOL1: std::ffi::c_int = S_PISTOLUP + 1;
pub const S_PISTOL2: std::ffi::c_int = S_PISTOL1 + 1;
pub const S_PISTOL3: std::ffi::c_int = S_PISTOL2 + 1;
pub const S_PISTOL4: std::ffi::c_int = S_PISTOL3 + 1;
pub const S_PISTOLFLASH: std::ffi::c_int = S_PISTOL4 + 1;
pub const S_SGUN: std::ffi::c_int = S_PISTOLFLASH + 1;
pub const S_SGUNDOWN: std::ffi::c_int = S_SGUN + 1;
pub const S_SGUNUP: std::ffi::c_int = S_SGUNDOWN + 1;
pub const S_SGUN1: std::ffi::c_int = S_SGUNUP + 1;
pub const S_SGUN2: std::ffi::c_int = S_SGUN1 + 1;
pub const S_SGUN3: std::ffi::c_int = S_SGUN2 + 1;
pub const S_SGUN4: std::ffi::c_int = S_SGUN3 + 1;
pub const S_SGUN5: std::ffi::c_int = S_SGUN4 + 1;
pub const S_SGUN6: std::ffi::c_int = S_SGUN5 + 1;
pub const S_SGUN7: std::ffi::c_int = S_SGUN6 + 1;
pub const S_SGUN8: std::ffi::c_int = S_SGUN7 + 1;
pub const S_SGUN9: std::ffi::c_int = S_SGUN8 + 1;
pub const S_SGUNFLASH1: std::ffi::c_int = S_SGUN9 + 1;
pub const S_SGUNFLASH2: std::ffi::c_int = S_SGUNFLASH1 + 1;
pub const S_DSGUN: std::ffi::c_int = S_SGUNFLASH2 + 1;
pub const S_DSGUNDOWN: std::ffi::c_int = S_DSGUN + 1;
pub const S_DSGUNUP: std::ffi::c_int = S_DSGUNDOWN + 1;
pub const S_DSGUN1: std::ffi::c_int = S_DSGUNUP + 1;
pub const S_DSGUN2: std::ffi::c_int = S_DSGUN1 + 1;
pub const S_DSGUN3: std::ffi::c_int = S_DSGUN2 + 1;
pub const S_DSGUN4: std::ffi::c_int = S_DSGUN3 + 1;
pub const S_DSGUN5: std::ffi::c_int = S_DSGUN4 + 1;
pub const S_DSGUN6: std::ffi::c_int = S_DSGUN5 + 1;
pub const S_DSGUN7: std::ffi::c_int = S_DSGUN6 + 1;
pub const S_DSGUN8: std::ffi::c_int = S_DSGUN7 + 1;
pub const S_DSGUN9: std::ffi::c_int = S_DSGUN8 + 1;
pub const S_DSGUN10: std::ffi::c_int = S_DSGUN9 + 1;
pub const S_DSNR1: std::ffi::c_int = S_DSGUN10 + 1;
pub const S_DSNR2: std::ffi::c_int = S_DSNR1 + 1;
pub const S_DSGUNFLASH1: std::ffi::c_int = S_DSNR2 + 1;
pub const S_DSGUNFLASH2: std::ffi::c_int = S_DSGUNFLASH1 + 1;
pub const S_CHAIN: std::ffi::c_int = S_DSGUNFLASH2 + 1;
pub const S_CHAINDOWN: std::ffi::c_int = S_CHAIN + 1;
pub const S_CHAINUP: std::ffi::c_int = S_CHAINDOWN + 1;
pub const S_CHAIN1: std::ffi::c_int = S_CHAINUP + 1;
pub const S_CHAIN2: std::ffi::c_int = S_CHAIN1 + 1;
pub const S_CHAIN3: std::ffi::c_int = S_CHAIN2 + 1;
pub const S_CHAINFLASH1: std::ffi::c_int = S_CHAIN3 + 1;
pub const S_CHAINFLASH2: std::ffi::c_int = S_CHAINFLASH1 + 1;
pub const S_MISSILE: std::ffi::c_int = S_CHAINFLASH2 + 1;
pub const S_MISSILEDOWN: std::ffi::c_int = S_MISSILE + 1;
pub const S_MISSILEUP: std::ffi::c_int = S_MISSILEDOWN + 1;
pub const S_MISSILE1: std::ffi::c_int = S_MISSILEUP + 1;
pub const S_MISSILE2: std::ffi::c_int = S_MISSILE1 + 1;
pub const S_MISSILE3: std::ffi::c_int = S_MISSILE2 + 1;
pub const S_MISSILEFLASH1: std::ffi::c_int = S_MISSILE3 + 1;
pub const S_MISSILEFLASH2: std::ffi::c_int = S_MISSILEFLASH1 + 1;
pub const S_MISSILEFLASH3: std::ffi::c_int = S_MISSILEFLASH2 + 1;
pub const S_MISSILEFLASH4: std::ffi::c_int = S_MISSILEFLASH3 + 1;
pub const S_SAW: std::ffi::c_int = S_MISSILEFLASH4 + 1;
pub const S_SAWB: std::ffi::c_int = S_SAW + 1;
pub const S_SAWDOWN: std::ffi::c_int = S_SAWB + 1;
pub const S_SAWUP: std::ffi::c_int = S_SAWDOWN + 1;
pub const S_SAW1: std::ffi::c_int = S_SAWUP + 1;
pub const S_SAW2: std::ffi::c_int = S_SAW1 + 1;
pub const S_SAW3: std::ffi::c_int = S_SAW2 + 1;
pub const S_PLASMA: std::ffi::c_int = S_SAW3 + 1;
pub const S_PLASMADOWN: std::ffi::c_int = S_PLASMA + 1;
pub const S_PLASMAUP: std::ffi::c_int = S_PLASMADOWN + 1;
pub const S_PLASMA1: std::ffi::c_int = S_PLASMAUP + 1;
pub const S_PLASMA2: std::ffi::c_int = S_PLASMA1 + 1;
pub const S_PLASMAFLASH1: std::ffi::c_int = S_PLASMA2 + 1;
pub const S_PLASMAFLASH2: std::ffi::c_int = S_PLASMAFLASH1 + 1;
pub const S_BFG: std::ffi::c_int = S_PLASMAFLASH2 + 1;
pub const S_BFGDOWN: std::ffi::c_int = S_BFG + 1;
pub const S_BFGUP: std::ffi::c_int = S_BFGDOWN + 1;
pub const S_BFG1: std::ffi::c_int = S_BFGUP + 1;
pub const S_BFG2: std::ffi::c_int = S_BFG1 + 1;
pub const S_BFG3: std::ffi::c_int = S_BFG2 + 1;
pub const S_BFG4: std::ffi::c_int = S_BFG3 + 1;
pub const S_BFGFLASH1: std::ffi::c_int = S_BFG4 + 1;
pub const S_BFGFLASH2: std::ffi::c_int = S_BFGFLASH1 + 1;
pub const S_BLOOD1: std::ffi::c_int = S_BFGFLASH2 + 1;
pub const S_BLOOD2: std::ffi::c_int = S_BLOOD1 + 1;
pub const S_BLOOD3: std::ffi::c_int = S_BLOOD2 + 1;
pub const S_PUFF1: std::ffi::c_int = S_BLOOD3 + 1;
pub const S_PUFF2: std::ffi::c_int = S_PUFF1 + 1;
pub const S_PUFF3: std::ffi::c_int = S_PUFF2 + 1;
pub const S_PUFF4: std::ffi::c_int = S_PUFF3 + 1;
pub const S_TBALL1: std::ffi::c_int = S_PUFF4 + 1;
pub const S_TBALL2: std::ffi::c_int = S_TBALL1 + 1;
pub const S_TBALLX1: std::ffi::c_int = S_TBALL2 + 1;
pub const S_TBALLX2: std::ffi::c_int = S_TBALLX1 + 1;
pub const S_TBALLX3: std::ffi::c_int = S_TBALLX2 + 1;
pub const S_RBALL1: std::ffi::c_int = S_TBALLX3 + 1;
pub const S_RBALL2: std::ffi::c_int = S_RBALL1 + 1;
pub const S_RBALLX1: std::ffi::c_int = S_RBALL2 + 1;
pub const S_RBALLX2: std::ffi::c_int = S_RBALLX1 + 1;
pub const S_RBALLX3: std::ffi::c_int = S_RBALLX2 + 1;
pub const S_PLASBALL: std::ffi::c_int = S_RBALLX3 + 1;
pub const S_PLASBALL2: std::ffi::c_int = S_PLASBALL + 1;
pub const S_PLASEXP: std::ffi::c_int = S_PLASBALL2 + 1;
pub const S_PLASEXP2: std::ffi::c_int = S_PLASEXP + 1;
pub const S_PLASEXP3: std::ffi::c_int = S_PLASEXP2 + 1;
pub const S_PLASEXP4: std::ffi::c_int = S_PLASEXP3 + 1;
pub const S_PLASEXP5: std::ffi::c_int = S_PLASEXP4 + 1;
pub const S_ROCKET: std::ffi::c_int = S_PLASEXP5 + 1;
pub const S_BFGSHOT: std::ffi::c_int = S_ROCKET + 1;
pub const S_BFGSHOT2: std::ffi::c_int = S_BFGSHOT + 1;
pub const S_BFGLAND: std::ffi::c_int = S_BFGSHOT2 + 1;
pub const S_BFGLAND2: std::ffi::c_int = S_BFGLAND + 1;
pub const S_BFGLAND3: std::ffi::c_int = S_BFGLAND2 + 1;
pub const S_BFGLAND4: std::ffi::c_int = S_BFGLAND3 + 1;
pub const S_BFGLAND5: std::ffi::c_int = S_BFGLAND4 + 1;
pub const S_BFGLAND6: std::ffi::c_int = S_BFGLAND5 + 1;
pub const S_BFGEXP: std::ffi::c_int = S_BFGLAND6 + 1;
pub const S_BFGEXP2: std::ffi::c_int = S_BFGEXP + 1;
pub const S_BFGEXP3: std::ffi::c_int = S_BFGEXP2 + 1;
pub const S_BFGEXP4: std::ffi::c_int = S_BFGEXP3 + 1;
pub const S_EXPLODE1: std::ffi::c_int = S_BFGEXP4 + 1;
pub const S_EXPLODE2: std::ffi::c_int = S_EXPLODE1 + 1;
pub const S_EXPLODE3: std::ffi::c_int = S_EXPLODE2 + 1;
pub const S_TFOG: std::ffi::c_int = S_EXPLODE3 + 1;
pub const S_TFOG01: std::ffi::c_int = S_TFOG + 1;
pub const S_TFOG02: std::ffi::c_int = S_TFOG01 + 1;
pub const S_TFOG2: std::ffi::c_int = S_TFOG02 + 1;
pub const S_TFOG3: std::ffi::c_int = S_TFOG2 + 1;
pub const S_TFOG4: std::ffi::c_int = S_TFOG3 + 1;
pub const S_TFOG5: std::ffi::c_int = S_TFOG4 + 1;
pub const S_TFOG6: std::ffi::c_int = S_TFOG5 + 1;
pub const S_TFOG7: std::ffi::c_int = S_TFOG6 + 1;
pub const S_TFOG8: std::ffi::c_int = S_TFOG7 + 1;
pub const S_TFOG9: std::ffi::c_int = S_TFOG8 + 1;
pub const S_TFOG10: std::ffi::c_int = S_TFOG9 + 1;
pub const S_IFOG: std::ffi::c_int = S_TFOG10 + 1;
pub const S_IFOG01: std::ffi::c_int = S_IFOG + 1;
pub const S_IFOG02: std::ffi::c_int = S_IFOG01 + 1;
pub const S_IFOG2: std::ffi::c_int = S_IFOG02 + 1;
pub const S_IFOG3: std::ffi::c_int = S_IFOG2 + 1;
pub const S_IFOG4: std::ffi::c_int = S_IFOG3 + 1;
pub const S_IFOG5: std::ffi::c_int = S_IFOG4 + 1;
pub const S_PLAY: std::ffi::c_int = S_IFOG5 + 1;
pub const S_PLAY_RUN1: std::ffi::c_int = S_PLAY + 1;
pub const S_PLAY_RUN2: std::ffi::c_int = S_PLAY_RUN1 + 1;
pub const S_PLAY_RUN3: std::ffi::c_int = S_PLAY_RUN2 + 1;
pub const S_PLAY_RUN4: std::ffi::c_int = S_PLAY_RUN3 + 1;
pub const S_PLAY_ATK1: std::ffi::c_int = S_PLAY_RUN4 + 1;
pub const S_PLAY_ATK2: std::ffi::c_int = S_PLAY_ATK1 + 1;
pub const S_PLAY_PAIN: std::ffi::c_int = S_PLAY_ATK2 + 1;
pub const S_PLAY_PAIN2: std::ffi::c_int = S_PLAY_PAIN + 1;
pub const S_PLAY_DIE1: std::ffi::c_int = S_PLAY_PAIN2 + 1;
pub const S_PLAY_DIE2: std::ffi::c_int = S_PLAY_DIE1 + 1;
pub const S_PLAY_DIE3: std::ffi::c_int = S_PLAY_DIE2 + 1;
pub const S_PLAY_DIE4: std::ffi::c_int = S_PLAY_DIE3 + 1;
pub const S_PLAY_DIE5: std::ffi::c_int = S_PLAY_DIE4 + 1;
pub const S_PLAY_DIE6: std::ffi::c_int = S_PLAY_DIE5 + 1;
pub const S_PLAY_DIE7: std::ffi::c_int = S_PLAY_DIE6 + 1;
pub const S_PLAY_XDIE1: std::ffi::c_int = S_PLAY_DIE7 + 1;
pub const S_PLAY_XDIE2: std::ffi::c_int = S_PLAY_XDIE1 + 1;
pub const S_PLAY_XDIE3: std::ffi::c_int = S_PLAY_XDIE2 + 1;
pub const S_PLAY_XDIE4: std::ffi::c_int = S_PLAY_XDIE3 + 1;
pub const S_PLAY_XDIE5: std::ffi::c_int = S_PLAY_XDIE4 + 1;
pub const S_PLAY_XDIE6: std::ffi::c_int = S_PLAY_XDIE5 + 1;
pub const S_PLAY_XDIE7: std::ffi::c_int = S_PLAY_XDIE6 + 1;
pub const S_PLAY_XDIE8: std::ffi::c_int = S_PLAY_XDIE7 + 1;
pub const S_PLAY_XDIE9: std::ffi::c_int = S_PLAY_XDIE8 + 1;
pub const S_POSS_STND: std::ffi::c_int = S_PLAY_XDIE9 + 1;
pub const S_POSS_STND2: std::ffi::c_int = S_POSS_STND + 1;
pub const S_POSS_RUN1: std::ffi::c_int = S_POSS_STND2 + 1;
pub const S_POSS_RUN2: std::ffi::c_int = S_POSS_RUN1 + 1;
pub const S_POSS_RUN3: std::ffi::c_int = S_POSS_RUN2 + 1;
pub const S_POSS_RUN4: std::ffi::c_int = S_POSS_RUN3 + 1;
pub const S_POSS_RUN5: std::ffi::c_int = S_POSS_RUN4 + 1;
pub const S_POSS_RUN6: std::ffi::c_int = S_POSS_RUN5 + 1;
pub const S_POSS_RUN7: std::ffi::c_int = S_POSS_RUN6 + 1;
pub const S_POSS_RUN8: std::ffi::c_int = S_POSS_RUN7 + 1;
pub const S_POSS_ATK1: std::ffi::c_int = S_POSS_RUN8 + 1;
pub const S_POSS_ATK2: std::ffi::c_int = S_POSS_ATK1 + 1;
pub const S_POSS_ATK3: std::ffi::c_int = S_POSS_ATK2 + 1;
pub const S_POSS_PAIN: std::ffi::c_int = S_POSS_ATK3 + 1;
pub const S_POSS_PAIN2: std::ffi::c_int = S_POSS_PAIN + 1;
pub const S_POSS_DIE1: std::ffi::c_int = S_POSS_PAIN2 + 1;
pub const S_POSS_DIE2: std::ffi::c_int = S_POSS_DIE1 + 1;
pub const S_POSS_DIE3: std::ffi::c_int = S_POSS_DIE2 + 1;
pub const S_POSS_DIE4: std::ffi::c_int = S_POSS_DIE3 + 1;
pub const S_POSS_DIE5: std::ffi::c_int = S_POSS_DIE4 + 1;
pub const S_POSS_XDIE1: std::ffi::c_int = S_POSS_DIE5 + 1;
pub const S_POSS_XDIE2: std::ffi::c_int = S_POSS_XDIE1 + 1;
pub const S_POSS_XDIE3: std::ffi::c_int = S_POSS_XDIE2 + 1;
pub const S_POSS_XDIE4: std::ffi::c_int = S_POSS_XDIE3 + 1;
pub const S_POSS_XDIE5: std::ffi::c_int = S_POSS_XDIE4 + 1;
pub const S_POSS_XDIE6: std::ffi::c_int = S_POSS_XDIE5 + 1;
pub const S_POSS_XDIE7: std::ffi::c_int = S_POSS_XDIE6 + 1;
pub const S_POSS_XDIE8: std::ffi::c_int = S_POSS_XDIE7 + 1;
pub const S_POSS_XDIE9: std::ffi::c_int = S_POSS_XDIE8 + 1;
pub const S_POSS_RAISE1: std::ffi::c_int = S_POSS_XDIE9 + 1;
pub const S_POSS_RAISE2: std::ffi::c_int = S_POSS_RAISE1 + 1;
pub const S_POSS_RAISE3: std::ffi::c_int = S_POSS_RAISE2 + 1;
pub const S_POSS_RAISE4: std::ffi::c_int = S_POSS_RAISE3 + 1;
pub const S_SPOS_STND: std::ffi::c_int = S_POSS_RAISE4 + 1;
pub const S_SPOS_STND2: std::ffi::c_int = S_SPOS_STND + 1;
pub const S_SPOS_RUN1: std::ffi::c_int = S_SPOS_STND2 + 1;
pub const S_SPOS_RUN2: std::ffi::c_int = S_SPOS_RUN1 + 1;
pub const S_SPOS_RUN3: std::ffi::c_int = S_SPOS_RUN2 + 1;
pub const S_SPOS_RUN4: std::ffi::c_int = S_SPOS_RUN3 + 1;
pub const S_SPOS_RUN5: std::ffi::c_int = S_SPOS_RUN4 + 1;
pub const S_SPOS_RUN6: std::ffi::c_int = S_SPOS_RUN5 + 1;
pub const S_SPOS_RUN7: std::ffi::c_int = S_SPOS_RUN6 + 1;
pub const S_SPOS_RUN8: std::ffi::c_int = S_SPOS_RUN7 + 1;
pub const S_SPOS_ATK1: std::ffi::c_int = S_SPOS_RUN8 + 1;
pub const S_SPOS_ATK2: std::ffi::c_int = S_SPOS_ATK1 + 1;
pub const S_SPOS_ATK3: std::ffi::c_int = S_SPOS_ATK2 + 1;
pub const S_SPOS_PAIN: std::ffi::c_int = S_SPOS_ATK3 + 1;
pub const S_SPOS_PAIN2: std::ffi::c_int = S_SPOS_PAIN + 1;
pub const S_SPOS_DIE1: std::ffi::c_int = S_SPOS_PAIN2 + 1;
pub const S_SPOS_DIE2: std::ffi::c_int = S_SPOS_DIE1 + 1;
pub const S_SPOS_DIE3: std::ffi::c_int = S_SPOS_DIE2 + 1;
pub const S_SPOS_DIE4: std::ffi::c_int = S_SPOS_DIE3 + 1;
pub const S_SPOS_DIE5: std::ffi::c_int = S_SPOS_DIE4 + 1;
pub const S_SPOS_XDIE1: std::ffi::c_int = S_SPOS_DIE5 + 1;
pub const S_SPOS_XDIE2: std::ffi::c_int = S_SPOS_XDIE1 + 1;
pub const S_SPOS_XDIE3: std::ffi::c_int = S_SPOS_XDIE2 + 1;
pub const S_SPOS_XDIE4: std::ffi::c_int = S_SPOS_XDIE3 + 1;
pub const S_SPOS_XDIE5: std::ffi::c_int = S_SPOS_XDIE4 + 1;
pub const S_SPOS_XDIE6: std::ffi::c_int = S_SPOS_XDIE5 + 1;
pub const S_SPOS_XDIE7: std::ffi::c_int = S_SPOS_XDIE6 + 1;
pub const S_SPOS_XDIE8: std::ffi::c_int = S_SPOS_XDIE7 + 1;
pub const S_SPOS_XDIE9: std::ffi::c_int = S_SPOS_XDIE8 + 1;
pub const S_SPOS_RAISE1: std::ffi::c_int = S_SPOS_XDIE9 + 1;
pub const S_SPOS_RAISE2: std::ffi::c_int = S_SPOS_RAISE1 + 1;
pub const S_SPOS_RAISE3: std::ffi::c_int = S_SPOS_RAISE2 + 1;
pub const S_SPOS_RAISE4: std::ffi::c_int = S_SPOS_RAISE3 + 1;
pub const S_SPOS_RAISE5: std::ffi::c_int = S_SPOS_RAISE4 + 1;
pub const S_VILE_STND: std::ffi::c_int = S_SPOS_RAISE5 + 1;
pub const S_VILE_STND2: std::ffi::c_int = S_VILE_STND + 1;
pub const S_VILE_RUN1: std::ffi::c_int = S_VILE_STND2 + 1;
pub const S_VILE_RUN2: std::ffi::c_int = S_VILE_RUN1 + 1;
pub const S_VILE_RUN3: std::ffi::c_int = S_VILE_RUN2 + 1;
pub const S_VILE_RUN4: std::ffi::c_int = S_VILE_RUN3 + 1;
pub const S_VILE_RUN5: std::ffi::c_int = S_VILE_RUN4 + 1;
pub const S_VILE_RUN6: std::ffi::c_int = S_VILE_RUN5 + 1;
pub const S_VILE_RUN7: std::ffi::c_int = S_VILE_RUN6 + 1;
pub const S_VILE_RUN8: std::ffi::c_int = S_VILE_RUN7 + 1;
pub const S_VILE_RUN9: std::ffi::c_int = S_VILE_RUN8 + 1;
pub const S_VILE_RUN10: std::ffi::c_int = S_VILE_RUN9 + 1;
pub const S_VILE_RUN11: std::ffi::c_int = S_VILE_RUN10 + 1;
pub const S_VILE_RUN12: std::ffi::c_int = S_VILE_RUN11 + 1;
pub const S_VILE_ATK1: std::ffi::c_int = S_VILE_RUN12 + 1;
pub const S_VILE_ATK2: std::ffi::c_int = S_VILE_ATK1 + 1;
pub const S_VILE_ATK3: std::ffi::c_int = S_VILE_ATK2 + 1;
pub const S_VILE_ATK4: std::ffi::c_int = S_VILE_ATK3 + 1;
pub const S_VILE_ATK5: std::ffi::c_int = S_VILE_ATK4 + 1;
pub const S_VILE_ATK6: std::ffi::c_int = S_VILE_ATK5 + 1;
pub const S_VILE_ATK7: std::ffi::c_int = S_VILE_ATK6 + 1;
pub const S_VILE_ATK8: std::ffi::c_int = S_VILE_ATK7 + 1;
pub const S_VILE_ATK9: std::ffi::c_int = S_VILE_ATK8 + 1;
pub const S_VILE_ATK10: std::ffi::c_int = S_VILE_ATK9 + 1;
pub const S_VILE_ATK11: std::ffi::c_int = S_VILE_ATK10 + 1;
pub const S_VILE_HEAL1: std::ffi::c_int = S_VILE_ATK11 + 1;
pub const S_VILE_HEAL2: std::ffi::c_int = S_VILE_HEAL1 + 1;
pub const S_VILE_HEAL3: std::ffi::c_int = S_VILE_HEAL2 + 1;
pub const S_VILE_PAIN: std::ffi::c_int = S_VILE_HEAL3 + 1;
pub const S_VILE_PAIN2: std::ffi::c_int = S_VILE_PAIN + 1;
pub const S_VILE_DIE1: std::ffi::c_int = S_VILE_PAIN2 + 1;
pub const S_VILE_DIE2: std::ffi::c_int = S_VILE_DIE1 + 1;
pub const S_VILE_DIE3: std::ffi::c_int = S_VILE_DIE2 + 1;
pub const S_VILE_DIE4: std::ffi::c_int = S_VILE_DIE3 + 1;
pub const S_VILE_DIE5: std::ffi::c_int = S_VILE_DIE4 + 1;
pub const S_VILE_DIE6: std::ffi::c_int = S_VILE_DIE5 + 1;
pub const S_VILE_DIE7: std::ffi::c_int = S_VILE_DIE6 + 1;
pub const S_VILE_DIE8: std::ffi::c_int = S_VILE_DIE7 + 1;
pub const S_VILE_DIE9: std::ffi::c_int = S_VILE_DIE8 + 1;
pub const S_VILE_DIE10: std::ffi::c_int = S_VILE_DIE9 + 1;
pub const S_FIRE1: std::ffi::c_int = S_VILE_DIE10 + 1;
pub const S_FIRE2: std::ffi::c_int = S_FIRE1 + 1;
pub const S_FIRE3: std::ffi::c_int = S_FIRE2 + 1;
pub const S_FIRE4: std::ffi::c_int = S_FIRE3 + 1;
pub const S_FIRE5: std::ffi::c_int = S_FIRE4 + 1;
pub const S_FIRE6: std::ffi::c_int = S_FIRE5 + 1;
pub const S_FIRE7: std::ffi::c_int = S_FIRE6 + 1;
pub const S_FIRE8: std::ffi::c_int = S_FIRE7 + 1;
pub const S_FIRE9: std::ffi::c_int = S_FIRE8 + 1;
pub const S_FIRE10: std::ffi::c_int = S_FIRE9 + 1;
pub const S_FIRE11: std::ffi::c_int = S_FIRE10 + 1;
pub const S_FIRE12: std::ffi::c_int = S_FIRE11 + 1;
pub const S_FIRE13: std::ffi::c_int = S_FIRE12 + 1;
pub const S_FIRE14: std::ffi::c_int = S_FIRE13 + 1;
pub const S_FIRE15: std::ffi::c_int = S_FIRE14 + 1;
pub const S_FIRE16: std::ffi::c_int = S_FIRE15 + 1;
pub const S_FIRE17: std::ffi::c_int = S_FIRE16 + 1;
pub const S_FIRE18: std::ffi::c_int = S_FIRE17 + 1;
pub const S_FIRE19: std::ffi::c_int = S_FIRE18 + 1;
pub const S_FIRE20: std::ffi::c_int = S_FIRE19 + 1;
pub const S_FIRE21: std::ffi::c_int = S_FIRE20 + 1;
pub const S_FIRE22: std::ffi::c_int = S_FIRE21 + 1;
pub const S_FIRE23: std::ffi::c_int = S_FIRE22 + 1;
pub const S_FIRE24: std::ffi::c_int = S_FIRE23 + 1;
pub const S_FIRE25: std::ffi::c_int = S_FIRE24 + 1;
pub const S_FIRE26: std::ffi::c_int = S_FIRE25 + 1;
pub const S_FIRE27: std::ffi::c_int = S_FIRE26 + 1;
pub const S_FIRE28: std::ffi::c_int = S_FIRE27 + 1;
pub const S_FIRE29: std::ffi::c_int = S_FIRE28 + 1;
pub const S_FIRE30: std::ffi::c_int = S_FIRE29 + 1;
pub const S_SMOKE1: std::ffi::c_int = S_FIRE30 + 1;
pub const S_SMOKE2: std::ffi::c_int = S_SMOKE1 + 1;
pub const S_SMOKE3: std::ffi::c_int = S_SMOKE2 + 1;
pub const S_SMOKE4: std::ffi::c_int = S_SMOKE3 + 1;
pub const S_SMOKE5: std::ffi::c_int = S_SMOKE4 + 1;
pub const S_TRACER: std::ffi::c_int = S_SMOKE5 + 1;
pub const S_TRACER2: std::ffi::c_int = S_TRACER + 1;
pub const S_TRACEEXP1: std::ffi::c_int = S_TRACER2 + 1;
pub const S_TRACEEXP2: std::ffi::c_int = S_TRACEEXP1 + 1;
pub const S_TRACEEXP3: std::ffi::c_int = S_TRACEEXP2 + 1;
pub const S_SKEL_STND: std::ffi::c_int = S_TRACEEXP3 + 1;
pub const S_SKEL_STND2: std::ffi::c_int = S_SKEL_STND + 1;
pub const S_SKEL_RUN1: std::ffi::c_int = S_SKEL_STND2 + 1;
pub const S_SKEL_RUN2: std::ffi::c_int = S_SKEL_RUN1 + 1;
pub const S_SKEL_RUN3: std::ffi::c_int = S_SKEL_RUN2 + 1;
pub const S_SKEL_RUN4: std::ffi::c_int = S_SKEL_RUN3 + 1;
pub const S_SKEL_RUN5: std::ffi::c_int = S_SKEL_RUN4 + 1;
pub const S_SKEL_RUN6: std::ffi::c_int = S_SKEL_RUN5 + 1;
pub const S_SKEL_RUN7: std::ffi::c_int = S_SKEL_RUN6 + 1;
pub const S_SKEL_RUN8: std::ffi::c_int = S_SKEL_RUN7 + 1;
pub const S_SKEL_RUN9: std::ffi::c_int = S_SKEL_RUN8 + 1;
pub const S_SKEL_RUN10: std::ffi::c_int = S_SKEL_RUN9 + 1;
pub const S_SKEL_RUN11: std::ffi::c_int = S_SKEL_RUN10 + 1;
pub const S_SKEL_RUN12: std::ffi::c_int = S_SKEL_RUN11 + 1;
pub const S_SKEL_FIST1: std::ffi::c_int = S_SKEL_RUN12 + 1;
pub const S_SKEL_FIST2: std::ffi::c_int = S_SKEL_FIST1 + 1;
pub const S_SKEL_FIST3: std::ffi::c_int = S_SKEL_FIST2 + 1;
pub const S_SKEL_FIST4: std::ffi::c_int = S_SKEL_FIST3 + 1;
pub const S_SKEL_MISS1: std::ffi::c_int = S_SKEL_FIST4 + 1;
pub const S_SKEL_MISS2: std::ffi::c_int = S_SKEL_MISS1 + 1;
pub const S_SKEL_MISS3: std::ffi::c_int = S_SKEL_MISS2 + 1;
pub const S_SKEL_MISS4: std::ffi::c_int = S_SKEL_MISS3 + 1;
pub const S_SKEL_PAIN: std::ffi::c_int = S_SKEL_MISS4 + 1;
pub const S_SKEL_PAIN2: std::ffi::c_int = S_SKEL_PAIN + 1;
pub const S_SKEL_DIE1: std::ffi::c_int = S_SKEL_PAIN2 + 1;
pub const S_SKEL_DIE2: std::ffi::c_int = S_SKEL_DIE1 + 1;
pub const S_SKEL_DIE3: std::ffi::c_int = S_SKEL_DIE2 + 1;
pub const S_SKEL_DIE4: std::ffi::c_int = S_SKEL_DIE3 + 1;
pub const S_SKEL_DIE5: std::ffi::c_int = S_SKEL_DIE4 + 1;
pub const S_SKEL_DIE6: std::ffi::c_int = S_SKEL_DIE5 + 1;
pub const S_SKEL_RAISE1: std::ffi::c_int = S_SKEL_DIE6 + 1;
pub const S_SKEL_RAISE2: std::ffi::c_int = S_SKEL_RAISE1 + 1;
pub const S_SKEL_RAISE3: std::ffi::c_int = S_SKEL_RAISE2 + 1;
pub const S_SKEL_RAISE4: std::ffi::c_int = S_SKEL_RAISE3 + 1;
pub const S_SKEL_RAISE5: std::ffi::c_int = S_SKEL_RAISE4 + 1;
pub const S_SKEL_RAISE6: std::ffi::c_int = S_SKEL_RAISE5 + 1;
pub const S_FATSHOT1: std::ffi::c_int = S_SKEL_RAISE6 + 1;
pub const S_FATSHOT2: std::ffi::c_int = S_FATSHOT1 + 1;
pub const S_FATSHOTX1: std::ffi::c_int = S_FATSHOT2 + 1;
pub const S_FATSHOTX2: std::ffi::c_int = S_FATSHOTX1 + 1;
pub const S_FATSHOTX3: std::ffi::c_int = S_FATSHOTX2 + 1;
pub const S_FATT_STND: std::ffi::c_int = S_FATSHOTX3 + 1;
pub const S_FATT_STND2: std::ffi::c_int = S_FATT_STND + 1;
pub const S_FATT_RUN1: std::ffi::c_int = S_FATT_STND2 + 1;
pub const S_FATT_RUN2: std::ffi::c_int = S_FATT_RUN1 + 1;
pub const S_FATT_RUN3: std::ffi::c_int = S_FATT_RUN2 + 1;
pub const S_FATT_RUN4: std::ffi::c_int = S_FATT_RUN3 + 1;
pub const S_FATT_RUN5: std::ffi::c_int = S_FATT_RUN4 + 1;
pub const S_FATT_RUN6: std::ffi::c_int = S_FATT_RUN5 + 1;
pub const S_FATT_RUN7: std::ffi::c_int = S_FATT_RUN6 + 1;
pub const S_FATT_RUN8: std::ffi::c_int = S_FATT_RUN7 + 1;
pub const S_FATT_RUN9: std::ffi::c_int = S_FATT_RUN8 + 1;
pub const S_FATT_RUN10: std::ffi::c_int = S_FATT_RUN9 + 1;
pub const S_FATT_RUN11: std::ffi::c_int = S_FATT_RUN10 + 1;
pub const S_FATT_RUN12: std::ffi::c_int = S_FATT_RUN11 + 1;
pub const S_FATT_ATK1: std::ffi::c_int = S_FATT_RUN12 + 1;
pub const S_FATT_ATK2: std::ffi::c_int = S_FATT_ATK1 + 1;
pub const S_FATT_ATK3: std::ffi::c_int = S_FATT_ATK2 + 1;
pub const S_FATT_ATK4: std::ffi::c_int = S_FATT_ATK3 + 1;
pub const S_FATT_ATK5: std::ffi::c_int = S_FATT_ATK4 + 1;
pub const S_FATT_ATK6: std::ffi::c_int = S_FATT_ATK5 + 1;
pub const S_FATT_ATK7: std::ffi::c_int = S_FATT_ATK6 + 1;
pub const S_FATT_ATK8: std::ffi::c_int = S_FATT_ATK7 + 1;
pub const S_FATT_ATK9: std::ffi::c_int = S_FATT_ATK8 + 1;
pub const S_FATT_ATK10: std::ffi::c_int = S_FATT_ATK9 + 1;
pub const S_FATT_PAIN: std::ffi::c_int = S_FATT_ATK10 + 1;
pub const S_FATT_PAIN2: std::ffi::c_int = S_FATT_PAIN + 1;
pub const S_FATT_DIE1: std::ffi::c_int = S_FATT_PAIN2 + 1;
pub const S_FATT_DIE2: std::ffi::c_int = S_FATT_DIE1 + 1;
pub const S_FATT_DIE3: std::ffi::c_int = S_FATT_DIE2 + 1;
pub const S_FATT_DIE4: std::ffi::c_int = S_FATT_DIE3 + 1;
pub const S_FATT_DIE5: std::ffi::c_int = S_FATT_DIE4 + 1;
pub const S_FATT_DIE6: std::ffi::c_int = S_FATT_DIE5 + 1;
pub const S_FATT_DIE7: std::ffi::c_int = S_FATT_DIE6 + 1;
pub const S_FATT_DIE8: std::ffi::c_int = S_FATT_DIE7 + 1;
pub const S_FATT_DIE9: std::ffi::c_int = S_FATT_DIE8 + 1;
pub const S_FATT_DIE10: std::ffi::c_int = S_FATT_DIE9 + 1;
pub const S_FATT_RAISE1: std::ffi::c_int = S_FATT_DIE10 + 1;
pub const S_FATT_RAISE2: std::ffi::c_int = S_FATT_RAISE1 + 1;
pub const S_FATT_RAISE3: std::ffi::c_int = S_FATT_RAISE2 + 1;
pub const S_FATT_RAISE4: std::ffi::c_int = S_FATT_RAISE3 + 1;
pub const S_FATT_RAISE5: std::ffi::c_int = S_FATT_RAISE4 + 1;
pub const S_FATT_RAISE6: std::ffi::c_int = S_FATT_RAISE5 + 1;
pub const S_FATT_RAISE7: std::ffi::c_int = S_FATT_RAISE6 + 1;
pub const S_FATT_RAISE8: std::ffi::c_int = S_FATT_RAISE7 + 1;
pub const S_CPOS_STND: std::ffi::c_int = S_FATT_RAISE8 + 1;
pub const S_CPOS_STND2: std::ffi::c_int = S_CPOS_STND + 1;
pub const S_CPOS_RUN1: std::ffi::c_int = S_CPOS_STND2 + 1;
pub const S_CPOS_RUN2: std::ffi::c_int = S_CPOS_RUN1 + 1;
pub const S_CPOS_RUN3: std::ffi::c_int = S_CPOS_RUN2 + 1;
pub const S_CPOS_RUN4: std::ffi::c_int = S_CPOS_RUN3 + 1;
pub const S_CPOS_RUN5: std::ffi::c_int = S_CPOS_RUN4 + 1;
pub const S_CPOS_RUN6: std::ffi::c_int = S_CPOS_RUN5 + 1;
pub const S_CPOS_RUN7: std::ffi::c_int = S_CPOS_RUN6 + 1;
pub const S_CPOS_RUN8: std::ffi::c_int = S_CPOS_RUN7 + 1;
pub const S_CPOS_ATK1: std::ffi::c_int = S_CPOS_RUN8 + 1;
pub const S_CPOS_ATK2: std::ffi::c_int = S_CPOS_ATK1 + 1;
pub const S_CPOS_ATK3: std::ffi::c_int = S_CPOS_ATK2 + 1;
pub const S_CPOS_ATK4: std::ffi::c_int = S_CPOS_ATK3 + 1;
pub const S_CPOS_PAIN: std::ffi::c_int = S_CPOS_ATK4 + 1;
pub const S_CPOS_PAIN2: std::ffi::c_int = S_CPOS_PAIN + 1;
pub const S_CPOS_DIE1: std::ffi::c_int = S_CPOS_PAIN2 + 1;
pub const S_CPOS_DIE2: std::ffi::c_int = S_CPOS_DIE1 + 1;
pub const S_CPOS_DIE3: std::ffi::c_int = S_CPOS_DIE2 + 1;
pub const S_CPOS_DIE4: std::ffi::c_int = S_CPOS_DIE3 + 1;
pub const S_CPOS_DIE5: std::ffi::c_int = S_CPOS_DIE4 + 1;
pub const S_CPOS_DIE6: std::ffi::c_int = S_CPOS_DIE5 + 1;
pub const S_CPOS_DIE7: std::ffi::c_int = S_CPOS_DIE6 + 1;
pub const S_CPOS_XDIE1: std::ffi::c_int = S_CPOS_DIE7 + 1;
pub const S_CPOS_XDIE2: std::ffi::c_int = S_CPOS_XDIE1 + 1;
pub const S_CPOS_XDIE3: std::ffi::c_int = S_CPOS_XDIE2 + 1;
pub const S_CPOS_XDIE4: std::ffi::c_int = S_CPOS_XDIE3 + 1;
pub const S_CPOS_XDIE5: std::ffi::c_int = S_CPOS_XDIE4 + 1;
pub const S_CPOS_XDIE6: std::ffi::c_int = S_CPOS_XDIE5 + 1;
pub const S_CPOS_RAISE1: std::ffi::c_int = S_CPOS_XDIE6 + 1;
pub const S_CPOS_RAISE2: std::ffi::c_int = S_CPOS_RAISE1 + 1;
pub const S_CPOS_RAISE3: std::ffi::c_int = S_CPOS_RAISE2 + 1;
pub const S_CPOS_RAISE4: std::ffi::c_int = S_CPOS_RAISE3 + 1;
pub const S_CPOS_RAISE5: std::ffi::c_int = S_CPOS_RAISE4 + 1;
pub const S_CPOS_RAISE6: std::ffi::c_int = S_CPOS_RAISE5 + 1;
pub const S_CPOS_RAISE7: std::ffi::c_int = S_CPOS_RAISE6 + 1;
pub const S_TROO_STND: std::ffi::c_int = S_CPOS_RAISE7 + 1;
pub const S_TROO_STND2: std::ffi::c_int = S_TROO_STND + 1;
pub const S_TROO_RUN1: std::ffi::c_int = S_TROO_STND2 + 1;
pub const S_TROO_RUN2: std::ffi::c_int = S_TROO_RUN1 + 1;
pub const S_TROO_RUN3: std::ffi::c_int = S_TROO_RUN2 + 1;
pub const S_TROO_RUN4: std::ffi::c_int = S_TROO_RUN3 + 1;
pub const S_TROO_RUN5: std::ffi::c_int = S_TROO_RUN4 + 1;
pub const S_TROO_RUN6: std::ffi::c_int = S_TROO_RUN5 + 1;
pub const S_TROO_RUN7: std::ffi::c_int = S_TROO_RUN6 + 1;
pub const S_TROO_RUN8: std::ffi::c_int = S_TROO_RUN7 + 1;
pub const S_TROO_ATK1: std::ffi::c_int = S_TROO_RUN8 + 1;
pub const S_TROO_ATK2: std::ffi::c_int = S_TROO_ATK1 + 1;
pub const S_TROO_ATK3: std::ffi::c_int = S_TROO_ATK2 + 1;
pub const S_TROO_PAIN: std::ffi::c_int = S_TROO_ATK3 + 1;
pub const S_TROO_PAIN2: std::ffi::c_int = S_TROO_PAIN + 1;
pub const S_TROO_DIE1: std::ffi::c_int = S_TROO_PAIN2 + 1;
pub const S_TROO_DIE2: std::ffi::c_int = S_TROO_DIE1 + 1;
pub const S_TROO_DIE3: std::ffi::c_int = S_TROO_DIE2 + 1;
pub const S_TROO_DIE4: std::ffi::c_int = S_TROO_DIE3 + 1;
pub const S_TROO_DIE5: std::ffi::c_int = S_TROO_DIE4 + 1;
pub const S_TROO_XDIE1: std::ffi::c_int = S_TROO_DIE5 + 1;
pub const S_TROO_XDIE2: std::ffi::c_int = S_TROO_XDIE1 + 1;
pub const S_TROO_XDIE3: std::ffi::c_int = S_TROO_XDIE2 + 1;
pub const S_TROO_XDIE4: std::ffi::c_int = S_TROO_XDIE3 + 1;
pub const S_TROO_XDIE5: std::ffi::c_int = S_TROO_XDIE4 + 1;
pub const S_TROO_XDIE6: std::ffi::c_int = S_TROO_XDIE5 + 1;
pub const S_TROO_XDIE7: std::ffi::c_int = S_TROO_XDIE6 + 1;
pub const S_TROO_XDIE8: std::ffi::c_int = S_TROO_XDIE7 + 1;
pub const S_TROO_RAISE1: std::ffi::c_int = S_TROO_XDIE8 + 1;
pub const S_TROO_RAISE2: std::ffi::c_int = S_TROO_RAISE1 + 1;
pub const S_TROO_RAISE3: std::ffi::c_int = S_TROO_RAISE2 + 1;
pub const S_TROO_RAISE4: std::ffi::c_int = S_TROO_RAISE3 + 1;
pub const S_TROO_RAISE5: std::ffi::c_int = S_TROO_RAISE4 + 1;
pub const S_SARG_STND: std::ffi::c_int = S_TROO_RAISE5 + 1;
pub const S_SARG_STND2: std::ffi::c_int = S_SARG_STND + 1;
pub const S_SARG_RUN1: std::ffi::c_int = S_SARG_STND2 + 1;
pub const S_SARG_RUN2: std::ffi::c_int = S_SARG_RUN1 + 1;
pub const S_SARG_RUN3: std::ffi::c_int = S_SARG_RUN2 + 1;
pub const S_SARG_RUN4: std::ffi::c_int = S_SARG_RUN3 + 1;
pub const S_SARG_RUN5: std::ffi::c_int = S_SARG_RUN4 + 1;
pub const S_SARG_RUN6: std::ffi::c_int = S_SARG_RUN5 + 1;
pub const S_SARG_RUN7: std::ffi::c_int = S_SARG_RUN6 + 1;
pub const S_SARG_RUN8: std::ffi::c_int = S_SARG_RUN7 + 1;
pub const S_SARG_ATK1: std::ffi::c_int = S_SARG_RUN8 + 1;
pub const S_SARG_ATK2: std::ffi::c_int = S_SARG_ATK1 + 1;
pub const S_SARG_ATK3: std::ffi::c_int = S_SARG_ATK2 + 1;
pub const S_SARG_PAIN: std::ffi::c_int = S_SARG_ATK3 + 1;
pub const S_SARG_PAIN2: std::ffi::c_int = S_SARG_PAIN + 1;
pub const S_SARG_DIE1: std::ffi::c_int = S_SARG_PAIN2 + 1;
pub const S_SARG_DIE2: std::ffi::c_int = S_SARG_DIE1 + 1;
pub const S_SARG_DIE3: std::ffi::c_int = S_SARG_DIE2 + 1;
pub const S_SARG_DIE4: std::ffi::c_int = S_SARG_DIE3 + 1;
pub const S_SARG_DIE5: std::ffi::c_int = S_SARG_DIE4 + 1;
pub const S_SARG_DIE6: std::ffi::c_int = S_SARG_DIE5 + 1;
pub const S_SARG_RAISE1: std::ffi::c_int = S_SARG_DIE6 + 1;
pub const S_SARG_RAISE2: std::ffi::c_int = S_SARG_RAISE1 + 1;
pub const S_SARG_RAISE3: std::ffi::c_int = S_SARG_RAISE2 + 1;
pub const S_SARG_RAISE4: std::ffi::c_int = S_SARG_RAISE3 + 1;
pub const S_SARG_RAISE5: std::ffi::c_int = S_SARG_RAISE4 + 1;
pub const S_SARG_RAISE6: std::ffi::c_int = S_SARG_RAISE5 + 1;
pub const S_HEAD_STND: std::ffi::c_int = S_SARG_RAISE6 + 1;
pub const S_HEAD_RUN1: std::ffi::c_int = S_HEAD_STND + 1;
pub const S_HEAD_ATK1: std::ffi::c_int = S_HEAD_RUN1 + 1;
pub const S_HEAD_ATK2: std::ffi::c_int = S_HEAD_ATK1 + 1;
pub const S_HEAD_ATK3: std::ffi::c_int = S_HEAD_ATK2 + 1;
pub const S_HEAD_PAIN: std::ffi::c_int = S_HEAD_ATK3 + 1;
pub const S_HEAD_PAIN2: std::ffi::c_int = S_HEAD_PAIN + 1;
pub const S_HEAD_PAIN3: std::ffi::c_int = S_HEAD_PAIN2 + 1;
pub const S_HEAD_DIE1: std::ffi::c_int = S_HEAD_PAIN3 + 1;
pub const S_HEAD_DIE2: std::ffi::c_int = S_HEAD_DIE1 + 1;
pub const S_HEAD_DIE3: std::ffi::c_int = S_HEAD_DIE2 + 1;
pub const S_HEAD_DIE4: std::ffi::c_int = S_HEAD_DIE3 + 1;
pub const S_HEAD_DIE5: std::ffi::c_int = S_HEAD_DIE4 + 1;
pub const S_HEAD_DIE6: std::ffi::c_int = S_HEAD_DIE5 + 1;
pub const S_HEAD_RAISE1: std::ffi::c_int = S_HEAD_DIE6 + 1;
pub const S_HEAD_RAISE2: std::ffi::c_int = S_HEAD_RAISE1 + 1;
pub const S_HEAD_RAISE3: std::ffi::c_int = S_HEAD_RAISE2 + 1;
pub const S_HEAD_RAISE4: std::ffi::c_int = S_HEAD_RAISE3 + 1;
pub const S_HEAD_RAISE5: std::ffi::c_int = S_HEAD_RAISE4 + 1;
pub const S_HEAD_RAISE6: std::ffi::c_int = S_HEAD_RAISE5 + 1;
pub const S_BRBALL1: std::ffi::c_int = S_HEAD_RAISE6 + 1;
pub const S_BRBALL2: std::ffi::c_int = S_BRBALL1 + 1;
pub const S_BRBALLX1: std::ffi::c_int = S_BRBALL2 + 1;
pub const S_BRBALLX2: std::ffi::c_int = S_BRBALLX1 + 1;
pub const S_BRBALLX3: std::ffi::c_int = S_BRBALLX2 + 1;
pub const S_BOSS_STND: std::ffi::c_int = S_BRBALLX3 + 1;
pub const S_BOSS_STND2: std::ffi::c_int = S_BOSS_STND + 1;
pub const S_BOSS_RUN1: std::ffi::c_int = S_BOSS_STND2 + 1;
pub const S_BOSS_RUN2: std::ffi::c_int = S_BOSS_RUN1 + 1;
pub const S_BOSS_RUN3: std::ffi::c_int = S_BOSS_RUN2 + 1;
pub const S_BOSS_RUN4: std::ffi::c_int = S_BOSS_RUN3 + 1;
pub const S_BOSS_RUN5: std::ffi::c_int = S_BOSS_RUN4 + 1;
pub const S_BOSS_RUN6: std::ffi::c_int = S_BOSS_RUN5 + 1;
pub const S_BOSS_RUN7: std::ffi::c_int = S_BOSS_RUN6 + 1;
pub const S_BOSS_RUN8: std::ffi::c_int = S_BOSS_RUN7 + 1;
pub const S_BOSS_ATK1: std::ffi::c_int = S_BOSS_RUN8 + 1;
pub const S_BOSS_ATK2: std::ffi::c_int = S_BOSS_ATK1 + 1;
pub const S_BOSS_ATK3: std::ffi::c_int = S_BOSS_ATK2 + 1;
pub const S_BOSS_PAIN: std::ffi::c_int = S_BOSS_ATK3 + 1;
pub const S_BOSS_PAIN2: std::ffi::c_int = S_BOSS_PAIN + 1;
pub const S_BOSS_DIE1: std::ffi::c_int = S_BOSS_PAIN2 + 1;
pub const S_BOSS_DIE2: std::ffi::c_int = S_BOSS_DIE1 + 1;
pub const S_BOSS_DIE3: std::ffi::c_int = S_BOSS_DIE2 + 1;
pub const S_BOSS_DIE4: std::ffi::c_int = S_BOSS_DIE3 + 1;
pub const S_BOSS_DIE5: std::ffi::c_int = S_BOSS_DIE4 + 1;
pub const S_BOSS_DIE6: std::ffi::c_int = S_BOSS_DIE5 + 1;
pub const S_BOSS_DIE7: std::ffi::c_int = S_BOSS_DIE6 + 1;
pub const S_BOSS_RAISE1: std::ffi::c_int = S_BOSS_DIE7 + 1;
pub const S_BOSS_RAISE2: std::ffi::c_int = S_BOSS_RAISE1 + 1;
pub const S_BOSS_RAISE3: std::ffi::c_int = S_BOSS_RAISE2 + 1;
pub const S_BOSS_RAISE4: std::ffi::c_int = S_BOSS_RAISE3 + 1;
pub const S_BOSS_RAISE5: std::ffi::c_int = S_BOSS_RAISE4 + 1;
pub const S_BOSS_RAISE6: std::ffi::c_int = S_BOSS_RAISE5 + 1;
pub const S_BOSS_RAISE7: std::ffi::c_int = S_BOSS_RAISE6 + 1;
pub const S_BOS2_STND: std::ffi::c_int = S_BOSS_RAISE7 + 1;
pub const S_BOS2_STND2: std::ffi::c_int = S_BOS2_STND + 1;
pub const S_BOS2_RUN1: std::ffi::c_int = S_BOS2_STND2 + 1;
pub const S_BOS2_RUN2: std::ffi::c_int = S_BOS2_RUN1 + 1;
pub const S_BOS2_RUN3: std::ffi::c_int = S_BOS2_RUN2 + 1;
pub const S_BOS2_RUN4: std::ffi::c_int = S_BOS2_RUN3 + 1;
pub const S_BOS2_RUN5: std::ffi::c_int = S_BOS2_RUN4 + 1;
pub const S_BOS2_RUN6: std::ffi::c_int = S_BOS2_RUN5 + 1;
pub const S_BOS2_RUN7: std::ffi::c_int = S_BOS2_RUN6 + 1;
pub const S_BOS2_RUN8: std::ffi::c_int = S_BOS2_RUN7 + 1;
pub const S_BOS2_ATK1: std::ffi::c_int = S_BOS2_RUN8 + 1;
pub const S_BOS2_ATK2: std::ffi::c_int = S_BOS2_ATK1 + 1;
pub const S_BOS2_ATK3: std::ffi::c_int = S_BOS2_ATK2 + 1;
pub const S_BOS2_PAIN: std::ffi::c_int = S_BOS2_ATK3 + 1;
pub const S_BOS2_PAIN2: std::ffi::c_int = S_BOS2_PAIN + 1;
pub const S_BOS2_DIE1: std::ffi::c_int = S_BOS2_PAIN2 + 1;
pub const S_BOS2_DIE2: std::ffi::c_int = S_BOS2_DIE1 + 1;
pub const S_BOS2_DIE3: std::ffi::c_int = S_BOS2_DIE2 + 1;
pub const S_BOS2_DIE4: std::ffi::c_int = S_BOS2_DIE3 + 1;
pub const S_BOS2_DIE5: std::ffi::c_int = S_BOS2_DIE4 + 1;
pub const S_BOS2_DIE6: std::ffi::c_int = S_BOS2_DIE5 + 1;
pub const S_BOS2_DIE7: std::ffi::c_int = S_BOS2_DIE6 + 1;
pub const S_BOS2_RAISE1: std::ffi::c_int = S_BOS2_DIE7 + 1;
pub const S_BOS2_RAISE2: std::ffi::c_int = S_BOS2_RAISE1 + 1;
pub const S_BOS2_RAISE3: std::ffi::c_int = S_BOS2_RAISE2 + 1;
pub const S_BOS2_RAISE4: std::ffi::c_int = S_BOS2_RAISE3 + 1;
pub const S_BOS2_RAISE5: std::ffi::c_int = S_BOS2_RAISE4 + 1;
pub const S_BOS2_RAISE6: std::ffi::c_int = S_BOS2_RAISE5 + 1;
pub const S_BOS2_RAISE7: std::ffi::c_int = S_BOS2_RAISE6 + 1;
pub const S_SKULL_STND: std::ffi::c_int = S_BOS2_RAISE7 + 1;
pub const S_SKULL_STND2: std::ffi::c_int = S_SKULL_STND + 1;
pub const S_SKULL_RUN1: std::ffi::c_int = S_SKULL_STND2 + 1;
pub const S_SKULL_RUN2: std::ffi::c_int = S_SKULL_RUN1 + 1;
pub const S_SKULL_ATK1: std::ffi::c_int = S_SKULL_RUN2 + 1;
pub const S_SKULL_ATK2: std::ffi::c_int = S_SKULL_ATK1 + 1;
pub const S_SKULL_ATK3: std::ffi::c_int = S_SKULL_ATK2 + 1;
pub const S_SKULL_ATK4: std::ffi::c_int = S_SKULL_ATK3 + 1;
pub const S_SKULL_PAIN: std::ffi::c_int = S_SKULL_ATK4 + 1;
pub const S_SKULL_PAIN2: std::ffi::c_int = S_SKULL_PAIN + 1;
pub const S_SKULL_DIE1: std::ffi::c_int = S_SKULL_PAIN2 + 1;
pub const S_SKULL_DIE2: std::ffi::c_int = S_SKULL_DIE1 + 1;
pub const S_SKULL_DIE3: std::ffi::c_int = S_SKULL_DIE2 + 1;
pub const S_SKULL_DIE4: std::ffi::c_int = S_SKULL_DIE3 + 1;
pub const S_SKULL_DIE5: std::ffi::c_int = S_SKULL_DIE4 + 1;
pub const S_SKULL_DIE6: std::ffi::c_int = S_SKULL_DIE5 + 1;
pub const S_SPID_STND: std::ffi::c_int = S_SKULL_DIE6 + 1;
pub const S_SPID_STND2: std::ffi::c_int = S_SPID_STND + 1;
pub const S_SPID_RUN1: std::ffi::c_int = S_SPID_STND2 + 1;
pub const S_SPID_RUN2: std::ffi::c_int = S_SPID_RUN1 + 1;
pub const S_SPID_RUN3: std::ffi::c_int = S_SPID_RUN2 + 1;
pub const S_SPID_RUN4: std::ffi::c_int = S_SPID_RUN3 + 1;
pub const S_SPID_RUN5: std::ffi::c_int = S_SPID_RUN4 + 1;
pub const S_SPID_RUN6: std::ffi::c_int = S_SPID_RUN5 + 1;
pub const S_SPID_RUN7: std::ffi::c_int = S_SPID_RUN6 + 1;
pub const S_SPID_RUN8: std::ffi::c_int = S_SPID_RUN7 + 1;
pub const S_SPID_RUN9: std::ffi::c_int = S_SPID_RUN8 + 1;
pub const S_SPID_RUN10: std::ffi::c_int = S_SPID_RUN9 + 1;
pub const S_SPID_RUN11: std::ffi::c_int = S_SPID_RUN10 + 1;
pub const S_SPID_RUN12: std::ffi::c_int = S_SPID_RUN11 + 1;
pub const S_SPID_ATK1: std::ffi::c_int = S_SPID_RUN12 + 1;
pub const S_SPID_ATK2: std::ffi::c_int = S_SPID_ATK1 + 1;
pub const S_SPID_ATK3: std::ffi::c_int = S_SPID_ATK2 + 1;
pub const S_SPID_ATK4: std::ffi::c_int = S_SPID_ATK3 + 1;
pub const S_SPID_PAIN: std::ffi::c_int = S_SPID_ATK4 + 1;
pub const S_SPID_PAIN2: std::ffi::c_int = S_SPID_PAIN + 1;
pub const S_SPID_DIE1: std::ffi::c_int = S_SPID_PAIN2 + 1;
pub const S_SPID_DIE2: std::ffi::c_int = S_SPID_DIE1 + 1;
pub const S_SPID_DIE3: std::ffi::c_int = S_SPID_DIE2 + 1;
pub const S_SPID_DIE4: std::ffi::c_int = S_SPID_DIE3 + 1;
pub const S_SPID_DIE5: std::ffi::c_int = S_SPID_DIE4 + 1;
pub const S_SPID_DIE6: std::ffi::c_int = S_SPID_DIE5 + 1;
pub const S_SPID_DIE7: std::ffi::c_int = S_SPID_DIE6 + 1;
pub const S_SPID_DIE8: std::ffi::c_int = S_SPID_DIE7 + 1;
pub const S_SPID_DIE9: std::ffi::c_int = S_SPID_DIE8 + 1;
pub const S_SPID_DIE10: std::ffi::c_int = S_SPID_DIE9 + 1;
pub const S_SPID_DIE11: std::ffi::c_int = S_SPID_DIE10 + 1;
pub const S_BSPI_STND: std::ffi::c_int = S_SPID_DIE11 + 1;
pub const S_BSPI_STND2: std::ffi::c_int = S_BSPI_STND + 1;
pub const S_BSPI_SIGHT: std::ffi::c_int = S_BSPI_STND2 + 1;
pub const S_BSPI_RUN1: std::ffi::c_int = S_BSPI_SIGHT + 1;
pub const S_BSPI_RUN2: std::ffi::c_int = S_BSPI_RUN1 + 1;
pub const S_BSPI_RUN3: std::ffi::c_int = S_BSPI_RUN2 + 1;
pub const S_BSPI_RUN4: std::ffi::c_int = S_BSPI_RUN3 + 1;
pub const S_BSPI_RUN5: std::ffi::c_int = S_BSPI_RUN4 + 1;
pub const S_BSPI_RUN6: std::ffi::c_int = S_BSPI_RUN5 + 1;
pub const S_BSPI_RUN7: std::ffi::c_int = S_BSPI_RUN6 + 1;
pub const S_BSPI_RUN8: std::ffi::c_int = S_BSPI_RUN7 + 1;
pub const S_BSPI_RUN9: std::ffi::c_int = S_BSPI_RUN8 + 1;
pub const S_BSPI_RUN10: std::ffi::c_int = S_BSPI_RUN9 + 1;
pub const S_BSPI_RUN11: std::ffi::c_int = S_BSPI_RUN10 + 1;
pub const S_BSPI_RUN12: std::ffi::c_int = S_BSPI_RUN11 + 1;
pub const S_BSPI_ATK1: std::ffi::c_int = S_BSPI_RUN12 + 1;
pub const S_BSPI_ATK2: std::ffi::c_int = S_BSPI_ATK1 + 1;
pub const S_BSPI_ATK3: std::ffi::c_int = S_BSPI_ATK2 + 1;
pub const S_BSPI_ATK4: std::ffi::c_int = S_BSPI_ATK3 + 1;
pub const S_BSPI_PAIN: std::ffi::c_int = S_BSPI_ATK4 + 1;
pub const S_BSPI_PAIN2: std::ffi::c_int = S_BSPI_PAIN + 1;
pub const S_BSPI_DIE1: std::ffi::c_int = S_BSPI_PAIN2 + 1;
pub const S_BSPI_DIE2: std::ffi::c_int = S_BSPI_DIE1 + 1;
pub const S_BSPI_DIE3: std::ffi::c_int = S_BSPI_DIE2 + 1;
pub const S_BSPI_DIE4: std::ffi::c_int = S_BSPI_DIE3 + 1;
pub const S_BSPI_DIE5: std::ffi::c_int = S_BSPI_DIE4 + 1;
pub const S_BSPI_DIE6: std::ffi::c_int = S_BSPI_DIE5 + 1;
pub const S_BSPI_DIE7: std::ffi::c_int = S_BSPI_DIE6 + 1;
pub const S_BSPI_RAISE1: std::ffi::c_int = S_BSPI_DIE7 + 1;
pub const S_BSPI_RAISE2: std::ffi::c_int = S_BSPI_RAISE1 + 1;
pub const S_BSPI_RAISE3: std::ffi::c_int = S_BSPI_RAISE2 + 1;
pub const S_BSPI_RAISE4: std::ffi::c_int = S_BSPI_RAISE3 + 1;
pub const S_BSPI_RAISE5: std::ffi::c_int = S_BSPI_RAISE4 + 1;
pub const S_BSPI_RAISE6: std::ffi::c_int = S_BSPI_RAISE5 + 1;
pub const S_BSPI_RAISE7: std::ffi::c_int = S_BSPI_RAISE6 + 1;
pub const S_ARACH_PLAZ: std::ffi::c_int = S_BSPI_RAISE7 + 1;
pub const S_ARACH_PLAZ2: std::ffi::c_int = S_ARACH_PLAZ + 1;
pub const S_ARACH_PLEX: std::ffi::c_int = S_ARACH_PLAZ2 + 1;
pub const S_ARACH_PLEX2: std::ffi::c_int = S_ARACH_PLEX + 1;
pub const S_ARACH_PLEX3: std::ffi::c_int = S_ARACH_PLEX2 + 1;
pub const S_ARACH_PLEX4: std::ffi::c_int = S_ARACH_PLEX3 + 1;
pub const S_ARACH_PLEX5: std::ffi::c_int = S_ARACH_PLEX4 + 1;
pub const S_CYBER_STND: std::ffi::c_int = S_ARACH_PLEX5 + 1;
pub const S_CYBER_STND2: std::ffi::c_int = S_CYBER_STND + 1;
pub const S_CYBER_RUN1: std::ffi::c_int = S_CYBER_STND2 + 1;
pub const S_CYBER_RUN2: std::ffi::c_int = S_CYBER_RUN1 + 1;
pub const S_CYBER_RUN3: std::ffi::c_int = S_CYBER_RUN2 + 1;
pub const S_CYBER_RUN4: std::ffi::c_int = S_CYBER_RUN3 + 1;
pub const S_CYBER_RUN5: std::ffi::c_int = S_CYBER_RUN4 + 1;
pub const S_CYBER_RUN6: std::ffi::c_int = S_CYBER_RUN5 + 1;
pub const S_CYBER_RUN7: std::ffi::c_int = S_CYBER_RUN6 + 1;
pub const S_CYBER_RUN8: std::ffi::c_int = S_CYBER_RUN7 + 1;
pub const S_CYBER_ATK1: std::ffi::c_int = S_CYBER_RUN8 + 1;
pub const S_CYBER_ATK2: std::ffi::c_int = S_CYBER_ATK1 + 1;
pub const S_CYBER_ATK3: std::ffi::c_int = S_CYBER_ATK2 + 1;
pub const S_CYBER_ATK4: std::ffi::c_int = S_CYBER_ATK3 + 1;
pub const S_CYBER_ATK5: std::ffi::c_int = S_CYBER_ATK4 + 1;
pub const S_CYBER_ATK6: std::ffi::c_int = S_CYBER_ATK5 + 1;
pub const S_CYBER_PAIN: std::ffi::c_int = S_CYBER_ATK6 + 1;
pub const S_CYBER_DIE1: std::ffi::c_int = S_CYBER_PAIN + 1;
pub const S_CYBER_DIE2: std::ffi::c_int = S_CYBER_DIE1 + 1;
pub const S_CYBER_DIE3: std::ffi::c_int = S_CYBER_DIE2 + 1;
pub const S_CYBER_DIE4: std::ffi::c_int = S_CYBER_DIE3 + 1;
pub const S_CYBER_DIE5: std::ffi::c_int = S_CYBER_DIE4 + 1;
pub const S_CYBER_DIE6: std::ffi::c_int = S_CYBER_DIE5 + 1;
pub const S_CYBER_DIE7: std::ffi::c_int = S_CYBER_DIE6 + 1;
pub const S_CYBER_DIE8: std::ffi::c_int = S_CYBER_DIE7 + 1;
pub const S_CYBER_DIE9: std::ffi::c_int = S_CYBER_DIE8 + 1;
pub const S_CYBER_DIE10: std::ffi::c_int = S_CYBER_DIE9 + 1;
pub const S_PAIN_STND: std::ffi::c_int = S_CYBER_DIE10 + 1;
pub const S_PAIN_RUN1: std::ffi::c_int = S_PAIN_STND + 1;
pub const S_PAIN_RUN2: std::ffi::c_int = S_PAIN_RUN1 + 1;
pub const S_PAIN_RUN3: std::ffi::c_int = S_PAIN_RUN2 + 1;
pub const S_PAIN_RUN4: std::ffi::c_int = S_PAIN_RUN3 + 1;
pub const S_PAIN_RUN5: std::ffi::c_int = S_PAIN_RUN4 + 1;
pub const S_PAIN_RUN6: std::ffi::c_int = S_PAIN_RUN5 + 1;
pub const S_PAIN_ATK1: std::ffi::c_int = S_PAIN_RUN6 + 1;
pub const S_PAIN_ATK2: std::ffi::c_int = S_PAIN_ATK1 + 1;
pub const S_PAIN_ATK3: std::ffi::c_int = S_PAIN_ATK2 + 1;
pub const S_PAIN_ATK4: std::ffi::c_int = S_PAIN_ATK3 + 1;
pub const S_PAIN_PAIN: std::ffi::c_int = S_PAIN_ATK4 + 1;
pub const S_PAIN_PAIN2: std::ffi::c_int = S_PAIN_PAIN + 1;
pub const S_PAIN_DIE1: std::ffi::c_int = S_PAIN_PAIN2 + 1;
pub const S_PAIN_DIE2: std::ffi::c_int = S_PAIN_DIE1 + 1;
pub const S_PAIN_DIE3: std::ffi::c_int = S_PAIN_DIE2 + 1;
pub const S_PAIN_DIE4: std::ffi::c_int = S_PAIN_DIE3 + 1;
pub const S_PAIN_DIE5: std::ffi::c_int = S_PAIN_DIE4 + 1;
pub const S_PAIN_DIE6: std::ffi::c_int = S_PAIN_DIE5 + 1;
pub const S_PAIN_RAISE1: std::ffi::c_int = S_PAIN_DIE6 + 1;
pub const S_PAIN_RAISE2: std::ffi::c_int = S_PAIN_RAISE1 + 1;
pub const S_PAIN_RAISE3: std::ffi::c_int = S_PAIN_RAISE2 + 1;
pub const S_PAIN_RAISE4: std::ffi::c_int = S_PAIN_RAISE3 + 1;
pub const S_PAIN_RAISE5: std::ffi::c_int = S_PAIN_RAISE4 + 1;
pub const S_PAIN_RAISE6: std::ffi::c_int = S_PAIN_RAISE5 + 1;
pub const S_SSWV_STND: std::ffi::c_int = S_PAIN_RAISE6 + 1;
pub const S_SSWV_STND2: std::ffi::c_int = S_SSWV_STND + 1;
pub const S_SSWV_RUN1: std::ffi::c_int = S_SSWV_STND2 + 1;
pub const S_SSWV_RUN2: std::ffi::c_int = S_SSWV_RUN1 + 1;
pub const S_SSWV_RUN3: std::ffi::c_int = S_SSWV_RUN2 + 1;
pub const S_SSWV_RUN4: std::ffi::c_int = S_SSWV_RUN3 + 1;
pub const S_SSWV_RUN5: std::ffi::c_int = S_SSWV_RUN4 + 1;
pub const S_SSWV_RUN6: std::ffi::c_int = S_SSWV_RUN5 + 1;
pub const S_SSWV_RUN7: std::ffi::c_int = S_SSWV_RUN6 + 1;
pub const S_SSWV_RUN8: std::ffi::c_int = S_SSWV_RUN7 + 1;
pub const S_SSWV_ATK1: std::ffi::c_int = S_SSWV_RUN8 + 1;
pub const S_SSWV_ATK2: std::ffi::c_int = S_SSWV_ATK1 + 1;
pub const S_SSWV_ATK3: std::ffi::c_int = S_SSWV_ATK2 + 1;
pub const S_SSWV_ATK4: std::ffi::c_int = S_SSWV_ATK3 + 1;
pub const S_SSWV_ATK5: std::ffi::c_int = S_SSWV_ATK4 + 1;
pub const S_SSWV_ATK6: std::ffi::c_int = S_SSWV_ATK5 + 1;
pub const S_SSWV_PAIN: std::ffi::c_int = S_SSWV_ATK6 + 1;
pub const S_SSWV_PAIN2: std::ffi::c_int = S_SSWV_PAIN + 1;
pub const S_SSWV_DIE1: std::ffi::c_int = S_SSWV_PAIN2 + 1;
pub const S_SSWV_DIE2: std::ffi::c_int = S_SSWV_DIE1 + 1;
pub const S_SSWV_DIE3: std::ffi::c_int = S_SSWV_DIE2 + 1;
pub const S_SSWV_DIE4: std::ffi::c_int = S_SSWV_DIE3 + 1;
pub const S_SSWV_DIE5: std::ffi::c_int = S_SSWV_DIE4 + 1;
pub const S_SSWV_XDIE1: std::ffi::c_int = S_SSWV_DIE5 + 1;
pub const S_SSWV_XDIE2: std::ffi::c_int = S_SSWV_XDIE1 + 1;
pub const S_SSWV_XDIE3: std::ffi::c_int = S_SSWV_XDIE2 + 1;
pub const S_SSWV_XDIE4: std::ffi::c_int = S_SSWV_XDIE3 + 1;
pub const S_SSWV_XDIE5: std::ffi::c_int = S_SSWV_XDIE4 + 1;
pub const S_SSWV_XDIE6: std::ffi::c_int = S_SSWV_XDIE5 + 1;
pub const S_SSWV_XDIE7: std::ffi::c_int = S_SSWV_XDIE6 + 1;
pub const S_SSWV_XDIE8: std::ffi::c_int = S_SSWV_XDIE7 + 1;
pub const S_SSWV_XDIE9: std::ffi::c_int = S_SSWV_XDIE8 + 1;
pub const S_SSWV_RAISE1: std::ffi::c_int = S_SSWV_XDIE9 + 1;
pub const S_SSWV_RAISE2: std::ffi::c_int = S_SSWV_RAISE1 + 1;
pub const S_SSWV_RAISE3: std::ffi::c_int = S_SSWV_RAISE2 + 1;
pub const S_SSWV_RAISE4: std::ffi::c_int = S_SSWV_RAISE3 + 1;
pub const S_SSWV_RAISE5: std::ffi::c_int = S_SSWV_RAISE4 + 1;
pub const S_KEENSTND: std::ffi::c_int = S_SSWV_RAISE5 + 1;
pub const S_COMMKEEN: std::ffi::c_int = S_KEENSTND + 1;
pub const S_COMMKEEN2: std::ffi::c_int = S_COMMKEEN + 1;
pub const S_COMMKEEN3: std::ffi::c_int = S_COMMKEEN2 + 1;
pub const S_COMMKEEN4: std::ffi::c_int = S_COMMKEEN3 + 1;
pub const S_COMMKEEN5: std::ffi::c_int = S_COMMKEEN4 + 1;
pub const S_COMMKEEN6: std::ffi::c_int = S_COMMKEEN5 + 1;
pub const S_COMMKEEN7: std::ffi::c_int = S_COMMKEEN6 + 1;
pub const S_COMMKEEN8: std::ffi::c_int = S_COMMKEEN7 + 1;
pub const S_COMMKEEN9: std::ffi::c_int = S_COMMKEEN8 + 1;
pub const S_COMMKEEN10: std::ffi::c_int = S_COMMKEEN9 + 1;
pub const S_COMMKEEN11: std::ffi::c_int = S_COMMKEEN10 + 1;
pub const S_COMMKEEN12: std::ffi::c_int = S_COMMKEEN11 + 1;
pub const S_KEENPAIN: std::ffi::c_int = S_COMMKEEN12 + 1;
pub const S_KEENPAIN2: std::ffi::c_int = S_KEENPAIN + 1;
pub const S_BRAIN: std::ffi::c_int = S_KEENPAIN2 + 1;
pub const S_BRAIN_PAIN: std::ffi::c_int = S_BRAIN + 1;
pub const S_BRAIN_DIE1: std::ffi::c_int = S_BRAIN_PAIN + 1;
pub const S_BRAIN_DIE2: std::ffi::c_int = S_BRAIN_DIE1 + 1;
pub const S_BRAIN_DIE3: std::ffi::c_int = S_BRAIN_DIE2 + 1;
pub const S_BRAIN_DIE4: std::ffi::c_int = S_BRAIN_DIE3 + 1;
pub const S_BRAINEYE: std::ffi::c_int = S_BRAIN_DIE4 + 1;
pub const S_BRAINEYESEE: std::ffi::c_int = S_BRAINEYE + 1;
pub const S_BRAINEYE1: std::ffi::c_int = S_BRAINEYESEE + 1;
pub const S_SPAWN1: std::ffi::c_int = S_BRAINEYE1 + 1;
pub const S_SPAWN2: std::ffi::c_int = S_SPAWN1 + 1;
pub const S_SPAWN3: std::ffi::c_int = S_SPAWN2 + 1;
pub const S_SPAWN4: std::ffi::c_int = S_SPAWN3 + 1;
pub const S_SPAWNFIRE1: std::ffi::c_int = S_SPAWN4 + 1;
pub const S_SPAWNFIRE2: std::ffi::c_int = S_SPAWNFIRE1 + 1;
pub const S_SPAWNFIRE3: std::ffi::c_int = S_SPAWNFIRE2 + 1;
pub const S_SPAWNFIRE4: std::ffi::c_int = S_SPAWNFIRE3 + 1;
pub const S_SPAWNFIRE5: std::ffi::c_int = S_SPAWNFIRE4 + 1;
pub const S_SPAWNFIRE6: std::ffi::c_int = S_SPAWNFIRE5 + 1;
pub const S_SPAWNFIRE7: std::ffi::c_int = S_SPAWNFIRE6 + 1;
pub const S_SPAWNFIRE8: std::ffi::c_int = S_SPAWNFIRE7 + 1;
pub const S_BRAINEXPLODE1: std::ffi::c_int = S_SPAWNFIRE8 + 1;
pub const S_BRAINEXPLODE2: std::ffi::c_int = S_BRAINEXPLODE1 + 1;
pub const S_BRAINEXPLODE3: std::ffi::c_int = S_BRAINEXPLODE2 + 1;
pub const S_ARM1: std::ffi::c_int = S_BRAINEXPLODE3 + 1;
pub const S_ARM1A: std::ffi::c_int = S_ARM1 + 1;
pub const S_ARM2: std::ffi::c_int = S_ARM1A + 1;
pub const S_ARM2A: std::ffi::c_int = S_ARM2 + 1;
pub const S_BAR1: std::ffi::c_int = S_ARM2A + 1;
pub const S_BAR2: std::ffi::c_int = S_BAR1 + 1;
pub const S_BEXP: std::ffi::c_int = S_BAR2 + 1;
pub const S_BEXP2: std::ffi::c_int = S_BEXP + 1;
pub const S_BEXP3: std::ffi::c_int = S_BEXP2 + 1;
pub const S_BEXP4: std::ffi::c_int = S_BEXP3 + 1;
pub const S_BEXP5: std::ffi::c_int = S_BEXP4 + 1;
pub const S_BBAR1: std::ffi::c_int = S_BEXP5 + 1;
pub const S_BBAR2: std::ffi::c_int = S_BBAR1 + 1;
pub const S_BBAR3: std::ffi::c_int = S_BBAR2 + 1;
pub const S_BON1: std::ffi::c_int = S_BBAR3 + 1;
pub const S_BON1A: std::ffi::c_int = S_BON1 + 1;
pub const S_BON1B: std::ffi::c_int = S_BON1A + 1;
pub const S_BON1C: std::ffi::c_int = S_BON1B + 1;
pub const S_BON1D: std::ffi::c_int = S_BON1C + 1;
pub const S_BON1E: std::ffi::c_int = S_BON1D + 1;
pub const S_BON2: std::ffi::c_int = S_BON1E + 1;
pub const S_BON2A: std::ffi::c_int = S_BON2 + 1;
pub const S_BON2B: std::ffi::c_int = S_BON2A + 1;
pub const S_BON2C: std::ffi::c_int = S_BON2B + 1;
pub const S_BON2D: std::ffi::c_int = S_BON2C + 1;
pub const S_BON2E: std::ffi::c_int = S_BON2D + 1;
pub const S_BKEY: std::ffi::c_int = S_BON2E + 1;
pub const S_BKEY2: std::ffi::c_int = S_BKEY + 1;
pub const S_RKEY: std::ffi::c_int = S_BKEY2 + 1;
pub const S_RKEY2: std::ffi::c_int = S_RKEY + 1;
pub const S_YKEY: std::ffi::c_int = S_RKEY2 + 1;
pub const S_YKEY2: std::ffi::c_int = S_YKEY + 1;
pub const S_BSKULL: std::ffi::c_int = S_YKEY2 + 1;
pub const S_BSKULL2: std::ffi::c_int = S_BSKULL + 1;
pub const S_RSKULL: std::ffi::c_int = S_BSKULL2 + 1;
pub const S_RSKULL2: std::ffi::c_int = S_RSKULL + 1;
pub const S_YSKULL: std::ffi::c_int = S_RSKULL2 + 1;
pub const S_YSKULL2: std::ffi::c_int = S_YSKULL + 1;
pub const S_STIM: std::ffi::c_int = S_YSKULL2 + 1;
pub const S_MEDI: std::ffi::c_int = S_STIM + 1;
pub const S_SOUL: std::ffi::c_int = S_MEDI + 1;
pub const S_SOUL2: std::ffi::c_int = S_SOUL + 1;
pub const S_SOUL3: std::ffi::c_int = S_SOUL2 + 1;
pub const S_SOUL4: std::ffi::c_int = S_SOUL3 + 1;
pub const S_SOUL5: std::ffi::c_int = S_SOUL4 + 1;
pub const S_SOUL6: std::ffi::c_int = S_SOUL5 + 1;
pub const S_PINV: std::ffi::c_int = S_SOUL6 + 1;
pub const S_PINV2: std::ffi::c_int = S_PINV + 1;
pub const S_PINV3: std::ffi::c_int = S_PINV2 + 1;
pub const S_PINV4: std::ffi::c_int = S_PINV3 + 1;
pub const S_PSTR: std::ffi::c_int = S_PINV4 + 1;
pub const S_PINS: std::ffi::c_int = S_PSTR + 1;
pub const S_PINS2: std::ffi::c_int = S_PINS + 1;
pub const S_PINS3: std::ffi::c_int = S_PINS2 + 1;
pub const S_PINS4: std::ffi::c_int = S_PINS3 + 1;
pub const S_MEGA: std::ffi::c_int = S_PINS4 + 1;
pub const S_MEGA2: std::ffi::c_int = S_MEGA + 1;
pub const S_MEGA3: std::ffi::c_int = S_MEGA2 + 1;
pub const S_MEGA4: std::ffi::c_int = S_MEGA3 + 1;
pub const S_SUIT: std::ffi::c_int = S_MEGA4 + 1;
pub const S_PMAP: std::ffi::c_int = S_SUIT + 1;
pub const S_PMAP2: std::ffi::c_int = S_PMAP + 1;
pub const S_PMAP3: std::ffi::c_int = S_PMAP2 + 1;
pub const S_PMAP4: std::ffi::c_int = S_PMAP3 + 1;
pub const S_PMAP5: std::ffi::c_int = S_PMAP4 + 1;
pub const S_PMAP6: std::ffi::c_int = S_PMAP5 + 1;
pub const S_PVIS: std::ffi::c_int = S_PMAP6 + 1;
pub const S_PVIS2: std::ffi::c_int = S_PVIS + 1;
pub const S_CLIP: std::ffi::c_int = S_PVIS2 + 1;
pub const S_AMMO: std::ffi::c_int = S_CLIP + 1;
pub const S_ROCK: std::ffi::c_int = S_AMMO + 1;
pub const S_BROK: std::ffi::c_int = S_ROCK + 1;
pub const S_CELL: std::ffi::c_int = S_BROK + 1;
pub const S_CELP: std::ffi::c_int = S_CELL + 1;
pub const S_SHEL: std::ffi::c_int = S_CELP + 1;
pub const S_SBOX: std::ffi::c_int = S_SHEL + 1;
pub const S_BPAK: std::ffi::c_int = S_SBOX + 1;
pub const S_BFUG: std::ffi::c_int = S_BPAK + 1;
pub const S_MGUN: std::ffi::c_int = S_BFUG + 1;
pub const S_CSAW: std::ffi::c_int = S_MGUN + 1;
pub const S_LAUN: std::ffi::c_int = S_CSAW + 1;
pub const S_PLAS: std::ffi::c_int = S_LAUN + 1;
pub const S_SHOT: std::ffi::c_int = S_PLAS + 1;
pub const S_SHOT2: std::ffi::c_int = S_SHOT + 1;
pub const S_COLU: std::ffi::c_int = S_SHOT2 + 1;
pub const S_STALAG: std::ffi::c_int = S_COLU + 1;
pub const S_BLOODYTWITCH: std::ffi::c_int = S_STALAG + 1;
pub const S_BLOODYTWITCH2: std::ffi::c_int = S_BLOODYTWITCH + 1;
pub const S_BLOODYTWITCH3: std::ffi::c_int = S_BLOODYTWITCH2 + 1;
pub const S_BLOODYTWITCH4: std::ffi::c_int = S_BLOODYTWITCH3 + 1;
pub const S_DEADTORSO: std::ffi::c_int = S_BLOODYTWITCH4 + 1;
pub const S_DEADBOTTOM: std::ffi::c_int = S_DEADTORSO + 1;
pub const S_HEADSONSTICK: std::ffi::c_int = S_DEADBOTTOM + 1;
pub const S_GIBS: std::ffi::c_int = S_HEADSONSTICK + 1;
pub const S_HEADONASTICK: std::ffi::c_int = S_GIBS + 1;
pub const S_HEADCANDLES: std::ffi::c_int = S_HEADONASTICK + 1;
pub const S_HEADCANDLES2: std::ffi::c_int = S_HEADCANDLES + 1;
pub const S_DEADSTICK: std::ffi::c_int = S_HEADCANDLES2 + 1;
pub const S_LIVESTICK: std::ffi::c_int = S_DEADSTICK + 1;
pub const S_LIVESTICK2: std::ffi::c_int = S_LIVESTICK + 1;
pub const S_MEAT2: std::ffi::c_int = S_LIVESTICK2 + 1;
pub const S_MEAT3: std::ffi::c_int = S_MEAT2 + 1;
pub const S_MEAT4: std::ffi::c_int = S_MEAT3 + 1;
pub const S_MEAT5: std::ffi::c_int = S_MEAT4 + 1;
pub const S_STALAGTITE: std::ffi::c_int = S_MEAT5 + 1;
pub const S_TALLGRNCOL: std::ffi::c_int = S_STALAGTITE + 1;
pub const S_SHRTGRNCOL: std::ffi::c_int = S_TALLGRNCOL + 1;
pub const S_TALLREDCOL: std::ffi::c_int = S_SHRTGRNCOL + 1;
pub const S_SHRTREDCOL: std::ffi::c_int = S_TALLREDCOL + 1;
pub const S_CANDLESTIK: std::ffi::c_int = S_SHRTREDCOL + 1;
pub const S_CANDELABRA: std::ffi::c_int = S_CANDLESTIK + 1;
pub const S_SKULLCOL: std::ffi::c_int = S_CANDELABRA + 1;
pub const S_TORCHTREE: std::ffi::c_int = S_SKULLCOL + 1;
pub const S_BIGTREE: std::ffi::c_int = S_TORCHTREE + 1;
pub const S_TECHPILLAR: std::ffi::c_int = S_BIGTREE + 1;
pub const S_EVILEYE: std::ffi::c_int = S_TECHPILLAR + 1;
pub const S_EVILEYE2: std::ffi::c_int = S_EVILEYE + 1;
pub const S_EVILEYE3: std::ffi::c_int = S_EVILEYE2 + 1;
pub const S_EVILEYE4: std::ffi::c_int = S_EVILEYE3 + 1;
pub const S_FLOATSKULL: std::ffi::c_int = S_EVILEYE4 + 1;
pub const S_FLOATSKULL2: std::ffi::c_int = S_FLOATSKULL + 1;
pub const S_FLOATSKULL3: std::ffi::c_int = S_FLOATSKULL2 + 1;
pub const S_HEARTCOL: std::ffi::c_int = S_FLOATSKULL3 + 1;
pub const S_HEARTCOL2: std::ffi::c_int = S_HEARTCOL + 1;
pub const S_BLUETORCH: std::ffi::c_int = S_HEARTCOL2 + 1;
pub const S_BLUETORCH2: std::ffi::c_int = S_BLUETORCH + 1;
pub const S_BLUETORCH3: std::ffi::c_int = S_BLUETORCH2 + 1;
pub const S_BLUETORCH4: std::ffi::c_int = S_BLUETORCH3 + 1;
pub const S_GREENTORCH: std::ffi::c_int = S_BLUETORCH4 + 1;
pub const S_GREENTORCH2: std::ffi::c_int = S_GREENTORCH + 1;
pub const S_GREENTORCH3: std::ffi::c_int = S_GREENTORCH2 + 1;
pub const S_GREENTORCH4: std::ffi::c_int = S_GREENTORCH3 + 1;
pub const S_REDTORCH: std::ffi::c_int = S_GREENTORCH4 + 1;
pub const S_REDTORCH2: std::ffi::c_int = S_REDTORCH + 1;
pub const S_REDTORCH3: std::ffi::c_int = S_REDTORCH2 + 1;
pub const S_REDTORCH4: std::ffi::c_int = S_REDTORCH3 + 1;
pub const S_BTORCHSHRT: std::ffi::c_int = S_REDTORCH4 + 1;
pub const S_BTORCHSHRT2: std::ffi::c_int = S_BTORCHSHRT + 1;
pub const S_BTORCHSHRT3: std::ffi::c_int = S_BTORCHSHRT2 + 1;
pub const S_BTORCHSHRT4: std::ffi::c_int = S_BTORCHSHRT3 + 1;
pub const S_GTORCHSHRT: std::ffi::c_int = S_BTORCHSHRT4 + 1;
pub const S_GTORCHSHRT2: std::ffi::c_int = S_GTORCHSHRT + 1;
pub const S_GTORCHSHRT3: std::ffi::c_int = S_GTORCHSHRT2 + 1;
pub const S_GTORCHSHRT4: std::ffi::c_int = S_GTORCHSHRT3 + 1;
pub const S_RTORCHSHRT: std::ffi::c_int = S_GTORCHSHRT4 + 1;
pub const S_RTORCHSHRT2: std::ffi::c_int = S_RTORCHSHRT + 1;
pub const S_RTORCHSHRT3: std::ffi::c_int = S_RTORCHSHRT2 + 1;
pub const S_RTORCHSHRT4: std::ffi::c_int = S_RTORCHSHRT3 + 1;
pub const S_HANGNOGUTS: std::ffi::c_int = S_RTORCHSHRT4 + 1;
pub const S_HANGBNOBRAIN: std::ffi::c_int = S_HANGNOGUTS + 1;
pub const S_HANGTLOOKDN: std::ffi::c_int = S_HANGBNOBRAIN + 1;
pub const S_HANGTSKULL: std::ffi::c_int = S_HANGTLOOKDN + 1;
pub const S_HANGTLOOKUP: std::ffi::c_int = S_HANGTSKULL + 1;
pub const S_HANGTNOBRAIN: std::ffi::c_int = S_HANGTLOOKUP + 1;
pub const S_COLONGIBS: std::ffi::c_int = S_HANGTNOBRAIN + 1;
pub const S_SMALLPOOL: std::ffi::c_int = S_COLONGIBS + 1;
pub const S_BRAINSTEM: std::ffi::c_int = S_SMALLPOOL + 1;
pub const S_TECHLAMP: std::ffi::c_int = S_BRAINSTEM + 1;
pub const S_TECHLAMP2: std::ffi::c_int = S_TECHLAMP + 1;
pub const S_TECHLAMP3: std::ffi::c_int = S_TECHLAMP2 + 1;
pub const S_TECHLAMP4: std::ffi::c_int = S_TECHLAMP3 + 1;
pub const S_TECH2LAMP: std::ffi::c_int = S_TECHLAMP4 + 1;
pub const S_TECH2LAMP2: std::ffi::c_int = S_TECH2LAMP + 1;
pub const S_TECH2LAMP3: std::ffi::c_int = S_TECH2LAMP2 + 1;
pub const S_TECH2LAMP4: std::ffi::c_int = S_TECH2LAMP3 + 1;
pub const NUMSTATES: std::ffi::c_int = S_TECH2LAMP4 + 1;

pub type statenum_t = std::ffi::c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct state_t {
    pub sprite: spritenum_t,
    pub frame: std::ffi::c_long,
    pub tics: std::ffi::c_long,
    pub action: actionf_t,
    pub nextstate: statenum_t,
    pub misc1: std::ffi::c_long,
    pub misc2: std::ffi::c_long,
}

pub const MT_PLAYER: std::ffi::c_int = 0;
pub const MT_POSSESSED: std::ffi::c_int = MT_PLAYER + 1;
pub const MT_SHOTGUY: std::ffi::c_int = MT_POSSESSED + 1;
pub const MT_VILE: std::ffi::c_int = MT_SHOTGUY + 1;
pub const MT_FIRE: std::ffi::c_int = MT_VILE + 1;
pub const MT_UNDEAD: std::ffi::c_int = MT_FIRE + 1;
pub const MT_TRACER: std::ffi::c_int = MT_UNDEAD + 1;
pub const MT_SMOKE: std::ffi::c_int = MT_TRACER + 1;
pub const MT_FATSO: std::ffi::c_int = MT_SMOKE + 1;
pub const MT_FATSHOT: std::ffi::c_int = MT_FATSO + 1;
pub const MT_CHAINGUY: std::ffi::c_int = MT_FATSHOT + 1;
pub const MT_TROOP: std::ffi::c_int = MT_CHAINGUY + 1;
pub const MT_SERGEANT: std::ffi::c_int = MT_TROOP + 1;
pub const MT_SHADOWS: std::ffi::c_int = MT_SERGEANT + 1;
pub const MT_HEAD: std::ffi::c_int = MT_SHADOWS + 1;
pub const MT_BRUISER: std::ffi::c_int = MT_HEAD + 1;
pub const MT_BRUISERSHOT: std::ffi::c_int = MT_BRUISER + 1;
pub const MT_KNIGHT: std::ffi::c_int = MT_BRUISERSHOT + 1;
pub const MT_SKULL: std::ffi::c_int = MT_KNIGHT + 1;
pub const MT_SPIDER: std::ffi::c_int = MT_SKULL + 1;
pub const MT_BABY: std::ffi::c_int = MT_SPIDER + 1;
pub const MT_CYBORG: std::ffi::c_int = MT_BABY + 1;
pub const MT_PAIN: std::ffi::c_int = MT_CYBORG + 1;
pub const MT_WOLFSS: std::ffi::c_int = MT_PAIN + 1;
pub const MT_KEEN: std::ffi::c_int = MT_WOLFSS + 1;
pub const MT_BOSSBRAIN: std::ffi::c_int = MT_KEEN + 1;
pub const MT_BOSSSPIT: std::ffi::c_int = MT_BOSSBRAIN + 1;
pub const MT_BOSSTARGET: std::ffi::c_int = MT_BOSSSPIT + 1;
pub const MT_SPAWNSHOT: std::ffi::c_int = MT_BOSSTARGET + 1;
pub const MT_SPAWNFIRE: std::ffi::c_int = MT_SPAWNSHOT + 1;
pub const MT_BARREL: std::ffi::c_int = MT_SPAWNFIRE + 1;
pub const MT_TROOPSHOT: std::ffi::c_int = MT_BARREL + 1;
pub const MT_HEADSHOT: std::ffi::c_int = MT_TROOPSHOT + 1;
pub const MT_ROCKET: std::ffi::c_int = MT_HEADSHOT + 1;
pub const MT_PLASMA: std::ffi::c_int = MT_ROCKET + 1;
pub const MT_BFG: std::ffi::c_int = MT_PLASMA + 1;
pub const MT_ARACHPLAZ: std::ffi::c_int = MT_BFG + 1;
pub const MT_PUFF: std::ffi::c_int = MT_ARACHPLAZ + 1;
pub const MT_BLOOD: std::ffi::c_int = MT_PUFF + 1;
pub const MT_TFOG: std::ffi::c_int = MT_BLOOD + 1;
pub const MT_IFOG: std::ffi::c_int = MT_TFOG + 1;
pub const MT_TELEPORTMAN: std::ffi::c_int = MT_IFOG + 1;
pub const MT_EXTRABFG: std::ffi::c_int = MT_TELEPORTMAN + 1;
pub const MT_MISC0: std::ffi::c_int = MT_EXTRABFG + 1;
pub const MT_MISC1: std::ffi::c_int = MT_MISC0 + 1;
pub const MT_MISC2: std::ffi::c_int = MT_MISC1 + 1;
pub const MT_MISC3: std::ffi::c_int = MT_MISC2 + 1;
pub const MT_MISC4: std::ffi::c_int = MT_MISC3 + 1;
pub const MT_MISC5: std::ffi::c_int = MT_MISC4 + 1;
pub const MT_MISC6: std::ffi::c_int = MT_MISC5 + 1;
pub const MT_MISC7: std::ffi::c_int = MT_MISC6 + 1;
pub const MT_MISC8: std::ffi::c_int = MT_MISC7 + 1;
pub const MT_MISC9: std::ffi::c_int = MT_MISC8 + 1;
pub const MT_MISC10: std::ffi::c_int = MT_MISC9 + 1;
pub const MT_MISC11: std::ffi::c_int = MT_MISC10 + 1;
pub const MT_MISC12: std::ffi::c_int = MT_MISC11 + 1;
pub const MT_INV: std::ffi::c_int = MT_MISC12 + 1;
pub const MT_MISC13: std::ffi::c_int = MT_INV + 1;
pub const MT_INS: std::ffi::c_int = MT_MISC13 + 1;
pub const MT_MISC14: std::ffi::c_int = MT_INS + 1;
pub const MT_MISC15: std::ffi::c_int = MT_MISC14 + 1;
pub const MT_MISC16: std::ffi::c_int = MT_MISC15 + 1;
pub const MT_MEGA: std::ffi::c_int = MT_MISC16 + 1;
pub const MT_CLIP: std::ffi::c_int = MT_MEGA + 1;
pub const MT_MISC17: std::ffi::c_int = MT_CLIP + 1;
pub const MT_MISC18: std::ffi::c_int = MT_MISC17 + 1;
pub const MT_MISC19: std::ffi::c_int = MT_MISC18 + 1;
pub const MT_MISC20: std::ffi::c_int = MT_MISC19 + 1;
pub const MT_MISC21: std::ffi::c_int = MT_MISC20 + 1;
pub const MT_MISC22: std::ffi::c_int = MT_MISC21 + 1;
pub const MT_MISC23: std::ffi::c_int = MT_MISC22 + 1;
pub const MT_MISC24: std::ffi::c_int = MT_MISC23 + 1;
pub const MT_MISC25: std::ffi::c_int = MT_MISC24 + 1;
pub const MT_CHAINGUN: std::ffi::c_int = MT_MISC25 + 1;
pub const MT_MISC26: std::ffi::c_int = MT_CHAINGUN + 1;
pub const MT_MISC27: std::ffi::c_int = MT_MISC26 + 1;
pub const MT_MISC28: std::ffi::c_int = MT_MISC27 + 1;
pub const MT_SHOTGUN: std::ffi::c_int = MT_MISC28 + 1;
pub const MT_SUPERSHOTGUN: std::ffi::c_int = MT_SHOTGUN + 1;
pub const MT_MISC29: std::ffi::c_int = MT_SUPERSHOTGUN + 1;
pub const MT_MISC30: std::ffi::c_int = MT_MISC29 + 1;
pub const MT_MISC31: std::ffi::c_int = MT_MISC30 + 1;
pub const MT_MISC32: std::ffi::c_int = MT_MISC31 + 1;
pub const MT_MISC33: std::ffi::c_int = MT_MISC32 + 1;
pub const MT_MISC34: std::ffi::c_int = MT_MISC33 + 1;
pub const MT_MISC35: std::ffi::c_int = MT_MISC34 + 1;
pub const MT_MISC36: std::ffi::c_int = MT_MISC35 + 1;
pub const MT_MISC37: std::ffi::c_int = MT_MISC36 + 1;
pub const MT_MISC38: std::ffi::c_int = MT_MISC37 + 1;
pub const MT_MISC39: std::ffi::c_int = MT_MISC38 + 1;
pub const MT_MISC40: std::ffi::c_int = MT_MISC39 + 1;
pub const MT_MISC41: std::ffi::c_int = MT_MISC40 + 1;
pub const MT_MISC42: std::ffi::c_int = MT_MISC41 + 1;
pub const MT_MISC43: std::ffi::c_int = MT_MISC42 + 1;
pub const MT_MISC44: std::ffi::c_int = MT_MISC43 + 1;
pub const MT_MISC45: std::ffi::c_int = MT_MISC44 + 1;
pub const MT_MISC46: std::ffi::c_int = MT_MISC45 + 1;
pub const MT_MISC47: std::ffi::c_int = MT_MISC46 + 1;
pub const MT_MISC48: std::ffi::c_int = MT_MISC47 + 1;
pub const MT_MISC49: std::ffi::c_int = MT_MISC48 + 1;
pub const MT_MISC50: std::ffi::c_int = MT_MISC49 + 1;
pub const MT_MISC51: std::ffi::c_int = MT_MISC50 + 1;
pub const MT_MISC52: std::ffi::c_int = MT_MISC51 + 1;
pub const MT_MISC53: std::ffi::c_int = MT_MISC52 + 1;
pub const MT_MISC54: std::ffi::c_int = MT_MISC53 + 1;
pub const MT_MISC55: std::ffi::c_int = MT_MISC54 + 1;
pub const MT_MISC56: std::ffi::c_int = MT_MISC55 + 1;
pub const MT_MISC57: std::ffi::c_int = MT_MISC56 + 1;
pub const MT_MISC58: std::ffi::c_int = MT_MISC57 + 1;
pub const MT_MISC59: std::ffi::c_int = MT_MISC58 + 1;
pub const MT_MISC60: std::ffi::c_int = MT_MISC59 + 1;
pub const MT_MISC61: std::ffi::c_int = MT_MISC60 + 1;
pub const MT_MISC62: std::ffi::c_int = MT_MISC61 + 1;
pub const MT_MISC63: std::ffi::c_int = MT_MISC62 + 1;
pub const MT_MISC64: std::ffi::c_int = MT_MISC63 + 1;
pub const MT_MISC65: std::ffi::c_int = MT_MISC64 + 1;
pub const MT_MISC66: std::ffi::c_int = MT_MISC65 + 1;
pub const MT_MISC67: std::ffi::c_int = MT_MISC66 + 1;
pub const MT_MISC68: std::ffi::c_int = MT_MISC67 + 1;
pub const MT_MISC69: std::ffi::c_int = MT_MISC68 + 1;
pub const MT_MISC70: std::ffi::c_int = MT_MISC69 + 1;
pub const MT_MISC71: std::ffi::c_int = MT_MISC70 + 1;
pub const MT_MISC72: std::ffi::c_int = MT_MISC71 + 1;
pub const MT_MISC73: std::ffi::c_int = MT_MISC72 + 1;
pub const MT_MISC74: std::ffi::c_int = MT_MISC73 + 1;
pub const MT_MISC75: std::ffi::c_int = MT_MISC74 + 1;
pub const MT_MISC76: std::ffi::c_int = MT_MISC75 + 1;
pub const MT_MISC77: std::ffi::c_int = MT_MISC76 + 1;
pub const MT_MISC78: std::ffi::c_int = MT_MISC77 + 1;
pub const MT_MISC79: std::ffi::c_int = MT_MISC78 + 1;
pub const MT_MISC80: std::ffi::c_int = MT_MISC79 + 1;
pub const MT_MISC81: std::ffi::c_int = MT_MISC80 + 1;
pub const MT_MISC82: std::ffi::c_int = MT_MISC81 + 1;
pub const MT_MISC83: std::ffi::c_int = MT_MISC82 + 1;
pub const MT_MISC84: std::ffi::c_int = MT_MISC83 + 1;
pub const MT_MISC85: std::ffi::c_int = MT_MISC84 + 1;
pub const MT_MISC86: std::ffi::c_int = MT_MISC85 + 1;
pub const NUMMOBJTYPES: std::ffi::c_int = MT_MISC86 + 1;

pub type mobjtype_t = std::ffi::c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mobjinfo_t {
    pub doomednum: std::ffi::c_int,
    pub spawnstate: std::ffi::c_int,
    pub spawnhealth: std::ffi::c_int,
    pub seestate: std::ffi::c_int,
    pub seesound: std::ffi::c_int,
    pub reactiontime: std::ffi::c_int,
    pub attacksound: std::ffi::c_int,
    pub painstate: std::ffi::c_int,
    pub painchance: std::ffi::c_int,
    pub painsound: std::ffi::c_int,
    pub meleestate: std::ffi::c_int,
    pub missilestate: std::ffi::c_int,
    pub deathstate: std::ffi::c_int,
    pub xdeathstate: std::ffi::c_int,
    pub deathsound: std::ffi::c_int,
    pub speed: std::ffi::c_int,
    pub radius: std::ffi::c_int,
    pub height: std::ffi::c_int,
    pub mass: std::ffi::c_int,
    pub damage: std::ffi::c_int,
    pub activesound: std::ffi::c_int,
    pub flags: std::ffi::c_int,
    pub raisestate: std::ffi::c_int,
}

static mut rcsid: [std::ffi::c_char; 47] = unsafe {
    [
        36 as std::ffi::c_char,
        73 as std::ffi::c_char,
        100 as std::ffi::c_char,
        58 as std::ffi::c_char,
        32 as std::ffi::c_char,
        105 as std::ffi::c_char,
        110 as std::ffi::c_char,
        102 as std::ffi::c_char,
        111 as std::ffi::c_char,
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
        54 as std::ffi::c_char,
        32 as std::ffi::c_char,
        48 as std::ffi::c_char,
        55 as std::ffi::c_char,
        58 as std::ffi::c_char,
        52 as std::ffi::c_char,
        53 as std::ffi::c_char,
        58 as std::ffi::c_char,
        48 as std::ffi::c_char,
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

pub static mut sprnames: [*mut std::ffi::c_char; (NUMSPRITES) as usize] = unsafe {
    [
        (c"TROO").as_ptr() as *mut std::ffi::c_char,
        (c"SHTG").as_ptr() as *mut std::ffi::c_char,
        (c"PUNG").as_ptr() as *mut std::ffi::c_char,
        (c"PISG").as_ptr() as *mut std::ffi::c_char,
        (c"PISF").as_ptr() as *mut std::ffi::c_char,
        (c"SHTF").as_ptr() as *mut std::ffi::c_char,
        (c"SHT2").as_ptr() as *mut std::ffi::c_char,
        (c"CHGG").as_ptr() as *mut std::ffi::c_char,
        (c"CHGF").as_ptr() as *mut std::ffi::c_char,
        (c"MISG").as_ptr() as *mut std::ffi::c_char,
        (c"MISF").as_ptr() as *mut std::ffi::c_char,
        (c"SAWG").as_ptr() as *mut std::ffi::c_char,
        (c"PLSG").as_ptr() as *mut std::ffi::c_char,
        (c"PLSF").as_ptr() as *mut std::ffi::c_char,
        (c"BFGG").as_ptr() as *mut std::ffi::c_char,
        (c"BFGF").as_ptr() as *mut std::ffi::c_char,
        (c"BLUD").as_ptr() as *mut std::ffi::c_char,
        (c"PUFF").as_ptr() as *mut std::ffi::c_char,
        (c"BAL1").as_ptr() as *mut std::ffi::c_char,
        (c"BAL2").as_ptr() as *mut std::ffi::c_char,
        (c"PLSS").as_ptr() as *mut std::ffi::c_char,
        (c"PLSE").as_ptr() as *mut std::ffi::c_char,
        (c"MISL").as_ptr() as *mut std::ffi::c_char,
        (c"BFS1").as_ptr() as *mut std::ffi::c_char,
        (c"BFE1").as_ptr() as *mut std::ffi::c_char,
        (c"BFE2").as_ptr() as *mut std::ffi::c_char,
        (c"TFOG").as_ptr() as *mut std::ffi::c_char,
        (c"IFOG").as_ptr() as *mut std::ffi::c_char,
        (c"PLAY").as_ptr() as *mut std::ffi::c_char,
        (c"POSS").as_ptr() as *mut std::ffi::c_char,
        (c"SPOS").as_ptr() as *mut std::ffi::c_char,
        (c"VILE").as_ptr() as *mut std::ffi::c_char,
        (c"FIRE").as_ptr() as *mut std::ffi::c_char,
        (c"FATB").as_ptr() as *mut std::ffi::c_char,
        (c"FBXP").as_ptr() as *mut std::ffi::c_char,
        (c"SKEL").as_ptr() as *mut std::ffi::c_char,
        (c"MANF").as_ptr() as *mut std::ffi::c_char,
        (c"FATT").as_ptr() as *mut std::ffi::c_char,
        (c"CPOS").as_ptr() as *mut std::ffi::c_char,
        (c"SARG").as_ptr() as *mut std::ffi::c_char,
        (c"HEAD").as_ptr() as *mut std::ffi::c_char,
        (c"BAL7").as_ptr() as *mut std::ffi::c_char,
        (c"BOSS").as_ptr() as *mut std::ffi::c_char,
        (c"BOS2").as_ptr() as *mut std::ffi::c_char,
        (c"SKUL").as_ptr() as *mut std::ffi::c_char,
        (c"SPID").as_ptr() as *mut std::ffi::c_char,
        (c"BSPI").as_ptr() as *mut std::ffi::c_char,
        (c"APLS").as_ptr() as *mut std::ffi::c_char,
        (c"APBX").as_ptr() as *mut std::ffi::c_char,
        (c"CYBR").as_ptr() as *mut std::ffi::c_char,
        (c"PAIN").as_ptr() as *mut std::ffi::c_char,
        (c"SSWV").as_ptr() as *mut std::ffi::c_char,
        (c"KEEN").as_ptr() as *mut std::ffi::c_char,
        (c"BBRN").as_ptr() as *mut std::ffi::c_char,
        (c"BOSF").as_ptr() as *mut std::ffi::c_char,
        (c"ARM1").as_ptr() as *mut std::ffi::c_char,
        (c"ARM2").as_ptr() as *mut std::ffi::c_char,
        (c"BAR1").as_ptr() as *mut std::ffi::c_char,
        (c"BEXP").as_ptr() as *mut std::ffi::c_char,
        (c"FCAN").as_ptr() as *mut std::ffi::c_char,
        (c"BON1").as_ptr() as *mut std::ffi::c_char,
        (c"BON2").as_ptr() as *mut std::ffi::c_char,
        (c"BKEY").as_ptr() as *mut std::ffi::c_char,
        (c"RKEY").as_ptr() as *mut std::ffi::c_char,
        (c"YKEY").as_ptr() as *mut std::ffi::c_char,
        (c"BSKU").as_ptr() as *mut std::ffi::c_char,
        (c"RSKU").as_ptr() as *mut std::ffi::c_char,
        (c"YSKU").as_ptr() as *mut std::ffi::c_char,
        (c"STIM").as_ptr() as *mut std::ffi::c_char,
        (c"MEDI").as_ptr() as *mut std::ffi::c_char,
        (c"SOUL").as_ptr() as *mut std::ffi::c_char,
        (c"PINV").as_ptr() as *mut std::ffi::c_char,
        (c"PSTR").as_ptr() as *mut std::ffi::c_char,
        (c"PINS").as_ptr() as *mut std::ffi::c_char,
        (c"MEGA").as_ptr() as *mut std::ffi::c_char,
        (c"SUIT").as_ptr() as *mut std::ffi::c_char,
        (c"PMAP").as_ptr() as *mut std::ffi::c_char,
        (c"PVIS").as_ptr() as *mut std::ffi::c_char,
        (c"CLIP").as_ptr() as *mut std::ffi::c_char,
        (c"AMMO").as_ptr() as *mut std::ffi::c_char,
        (c"ROCK").as_ptr() as *mut std::ffi::c_char,
        (c"BROK").as_ptr() as *mut std::ffi::c_char,
        (c"CELL").as_ptr() as *mut std::ffi::c_char,
        (c"CELP").as_ptr() as *mut std::ffi::c_char,
        (c"SHEL").as_ptr() as *mut std::ffi::c_char,
        (c"SBOX").as_ptr() as *mut std::ffi::c_char,
        (c"BPAK").as_ptr() as *mut std::ffi::c_char,
        (c"BFUG").as_ptr() as *mut std::ffi::c_char,
        (c"MGUN").as_ptr() as *mut std::ffi::c_char,
        (c"CSAW").as_ptr() as *mut std::ffi::c_char,
        (c"LAUN").as_ptr() as *mut std::ffi::c_char,
        (c"PLAS").as_ptr() as *mut std::ffi::c_char,
        (c"SHOT").as_ptr() as *mut std::ffi::c_char,
        (c"SGN2").as_ptr() as *mut std::ffi::c_char,
        (c"COLU").as_ptr() as *mut std::ffi::c_char,
        (c"SMT2").as_ptr() as *mut std::ffi::c_char,
        (c"GOR1").as_ptr() as *mut std::ffi::c_char,
        (c"POL2").as_ptr() as *mut std::ffi::c_char,
        (c"POL5").as_ptr() as *mut std::ffi::c_char,
        (c"POL4").as_ptr() as *mut std::ffi::c_char,
        (c"POL3").as_ptr() as *mut std::ffi::c_char,
        (c"POL1").as_ptr() as *mut std::ffi::c_char,
        (c"POL6").as_ptr() as *mut std::ffi::c_char,
        (c"GOR2").as_ptr() as *mut std::ffi::c_char,
        (c"GOR3").as_ptr() as *mut std::ffi::c_char,
        (c"GOR4").as_ptr() as *mut std::ffi::c_char,
        (c"GOR5").as_ptr() as *mut std::ffi::c_char,
        (c"SMIT").as_ptr() as *mut std::ffi::c_char,
        (c"COL1").as_ptr() as *mut std::ffi::c_char,
        (c"COL2").as_ptr() as *mut std::ffi::c_char,
        (c"COL3").as_ptr() as *mut std::ffi::c_char,
        (c"COL4").as_ptr() as *mut std::ffi::c_char,
        (c"CAND").as_ptr() as *mut std::ffi::c_char,
        (c"CBRA").as_ptr() as *mut std::ffi::c_char,
        (c"COL6").as_ptr() as *mut std::ffi::c_char,
        (c"TRE1").as_ptr() as *mut std::ffi::c_char,
        (c"TRE2").as_ptr() as *mut std::ffi::c_char,
        (c"ELEC").as_ptr() as *mut std::ffi::c_char,
        (c"CEYE").as_ptr() as *mut std::ffi::c_char,
        (c"FSKU").as_ptr() as *mut std::ffi::c_char,
        (c"COL5").as_ptr() as *mut std::ffi::c_char,
        (c"TBLU").as_ptr() as *mut std::ffi::c_char,
        (c"TGRN").as_ptr() as *mut std::ffi::c_char,
        (c"TRED").as_ptr() as *mut std::ffi::c_char,
        (c"SMBT").as_ptr() as *mut std::ffi::c_char,
        (c"SMGT").as_ptr() as *mut std::ffi::c_char,
        (c"SMRT").as_ptr() as *mut std::ffi::c_char,
        (c"HDB1").as_ptr() as *mut std::ffi::c_char,
        (c"HDB2").as_ptr() as *mut std::ffi::c_char,
        (c"HDB3").as_ptr() as *mut std::ffi::c_char,
        (c"HDB4").as_ptr() as *mut std::ffi::c_char,
        (c"HDB5").as_ptr() as *mut std::ffi::c_char,
        (c"HDB6").as_ptr() as *mut std::ffi::c_char,
        (c"POB1").as_ptr() as *mut std::ffi::c_char,
        (c"POB2").as_ptr() as *mut std::ffi::c_char,
        (c"BRS1").as_ptr() as *mut std::ffi::c_char,
        (c"TLMP").as_ptr() as *mut std::ffi::c_char,
        (c"TLP2").as_ptr() as *mut std::ffi::c_char,
    ]
};

unsafe extern "C" {
    pub fn A_Light0();
}

unsafe extern "C" {
    pub fn A_WeaponReady();
}

unsafe extern "C" {
    pub fn A_Lower();
}

unsafe extern "C" {
    pub fn A_Raise();
}

unsafe extern "C" {
    pub fn A_Punch();
}

unsafe extern "C" {
    pub fn A_ReFire();
}

unsafe extern "C" {
    pub fn A_FirePistol();
}

unsafe extern "C" {
    pub fn A_Light1();
}

unsafe extern "C" {
    pub fn A_FireShotgun();
}

unsafe extern "C" {
    pub fn A_Light2();
}

unsafe extern "C" {
    pub fn A_FireShotgun2();
}

unsafe extern "C" {
    pub fn A_CheckReload();
}

unsafe extern "C" {
    pub fn A_OpenShotgun2();
}

unsafe extern "C" {
    pub fn A_LoadShotgun2();
}

unsafe extern "C" {
    pub fn A_CloseShotgun2();
}

unsafe extern "C" {
    pub fn A_FireCGun();
}

unsafe extern "C" {
    pub fn A_GunFlash();
}

unsafe extern "C" {
    pub fn A_FireMissile();
}

unsafe extern "C" {
    pub fn A_Saw();
}

unsafe extern "C" {
    pub fn A_FirePlasma();
}

unsafe extern "C" {
    pub fn A_BFGsound();
}

unsafe extern "C" {
    pub fn A_FireBFG();
}

unsafe extern "C" {
    pub fn A_BFGSpray();
}

unsafe extern "C" {
    pub fn A_Explode();
}

unsafe extern "C" {
    pub fn A_Pain();
}

unsafe extern "C" {
    pub fn A_PlayerScream();
}

unsafe extern "C" {
    pub fn A_Fall();
}

unsafe extern "C" {
    pub fn A_XScream();
}

unsafe extern "C" {
    pub fn A_Look();
}

unsafe extern "C" {
    pub fn A_Chase();
}

unsafe extern "C" {
    pub fn A_FaceTarget();
}

unsafe extern "C" {
    pub fn A_PosAttack();
}

unsafe extern "C" {
    pub fn A_Scream();
}

unsafe extern "C" {
    pub fn A_SPosAttack();
}

unsafe extern "C" {
    pub fn A_VileChase();
}

unsafe extern "C" {
    pub fn A_VileStart();
}

unsafe extern "C" {
    pub fn A_VileTarget();
}

unsafe extern "C" {
    pub fn A_VileAttack();
}

unsafe extern "C" {
    pub fn A_StartFire();
}

unsafe extern "C" {
    pub fn A_Fire();
}

unsafe extern "C" {
    pub fn A_FireCrackle();
}

unsafe extern "C" {
    pub fn A_Tracer();
}

unsafe extern "C" {
    pub fn A_SkelWhoosh();
}

unsafe extern "C" {
    pub fn A_SkelFist();
}

unsafe extern "C" {
    pub fn A_SkelMissile();
}

unsafe extern "C" {
    pub fn A_FatRaise();
}

unsafe extern "C" {
    pub fn A_FatAttack1();
}

unsafe extern "C" {
    pub fn A_FatAttack2();
}

unsafe extern "C" {
    pub fn A_FatAttack3();
}

unsafe extern "C" {
    pub fn A_BossDeath();
}

unsafe extern "C" {
    pub fn A_CPosAttack();
}

unsafe extern "C" {
    pub fn A_CPosRefire();
}

unsafe extern "C" {
    pub fn A_TroopAttack();
}

unsafe extern "C" {
    pub fn A_SargAttack();
}

unsafe extern "C" {
    pub fn A_HeadAttack();
}

unsafe extern "C" {
    pub fn A_BruisAttack();
}

unsafe extern "C" {
    pub fn A_SkullAttack();
}

unsafe extern "C" {
    pub fn A_Metal();
}

unsafe extern "C" {
    pub fn A_SpidRefire();
}

unsafe extern "C" {
    pub fn A_BabyMetal();
}

unsafe extern "C" {
    pub fn A_BspiAttack();
}

unsafe extern "C" {
    pub fn A_Hoof();
}

unsafe extern "C" {
    pub fn A_CyberAttack();
}

unsafe extern "C" {
    pub fn A_PainAttack();
}

unsafe extern "C" {
    pub fn A_PainDie();
}

unsafe extern "C" {
    pub fn A_KeenDie();
}

unsafe extern "C" {
    pub fn A_BrainPain();
}

unsafe extern "C" {
    pub fn A_BrainScream();
}

unsafe extern "C" {
    pub fn A_BrainDie();
}

unsafe extern "C" {
    pub fn A_BrainAwake();
}

unsafe extern "C" {
    pub fn A_BrainSpit();
}

unsafe extern "C" {
    pub fn A_SpawnSound();
}

unsafe extern "C" {
    pub fn A_SpawnFly();
}

unsafe extern "C" {
    pub fn A_BrainExplode();
}

pub static mut states: [state_t; (NUMSTATES) as usize] = unsafe {
    [
        state_t {
            sprite: SPR_TROO,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SHTG,
            frame: 4,
            tics: 0,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Light0 as *const (),
                    )
                }),
            },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PUNG,
            frame: 0,
            tics: 1,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_WeaponReady as *const (),
                    )
                }),
            },
            nextstate: S_PUNCH,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PUNG,
            frame: 0,
            tics: 1,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Lower as *const (),
                    )
                }),
            },
            nextstate: S_PUNCHDOWN,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PUNG,
            frame: 0,
            tics: 1,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Raise as *const (),
                    )
                }),
            },
            nextstate: S_PUNCHUP,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PUNG,
            frame: 1,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_PUNCH2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PUNG,
            frame: 2,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Punch as *const (),
                    )
                }),
            },
            nextstate: S_PUNCH3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PUNG,
            frame: 3,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_PUNCH4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PUNG,
            frame: 2,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_PUNCH5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PUNG,
            frame: 1,
            tics: 5,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_ReFire as *const (),
                    )
                }),
            },
            nextstate: S_PUNCH,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PISG,
            frame: 0,
            tics: 1,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_WeaponReady as *const (),
                    )
                }),
            },
            nextstate: S_PISTOL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PISG,
            frame: 0,
            tics: 1,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Lower as *const (),
                    )
                }),
            },
            nextstate: S_PISTOLDOWN,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PISG,
            frame: 0,
            tics: 1,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Raise as *const (),
                    )
                }),
            },
            nextstate: S_PISTOLUP,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PISG,
            frame: 0,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_PISTOL2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PISG,
            frame: 1,
            tics: 6,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FirePistol as *const (),
                    )
                }),
            },
            nextstate: S_PISTOL3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PISG,
            frame: 2,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_PISTOL4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PISG,
            frame: 1,
            tics: 5,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_ReFire as *const (),
                    )
                }),
            },
            nextstate: S_PISTOL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PISF,
            frame: 32768,
            tics: 7,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Light1 as *const (),
                    )
                }),
            },
            nextstate: S_LIGHTDONE,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SHTG,
            frame: 0,
            tics: 1,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_WeaponReady as *const (),
                    )
                }),
            },
            nextstate: S_SGUN,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SHTG,
            frame: 0,
            tics: 1,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Lower as *const (),
                    )
                }),
            },
            nextstate: S_SGUNDOWN,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SHTG,
            frame: 0,
            tics: 1,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Raise as *const (),
                    )
                }),
            },
            nextstate: S_SGUNUP,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SHTG,
            frame: 0,
            tics: 3,
            action: actionf_t { acp1: None },
            nextstate: S_SGUN2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SHTG,
            frame: 0,
            tics: 7,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FireShotgun as *const (),
                    )
                }),
            },
            nextstate: S_SGUN3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SHTG,
            frame: 1,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_SGUN4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SHTG,
            frame: 2,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_SGUN5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SHTG,
            frame: 3,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_SGUN6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SHTG,
            frame: 2,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_SGUN7,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SHTG,
            frame: 1,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_SGUN8,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SHTG,
            frame: 0,
            tics: 3,
            action: actionf_t { acp1: None },
            nextstate: S_SGUN9,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SHTG,
            frame: 0,
            tics: 7,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_ReFire as *const (),
                    )
                }),
            },
            nextstate: S_SGUN,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SHTF,
            frame: 32768,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Light1 as *const (),
                    )
                }),
            },
            nextstate: S_SGUNFLASH2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SHTF,
            frame: 32769,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Light2 as *const (),
                    )
                }),
            },
            nextstate: S_LIGHTDONE,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SHT2,
            frame: 0,
            tics: 1,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_WeaponReady as *const (),
                    )
                }),
            },
            nextstate: S_DSGUN,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SHT2,
            frame: 0,
            tics: 1,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Lower as *const (),
                    )
                }),
            },
            nextstate: S_DSGUNDOWN,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SHT2,
            frame: 0,
            tics: 1,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Raise as *const (),
                    )
                }),
            },
            nextstate: S_DSGUNUP,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SHT2,
            frame: 0,
            tics: 3,
            action: actionf_t { acp1: None },
            nextstate: S_DSGUN2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SHT2,
            frame: 0,
            tics: 7,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FireShotgun2 as *const (),
                    )
                }),
            },
            nextstate: S_DSGUN3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SHT2,
            frame: 1,
            tics: 7,
            action: actionf_t { acp1: None },
            nextstate: S_DSGUN4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SHT2,
            frame: 2,
            tics: 7,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_CheckReload as *const (),
                    )
                }),
            },
            nextstate: S_DSGUN5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SHT2,
            frame: 3,
            tics: 7,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_OpenShotgun2 as *const (),
                    )
                }),
            },
            nextstate: S_DSGUN6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SHT2,
            frame: 4,
            tics: 7,
            action: actionf_t { acp1: None },
            nextstate: S_DSGUN7,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SHT2,
            frame: 5,
            tics: 7,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_LoadShotgun2 as *const (),
                    )
                }),
            },
            nextstate: S_DSGUN8,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SHT2,
            frame: 6,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_DSGUN9,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SHT2,
            frame: 7,
            tics: 6,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_CloseShotgun2 as *const (),
                    )
                }),
            },
            nextstate: S_DSGUN10,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SHT2,
            frame: 0,
            tics: 5,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_ReFire as *const (),
                    )
                }),
            },
            nextstate: S_DSGUN,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SHT2,
            frame: 1,
            tics: 7,
            action: actionf_t { acp1: None },
            nextstate: S_DSNR2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SHT2,
            frame: 0,
            tics: 3,
            action: actionf_t { acp1: None },
            nextstate: S_DSGUNDOWN,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SHT2,
            frame: 32776,
            tics: 5,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Light1 as *const (),
                    )
                }),
            },
            nextstate: S_DSGUNFLASH2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SHT2,
            frame: 32777,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Light2 as *const (),
                    )
                }),
            },
            nextstate: S_LIGHTDONE,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CHGG,
            frame: 0,
            tics: 1,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_WeaponReady as *const (),
                    )
                }),
            },
            nextstate: S_CHAIN,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CHGG,
            frame: 0,
            tics: 1,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Lower as *const (),
                    )
                }),
            },
            nextstate: S_CHAINDOWN,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CHGG,
            frame: 0,
            tics: 1,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Raise as *const (),
                    )
                }),
            },
            nextstate: S_CHAINUP,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CHGG,
            frame: 0,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FireCGun as *const (),
                    )
                }),
            },
            nextstate: S_CHAIN2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CHGG,
            frame: 1,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FireCGun as *const (),
                    )
                }),
            },
            nextstate: S_CHAIN3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CHGG,
            frame: 1,
            tics: 0,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_ReFire as *const (),
                    )
                }),
            },
            nextstate: S_CHAIN,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CHGF,
            frame: 32768,
            tics: 5,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Light1 as *const (),
                    )
                }),
            },
            nextstate: S_LIGHTDONE,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CHGF,
            frame: 32769,
            tics: 5,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Light2 as *const (),
                    )
                }),
            },
            nextstate: S_LIGHTDONE,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_MISG,
            frame: 0,
            tics: 1,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_WeaponReady as *const (),
                    )
                }),
            },
            nextstate: S_MISSILE,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_MISG,
            frame: 0,
            tics: 1,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Lower as *const (),
                    )
                }),
            },
            nextstate: S_MISSILEDOWN,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_MISG,
            frame: 0,
            tics: 1,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Raise as *const (),
                    )
                }),
            },
            nextstate: S_MISSILEUP,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_MISG,
            frame: 1,
            tics: 8,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_GunFlash as *const (),
                    )
                }),
            },
            nextstate: S_MISSILE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_MISG,
            frame: 1,
            tics: 12,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FireMissile as *const (),
                    )
                }),
            },
            nextstate: S_MISSILE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_MISG,
            frame: 1,
            tics: 0,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_ReFire as *const (),
                    )
                }),
            },
            nextstate: S_MISSILE,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_MISF,
            frame: 32768,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Light1 as *const (),
                    )
                }),
            },
            nextstate: S_MISSILEFLASH2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_MISF,
            frame: 32769,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_MISSILEFLASH3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_MISF,
            frame: 32770,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Light2 as *const (),
                    )
                }),
            },
            nextstate: S_MISSILEFLASH4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_MISF,
            frame: 32771,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Light2 as *const (),
                    )
                }),
            },
            nextstate: S_LIGHTDONE,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SAWG,
            frame: 2,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_WeaponReady as *const (),
                    )
                }),
            },
            nextstate: S_SAWB,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SAWG,
            frame: 3,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_WeaponReady as *const (),
                    )
                }),
            },
            nextstate: S_SAW,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SAWG,
            frame: 2,
            tics: 1,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Lower as *const (),
                    )
                }),
            },
            nextstate: S_SAWDOWN,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SAWG,
            frame: 2,
            tics: 1,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Raise as *const (),
                    )
                }),
            },
            nextstate: S_SAWUP,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SAWG,
            frame: 0,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Saw as *const (),
                    )
                }),
            },
            nextstate: S_SAW2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SAWG,
            frame: 1,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Saw as *const (),
                    )
                }),
            },
            nextstate: S_SAW3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SAWG,
            frame: 1,
            tics: 0,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_ReFire as *const (),
                    )
                }),
            },
            nextstate: S_SAW,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PLSG,
            frame: 0,
            tics: 1,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_WeaponReady as *const (),
                    )
                }),
            },
            nextstate: S_PLASMA,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PLSG,
            frame: 0,
            tics: 1,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Lower as *const (),
                    )
                }),
            },
            nextstate: S_PLASMADOWN,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PLSG,
            frame: 0,
            tics: 1,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Raise as *const (),
                    )
                }),
            },
            nextstate: S_PLASMAUP,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PLSG,
            frame: 0,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FirePlasma as *const (),
                    )
                }),
            },
            nextstate: S_PLASMA2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PLSG,
            frame: 1,
            tics: 20,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_ReFire as *const (),
                    )
                }),
            },
            nextstate: S_PLASMA,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PLSF,
            frame: 32768,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Light1 as *const (),
                    )
                }),
            },
            nextstate: S_LIGHTDONE,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PLSF,
            frame: 32769,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Light1 as *const (),
                    )
                }),
            },
            nextstate: S_LIGHTDONE,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BFGG,
            frame: 0,
            tics: 1,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_WeaponReady as *const (),
                    )
                }),
            },
            nextstate: S_BFG,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BFGG,
            frame: 0,
            tics: 1,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Lower as *const (),
                    )
                }),
            },
            nextstate: S_BFGDOWN,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BFGG,
            frame: 0,
            tics: 1,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Raise as *const (),
                    )
                }),
            },
            nextstate: S_BFGUP,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BFGG,
            frame: 0,
            tics: 20,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_BFGsound as *const (),
                    )
                }),
            },
            nextstate: S_BFG2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BFGG,
            frame: 1,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_GunFlash as *const (),
                    )
                }),
            },
            nextstate: S_BFG3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BFGG,
            frame: 1,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FireBFG as *const (),
                    )
                }),
            },
            nextstate: S_BFG4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BFGG,
            frame: 1,
            tics: 20,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_ReFire as *const (),
                    )
                }),
            },
            nextstate: S_BFG,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BFGF,
            frame: 32768,
            tics: 11,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Light1 as *const (),
                    )
                }),
            },
            nextstate: S_BFGFLASH2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BFGF,
            frame: 32769,
            tics: 6,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Light2 as *const (),
                    )
                }),
            },
            nextstate: S_LIGHTDONE,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BLUD,
            frame: 2,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_BLOOD2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BLUD,
            frame: 1,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_BLOOD3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BLUD,
            frame: 0,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PUFF,
            frame: 32768,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_PUFF2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PUFF,
            frame: 1,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_PUFF3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PUFF,
            frame: 2,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_PUFF4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PUFF,
            frame: 3,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BAL1,
            frame: 32768,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_TBALL2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BAL1,
            frame: 32769,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_TBALL1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BAL1,
            frame: 32770,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_TBALLX2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BAL1,
            frame: 32771,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_TBALLX3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BAL1,
            frame: 32772,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BAL2,
            frame: 32768,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_RBALL2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BAL2,
            frame: 32769,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_RBALL1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BAL2,
            frame: 32770,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_RBALLX2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BAL2,
            frame: 32771,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_RBALLX3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BAL2,
            frame: 32772,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PLSS,
            frame: 32768,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_PLASBALL2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PLSS,
            frame: 32769,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_PLASBALL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PLSE,
            frame: 32768,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_PLASEXP2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PLSE,
            frame: 32769,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_PLASEXP3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PLSE,
            frame: 32770,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_PLASEXP4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PLSE,
            frame: 32771,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_PLASEXP5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PLSE,
            frame: 32772,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_MISL,
            frame: 32768,
            tics: 1,
            action: actionf_t { acp1: None },
            nextstate: S_ROCKET,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BFS1,
            frame: 32768,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_BFGSHOT2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BFS1,
            frame: 32769,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_BFGSHOT,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BFE1,
            frame: 32768,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_BFGLAND2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BFE1,
            frame: 32769,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_BFGLAND3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BFE1,
            frame: 32770,
            tics: 8,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_BFGSpray as *const (),
                    )
                }),
            },
            nextstate: S_BFGLAND4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BFE1,
            frame: 32771,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_BFGLAND5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BFE1,
            frame: 32772,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_BFGLAND6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BFE1,
            frame: 32773,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BFE2,
            frame: 32768,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_BFGEXP2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BFE2,
            frame: 32769,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_BFGEXP3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BFE2,
            frame: 32770,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_BFGEXP4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BFE2,
            frame: 32771,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_MISL,
            frame: 32769,
            tics: 8,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Explode as *const (),
                    )
                }),
            },
            nextstate: S_EXPLODE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_MISL,
            frame: 32770,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_EXPLODE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_MISL,
            frame: 32771,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TFOG,
            frame: 32768,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_TFOG01,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TFOG,
            frame: 32769,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_TFOG02,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TFOG,
            frame: 32768,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_TFOG2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TFOG,
            frame: 32769,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_TFOG3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TFOG,
            frame: 32770,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_TFOG4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TFOG,
            frame: 32771,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_TFOG5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TFOG,
            frame: 32772,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_TFOG6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TFOG,
            frame: 32773,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_TFOG7,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TFOG,
            frame: 32774,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_TFOG8,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TFOG,
            frame: 32775,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_TFOG9,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TFOG,
            frame: 32776,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_TFOG10,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TFOG,
            frame: 32777,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_IFOG,
            frame: 32768,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_IFOG01,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_IFOG,
            frame: 32769,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_IFOG02,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_IFOG,
            frame: 32768,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_IFOG2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_IFOG,
            frame: 32769,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_IFOG3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_IFOG,
            frame: 32770,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_IFOG4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_IFOG,
            frame: 32771,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_IFOG5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_IFOG,
            frame: 32772,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PLAY,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PLAY,
            frame: 0,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_PLAY_RUN2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PLAY,
            frame: 1,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_PLAY_RUN3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PLAY,
            frame: 2,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_PLAY_RUN4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PLAY,
            frame: 3,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_PLAY_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PLAY,
            frame: 4,
            tics: 12,
            action: actionf_t { acp1: None },
            nextstate: S_PLAY,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PLAY,
            frame: 32773,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_PLAY_ATK1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PLAY,
            frame: 6,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_PLAY_PAIN2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PLAY,
            frame: 6,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Pain as *const (),
                    )
                }),
            },
            nextstate: S_PLAY,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PLAY,
            frame: 7,
            tics: 10,
            action: actionf_t { acp1: None },
            nextstate: S_PLAY_DIE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PLAY,
            frame: 8,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_PlayerScream as *const (),
                    )
                }),
            },
            nextstate: S_PLAY_DIE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PLAY,
            frame: 9,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fall as *const (),
                    )
                }),
            },
            nextstate: S_PLAY_DIE4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PLAY,
            frame: 10,
            tics: 10,
            action: actionf_t { acp1: None },
            nextstate: S_PLAY_DIE5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PLAY,
            frame: 11,
            tics: 10,
            action: actionf_t { acp1: None },
            nextstate: S_PLAY_DIE6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PLAY,
            frame: 12,
            tics: 10,
            action: actionf_t { acp1: None },
            nextstate: S_PLAY_DIE7,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PLAY,
            frame: 13,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PLAY,
            frame: 14,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_PLAY_XDIE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PLAY,
            frame: 15,
            tics: 5,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_XScream as *const (),
                    )
                }),
            },
            nextstate: S_PLAY_XDIE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PLAY,
            frame: 16,
            tics: 5,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fall as *const (),
                    )
                }),
            },
            nextstate: S_PLAY_XDIE4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PLAY,
            frame: 17,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_PLAY_XDIE5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PLAY,
            frame: 18,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_PLAY_XDIE6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PLAY,
            frame: 19,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_PLAY_XDIE7,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PLAY,
            frame: 20,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_PLAY_XDIE8,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PLAY,
            frame: 21,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_PLAY_XDIE9,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PLAY,
            frame: 22,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_POSS,
            frame: 0,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Look as *const (),
                    )
                }),
            },
            nextstate: S_POSS_STND2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_POSS,
            frame: 1,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Look as *const (),
                    )
                }),
            },
            nextstate: S_POSS_STND,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_POSS,
            frame: 0,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_POSS_RUN2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_POSS,
            frame: 0,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_POSS_RUN3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_POSS,
            frame: 1,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_POSS_RUN4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_POSS,
            frame: 1,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_POSS_RUN5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_POSS,
            frame: 2,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_POSS_RUN6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_POSS,
            frame: 2,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_POSS_RUN7,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_POSS,
            frame: 3,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_POSS_RUN8,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_POSS,
            frame: 3,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_POSS_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_POSS,
            frame: 4,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FaceTarget as *const (),
                    )
                }),
            },
            nextstate: S_POSS_ATK2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_POSS,
            frame: 5,
            tics: 8,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_PosAttack as *const (),
                    )
                }),
            },
            nextstate: S_POSS_ATK3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_POSS,
            frame: 4,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_POSS_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_POSS,
            frame: 6,
            tics: 3,
            action: actionf_t { acp1: None },
            nextstate: S_POSS_PAIN2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_POSS,
            frame: 6,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Pain as *const (),
                    )
                }),
            },
            nextstate: S_POSS_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_POSS,
            frame: 7,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_POSS_DIE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_POSS,
            frame: 8,
            tics: 5,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Scream as *const (),
                    )
                }),
            },
            nextstate: S_POSS_DIE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_POSS,
            frame: 9,
            tics: 5,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fall as *const (),
                    )
                }),
            },
            nextstate: S_POSS_DIE4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_POSS,
            frame: 10,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_POSS_DIE5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_POSS,
            frame: 11,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_POSS,
            frame: 12,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_POSS_XDIE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_POSS,
            frame: 13,
            tics: 5,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_XScream as *const (),
                    )
                }),
            },
            nextstate: S_POSS_XDIE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_POSS,
            frame: 14,
            tics: 5,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fall as *const (),
                    )
                }),
            },
            nextstate: S_POSS_XDIE4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_POSS,
            frame: 15,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_POSS_XDIE5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_POSS,
            frame: 16,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_POSS_XDIE6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_POSS,
            frame: 17,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_POSS_XDIE7,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_POSS,
            frame: 18,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_POSS_XDIE8,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_POSS,
            frame: 19,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_POSS_XDIE9,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_POSS,
            frame: 20,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_POSS,
            frame: 10,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_POSS_RAISE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_POSS,
            frame: 9,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_POSS_RAISE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_POSS,
            frame: 8,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_POSS_RAISE4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_POSS,
            frame: 7,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_POSS_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPOS,
            frame: 0,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Look as *const (),
                    )
                }),
            },
            nextstate: S_SPOS_STND2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPOS,
            frame: 1,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Look as *const (),
                    )
                }),
            },
            nextstate: S_SPOS_STND,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPOS,
            frame: 0,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SPOS_RUN2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPOS,
            frame: 0,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SPOS_RUN3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPOS,
            frame: 1,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SPOS_RUN4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPOS,
            frame: 1,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SPOS_RUN5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPOS,
            frame: 2,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SPOS_RUN6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPOS,
            frame: 2,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SPOS_RUN7,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPOS,
            frame: 3,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SPOS_RUN8,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPOS,
            frame: 3,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SPOS_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPOS,
            frame: 4,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FaceTarget as *const (),
                    )
                }),
            },
            nextstate: S_SPOS_ATK2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPOS,
            frame: 32773,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_SPosAttack as *const (),
                    )
                }),
            },
            nextstate: S_SPOS_ATK3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPOS,
            frame: 4,
            tics: 10,
            action: actionf_t { acp1: None },
            nextstate: S_SPOS_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPOS,
            frame: 6,
            tics: 3,
            action: actionf_t { acp1: None },
            nextstate: S_SPOS_PAIN2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPOS,
            frame: 6,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Pain as *const (),
                    )
                }),
            },
            nextstate: S_SPOS_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPOS,
            frame: 7,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_SPOS_DIE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPOS,
            frame: 8,
            tics: 5,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Scream as *const (),
                    )
                }),
            },
            nextstate: S_SPOS_DIE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPOS,
            frame: 9,
            tics: 5,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fall as *const (),
                    )
                }),
            },
            nextstate: S_SPOS_DIE4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPOS,
            frame: 10,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_SPOS_DIE5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPOS,
            frame: 11,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPOS,
            frame: 12,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_SPOS_XDIE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPOS,
            frame: 13,
            tics: 5,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_XScream as *const (),
                    )
                }),
            },
            nextstate: S_SPOS_XDIE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPOS,
            frame: 14,
            tics: 5,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fall as *const (),
                    )
                }),
            },
            nextstate: S_SPOS_XDIE4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPOS,
            frame: 15,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_SPOS_XDIE5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPOS,
            frame: 16,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_SPOS_XDIE6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPOS,
            frame: 17,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_SPOS_XDIE7,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPOS,
            frame: 18,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_SPOS_XDIE8,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPOS,
            frame: 19,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_SPOS_XDIE9,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPOS,
            frame: 20,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPOS,
            frame: 11,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_SPOS_RAISE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPOS,
            frame: 10,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_SPOS_RAISE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPOS,
            frame: 9,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_SPOS_RAISE4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPOS,
            frame: 8,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_SPOS_RAISE5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPOS,
            frame: 7,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_SPOS_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_VILE,
            frame: 0,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Look as *const (),
                    )
                }),
            },
            nextstate: S_VILE_STND2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_VILE,
            frame: 1,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Look as *const (),
                    )
                }),
            },
            nextstate: S_VILE_STND,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_VILE,
            frame: 0,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_VileChase as *const (),
                    )
                }),
            },
            nextstate: S_VILE_RUN2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_VILE,
            frame: 0,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_VileChase as *const (),
                    )
                }),
            },
            nextstate: S_VILE_RUN3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_VILE,
            frame: 1,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_VileChase as *const (),
                    )
                }),
            },
            nextstate: S_VILE_RUN4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_VILE,
            frame: 1,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_VileChase as *const (),
                    )
                }),
            },
            nextstate: S_VILE_RUN5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_VILE,
            frame: 2,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_VileChase as *const (),
                    )
                }),
            },
            nextstate: S_VILE_RUN6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_VILE,
            frame: 2,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_VileChase as *const (),
                    )
                }),
            },
            nextstate: S_VILE_RUN7,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_VILE,
            frame: 3,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_VileChase as *const (),
                    )
                }),
            },
            nextstate: S_VILE_RUN8,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_VILE,
            frame: 3,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_VileChase as *const (),
                    )
                }),
            },
            nextstate: S_VILE_RUN9,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_VILE,
            frame: 4,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_VileChase as *const (),
                    )
                }),
            },
            nextstate: S_VILE_RUN10,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_VILE,
            frame: 4,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_VileChase as *const (),
                    )
                }),
            },
            nextstate: S_VILE_RUN11,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_VILE,
            frame: 5,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_VileChase as *const (),
                    )
                }),
            },
            nextstate: S_VILE_RUN12,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_VILE,
            frame: 5,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_VileChase as *const (),
                    )
                }),
            },
            nextstate: S_VILE_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_VILE,
            frame: 32774,
            tics: 0,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_VileStart as *const (),
                    )
                }),
            },
            nextstate: S_VILE_ATK2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_VILE,
            frame: 32774,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FaceTarget as *const (),
                    )
                }),
            },
            nextstate: S_VILE_ATK3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_VILE,
            frame: 32775,
            tics: 8,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_VileTarget as *const (),
                    )
                }),
            },
            nextstate: S_VILE_ATK4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_VILE,
            frame: 32776,
            tics: 8,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FaceTarget as *const (),
                    )
                }),
            },
            nextstate: S_VILE_ATK5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_VILE,
            frame: 32777,
            tics: 8,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FaceTarget as *const (),
                    )
                }),
            },
            nextstate: S_VILE_ATK6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_VILE,
            frame: 32778,
            tics: 8,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FaceTarget as *const (),
                    )
                }),
            },
            nextstate: S_VILE_ATK7,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_VILE,
            frame: 32779,
            tics: 8,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FaceTarget as *const (),
                    )
                }),
            },
            nextstate: S_VILE_ATK8,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_VILE,
            frame: 32780,
            tics: 8,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FaceTarget as *const (),
                    )
                }),
            },
            nextstate: S_VILE_ATK9,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_VILE,
            frame: 32781,
            tics: 8,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FaceTarget as *const (),
                    )
                }),
            },
            nextstate: S_VILE_ATK10,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_VILE,
            frame: 32782,
            tics: 8,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_VileAttack as *const (),
                    )
                }),
            },
            nextstate: S_VILE_ATK11,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_VILE,
            frame: 32783,
            tics: 20,
            action: actionf_t { acp1: None },
            nextstate: S_VILE_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_VILE,
            frame: 32794,
            tics: 10,
            action: actionf_t { acp1: None },
            nextstate: S_VILE_HEAL2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_VILE,
            frame: 32795,
            tics: 10,
            action: actionf_t { acp1: None },
            nextstate: S_VILE_HEAL3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_VILE,
            frame: 32796,
            tics: 10,
            action: actionf_t { acp1: None },
            nextstate: S_VILE_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_VILE,
            frame: 16,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_VILE_PAIN2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_VILE,
            frame: 16,
            tics: 5,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Pain as *const (),
                    )
                }),
            },
            nextstate: S_VILE_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_VILE,
            frame: 16,
            tics: 7,
            action: actionf_t { acp1: None },
            nextstate: S_VILE_DIE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_VILE,
            frame: 17,
            tics: 7,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Scream as *const (),
                    )
                }),
            },
            nextstate: S_VILE_DIE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_VILE,
            frame: 18,
            tics: 7,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fall as *const (),
                    )
                }),
            },
            nextstate: S_VILE_DIE4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_VILE,
            frame: 19,
            tics: 7,
            action: actionf_t { acp1: None },
            nextstate: S_VILE_DIE5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_VILE,
            frame: 20,
            tics: 7,
            action: actionf_t { acp1: None },
            nextstate: S_VILE_DIE6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_VILE,
            frame: 21,
            tics: 7,
            action: actionf_t { acp1: None },
            nextstate: S_VILE_DIE7,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_VILE,
            frame: 22,
            tics: 7,
            action: actionf_t { acp1: None },
            nextstate: S_VILE_DIE8,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_VILE,
            frame: 23,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_VILE_DIE9,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_VILE,
            frame: 24,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_VILE_DIE10,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_VILE,
            frame: 25,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FIRE,
            frame: 32768,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_StartFire as *const (),
                    )
                }),
            },
            nextstate: S_FIRE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FIRE,
            frame: 32769,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fire as *const (),
                    )
                }),
            },
            nextstate: S_FIRE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FIRE,
            frame: 32768,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fire as *const (),
                    )
                }),
            },
            nextstate: S_FIRE4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FIRE,
            frame: 32769,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fire as *const (),
                    )
                }),
            },
            nextstate: S_FIRE5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FIRE,
            frame: 32770,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FireCrackle as *const (),
                    )
                }),
            },
            nextstate: S_FIRE6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FIRE,
            frame: 32769,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fire as *const (),
                    )
                }),
            },
            nextstate: S_FIRE7,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FIRE,
            frame: 32770,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fire as *const (),
                    )
                }),
            },
            nextstate: S_FIRE8,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FIRE,
            frame: 32769,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fire as *const (),
                    )
                }),
            },
            nextstate: S_FIRE9,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FIRE,
            frame: 32770,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fire as *const (),
                    )
                }),
            },
            nextstate: S_FIRE10,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FIRE,
            frame: 32771,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fire as *const (),
                    )
                }),
            },
            nextstate: S_FIRE11,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FIRE,
            frame: 32770,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fire as *const (),
                    )
                }),
            },
            nextstate: S_FIRE12,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FIRE,
            frame: 32771,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fire as *const (),
                    )
                }),
            },
            nextstate: S_FIRE13,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FIRE,
            frame: 32770,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fire as *const (),
                    )
                }),
            },
            nextstate: S_FIRE14,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FIRE,
            frame: 32771,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fire as *const (),
                    )
                }),
            },
            nextstate: S_FIRE15,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FIRE,
            frame: 32772,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fire as *const (),
                    )
                }),
            },
            nextstate: S_FIRE16,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FIRE,
            frame: 32771,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fire as *const (),
                    )
                }),
            },
            nextstate: S_FIRE17,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FIRE,
            frame: 32772,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fire as *const (),
                    )
                }),
            },
            nextstate: S_FIRE18,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FIRE,
            frame: 32771,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fire as *const (),
                    )
                }),
            },
            nextstate: S_FIRE19,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FIRE,
            frame: 32772,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FireCrackle as *const (),
                    )
                }),
            },
            nextstate: S_FIRE20,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FIRE,
            frame: 32773,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fire as *const (),
                    )
                }),
            },
            nextstate: S_FIRE21,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FIRE,
            frame: 32772,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fire as *const (),
                    )
                }),
            },
            nextstate: S_FIRE22,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FIRE,
            frame: 32773,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fire as *const (),
                    )
                }),
            },
            nextstate: S_FIRE23,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FIRE,
            frame: 32772,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fire as *const (),
                    )
                }),
            },
            nextstate: S_FIRE24,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FIRE,
            frame: 32773,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fire as *const (),
                    )
                }),
            },
            nextstate: S_FIRE25,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FIRE,
            frame: 32774,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fire as *const (),
                    )
                }),
            },
            nextstate: S_FIRE26,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FIRE,
            frame: 32775,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fire as *const (),
                    )
                }),
            },
            nextstate: S_FIRE27,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FIRE,
            frame: 32774,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fire as *const (),
                    )
                }),
            },
            nextstate: S_FIRE28,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FIRE,
            frame: 32775,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fire as *const (),
                    )
                }),
            },
            nextstate: S_FIRE29,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FIRE,
            frame: 32774,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fire as *const (),
                    )
                }),
            },
            nextstate: S_FIRE30,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FIRE,
            frame: 32775,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fire as *const (),
                    )
                }),
            },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PUFF,
            frame: 1,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_SMOKE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PUFF,
            frame: 2,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_SMOKE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PUFF,
            frame: 1,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_SMOKE4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PUFF,
            frame: 2,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_SMOKE5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PUFF,
            frame: 3,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATB,
            frame: 32768,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Tracer as *const (),
                    )
                }),
            },
            nextstate: S_TRACER2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATB,
            frame: 32769,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Tracer as *const (),
                    )
                }),
            },
            nextstate: S_TRACER,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FBXP,
            frame: 32768,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_TRACEEXP2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FBXP,
            frame: 32769,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_TRACEEXP3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FBXP,
            frame: 32770,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKEL,
            frame: 0,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Look as *const (),
                    )
                }),
            },
            nextstate: S_SKEL_STND2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKEL,
            frame: 1,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Look as *const (),
                    )
                }),
            },
            nextstate: S_SKEL_STND,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKEL,
            frame: 0,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SKEL_RUN2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKEL,
            frame: 0,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SKEL_RUN3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKEL,
            frame: 1,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SKEL_RUN4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKEL,
            frame: 1,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SKEL_RUN5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKEL,
            frame: 2,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SKEL_RUN6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKEL,
            frame: 2,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SKEL_RUN7,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKEL,
            frame: 3,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SKEL_RUN8,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKEL,
            frame: 3,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SKEL_RUN9,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKEL,
            frame: 4,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SKEL_RUN10,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKEL,
            frame: 4,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SKEL_RUN11,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKEL,
            frame: 5,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SKEL_RUN12,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKEL,
            frame: 5,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SKEL_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKEL,
            frame: 6,
            tics: 0,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FaceTarget as *const (),
                    )
                }),
            },
            nextstate: S_SKEL_FIST2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKEL,
            frame: 6,
            tics: 6,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_SkelWhoosh as *const (),
                    )
                }),
            },
            nextstate: S_SKEL_FIST3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKEL,
            frame: 7,
            tics: 6,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FaceTarget as *const (),
                    )
                }),
            },
            nextstate: S_SKEL_FIST4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKEL,
            frame: 8,
            tics: 6,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_SkelFist as *const (),
                    )
                }),
            },
            nextstate: S_SKEL_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKEL,
            frame: 32777,
            tics: 0,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FaceTarget as *const (),
                    )
                }),
            },
            nextstate: S_SKEL_MISS2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKEL,
            frame: 32777,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FaceTarget as *const (),
                    )
                }),
            },
            nextstate: S_SKEL_MISS3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKEL,
            frame: 10,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_SkelMissile as *const (),
                    )
                }),
            },
            nextstate: S_SKEL_MISS4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKEL,
            frame: 10,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FaceTarget as *const (),
                    )
                }),
            },
            nextstate: S_SKEL_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKEL,
            frame: 11,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_SKEL_PAIN2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKEL,
            frame: 11,
            tics: 5,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Pain as *const (),
                    )
                }),
            },
            nextstate: S_SKEL_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKEL,
            frame: 11,
            tics: 7,
            action: actionf_t { acp1: None },
            nextstate: S_SKEL_DIE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKEL,
            frame: 12,
            tics: 7,
            action: actionf_t { acp1: None },
            nextstate: S_SKEL_DIE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKEL,
            frame: 13,
            tics: 7,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Scream as *const (),
                    )
                }),
            },
            nextstate: S_SKEL_DIE4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKEL,
            frame: 14,
            tics: 7,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fall as *const (),
                    )
                }),
            },
            nextstate: S_SKEL_DIE5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKEL,
            frame: 15,
            tics: 7,
            action: actionf_t { acp1: None },
            nextstate: S_SKEL_DIE6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKEL,
            frame: 16,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKEL,
            frame: 16,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_SKEL_RAISE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKEL,
            frame: 15,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_SKEL_RAISE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKEL,
            frame: 14,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_SKEL_RAISE4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKEL,
            frame: 13,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_SKEL_RAISE5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKEL,
            frame: 12,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_SKEL_RAISE6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKEL,
            frame: 11,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_SKEL_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_MANF,
            frame: 32768,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_FATSHOT2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_MANF,
            frame: 32769,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_FATSHOT1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_MISL,
            frame: 32769,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_FATSHOTX2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_MISL,
            frame: 32770,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_FATSHOTX3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_MISL,
            frame: 32771,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATT,
            frame: 0,
            tics: 15,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Look as *const (),
                    )
                }),
            },
            nextstate: S_FATT_STND2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATT,
            frame: 1,
            tics: 15,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Look as *const (),
                    )
                }),
            },
            nextstate: S_FATT_STND,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATT,
            frame: 0,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_FATT_RUN2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATT,
            frame: 0,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_FATT_RUN3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATT,
            frame: 1,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_FATT_RUN4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATT,
            frame: 1,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_FATT_RUN5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATT,
            frame: 2,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_FATT_RUN6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATT,
            frame: 2,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_FATT_RUN7,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATT,
            frame: 3,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_FATT_RUN8,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATT,
            frame: 3,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_FATT_RUN9,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATT,
            frame: 4,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_FATT_RUN10,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATT,
            frame: 4,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_FATT_RUN11,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATT,
            frame: 5,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_FATT_RUN12,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATT,
            frame: 5,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_FATT_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATT,
            frame: 6,
            tics: 20,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FatRaise as *const (),
                    )
                }),
            },
            nextstate: S_FATT_ATK2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATT,
            frame: 32775,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FatAttack1 as *const (),
                    )
                }),
            },
            nextstate: S_FATT_ATK3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATT,
            frame: 8,
            tics: 5,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FaceTarget as *const (),
                    )
                }),
            },
            nextstate: S_FATT_ATK4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATT,
            frame: 6,
            tics: 5,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FaceTarget as *const (),
                    )
                }),
            },
            nextstate: S_FATT_ATK5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATT,
            frame: 32775,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FatAttack2 as *const (),
                    )
                }),
            },
            nextstate: S_FATT_ATK6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATT,
            frame: 8,
            tics: 5,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FaceTarget as *const (),
                    )
                }),
            },
            nextstate: S_FATT_ATK7,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATT,
            frame: 6,
            tics: 5,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FaceTarget as *const (),
                    )
                }),
            },
            nextstate: S_FATT_ATK8,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATT,
            frame: 32775,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FatAttack3 as *const (),
                    )
                }),
            },
            nextstate: S_FATT_ATK9,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATT,
            frame: 8,
            tics: 5,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FaceTarget as *const (),
                    )
                }),
            },
            nextstate: S_FATT_ATK10,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATT,
            frame: 6,
            tics: 5,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FaceTarget as *const (),
                    )
                }),
            },
            nextstate: S_FATT_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATT,
            frame: 9,
            tics: 3,
            action: actionf_t { acp1: None },
            nextstate: S_FATT_PAIN2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATT,
            frame: 9,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Pain as *const (),
                    )
                }),
            },
            nextstate: S_FATT_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATT,
            frame: 10,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_FATT_DIE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATT,
            frame: 11,
            tics: 6,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Scream as *const (),
                    )
                }),
            },
            nextstate: S_FATT_DIE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATT,
            frame: 12,
            tics: 6,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fall as *const (),
                    )
                }),
            },
            nextstate: S_FATT_DIE4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATT,
            frame: 13,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_FATT_DIE5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATT,
            frame: 14,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_FATT_DIE6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATT,
            frame: 15,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_FATT_DIE7,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATT,
            frame: 16,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_FATT_DIE8,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATT,
            frame: 17,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_FATT_DIE9,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATT,
            frame: 18,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_FATT_DIE10,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATT,
            frame: 19,
            tics: (-(1)),
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_BossDeath as *const (),
                    )
                }),
            },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATT,
            frame: 17,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_FATT_RAISE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATT,
            frame: 16,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_FATT_RAISE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATT,
            frame: 15,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_FATT_RAISE4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATT,
            frame: 14,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_FATT_RAISE5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATT,
            frame: 13,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_FATT_RAISE6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATT,
            frame: 12,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_FATT_RAISE7,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATT,
            frame: 11,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_FATT_RAISE8,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FATT,
            frame: 10,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_FATT_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CPOS,
            frame: 0,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Look as *const (),
                    )
                }),
            },
            nextstate: S_CPOS_STND2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CPOS,
            frame: 1,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Look as *const (),
                    )
                }),
            },
            nextstate: S_CPOS_STND,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CPOS,
            frame: 0,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_CPOS_RUN2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CPOS,
            frame: 0,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_CPOS_RUN3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CPOS,
            frame: 1,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_CPOS_RUN4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CPOS,
            frame: 1,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_CPOS_RUN5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CPOS,
            frame: 2,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_CPOS_RUN6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CPOS,
            frame: 2,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_CPOS_RUN7,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CPOS,
            frame: 3,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_CPOS_RUN8,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CPOS,
            frame: 3,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_CPOS_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CPOS,
            frame: 4,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FaceTarget as *const (),
                    )
                }),
            },
            nextstate: S_CPOS_ATK2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CPOS,
            frame: 32773,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_CPosAttack as *const (),
                    )
                }),
            },
            nextstate: S_CPOS_ATK3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CPOS,
            frame: 32772,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_CPosAttack as *const (),
                    )
                }),
            },
            nextstate: S_CPOS_ATK4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CPOS,
            frame: 5,
            tics: 1,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_CPosRefire as *const (),
                    )
                }),
            },
            nextstate: S_CPOS_ATK2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CPOS,
            frame: 6,
            tics: 3,
            action: actionf_t { acp1: None },
            nextstate: S_CPOS_PAIN2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CPOS,
            frame: 6,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Pain as *const (),
                    )
                }),
            },
            nextstate: S_CPOS_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CPOS,
            frame: 7,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_CPOS_DIE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CPOS,
            frame: 8,
            tics: 5,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Scream as *const (),
                    )
                }),
            },
            nextstate: S_CPOS_DIE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CPOS,
            frame: 9,
            tics: 5,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fall as *const (),
                    )
                }),
            },
            nextstate: S_CPOS_DIE4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CPOS,
            frame: 10,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_CPOS_DIE5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CPOS,
            frame: 11,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_CPOS_DIE6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CPOS,
            frame: 12,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_CPOS_DIE7,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CPOS,
            frame: 13,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CPOS,
            frame: 14,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_CPOS_XDIE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CPOS,
            frame: 15,
            tics: 5,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_XScream as *const (),
                    )
                }),
            },
            nextstate: S_CPOS_XDIE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CPOS,
            frame: 16,
            tics: 5,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fall as *const (),
                    )
                }),
            },
            nextstate: S_CPOS_XDIE4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CPOS,
            frame: 17,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_CPOS_XDIE5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CPOS,
            frame: 18,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_CPOS_XDIE6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CPOS,
            frame: 19,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CPOS,
            frame: 13,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_CPOS_RAISE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CPOS,
            frame: 12,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_CPOS_RAISE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CPOS,
            frame: 11,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_CPOS_RAISE4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CPOS,
            frame: 10,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_CPOS_RAISE5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CPOS,
            frame: 9,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_CPOS_RAISE6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CPOS,
            frame: 8,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_CPOS_RAISE7,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CPOS,
            frame: 7,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_CPOS_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TROO,
            frame: 0,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Look as *const (),
                    )
                }),
            },
            nextstate: S_TROO_STND2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TROO,
            frame: 1,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Look as *const (),
                    )
                }),
            },
            nextstate: S_TROO_STND,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TROO,
            frame: 0,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_TROO_RUN2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TROO,
            frame: 0,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_TROO_RUN3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TROO,
            frame: 1,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_TROO_RUN4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TROO,
            frame: 1,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_TROO_RUN5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TROO,
            frame: 2,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_TROO_RUN6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TROO,
            frame: 2,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_TROO_RUN7,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TROO,
            frame: 3,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_TROO_RUN8,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TROO,
            frame: 3,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_TROO_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TROO,
            frame: 4,
            tics: 8,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FaceTarget as *const (),
                    )
                }),
            },
            nextstate: S_TROO_ATK2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TROO,
            frame: 5,
            tics: 8,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FaceTarget as *const (),
                    )
                }),
            },
            nextstate: S_TROO_ATK3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TROO,
            frame: 6,
            tics: 6,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_TroopAttack as *const (),
                    )
                }),
            },
            nextstate: S_TROO_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TROO,
            frame: 7,
            tics: 2,
            action: actionf_t { acp1: None },
            nextstate: S_TROO_PAIN2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TROO,
            frame: 7,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Pain as *const (),
                    )
                }),
            },
            nextstate: S_TROO_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TROO,
            frame: 8,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_TROO_DIE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TROO,
            frame: 9,
            tics: 8,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Scream as *const (),
                    )
                }),
            },
            nextstate: S_TROO_DIE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TROO,
            frame: 10,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_TROO_DIE4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TROO,
            frame: 11,
            tics: 6,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fall as *const (),
                    )
                }),
            },
            nextstate: S_TROO_DIE5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TROO,
            frame: 12,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TROO,
            frame: 13,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_TROO_XDIE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TROO,
            frame: 14,
            tics: 5,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_XScream as *const (),
                    )
                }),
            },
            nextstate: S_TROO_XDIE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TROO,
            frame: 15,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_TROO_XDIE4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TROO,
            frame: 16,
            tics: 5,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fall as *const (),
                    )
                }),
            },
            nextstate: S_TROO_XDIE5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TROO,
            frame: 17,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_TROO_XDIE6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TROO,
            frame: 18,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_TROO_XDIE7,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TROO,
            frame: 19,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_TROO_XDIE8,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TROO,
            frame: 20,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TROO,
            frame: 12,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_TROO_RAISE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TROO,
            frame: 11,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_TROO_RAISE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TROO,
            frame: 10,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_TROO_RAISE4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TROO,
            frame: 9,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_TROO_RAISE5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TROO,
            frame: 8,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_TROO_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SARG,
            frame: 0,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Look as *const (),
                    )
                }),
            },
            nextstate: S_SARG_STND2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SARG,
            frame: 1,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Look as *const (),
                    )
                }),
            },
            nextstate: S_SARG_STND,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SARG,
            frame: 0,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SARG_RUN2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SARG,
            frame: 0,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SARG_RUN3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SARG,
            frame: 1,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SARG_RUN4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SARG,
            frame: 1,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SARG_RUN5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SARG,
            frame: 2,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SARG_RUN6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SARG,
            frame: 2,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SARG_RUN7,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SARG,
            frame: 3,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SARG_RUN8,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SARG,
            frame: 3,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SARG_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SARG,
            frame: 4,
            tics: 8,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FaceTarget as *const (),
                    )
                }),
            },
            nextstate: S_SARG_ATK2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SARG,
            frame: 5,
            tics: 8,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FaceTarget as *const (),
                    )
                }),
            },
            nextstate: S_SARG_ATK3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SARG,
            frame: 6,
            tics: 8,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_SargAttack as *const (),
                    )
                }),
            },
            nextstate: S_SARG_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SARG,
            frame: 7,
            tics: 2,
            action: actionf_t { acp1: None },
            nextstate: S_SARG_PAIN2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SARG,
            frame: 7,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Pain as *const (),
                    )
                }),
            },
            nextstate: S_SARG_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SARG,
            frame: 8,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_SARG_DIE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SARG,
            frame: 9,
            tics: 8,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Scream as *const (),
                    )
                }),
            },
            nextstate: S_SARG_DIE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SARG,
            frame: 10,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_SARG_DIE4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SARG,
            frame: 11,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fall as *const (),
                    )
                }),
            },
            nextstate: S_SARG_DIE5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SARG,
            frame: 12,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_SARG_DIE6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SARG,
            frame: 13,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SARG,
            frame: 13,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_SARG_RAISE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SARG,
            frame: 12,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_SARG_RAISE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SARG,
            frame: 11,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_SARG_RAISE4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SARG,
            frame: 10,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_SARG_RAISE5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SARG,
            frame: 9,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_SARG_RAISE6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SARG,
            frame: 8,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_SARG_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_HEAD,
            frame: 0,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Look as *const (),
                    )
                }),
            },
            nextstate: S_HEAD_STND,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_HEAD,
            frame: 0,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_HEAD_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_HEAD,
            frame: 1,
            tics: 5,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FaceTarget as *const (),
                    )
                }),
            },
            nextstate: S_HEAD_ATK2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_HEAD,
            frame: 2,
            tics: 5,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FaceTarget as *const (),
                    )
                }),
            },
            nextstate: S_HEAD_ATK3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_HEAD,
            frame: 32771,
            tics: 5,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_HeadAttack as *const (),
                    )
                }),
            },
            nextstate: S_HEAD_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_HEAD,
            frame: 4,
            tics: 3,
            action: actionf_t { acp1: None },
            nextstate: S_HEAD_PAIN2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_HEAD,
            frame: 4,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Pain as *const (),
                    )
                }),
            },
            nextstate: S_HEAD_PAIN3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_HEAD,
            frame: 5,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_HEAD_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_HEAD,
            frame: 6,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_HEAD_DIE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_HEAD,
            frame: 7,
            tics: 8,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Scream as *const (),
                    )
                }),
            },
            nextstate: S_HEAD_DIE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_HEAD,
            frame: 8,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_HEAD_DIE4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_HEAD,
            frame: 9,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_HEAD_DIE5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_HEAD,
            frame: 10,
            tics: 8,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fall as *const (),
                    )
                }),
            },
            nextstate: S_HEAD_DIE6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_HEAD,
            frame: 11,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_HEAD,
            frame: 11,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_HEAD_RAISE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_HEAD,
            frame: 10,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_HEAD_RAISE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_HEAD,
            frame: 9,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_HEAD_RAISE4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_HEAD,
            frame: 8,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_HEAD_RAISE5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_HEAD,
            frame: 7,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_HEAD_RAISE6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_HEAD,
            frame: 6,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_HEAD_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BAL7,
            frame: 32768,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_BRBALL2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BAL7,
            frame: 32769,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_BRBALL1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BAL7,
            frame: 32770,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_BRBALLX2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BAL7,
            frame: 32771,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_BRBALLX3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BAL7,
            frame: 32772,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOSS,
            frame: 0,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Look as *const (),
                    )
                }),
            },
            nextstate: S_BOSS_STND2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOSS,
            frame: 1,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Look as *const (),
                    )
                }),
            },
            nextstate: S_BOSS_STND,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOSS,
            frame: 0,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_BOSS_RUN2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOSS,
            frame: 0,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_BOSS_RUN3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOSS,
            frame: 1,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_BOSS_RUN4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOSS,
            frame: 1,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_BOSS_RUN5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOSS,
            frame: 2,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_BOSS_RUN6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOSS,
            frame: 2,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_BOSS_RUN7,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOSS,
            frame: 3,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_BOSS_RUN8,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOSS,
            frame: 3,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_BOSS_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOSS,
            frame: 4,
            tics: 8,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FaceTarget as *const (),
                    )
                }),
            },
            nextstate: S_BOSS_ATK2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOSS,
            frame: 5,
            tics: 8,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FaceTarget as *const (),
                    )
                }),
            },
            nextstate: S_BOSS_ATK3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOSS,
            frame: 6,
            tics: 8,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_BruisAttack as *const (),
                    )
                }),
            },
            nextstate: S_BOSS_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOSS,
            frame: 7,
            tics: 2,
            action: actionf_t { acp1: None },
            nextstate: S_BOSS_PAIN2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOSS,
            frame: 7,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Pain as *const (),
                    )
                }),
            },
            nextstate: S_BOSS_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOSS,
            frame: 8,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_BOSS_DIE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOSS,
            frame: 9,
            tics: 8,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Scream as *const (),
                    )
                }),
            },
            nextstate: S_BOSS_DIE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOSS,
            frame: 10,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_BOSS_DIE4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOSS,
            frame: 11,
            tics: 8,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fall as *const (),
                    )
                }),
            },
            nextstate: S_BOSS_DIE5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOSS,
            frame: 12,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_BOSS_DIE6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOSS,
            frame: 13,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_BOSS_DIE7,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOSS,
            frame: 14,
            tics: (-(1)),
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_BossDeath as *const (),
                    )
                }),
            },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOSS,
            frame: 14,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_BOSS_RAISE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOSS,
            frame: 13,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_BOSS_RAISE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOSS,
            frame: 12,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_BOSS_RAISE4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOSS,
            frame: 11,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_BOSS_RAISE5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOSS,
            frame: 10,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_BOSS_RAISE6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOSS,
            frame: 9,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_BOSS_RAISE7,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOSS,
            frame: 8,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_BOSS_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOS2,
            frame: 0,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Look as *const (),
                    )
                }),
            },
            nextstate: S_BOS2_STND2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOS2,
            frame: 1,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Look as *const (),
                    )
                }),
            },
            nextstate: S_BOS2_STND,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOS2,
            frame: 0,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_BOS2_RUN2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOS2,
            frame: 0,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_BOS2_RUN3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOS2,
            frame: 1,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_BOS2_RUN4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOS2,
            frame: 1,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_BOS2_RUN5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOS2,
            frame: 2,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_BOS2_RUN6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOS2,
            frame: 2,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_BOS2_RUN7,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOS2,
            frame: 3,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_BOS2_RUN8,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOS2,
            frame: 3,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_BOS2_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOS2,
            frame: 4,
            tics: 8,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FaceTarget as *const (),
                    )
                }),
            },
            nextstate: S_BOS2_ATK2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOS2,
            frame: 5,
            tics: 8,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FaceTarget as *const (),
                    )
                }),
            },
            nextstate: S_BOS2_ATK3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOS2,
            frame: 6,
            tics: 8,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_BruisAttack as *const (),
                    )
                }),
            },
            nextstate: S_BOS2_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOS2,
            frame: 7,
            tics: 2,
            action: actionf_t { acp1: None },
            nextstate: S_BOS2_PAIN2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOS2,
            frame: 7,
            tics: 2,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Pain as *const (),
                    )
                }),
            },
            nextstate: S_BOS2_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOS2,
            frame: 8,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_BOS2_DIE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOS2,
            frame: 9,
            tics: 8,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Scream as *const (),
                    )
                }),
            },
            nextstate: S_BOS2_DIE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOS2,
            frame: 10,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_BOS2_DIE4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOS2,
            frame: 11,
            tics: 8,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fall as *const (),
                    )
                }),
            },
            nextstate: S_BOS2_DIE5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOS2,
            frame: 12,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_BOS2_DIE6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOS2,
            frame: 13,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_BOS2_DIE7,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOS2,
            frame: 14,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOS2,
            frame: 14,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_BOS2_RAISE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOS2,
            frame: 13,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_BOS2_RAISE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOS2,
            frame: 12,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_BOS2_RAISE4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOS2,
            frame: 11,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_BOS2_RAISE5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOS2,
            frame: 10,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_BOS2_RAISE6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOS2,
            frame: 9,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_BOS2_RAISE7,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOS2,
            frame: 8,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_BOS2_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKUL,
            frame: 32768,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Look as *const (),
                    )
                }),
            },
            nextstate: S_SKULL_STND2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKUL,
            frame: 32769,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Look as *const (),
                    )
                }),
            },
            nextstate: S_SKULL_STND,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKUL,
            frame: 32768,
            tics: 6,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SKULL_RUN2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKUL,
            frame: 32769,
            tics: 6,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SKULL_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKUL,
            frame: 32770,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FaceTarget as *const (),
                    )
                }),
            },
            nextstate: S_SKULL_ATK2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKUL,
            frame: 32771,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_SkullAttack as *const (),
                    )
                }),
            },
            nextstate: S_SKULL_ATK3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKUL,
            frame: 32770,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_SKULL_ATK4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKUL,
            frame: 32771,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_SKULL_ATK3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKUL,
            frame: 32772,
            tics: 3,
            action: actionf_t { acp1: None },
            nextstate: S_SKULL_PAIN2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKUL,
            frame: 32772,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Pain as *const (),
                    )
                }),
            },
            nextstate: S_SKULL_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKUL,
            frame: 32773,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_SKULL_DIE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKUL,
            frame: 32774,
            tics: 6,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Scream as *const (),
                    )
                }),
            },
            nextstate: S_SKULL_DIE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKUL,
            frame: 32775,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_SKULL_DIE4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKUL,
            frame: 32776,
            tics: 6,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fall as *const (),
                    )
                }),
            },
            nextstate: S_SKULL_DIE5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKUL,
            frame: 9,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_SKULL_DIE6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SKUL,
            frame: 10,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPID,
            frame: 0,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Look as *const (),
                    )
                }),
            },
            nextstate: S_SPID_STND2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPID,
            frame: 1,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Look as *const (),
                    )
                }),
            },
            nextstate: S_SPID_STND,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPID,
            frame: 0,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Metal as *const (),
                    )
                }),
            },
            nextstate: S_SPID_RUN2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPID,
            frame: 0,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SPID_RUN3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPID,
            frame: 1,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SPID_RUN4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPID,
            frame: 1,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SPID_RUN5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPID,
            frame: 2,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Metal as *const (),
                    )
                }),
            },
            nextstate: S_SPID_RUN6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPID,
            frame: 2,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SPID_RUN7,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPID,
            frame: 3,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SPID_RUN8,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPID,
            frame: 3,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SPID_RUN9,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPID,
            frame: 4,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Metal as *const (),
                    )
                }),
            },
            nextstate: S_SPID_RUN10,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPID,
            frame: 4,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SPID_RUN11,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPID,
            frame: 5,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SPID_RUN12,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPID,
            frame: 5,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SPID_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPID,
            frame: 32768,
            tics: 20,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FaceTarget as *const (),
                    )
                }),
            },
            nextstate: S_SPID_ATK2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPID,
            frame: 32774,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_SPosAttack as *const (),
                    )
                }),
            },
            nextstate: S_SPID_ATK3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPID,
            frame: 32775,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_SPosAttack as *const (),
                    )
                }),
            },
            nextstate: S_SPID_ATK4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPID,
            frame: 32775,
            tics: 1,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_SpidRefire as *const (),
                    )
                }),
            },
            nextstate: S_SPID_ATK2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPID,
            frame: 8,
            tics: 3,
            action: actionf_t { acp1: None },
            nextstate: S_SPID_PAIN2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPID,
            frame: 8,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Pain as *const (),
                    )
                }),
            },
            nextstate: S_SPID_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPID,
            frame: 9,
            tics: 20,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Scream as *const (),
                    )
                }),
            },
            nextstate: S_SPID_DIE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPID,
            frame: 10,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fall as *const (),
                    )
                }),
            },
            nextstate: S_SPID_DIE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPID,
            frame: 11,
            tics: 10,
            action: actionf_t { acp1: None },
            nextstate: S_SPID_DIE4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPID,
            frame: 12,
            tics: 10,
            action: actionf_t { acp1: None },
            nextstate: S_SPID_DIE5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPID,
            frame: 13,
            tics: 10,
            action: actionf_t { acp1: None },
            nextstate: S_SPID_DIE6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPID,
            frame: 14,
            tics: 10,
            action: actionf_t { acp1: None },
            nextstate: S_SPID_DIE7,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPID,
            frame: 15,
            tics: 10,
            action: actionf_t { acp1: None },
            nextstate: S_SPID_DIE8,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPID,
            frame: 16,
            tics: 10,
            action: actionf_t { acp1: None },
            nextstate: S_SPID_DIE9,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPID,
            frame: 17,
            tics: 10,
            action: actionf_t { acp1: None },
            nextstate: S_SPID_DIE10,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPID,
            frame: 18,
            tics: 30,
            action: actionf_t { acp1: None },
            nextstate: S_SPID_DIE11,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SPID,
            frame: 18,
            tics: (-(1)),
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_BossDeath as *const (),
                    )
                }),
            },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BSPI,
            frame: 0,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Look as *const (),
                    )
                }),
            },
            nextstate: S_BSPI_STND2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BSPI,
            frame: 1,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Look as *const (),
                    )
                }),
            },
            nextstate: S_BSPI_STND,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BSPI,
            frame: 0,
            tics: 20,
            action: actionf_t { acp1: None },
            nextstate: S_BSPI_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BSPI,
            frame: 0,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_BabyMetal as *const (),
                    )
                }),
            },
            nextstate: S_BSPI_RUN2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BSPI,
            frame: 0,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_BSPI_RUN3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BSPI,
            frame: 1,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_BSPI_RUN4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BSPI,
            frame: 1,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_BSPI_RUN5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BSPI,
            frame: 2,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_BSPI_RUN6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BSPI,
            frame: 2,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_BSPI_RUN7,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BSPI,
            frame: 3,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_BabyMetal as *const (),
                    )
                }),
            },
            nextstate: S_BSPI_RUN8,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BSPI,
            frame: 3,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_BSPI_RUN9,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BSPI,
            frame: 4,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_BSPI_RUN10,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BSPI,
            frame: 4,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_BSPI_RUN11,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BSPI,
            frame: 5,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_BSPI_RUN12,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BSPI,
            frame: 5,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_BSPI_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BSPI,
            frame: 32768,
            tics: 20,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FaceTarget as *const (),
                    )
                }),
            },
            nextstate: S_BSPI_ATK2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BSPI,
            frame: 32774,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_BspiAttack as *const (),
                    )
                }),
            },
            nextstate: S_BSPI_ATK3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BSPI,
            frame: 32775,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_BSPI_ATK4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BSPI,
            frame: 32775,
            tics: 1,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_SpidRefire as *const (),
                    )
                }),
            },
            nextstate: S_BSPI_ATK2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BSPI,
            frame: 8,
            tics: 3,
            action: actionf_t { acp1: None },
            nextstate: S_BSPI_PAIN2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BSPI,
            frame: 8,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Pain as *const (),
                    )
                }),
            },
            nextstate: S_BSPI_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BSPI,
            frame: 9,
            tics: 20,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Scream as *const (),
                    )
                }),
            },
            nextstate: S_BSPI_DIE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BSPI,
            frame: 10,
            tics: 7,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fall as *const (),
                    )
                }),
            },
            nextstate: S_BSPI_DIE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BSPI,
            frame: 11,
            tics: 7,
            action: actionf_t { acp1: None },
            nextstate: S_BSPI_DIE4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BSPI,
            frame: 12,
            tics: 7,
            action: actionf_t { acp1: None },
            nextstate: S_BSPI_DIE5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BSPI,
            frame: 13,
            tics: 7,
            action: actionf_t { acp1: None },
            nextstate: S_BSPI_DIE6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BSPI,
            frame: 14,
            tics: 7,
            action: actionf_t { acp1: None },
            nextstate: S_BSPI_DIE7,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BSPI,
            frame: 15,
            tics: (-(1)),
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_BossDeath as *const (),
                    )
                }),
            },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BSPI,
            frame: 15,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_BSPI_RAISE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BSPI,
            frame: 14,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_BSPI_RAISE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BSPI,
            frame: 13,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_BSPI_RAISE4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BSPI,
            frame: 12,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_BSPI_RAISE5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BSPI,
            frame: 11,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_BSPI_RAISE6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BSPI,
            frame: 10,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_BSPI_RAISE7,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BSPI,
            frame: 9,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_BSPI_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_APLS,
            frame: 32768,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_ARACH_PLAZ2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_APLS,
            frame: 32769,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_ARACH_PLAZ,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_APBX,
            frame: 32768,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_ARACH_PLEX2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_APBX,
            frame: 32769,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_ARACH_PLEX3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_APBX,
            frame: 32770,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_ARACH_PLEX4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_APBX,
            frame: 32771,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_ARACH_PLEX5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_APBX,
            frame: 32772,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CYBR,
            frame: 0,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Look as *const (),
                    )
                }),
            },
            nextstate: S_CYBER_STND2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CYBR,
            frame: 1,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Look as *const (),
                    )
                }),
            },
            nextstate: S_CYBER_STND,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CYBR,
            frame: 0,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Hoof as *const (),
                    )
                }),
            },
            nextstate: S_CYBER_RUN2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CYBR,
            frame: 0,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_CYBER_RUN3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CYBR,
            frame: 1,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_CYBER_RUN4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CYBR,
            frame: 1,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_CYBER_RUN5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CYBR,
            frame: 2,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_CYBER_RUN6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CYBR,
            frame: 2,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_CYBER_RUN7,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CYBR,
            frame: 3,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Metal as *const (),
                    )
                }),
            },
            nextstate: S_CYBER_RUN8,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CYBR,
            frame: 3,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_CYBER_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CYBR,
            frame: 4,
            tics: 6,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FaceTarget as *const (),
                    )
                }),
            },
            nextstate: S_CYBER_ATK2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CYBR,
            frame: 5,
            tics: 12,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_CyberAttack as *const (),
                    )
                }),
            },
            nextstate: S_CYBER_ATK3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CYBR,
            frame: 4,
            tics: 12,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FaceTarget as *const (),
                    )
                }),
            },
            nextstate: S_CYBER_ATK4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CYBR,
            frame: 5,
            tics: 12,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_CyberAttack as *const (),
                    )
                }),
            },
            nextstate: S_CYBER_ATK5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CYBR,
            frame: 4,
            tics: 12,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FaceTarget as *const (),
                    )
                }),
            },
            nextstate: S_CYBER_ATK6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CYBR,
            frame: 5,
            tics: 12,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_CyberAttack as *const (),
                    )
                }),
            },
            nextstate: S_CYBER_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CYBR,
            frame: 6,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Pain as *const (),
                    )
                }),
            },
            nextstate: S_CYBER_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CYBR,
            frame: 7,
            tics: 10,
            action: actionf_t { acp1: None },
            nextstate: S_CYBER_DIE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CYBR,
            frame: 8,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Scream as *const (),
                    )
                }),
            },
            nextstate: S_CYBER_DIE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CYBR,
            frame: 9,
            tics: 10,
            action: actionf_t { acp1: None },
            nextstate: S_CYBER_DIE4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CYBR,
            frame: 10,
            tics: 10,
            action: actionf_t { acp1: None },
            nextstate: S_CYBER_DIE5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CYBR,
            frame: 11,
            tics: 10,
            action: actionf_t { acp1: None },
            nextstate: S_CYBER_DIE6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CYBR,
            frame: 12,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fall as *const (),
                    )
                }),
            },
            nextstate: S_CYBER_DIE7,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CYBR,
            frame: 13,
            tics: 10,
            action: actionf_t { acp1: None },
            nextstate: S_CYBER_DIE8,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CYBR,
            frame: 14,
            tics: 10,
            action: actionf_t { acp1: None },
            nextstate: S_CYBER_DIE9,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CYBR,
            frame: 15,
            tics: 30,
            action: actionf_t { acp1: None },
            nextstate: S_CYBER_DIE10,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CYBR,
            frame: 15,
            tics: (-(1)),
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_BossDeath as *const (),
                    )
                }),
            },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PAIN,
            frame: 0,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Look as *const (),
                    )
                }),
            },
            nextstate: S_PAIN_STND,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PAIN,
            frame: 0,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_PAIN_RUN2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PAIN,
            frame: 0,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_PAIN_RUN3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PAIN,
            frame: 1,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_PAIN_RUN4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PAIN,
            frame: 1,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_PAIN_RUN5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PAIN,
            frame: 2,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_PAIN_RUN6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PAIN,
            frame: 2,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_PAIN_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PAIN,
            frame: 3,
            tics: 5,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FaceTarget as *const (),
                    )
                }),
            },
            nextstate: S_PAIN_ATK2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PAIN,
            frame: 4,
            tics: 5,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FaceTarget as *const (),
                    )
                }),
            },
            nextstate: S_PAIN_ATK3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PAIN,
            frame: 32773,
            tics: 5,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FaceTarget as *const (),
                    )
                }),
            },
            nextstate: S_PAIN_ATK4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PAIN,
            frame: 32773,
            tics: 0,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_PainAttack as *const (),
                    )
                }),
            },
            nextstate: S_PAIN_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PAIN,
            frame: 6,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_PAIN_PAIN2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PAIN,
            frame: 6,
            tics: 6,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Pain as *const (),
                    )
                }),
            },
            nextstate: S_PAIN_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PAIN,
            frame: 32775,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_PAIN_DIE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PAIN,
            frame: 32776,
            tics: 8,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Scream as *const (),
                    )
                }),
            },
            nextstate: S_PAIN_DIE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PAIN,
            frame: 32777,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_PAIN_DIE4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PAIN,
            frame: 32778,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_PAIN_DIE5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PAIN,
            frame: 32779,
            tics: 8,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_PainDie as *const (),
                    )
                }),
            },
            nextstate: S_PAIN_DIE6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PAIN,
            frame: 32780,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PAIN,
            frame: 12,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_PAIN_RAISE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PAIN,
            frame: 11,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_PAIN_RAISE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PAIN,
            frame: 10,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_PAIN_RAISE4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PAIN,
            frame: 9,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_PAIN_RAISE5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PAIN,
            frame: 8,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_PAIN_RAISE6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PAIN,
            frame: 7,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_PAIN_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SSWV,
            frame: 0,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Look as *const (),
                    )
                }),
            },
            nextstate: S_SSWV_STND2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SSWV,
            frame: 1,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Look as *const (),
                    )
                }),
            },
            nextstate: S_SSWV_STND,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SSWV,
            frame: 0,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SSWV_RUN2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SSWV,
            frame: 0,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SSWV_RUN3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SSWV,
            frame: 1,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SSWV_RUN4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SSWV,
            frame: 1,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SSWV_RUN5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SSWV,
            frame: 2,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SSWV_RUN6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SSWV,
            frame: 2,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SSWV_RUN7,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SSWV,
            frame: 3,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SSWV_RUN8,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SSWV,
            frame: 3,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Chase as *const (),
                    )
                }),
            },
            nextstate: S_SSWV_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SSWV,
            frame: 4,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FaceTarget as *const (),
                    )
                }),
            },
            nextstate: S_SSWV_ATK2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SSWV,
            frame: 5,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FaceTarget as *const (),
                    )
                }),
            },
            nextstate: S_SSWV_ATK3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SSWV,
            frame: 32774,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_CPosAttack as *const (),
                    )
                }),
            },
            nextstate: S_SSWV_ATK4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SSWV,
            frame: 5,
            tics: 6,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_FaceTarget as *const (),
                    )
                }),
            },
            nextstate: S_SSWV_ATK5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SSWV,
            frame: 32774,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_CPosAttack as *const (),
                    )
                }),
            },
            nextstate: S_SSWV_ATK6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SSWV,
            frame: 5,
            tics: 1,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_CPosRefire as *const (),
                    )
                }),
            },
            nextstate: S_SSWV_ATK2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SSWV,
            frame: 7,
            tics: 3,
            action: actionf_t { acp1: None },
            nextstate: S_SSWV_PAIN2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SSWV,
            frame: 7,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Pain as *const (),
                    )
                }),
            },
            nextstate: S_SSWV_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SSWV,
            frame: 8,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_SSWV_DIE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SSWV,
            frame: 9,
            tics: 5,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Scream as *const (),
                    )
                }),
            },
            nextstate: S_SSWV_DIE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SSWV,
            frame: 10,
            tics: 5,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fall as *const (),
                    )
                }),
            },
            nextstate: S_SSWV_DIE4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SSWV,
            frame: 11,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_SSWV_DIE5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SSWV,
            frame: 12,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SSWV,
            frame: 13,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_SSWV_XDIE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SSWV,
            frame: 14,
            tics: 5,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_XScream as *const (),
                    )
                }),
            },
            nextstate: S_SSWV_XDIE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SSWV,
            frame: 15,
            tics: 5,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fall as *const (),
                    )
                }),
            },
            nextstate: S_SSWV_XDIE4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SSWV,
            frame: 16,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_SSWV_XDIE5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SSWV,
            frame: 17,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_SSWV_XDIE6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SSWV,
            frame: 18,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_SSWV_XDIE7,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SSWV,
            frame: 19,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_SSWV_XDIE8,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SSWV,
            frame: 20,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_SSWV_XDIE9,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SSWV,
            frame: 21,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SSWV,
            frame: 12,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_SSWV_RAISE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SSWV,
            frame: 11,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_SSWV_RAISE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SSWV,
            frame: 10,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_SSWV_RAISE4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SSWV,
            frame: 9,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_SSWV_RAISE5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SSWV,
            frame: 8,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_SSWV_RUN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_KEEN,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_KEENSTND,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_KEEN,
            frame: 0,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_COMMKEEN2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_KEEN,
            frame: 1,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_COMMKEEN3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_KEEN,
            frame: 2,
            tics: 6,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Scream as *const (),
                    )
                }),
            },
            nextstate: S_COMMKEEN4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_KEEN,
            frame: 3,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_COMMKEEN5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_KEEN,
            frame: 4,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_COMMKEEN6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_KEEN,
            frame: 5,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_COMMKEEN7,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_KEEN,
            frame: 6,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_COMMKEEN8,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_KEEN,
            frame: 7,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_COMMKEEN9,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_KEEN,
            frame: 8,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_COMMKEEN10,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_KEEN,
            frame: 9,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_COMMKEEN11,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_KEEN,
            frame: 10,
            tics: 6,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_KeenDie as *const (),
                    )
                }),
            },
            nextstate: S_COMMKEEN12,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_KEEN,
            frame: 11,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_KEEN,
            frame: 12,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_KEENPAIN2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_KEEN,
            frame: 12,
            tics: 8,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Pain as *const (),
                    )
                }),
            },
            nextstate: S_KEENSTND,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BBRN,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BBRN,
            frame: 1,
            tics: 36,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_BrainPain as *const (),
                    )
                }),
            },
            nextstate: S_BRAIN,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BBRN,
            frame: 0,
            tics: 100,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_BrainScream as *const (),
                    )
                }),
            },
            nextstate: S_BRAIN_DIE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BBRN,
            frame: 0,
            tics: 10,
            action: actionf_t { acp1: None },
            nextstate: S_BRAIN_DIE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BBRN,
            frame: 0,
            tics: 10,
            action: actionf_t { acp1: None },
            nextstate: S_BRAIN_DIE4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BBRN,
            frame: 0,
            tics: (-(1)),
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_BrainDie as *const (),
                    )
                }),
            },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SSWV,
            frame: 0,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Look as *const (),
                    )
                }),
            },
            nextstate: S_BRAINEYE,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SSWV,
            frame: 0,
            tics: 181,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_BrainAwake as *const (),
                    )
                }),
            },
            nextstate: S_BRAINEYE1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SSWV,
            frame: 0,
            tics: 150,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_BrainSpit as *const (),
                    )
                }),
            },
            nextstate: S_BRAINEYE1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOSF,
            frame: 32768,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_SpawnSound as *const (),
                    )
                }),
            },
            nextstate: S_SPAWN2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOSF,
            frame: 32769,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_SpawnFly as *const (),
                    )
                }),
            },
            nextstate: S_SPAWN3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOSF,
            frame: 32770,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_SpawnFly as *const (),
                    )
                }),
            },
            nextstate: S_SPAWN4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BOSF,
            frame: 32771,
            tics: 3,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_SpawnFly as *const (),
                    )
                }),
            },
            nextstate: S_SPAWN1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FIRE,
            frame: 32768,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fire as *const (),
                    )
                }),
            },
            nextstate: S_SPAWNFIRE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FIRE,
            frame: 32769,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fire as *const (),
                    )
                }),
            },
            nextstate: S_SPAWNFIRE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FIRE,
            frame: 32770,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fire as *const (),
                    )
                }),
            },
            nextstate: S_SPAWNFIRE4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FIRE,
            frame: 32771,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fire as *const (),
                    )
                }),
            },
            nextstate: S_SPAWNFIRE5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FIRE,
            frame: 32772,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fire as *const (),
                    )
                }),
            },
            nextstate: S_SPAWNFIRE6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FIRE,
            frame: 32773,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fire as *const (),
                    )
                }),
            },
            nextstate: S_SPAWNFIRE7,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FIRE,
            frame: 32774,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fire as *const (),
                    )
                }),
            },
            nextstate: S_SPAWNFIRE8,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FIRE,
            frame: 32775,
            tics: 4,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Fire as *const (),
                    )
                }),
            },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_MISL,
            frame: 32769,
            tics: 10,
            action: actionf_t { acp1: None },
            nextstate: S_BRAINEXPLODE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_MISL,
            frame: 32770,
            tics: 10,
            action: actionf_t { acp1: None },
            nextstate: S_BRAINEXPLODE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_MISL,
            frame: 32771,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_BrainExplode as *const (),
                    )
                }),
            },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_ARM1,
            frame: 0,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_ARM1A,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_ARM1,
            frame: 32769,
            tics: 7,
            action: actionf_t { acp1: None },
            nextstate: S_ARM1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_ARM2,
            frame: 0,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_ARM2A,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_ARM2,
            frame: 32769,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_ARM2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BAR1,
            frame: 0,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_BAR2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BAR1,
            frame: 1,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_BAR1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BEXP,
            frame: 32768,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_BEXP2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BEXP,
            frame: 32769,
            tics: 5,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Scream as *const (),
                    )
                }),
            },
            nextstate: S_BEXP3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BEXP,
            frame: 32770,
            tics: 5,
            action: actionf_t { acp1: None },
            nextstate: S_BEXP4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BEXP,
            frame: 32771,
            tics: 10,
            action: actionf_t {
                acp1: Some(unsafe {
                    std::mem::transmute::<*const (), unsafe extern "C" fn(*mut std::ffi::c_void)>(
                        A_Explode as *const (),
                    )
                }),
            },
            nextstate: S_BEXP5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BEXP,
            frame: 32772,
            tics: 10,
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FCAN,
            frame: 32768,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_BBAR2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FCAN,
            frame: 32769,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_BBAR3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FCAN,
            frame: 32770,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_BBAR1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BON1,
            frame: 0,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_BON1A,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BON1,
            frame: 1,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_BON1B,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BON1,
            frame: 2,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_BON1C,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BON1,
            frame: 3,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_BON1D,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BON1,
            frame: 2,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_BON1E,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BON1,
            frame: 1,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_BON1,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BON2,
            frame: 0,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_BON2A,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BON2,
            frame: 1,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_BON2B,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BON2,
            frame: 2,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_BON2C,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BON2,
            frame: 3,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_BON2D,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BON2,
            frame: 2,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_BON2E,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BON2,
            frame: 1,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_BON2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BKEY,
            frame: 0,
            tics: 10,
            action: actionf_t { acp1: None },
            nextstate: S_BKEY2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BKEY,
            frame: 32769,
            tics: 10,
            action: actionf_t { acp1: None },
            nextstate: S_BKEY,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_RKEY,
            frame: 0,
            tics: 10,
            action: actionf_t { acp1: None },
            nextstate: S_RKEY2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_RKEY,
            frame: 32769,
            tics: 10,
            action: actionf_t { acp1: None },
            nextstate: S_RKEY,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_YKEY,
            frame: 0,
            tics: 10,
            action: actionf_t { acp1: None },
            nextstate: S_YKEY2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_YKEY,
            frame: 32769,
            tics: 10,
            action: actionf_t { acp1: None },
            nextstate: S_YKEY,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BSKU,
            frame: 0,
            tics: 10,
            action: actionf_t { acp1: None },
            nextstate: S_BSKULL2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BSKU,
            frame: 32769,
            tics: 10,
            action: actionf_t { acp1: None },
            nextstate: S_BSKULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_RSKU,
            frame: 0,
            tics: 10,
            action: actionf_t { acp1: None },
            nextstate: S_RSKULL2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_RSKU,
            frame: 32769,
            tics: 10,
            action: actionf_t { acp1: None },
            nextstate: S_RSKULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_YSKU,
            frame: 0,
            tics: 10,
            action: actionf_t { acp1: None },
            nextstate: S_YSKULL2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_YSKU,
            frame: 32769,
            tics: 10,
            action: actionf_t { acp1: None },
            nextstate: S_YSKULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_STIM,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_MEDI,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SOUL,
            frame: 32768,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_SOUL2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SOUL,
            frame: 32769,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_SOUL3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SOUL,
            frame: 32770,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_SOUL4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SOUL,
            frame: 32771,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_SOUL5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SOUL,
            frame: 32770,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_SOUL6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SOUL,
            frame: 32769,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_SOUL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PINV,
            frame: 32768,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_PINV2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PINV,
            frame: 32769,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_PINV3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PINV,
            frame: 32770,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_PINV4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PINV,
            frame: 32771,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_PINV,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PSTR,
            frame: 32768,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PINS,
            frame: 32768,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_PINS2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PINS,
            frame: 32769,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_PINS3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PINS,
            frame: 32770,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_PINS4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PINS,
            frame: 32771,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_PINS,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_MEGA,
            frame: 32768,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_MEGA2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_MEGA,
            frame: 32769,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_MEGA3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_MEGA,
            frame: 32770,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_MEGA4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_MEGA,
            frame: 32771,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_MEGA,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SUIT,
            frame: 32768,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PMAP,
            frame: 32768,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_PMAP2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PMAP,
            frame: 32769,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_PMAP3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PMAP,
            frame: 32770,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_PMAP4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PMAP,
            frame: 32771,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_PMAP5,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PMAP,
            frame: 32770,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_PMAP6,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PMAP,
            frame: 32769,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_PMAP,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PVIS,
            frame: 32768,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_PVIS2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PVIS,
            frame: 1,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_PVIS,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CLIP,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_AMMO,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_ROCK,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BROK,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CELL,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CELP,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SHEL,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SBOX,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BPAK,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BFUG,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_MGUN,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CSAW,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_LAUN,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PLAS,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SHOT,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SGN2,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_COLU,
            frame: 32768,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SMT2,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_GOR1,
            frame: 0,
            tics: 10,
            action: actionf_t { acp1: None },
            nextstate: S_BLOODYTWITCH2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_GOR1,
            frame: 1,
            tics: 15,
            action: actionf_t { acp1: None },
            nextstate: S_BLOODYTWITCH3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_GOR1,
            frame: 2,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_BLOODYTWITCH4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_GOR1,
            frame: 1,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_BLOODYTWITCH,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PLAY,
            frame: 13,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_PLAY,
            frame: 18,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_POL2,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_POL5,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_POL4,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_POL3,
            frame: 32768,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_HEADCANDLES2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_POL3,
            frame: 32769,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_HEADCANDLES,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_POL1,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_POL6,
            frame: 0,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_LIVESTICK2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_POL6,
            frame: 1,
            tics: 8,
            action: actionf_t { acp1: None },
            nextstate: S_LIVESTICK,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_GOR2,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_GOR3,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_GOR4,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_GOR5,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SMIT,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_COL1,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_COL2,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_COL3,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_COL4,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CAND,
            frame: 32768,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CBRA,
            frame: 32768,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_COL6,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TRE1,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TRE2,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_ELEC,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CEYE,
            frame: 32768,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_EVILEYE2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CEYE,
            frame: 32769,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_EVILEYE3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CEYE,
            frame: 32770,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_EVILEYE4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_CEYE,
            frame: 32769,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_EVILEYE,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FSKU,
            frame: 32768,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_FLOATSKULL2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FSKU,
            frame: 32769,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_FLOATSKULL3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_FSKU,
            frame: 32770,
            tics: 6,
            action: actionf_t { acp1: None },
            nextstate: S_FLOATSKULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_COL5,
            frame: 0,
            tics: 14,
            action: actionf_t { acp1: None },
            nextstate: S_HEARTCOL2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_COL5,
            frame: 1,
            tics: 14,
            action: actionf_t { acp1: None },
            nextstate: S_HEARTCOL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TBLU,
            frame: 32768,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_BLUETORCH2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TBLU,
            frame: 32769,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_BLUETORCH3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TBLU,
            frame: 32770,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_BLUETORCH4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TBLU,
            frame: 32771,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_BLUETORCH,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TGRN,
            frame: 32768,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_GREENTORCH2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TGRN,
            frame: 32769,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_GREENTORCH3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TGRN,
            frame: 32770,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_GREENTORCH4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TGRN,
            frame: 32771,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_GREENTORCH,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TRED,
            frame: 32768,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_REDTORCH2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TRED,
            frame: 32769,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_REDTORCH3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TRED,
            frame: 32770,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_REDTORCH4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TRED,
            frame: 32771,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_REDTORCH,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SMBT,
            frame: 32768,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_BTORCHSHRT2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SMBT,
            frame: 32769,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_BTORCHSHRT3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SMBT,
            frame: 32770,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_BTORCHSHRT4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SMBT,
            frame: 32771,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_BTORCHSHRT,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SMGT,
            frame: 32768,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_GTORCHSHRT2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SMGT,
            frame: 32769,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_GTORCHSHRT3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SMGT,
            frame: 32770,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_GTORCHSHRT4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SMGT,
            frame: 32771,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_GTORCHSHRT,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SMRT,
            frame: 32768,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_RTORCHSHRT2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SMRT,
            frame: 32769,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_RTORCHSHRT3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SMRT,
            frame: 32770,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_RTORCHSHRT4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_SMRT,
            frame: 32771,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_RTORCHSHRT,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_HDB1,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_HDB2,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_HDB3,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_HDB4,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_HDB5,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_HDB6,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_POB1,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_POB2,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_BRS1,
            frame: 0,
            tics: (-(1)),
            action: actionf_t { acp1: None },
            nextstate: S_NULL,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TLMP,
            frame: 32768,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_TECHLAMP2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TLMP,
            frame: 32769,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_TECHLAMP3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TLMP,
            frame: 32770,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_TECHLAMP4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TLMP,
            frame: 32771,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_TECHLAMP,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TLP2,
            frame: 32768,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_TECH2LAMP2,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TLP2,
            frame: 32769,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_TECH2LAMP3,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TLP2,
            frame: 32770,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_TECH2LAMP4,
            misc1: 0,
            misc2: 0,
        },
        state_t {
            sprite: SPR_TLP2,
            frame: 32771,
            tics: 4,
            action: actionf_t { acp1: None },
            nextstate: S_TECH2LAMP,
            misc1: 0,
            misc2: 0,
        },
    ]
};

pub static mut mobjinfo: [mobjinfo_t; (NUMMOBJTYPES) as usize] = unsafe {
    [
        mobjinfo_t {
            doomednum: (-(1)),
            spawnstate: S_PLAY,
            spawnhealth: 100,
            seestate: S_PLAY_RUN1,
            seesound: sfx_None,
            reactiontime: 0,
            attacksound: sfx_None,
            painstate: S_PLAY_PAIN,
            painchance: 255,
            painsound: sfx_plpain,
            meleestate: S_NULL,
            missilestate: S_PLAY_ATK1,
            deathstate: S_PLAY_DIE1,
            xdeathstate: S_PLAY_XDIE1,
            deathsound: sfx_pldeth,
            speed: 0,
            radius: (16 * FRACUNIT),
            height: (56 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: ((((MF_SOLID | MF_SHOOTABLE) | MF_DROPOFF) | MF_PICKUP) | MF_NOTDMATCH),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 3004,
            spawnstate: S_POSS_STND,
            spawnhealth: 20,
            seestate: S_POSS_RUN1,
            seesound: sfx_posit1,
            reactiontime: 8,
            attacksound: sfx_pistol,
            painstate: S_POSS_PAIN,
            painchance: 200,
            painsound: sfx_popain,
            meleestate: 0,
            missilestate: S_POSS_ATK1,
            deathstate: S_POSS_DIE1,
            xdeathstate: S_POSS_XDIE1,
            deathsound: sfx_podth1,
            speed: 8,
            radius: (20 * FRACUNIT),
            height: (56 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_posact,
            flags: ((MF_SOLID | MF_SHOOTABLE) | MF_COUNTKILL),
            raisestate: S_POSS_RAISE1,
        },
        mobjinfo_t {
            doomednum: 9,
            spawnstate: S_SPOS_STND,
            spawnhealth: 30,
            seestate: S_SPOS_RUN1,
            seesound: sfx_posit2,
            reactiontime: 8,
            attacksound: 0,
            painstate: S_SPOS_PAIN,
            painchance: 170,
            painsound: sfx_popain,
            meleestate: 0,
            missilestate: S_SPOS_ATK1,
            deathstate: S_SPOS_DIE1,
            xdeathstate: S_SPOS_XDIE1,
            deathsound: sfx_podth2,
            speed: 8,
            radius: (20 * FRACUNIT),
            height: (56 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_posact,
            flags: ((MF_SOLID | MF_SHOOTABLE) | MF_COUNTKILL),
            raisestate: S_SPOS_RAISE1,
        },
        mobjinfo_t {
            doomednum: 64,
            spawnstate: S_VILE_STND,
            spawnhealth: 700,
            seestate: S_VILE_RUN1,
            seesound: sfx_vilsit,
            reactiontime: 8,
            attacksound: 0,
            painstate: S_VILE_PAIN,
            painchance: 10,
            painsound: sfx_vipain,
            meleestate: 0,
            missilestate: S_VILE_ATK1,
            deathstate: S_VILE_DIE1,
            xdeathstate: S_NULL,
            deathsound: sfx_vildth,
            speed: 15,
            radius: (20 * FRACUNIT),
            height: (56 * FRACUNIT),
            mass: 500,
            damage: 0,
            activesound: sfx_vilact,
            flags: ((MF_SOLID | MF_SHOOTABLE) | MF_COUNTKILL),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: (-(1)),
            spawnstate: S_FIRE1,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: (MF_NOBLOCKMAP | MF_NOGRAVITY),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 66,
            spawnstate: S_SKEL_STND,
            spawnhealth: 300,
            seestate: S_SKEL_RUN1,
            seesound: sfx_skesit,
            reactiontime: 8,
            attacksound: 0,
            painstate: S_SKEL_PAIN,
            painchance: 100,
            painsound: sfx_popain,
            meleestate: S_SKEL_FIST1,
            missilestate: S_SKEL_MISS1,
            deathstate: S_SKEL_DIE1,
            xdeathstate: S_NULL,
            deathsound: sfx_skedth,
            speed: 10,
            radius: (20 * FRACUNIT),
            height: (56 * FRACUNIT),
            mass: 500,
            damage: 0,
            activesound: sfx_skeact,
            flags: ((MF_SOLID | MF_SHOOTABLE) | MF_COUNTKILL),
            raisestate: S_SKEL_RAISE1,
        },
        mobjinfo_t {
            doomednum: (-(1)),
            spawnstate: S_TRACER,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_skeatk,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_TRACEEXP1,
            xdeathstate: S_NULL,
            deathsound: sfx_barexp,
            speed: (10 * FRACUNIT),
            radius: (11 * FRACUNIT),
            height: (8 * FRACUNIT),
            mass: 100,
            damage: 10,
            activesound: sfx_None,
            flags: (((MF_NOBLOCKMAP | MF_MISSILE) | MF_DROPOFF) | MF_NOGRAVITY),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: (-(1)),
            spawnstate: S_SMOKE1,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: (MF_NOBLOCKMAP | MF_NOGRAVITY),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 67,
            spawnstate: S_FATT_STND,
            spawnhealth: 600,
            seestate: S_FATT_RUN1,
            seesound: sfx_mansit,
            reactiontime: 8,
            attacksound: 0,
            painstate: S_FATT_PAIN,
            painchance: 80,
            painsound: sfx_mnpain,
            meleestate: 0,
            missilestate: S_FATT_ATK1,
            deathstate: S_FATT_DIE1,
            xdeathstate: S_NULL,
            deathsound: sfx_mandth,
            speed: 8,
            radius: (48 * FRACUNIT),
            height: (64 * FRACUNIT),
            mass: 1000,
            damage: 0,
            activesound: sfx_posact,
            flags: ((MF_SOLID | MF_SHOOTABLE) | MF_COUNTKILL),
            raisestate: S_FATT_RAISE1,
        },
        mobjinfo_t {
            doomednum: (-(1)),
            spawnstate: S_FATSHOT1,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_firsht,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_FATSHOTX1,
            xdeathstate: S_NULL,
            deathsound: sfx_firxpl,
            speed: (20 * FRACUNIT),
            radius: (6 * FRACUNIT),
            height: (8 * FRACUNIT),
            mass: 100,
            damage: 8,
            activesound: sfx_None,
            flags: (((MF_NOBLOCKMAP | MF_MISSILE) | MF_DROPOFF) | MF_NOGRAVITY),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 65,
            spawnstate: S_CPOS_STND,
            spawnhealth: 70,
            seestate: S_CPOS_RUN1,
            seesound: sfx_posit2,
            reactiontime: 8,
            attacksound: 0,
            painstate: S_CPOS_PAIN,
            painchance: 170,
            painsound: sfx_popain,
            meleestate: 0,
            missilestate: S_CPOS_ATK1,
            deathstate: S_CPOS_DIE1,
            xdeathstate: S_CPOS_XDIE1,
            deathsound: sfx_podth2,
            speed: 8,
            radius: (20 * FRACUNIT),
            height: (56 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_posact,
            flags: ((MF_SOLID | MF_SHOOTABLE) | MF_COUNTKILL),
            raisestate: S_CPOS_RAISE1,
        },
        mobjinfo_t {
            doomednum: 3001,
            spawnstate: S_TROO_STND,
            spawnhealth: 60,
            seestate: S_TROO_RUN1,
            seesound: sfx_bgsit1,
            reactiontime: 8,
            attacksound: 0,
            painstate: S_TROO_PAIN,
            painchance: 200,
            painsound: sfx_popain,
            meleestate: S_TROO_ATK1,
            missilestate: S_TROO_ATK1,
            deathstate: S_TROO_DIE1,
            xdeathstate: S_TROO_XDIE1,
            deathsound: sfx_bgdth1,
            speed: 8,
            radius: (20 * FRACUNIT),
            height: (56 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_bgact,
            flags: ((MF_SOLID | MF_SHOOTABLE) | MF_COUNTKILL),
            raisestate: S_TROO_RAISE1,
        },
        mobjinfo_t {
            doomednum: 3002,
            spawnstate: S_SARG_STND,
            spawnhealth: 150,
            seestate: S_SARG_RUN1,
            seesound: sfx_sgtsit,
            reactiontime: 8,
            attacksound: sfx_sgtatk,
            painstate: S_SARG_PAIN,
            painchance: 180,
            painsound: sfx_dmpain,
            meleestate: S_SARG_ATK1,
            missilestate: 0,
            deathstate: S_SARG_DIE1,
            xdeathstate: S_NULL,
            deathsound: sfx_sgtdth,
            speed: 10,
            radius: (30 * FRACUNIT),
            height: (56 * FRACUNIT),
            mass: 400,
            damage: 0,
            activesound: sfx_dmact,
            flags: ((MF_SOLID | MF_SHOOTABLE) | MF_COUNTKILL),
            raisestate: S_SARG_RAISE1,
        },
        mobjinfo_t {
            doomednum: 58,
            spawnstate: S_SARG_STND,
            spawnhealth: 150,
            seestate: S_SARG_RUN1,
            seesound: sfx_sgtsit,
            reactiontime: 8,
            attacksound: sfx_sgtatk,
            painstate: S_SARG_PAIN,
            painchance: 180,
            painsound: sfx_dmpain,
            meleestate: S_SARG_ATK1,
            missilestate: 0,
            deathstate: S_SARG_DIE1,
            xdeathstate: S_NULL,
            deathsound: sfx_sgtdth,
            speed: 10,
            radius: (30 * FRACUNIT),
            height: (56 * FRACUNIT),
            mass: 400,
            damage: 0,
            activesound: sfx_dmact,
            flags: (((MF_SOLID | MF_SHOOTABLE) | MF_SHADOW) | MF_COUNTKILL),
            raisestate: S_SARG_RAISE1,
        },
        mobjinfo_t {
            doomednum: 3005,
            spawnstate: S_HEAD_STND,
            spawnhealth: 400,
            seestate: S_HEAD_RUN1,
            seesound: sfx_cacsit,
            reactiontime: 8,
            attacksound: 0,
            painstate: S_HEAD_PAIN,
            painchance: 128,
            painsound: sfx_dmpain,
            meleestate: 0,
            missilestate: S_HEAD_ATK1,
            deathstate: S_HEAD_DIE1,
            xdeathstate: S_NULL,
            deathsound: sfx_cacdth,
            speed: 8,
            radius: (31 * FRACUNIT),
            height: (56 * FRACUNIT),
            mass: 400,
            damage: 0,
            activesound: sfx_dmact,
            flags: ((((MF_SOLID | MF_SHOOTABLE) | MF_FLOAT) | MF_NOGRAVITY) | MF_COUNTKILL),
            raisestate: S_HEAD_RAISE1,
        },
        mobjinfo_t {
            doomednum: 3003,
            spawnstate: S_BOSS_STND,
            spawnhealth: 1000,
            seestate: S_BOSS_RUN1,
            seesound: sfx_brssit,
            reactiontime: 8,
            attacksound: 0,
            painstate: S_BOSS_PAIN,
            painchance: 50,
            painsound: sfx_dmpain,
            meleestate: S_BOSS_ATK1,
            missilestate: S_BOSS_ATK1,
            deathstate: S_BOSS_DIE1,
            xdeathstate: S_NULL,
            deathsound: sfx_brsdth,
            speed: 8,
            radius: (24 * FRACUNIT),
            height: (64 * FRACUNIT),
            mass: 1000,
            damage: 0,
            activesound: sfx_dmact,
            flags: ((MF_SOLID | MF_SHOOTABLE) | MF_COUNTKILL),
            raisestate: S_BOSS_RAISE1,
        },
        mobjinfo_t {
            doomednum: (-(1)),
            spawnstate: S_BRBALL1,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_firsht,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_BRBALLX1,
            xdeathstate: S_NULL,
            deathsound: sfx_firxpl,
            speed: (15 * FRACUNIT),
            radius: (6 * FRACUNIT),
            height: (8 * FRACUNIT),
            mass: 100,
            damage: 8,
            activesound: sfx_None,
            flags: (((MF_NOBLOCKMAP | MF_MISSILE) | MF_DROPOFF) | MF_NOGRAVITY),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 69,
            spawnstate: S_BOS2_STND,
            spawnhealth: 500,
            seestate: S_BOS2_RUN1,
            seesound: sfx_kntsit,
            reactiontime: 8,
            attacksound: 0,
            painstate: S_BOS2_PAIN,
            painchance: 50,
            painsound: sfx_dmpain,
            meleestate: S_BOS2_ATK1,
            missilestate: S_BOS2_ATK1,
            deathstate: S_BOS2_DIE1,
            xdeathstate: S_NULL,
            deathsound: sfx_kntdth,
            speed: 8,
            radius: (24 * FRACUNIT),
            height: (64 * FRACUNIT),
            mass: 1000,
            damage: 0,
            activesound: sfx_dmact,
            flags: ((MF_SOLID | MF_SHOOTABLE) | MF_COUNTKILL),
            raisestate: S_BOS2_RAISE1,
        },
        mobjinfo_t {
            doomednum: 3006,
            spawnstate: S_SKULL_STND,
            spawnhealth: 100,
            seestate: S_SKULL_RUN1,
            seesound: 0,
            reactiontime: 8,
            attacksound: sfx_sklatk,
            painstate: S_SKULL_PAIN,
            painchance: 256,
            painsound: sfx_dmpain,
            meleestate: 0,
            missilestate: S_SKULL_ATK1,
            deathstate: S_SKULL_DIE1,
            xdeathstate: S_NULL,
            deathsound: sfx_firxpl,
            speed: 8,
            radius: (16 * FRACUNIT),
            height: (56 * FRACUNIT),
            mass: 50,
            damage: 3,
            activesound: sfx_dmact,
            flags: (((MF_SOLID | MF_SHOOTABLE) | MF_FLOAT) | MF_NOGRAVITY),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 7,
            spawnstate: S_SPID_STND,
            spawnhealth: 3000,
            seestate: S_SPID_RUN1,
            seesound: sfx_spisit,
            reactiontime: 8,
            attacksound: sfx_shotgn,
            painstate: S_SPID_PAIN,
            painchance: 40,
            painsound: sfx_dmpain,
            meleestate: 0,
            missilestate: S_SPID_ATK1,
            deathstate: S_SPID_DIE1,
            xdeathstate: S_NULL,
            deathsound: sfx_spidth,
            speed: 12,
            radius: (128 * FRACUNIT),
            height: (100 * FRACUNIT),
            mass: 1000,
            damage: 0,
            activesound: sfx_dmact,
            flags: ((MF_SOLID | MF_SHOOTABLE) | MF_COUNTKILL),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 68,
            spawnstate: S_BSPI_STND,
            spawnhealth: 500,
            seestate: S_BSPI_SIGHT,
            seesound: sfx_bspsit,
            reactiontime: 8,
            attacksound: 0,
            painstate: S_BSPI_PAIN,
            painchance: 128,
            painsound: sfx_dmpain,
            meleestate: 0,
            missilestate: S_BSPI_ATK1,
            deathstate: S_BSPI_DIE1,
            xdeathstate: S_NULL,
            deathsound: sfx_bspdth,
            speed: 12,
            radius: (64 * FRACUNIT),
            height: (64 * FRACUNIT),
            mass: 600,
            damage: 0,
            activesound: sfx_bspact,
            flags: ((MF_SOLID | MF_SHOOTABLE) | MF_COUNTKILL),
            raisestate: S_BSPI_RAISE1,
        },
        mobjinfo_t {
            doomednum: 16,
            spawnstate: S_CYBER_STND,
            spawnhealth: 4000,
            seestate: S_CYBER_RUN1,
            seesound: sfx_cybsit,
            reactiontime: 8,
            attacksound: 0,
            painstate: S_CYBER_PAIN,
            painchance: 20,
            painsound: sfx_dmpain,
            meleestate: 0,
            missilestate: S_CYBER_ATK1,
            deathstate: S_CYBER_DIE1,
            xdeathstate: S_NULL,
            deathsound: sfx_cybdth,
            speed: 16,
            radius: (40 * FRACUNIT),
            height: (110 * FRACUNIT),
            mass: 1000,
            damage: 0,
            activesound: sfx_dmact,
            flags: ((MF_SOLID | MF_SHOOTABLE) | MF_COUNTKILL),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 71,
            spawnstate: S_PAIN_STND,
            spawnhealth: 400,
            seestate: S_PAIN_RUN1,
            seesound: sfx_pesit,
            reactiontime: 8,
            attacksound: 0,
            painstate: S_PAIN_PAIN,
            painchance: 128,
            painsound: sfx_pepain,
            meleestate: 0,
            missilestate: S_PAIN_ATK1,
            deathstate: S_PAIN_DIE1,
            xdeathstate: S_NULL,
            deathsound: sfx_pedth,
            speed: 8,
            radius: (31 * FRACUNIT),
            height: (56 * FRACUNIT),
            mass: 400,
            damage: 0,
            activesound: sfx_dmact,
            flags: ((((MF_SOLID | MF_SHOOTABLE) | MF_FLOAT) | MF_NOGRAVITY) | MF_COUNTKILL),
            raisestate: S_PAIN_RAISE1,
        },
        mobjinfo_t {
            doomednum: 84,
            spawnstate: S_SSWV_STND,
            spawnhealth: 50,
            seestate: S_SSWV_RUN1,
            seesound: sfx_sssit,
            reactiontime: 8,
            attacksound: 0,
            painstate: S_SSWV_PAIN,
            painchance: 170,
            painsound: sfx_popain,
            meleestate: 0,
            missilestate: S_SSWV_ATK1,
            deathstate: S_SSWV_DIE1,
            xdeathstate: S_SSWV_XDIE1,
            deathsound: sfx_ssdth,
            speed: 8,
            radius: (20 * FRACUNIT),
            height: (56 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_posact,
            flags: ((MF_SOLID | MF_SHOOTABLE) | MF_COUNTKILL),
            raisestate: S_SSWV_RAISE1,
        },
        mobjinfo_t {
            doomednum: 72,
            spawnstate: S_KEENSTND,
            spawnhealth: 100,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_KEENPAIN,
            painchance: 256,
            painsound: sfx_keenpn,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_COMMKEEN,
            xdeathstate: S_NULL,
            deathsound: sfx_keendt,
            speed: 0,
            radius: (16 * FRACUNIT),
            height: (72 * FRACUNIT),
            mass: 10000000,
            damage: 0,
            activesound: sfx_None,
            flags: ((((MF_SOLID | MF_SPAWNCEILING) | MF_NOGRAVITY) | MF_SHOOTABLE) | MF_COUNTKILL),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 88,
            spawnstate: S_BRAIN,
            spawnhealth: 250,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_BRAIN_PAIN,
            painchance: 255,
            painsound: sfx_bospn,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_BRAIN_DIE1,
            xdeathstate: S_NULL,
            deathsound: sfx_bosdth,
            speed: 0,
            radius: (16 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 10000000,
            damage: 0,
            activesound: sfx_None,
            flags: (MF_SOLID | MF_SHOOTABLE),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 89,
            spawnstate: S_BRAINEYE,
            spawnhealth: 1000,
            seestate: S_BRAINEYESEE,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (32 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: (MF_NOBLOCKMAP | MF_NOSECTOR),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 87,
            spawnstate: S_NULL,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (32 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: (MF_NOBLOCKMAP | MF_NOSECTOR),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: (-(1)),
            spawnstate: S_SPAWN1,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_bospit,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_firxpl,
            speed: (10 * FRACUNIT),
            radius: (6 * FRACUNIT),
            height: (32 * FRACUNIT),
            mass: 100,
            damage: 3,
            activesound: sfx_None,
            flags: ((((MF_NOBLOCKMAP | MF_MISSILE) | MF_DROPOFF) | MF_NOGRAVITY) | MF_NOCLIP),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: (-(1)),
            spawnstate: S_SPAWNFIRE1,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: (MF_NOBLOCKMAP | MF_NOGRAVITY),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 2035,
            spawnstate: S_BAR1,
            spawnhealth: 20,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_BEXP,
            xdeathstate: S_NULL,
            deathsound: sfx_barexp,
            speed: 0,
            radius: (10 * FRACUNIT),
            height: (42 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: ((MF_SOLID | MF_SHOOTABLE) | MF_NOBLOOD),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: (-(1)),
            spawnstate: S_TBALL1,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_firsht,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_TBALLX1,
            xdeathstate: S_NULL,
            deathsound: sfx_firxpl,
            speed: (10 * FRACUNIT),
            radius: (6 * FRACUNIT),
            height: (8 * FRACUNIT),
            mass: 100,
            damage: 3,
            activesound: sfx_None,
            flags: (((MF_NOBLOCKMAP | MF_MISSILE) | MF_DROPOFF) | MF_NOGRAVITY),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: (-(1)),
            spawnstate: S_RBALL1,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_firsht,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_RBALLX1,
            xdeathstate: S_NULL,
            deathsound: sfx_firxpl,
            speed: (10 * FRACUNIT),
            radius: (6 * FRACUNIT),
            height: (8 * FRACUNIT),
            mass: 100,
            damage: 5,
            activesound: sfx_None,
            flags: (((MF_NOBLOCKMAP | MF_MISSILE) | MF_DROPOFF) | MF_NOGRAVITY),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: (-(1)),
            spawnstate: S_ROCKET,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_rlaunc,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_EXPLODE1,
            xdeathstate: S_NULL,
            deathsound: sfx_barexp,
            speed: (20 * FRACUNIT),
            radius: (11 * FRACUNIT),
            height: (8 * FRACUNIT),
            mass: 100,
            damage: 20,
            activesound: sfx_None,
            flags: (((MF_NOBLOCKMAP | MF_MISSILE) | MF_DROPOFF) | MF_NOGRAVITY),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: (-(1)),
            spawnstate: S_PLASBALL,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_plasma,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_PLASEXP,
            xdeathstate: S_NULL,
            deathsound: sfx_firxpl,
            speed: (25 * FRACUNIT),
            radius: (13 * FRACUNIT),
            height: (8 * FRACUNIT),
            mass: 100,
            damage: 5,
            activesound: sfx_None,
            flags: (((MF_NOBLOCKMAP | MF_MISSILE) | MF_DROPOFF) | MF_NOGRAVITY),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: (-(1)),
            spawnstate: S_BFGSHOT,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: 0,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_BFGLAND,
            xdeathstate: S_NULL,
            deathsound: sfx_rxplod,
            speed: (25 * FRACUNIT),
            radius: (13 * FRACUNIT),
            height: (8 * FRACUNIT),
            mass: 100,
            damage: 100,
            activesound: sfx_None,
            flags: (((MF_NOBLOCKMAP | MF_MISSILE) | MF_DROPOFF) | MF_NOGRAVITY),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: (-(1)),
            spawnstate: S_ARACH_PLAZ,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_plasma,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_ARACH_PLEX,
            xdeathstate: S_NULL,
            deathsound: sfx_firxpl,
            speed: (25 * FRACUNIT),
            radius: (13 * FRACUNIT),
            height: (8 * FRACUNIT),
            mass: 100,
            damage: 5,
            activesound: sfx_None,
            flags: (((MF_NOBLOCKMAP | MF_MISSILE) | MF_DROPOFF) | MF_NOGRAVITY),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: (-(1)),
            spawnstate: S_PUFF1,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: (MF_NOBLOCKMAP | MF_NOGRAVITY),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: (-(1)),
            spawnstate: S_BLOOD1,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_NOBLOCKMAP,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: (-(1)),
            spawnstate: S_TFOG,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: (MF_NOBLOCKMAP | MF_NOGRAVITY),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: (-(1)),
            spawnstate: S_IFOG,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: (MF_NOBLOCKMAP | MF_NOGRAVITY),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 14,
            spawnstate: S_NULL,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: (MF_NOBLOCKMAP | MF_NOSECTOR),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: (-(1)),
            spawnstate: S_BFGEXP,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: (MF_NOBLOCKMAP | MF_NOGRAVITY),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 2018,
            spawnstate: S_ARM1,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SPECIAL,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 2019,
            spawnstate: S_ARM2,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SPECIAL,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 2014,
            spawnstate: S_BON1,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: (MF_SPECIAL | MF_COUNTITEM),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 2015,
            spawnstate: S_BON2,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: (MF_SPECIAL | MF_COUNTITEM),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 5,
            spawnstate: S_BKEY,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: (MF_SPECIAL | MF_NOTDMATCH),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 13,
            spawnstate: S_RKEY,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: (MF_SPECIAL | MF_NOTDMATCH),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 6,
            spawnstate: S_YKEY,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: (MF_SPECIAL | MF_NOTDMATCH),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 39,
            spawnstate: S_YSKULL,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: (MF_SPECIAL | MF_NOTDMATCH),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 38,
            spawnstate: S_RSKULL,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: (MF_SPECIAL | MF_NOTDMATCH),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 40,
            spawnstate: S_BSKULL,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: (MF_SPECIAL | MF_NOTDMATCH),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 2011,
            spawnstate: S_STIM,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SPECIAL,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 2012,
            spawnstate: S_MEDI,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SPECIAL,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 2013,
            spawnstate: S_SOUL,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: (MF_SPECIAL | MF_COUNTITEM),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 2022,
            spawnstate: S_PINV,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: (MF_SPECIAL | MF_COUNTITEM),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 2023,
            spawnstate: S_PSTR,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: (MF_SPECIAL | MF_COUNTITEM),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 2024,
            spawnstate: S_PINS,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: (MF_SPECIAL | MF_COUNTITEM),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 2025,
            spawnstate: S_SUIT,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SPECIAL,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 2026,
            spawnstate: S_PMAP,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: (MF_SPECIAL | MF_COUNTITEM),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 2045,
            spawnstate: S_PVIS,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: (MF_SPECIAL | MF_COUNTITEM),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 83,
            spawnstate: S_MEGA,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: (MF_SPECIAL | MF_COUNTITEM),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 2007,
            spawnstate: S_CLIP,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SPECIAL,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 2048,
            spawnstate: S_AMMO,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SPECIAL,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 2010,
            spawnstate: S_ROCK,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SPECIAL,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 2046,
            spawnstate: S_BROK,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SPECIAL,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 2047,
            spawnstate: S_CELL,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SPECIAL,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 17,
            spawnstate: S_CELP,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SPECIAL,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 2008,
            spawnstate: S_SHEL,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SPECIAL,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 2049,
            spawnstate: S_SBOX,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SPECIAL,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 8,
            spawnstate: S_BPAK,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SPECIAL,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 2006,
            spawnstate: S_BFUG,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SPECIAL,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 2002,
            spawnstate: S_MGUN,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SPECIAL,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 2005,
            spawnstate: S_CSAW,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SPECIAL,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 2003,
            spawnstate: S_LAUN,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SPECIAL,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 2004,
            spawnstate: S_PLAS,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SPECIAL,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 2001,
            spawnstate: S_SHOT,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SPECIAL,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 82,
            spawnstate: S_SHOT2,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SPECIAL,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 85,
            spawnstate: S_TECHLAMP,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (16 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SOLID,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 86,
            spawnstate: S_TECH2LAMP,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (16 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SOLID,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 2028,
            spawnstate: S_COLU,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (16 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SOLID,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 30,
            spawnstate: S_TALLGRNCOL,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (16 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SOLID,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 31,
            spawnstate: S_SHRTGRNCOL,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (16 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SOLID,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 32,
            spawnstate: S_TALLREDCOL,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (16 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SOLID,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 33,
            spawnstate: S_SHRTREDCOL,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (16 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SOLID,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 37,
            spawnstate: S_SKULLCOL,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (16 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SOLID,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 36,
            spawnstate: S_HEARTCOL,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (16 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SOLID,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 41,
            spawnstate: S_EVILEYE,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (16 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SOLID,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 42,
            spawnstate: S_FLOATSKULL,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (16 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SOLID,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 43,
            spawnstate: S_TORCHTREE,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (16 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SOLID,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 44,
            spawnstate: S_BLUETORCH,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (16 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SOLID,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 45,
            spawnstate: S_GREENTORCH,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (16 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SOLID,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 46,
            spawnstate: S_REDTORCH,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (16 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SOLID,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 55,
            spawnstate: S_BTORCHSHRT,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (16 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SOLID,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 56,
            spawnstate: S_GTORCHSHRT,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (16 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SOLID,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 57,
            spawnstate: S_RTORCHSHRT,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (16 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SOLID,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 47,
            spawnstate: S_STALAGTITE,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (16 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SOLID,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 48,
            spawnstate: S_TECHPILLAR,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (16 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SOLID,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 34,
            spawnstate: S_CANDLESTIK,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: 0,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 35,
            spawnstate: S_CANDELABRA,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (16 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SOLID,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 49,
            spawnstate: S_BLOODYTWITCH,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (16 * FRACUNIT),
            height: (68 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: ((MF_SOLID | MF_SPAWNCEILING) | MF_NOGRAVITY),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 50,
            spawnstate: S_MEAT2,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (16 * FRACUNIT),
            height: (84 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: ((MF_SOLID | MF_SPAWNCEILING) | MF_NOGRAVITY),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 51,
            spawnstate: S_MEAT3,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (16 * FRACUNIT),
            height: (84 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: ((MF_SOLID | MF_SPAWNCEILING) | MF_NOGRAVITY),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 52,
            spawnstate: S_MEAT4,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (16 * FRACUNIT),
            height: (68 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: ((MF_SOLID | MF_SPAWNCEILING) | MF_NOGRAVITY),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 53,
            spawnstate: S_MEAT5,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (16 * FRACUNIT),
            height: (52 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: ((MF_SOLID | MF_SPAWNCEILING) | MF_NOGRAVITY),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 59,
            spawnstate: S_MEAT2,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (84 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: (MF_SPAWNCEILING | MF_NOGRAVITY),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 60,
            spawnstate: S_MEAT4,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (68 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: (MF_SPAWNCEILING | MF_NOGRAVITY),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 61,
            spawnstate: S_MEAT3,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (52 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: (MF_SPAWNCEILING | MF_NOGRAVITY),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 62,
            spawnstate: S_MEAT5,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (52 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: (MF_SPAWNCEILING | MF_NOGRAVITY),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 63,
            spawnstate: S_BLOODYTWITCH,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (68 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: (MF_SPAWNCEILING | MF_NOGRAVITY),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 22,
            spawnstate: S_HEAD_DIE6,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: 0,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 15,
            spawnstate: S_PLAY_DIE7,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: 0,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 18,
            spawnstate: S_POSS_DIE5,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: 0,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 21,
            spawnstate: S_SARG_DIE6,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: 0,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 23,
            spawnstate: S_SKULL_DIE6,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: 0,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 20,
            spawnstate: S_TROO_DIE5,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: 0,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 19,
            spawnstate: S_SPOS_DIE5,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: 0,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 10,
            spawnstate: S_PLAY_XDIE9,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: 0,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 12,
            spawnstate: S_PLAY_XDIE9,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: 0,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 28,
            spawnstate: S_HEADSONSTICK,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (16 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SOLID,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 24,
            spawnstate: S_GIBS,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: 0,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 27,
            spawnstate: S_HEADONASTICK,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (16 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SOLID,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 29,
            spawnstate: S_HEADCANDLES,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (16 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SOLID,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 25,
            spawnstate: S_DEADSTICK,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (16 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SOLID,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 26,
            spawnstate: S_LIVESTICK,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (16 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SOLID,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 54,
            spawnstate: S_BIGTREE,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (32 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SOLID,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 70,
            spawnstate: S_BBAR1,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (16 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_SOLID,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 73,
            spawnstate: S_HANGNOGUTS,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (16 * FRACUNIT),
            height: (88 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: ((MF_SOLID | MF_SPAWNCEILING) | MF_NOGRAVITY),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 74,
            spawnstate: S_HANGBNOBRAIN,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (16 * FRACUNIT),
            height: (88 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: ((MF_SOLID | MF_SPAWNCEILING) | MF_NOGRAVITY),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 75,
            spawnstate: S_HANGTLOOKDN,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (16 * FRACUNIT),
            height: (64 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: ((MF_SOLID | MF_SPAWNCEILING) | MF_NOGRAVITY),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 76,
            spawnstate: S_HANGTSKULL,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (16 * FRACUNIT),
            height: (64 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: ((MF_SOLID | MF_SPAWNCEILING) | MF_NOGRAVITY),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 77,
            spawnstate: S_HANGTLOOKUP,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (16 * FRACUNIT),
            height: (64 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: ((MF_SOLID | MF_SPAWNCEILING) | MF_NOGRAVITY),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 78,
            spawnstate: S_HANGTNOBRAIN,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (16 * FRACUNIT),
            height: (64 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: ((MF_SOLID | MF_SPAWNCEILING) | MF_NOGRAVITY),
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 79,
            spawnstate: S_COLONGIBS,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_NOBLOCKMAP,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 80,
            spawnstate: S_SMALLPOOL,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_NOBLOCKMAP,
            raisestate: S_NULL,
        },
        mobjinfo_t {
            doomednum: 81,
            spawnstate: S_BRAINSTEM,
            spawnhealth: 1000,
            seestate: S_NULL,
            seesound: sfx_None,
            reactiontime: 8,
            attacksound: sfx_None,
            painstate: S_NULL,
            painchance: 0,
            painsound: sfx_None,
            meleestate: S_NULL,
            missilestate: S_NULL,
            deathstate: S_NULL,
            xdeathstate: S_NULL,
            deathsound: sfx_None,
            speed: 0,
            radius: (20 * FRACUNIT),
            height: (16 * FRACUNIT),
            mass: 100,
            damage: 0,
            activesound: sfx_None,
            flags: MF_NOBLOCKMAP,
            raisestate: S_NULL,
        },
    ]
};
