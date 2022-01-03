use usiagent::shogi::*;

use super::super::error::*;
use super::super::traits;
use super::reader::BestMove;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GameResult {
    Draw,
    SenteWin,
    GoteWin
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HuffmanCode {
    pub value: u8,
    pub bit_length: u8,
}

impl HuffmanCode {
    pub const BLANK: HuffmanCode = HuffmanCode { value: 0b0, bit_length: 1 };
    pub const S_FU: HuffmanCode = HuffmanCode { value: 0b1, bit_length: 4 };
    pub const S_KYOU: HuffmanCode = HuffmanCode { value: 0b11, bit_length: 6 };
    pub const S_KEI: HuffmanCode = HuffmanCode { value: 0b111, bit_length: 6 };
    pub const S_GIN: HuffmanCode = HuffmanCode { value: 0b1011, bit_length: 6 };
    pub const S_KAKU: HuffmanCode = HuffmanCode { value: 0b11111, bit_length: 8 };
    pub const S_HISHA: HuffmanCode = HuffmanCode { value: 0b111111, bit_length: 8 };
    pub const S_KIN: HuffmanCode = HuffmanCode { value: 0b1111, bit_length: 6 };
    pub const S_FUN: HuffmanCode = HuffmanCode { value: 0b1001, bit_length: 4 };
    pub const S_KYOUN: HuffmanCode = HuffmanCode { value: 0b100011, bit_length: 6 };
    pub const S_KEIN: HuffmanCode = HuffmanCode { value: 0b100111, bit_length: 6 };
    pub const S_GINN: HuffmanCode = HuffmanCode { value: 0b101011, bit_length: 6 };
    pub const S_KAKUN: HuffmanCode = HuffmanCode { value: 0b10011111, bit_length: 8 };
    pub const S_HISHAN: HuffmanCode = HuffmanCode { value: 0b10111111, bit_length: 8 };
    pub const G_FU: HuffmanCode = HuffmanCode { value: 0b101, bit_length: 4 };
    pub const G_KYOU: HuffmanCode = HuffmanCode { value: 0b10011, bit_length: 6 };
    pub const G_KEI: HuffmanCode = HuffmanCode { value: 0b10111, bit_length: 6 };
    pub const G_GIN: HuffmanCode = HuffmanCode { value: 0b11011, bit_length: 6 };
    pub const G_KAKU: HuffmanCode = HuffmanCode { value: 0b1011111, bit_length: 8 };
    pub const G_HISHA: HuffmanCode = HuffmanCode { value: 0b1111111, bit_length: 8 };
    pub const G_KIN: HuffmanCode = HuffmanCode { value: 0b101111, bit_length: 6 };
    pub const G_FUN: HuffmanCode = HuffmanCode { value: 0b1101, bit_length: 4 };
    pub const G_KYOUN: HuffmanCode = HuffmanCode { value: 0b110011, bit_length: 6 };
    pub const G_KEIN: HuffmanCode = HuffmanCode { value: 0b110111, bit_length: 6 };
    pub const G_GINN: HuffmanCode = HuffmanCode { value: 0b111011, bit_length: 6 };
    pub const G_KAKUN: HuffmanCode = HuffmanCode { value: 0b11011111, bit_length: 8 };
    pub const G_HISHAN: HuffmanCode = HuffmanCode { value: 0b11111111, bit_length: 8 };

    const MAX_BIT_LENGTH_ON_BANMEN: u8 = 8;

    pub const SM_FU: HuffmanCode = HuffmanCode { value: 0b0, bit_length: 3 };
    pub const SM_KYOU: HuffmanCode = HuffmanCode { value: 0b1, bit_length: 5 };
    pub const SM_KEI: HuffmanCode = HuffmanCode { value: 0b11, bit_length: 5 };
    pub const SM_GIN: HuffmanCode = HuffmanCode { value: 0b101, bit_length: 5 };
    pub const SM_KIN: HuffmanCode = HuffmanCode { value: 0b111, bit_length: 5 };
    pub const SM_KAKU: HuffmanCode = HuffmanCode { value: 0b11111, bit_length: 7 };
    pub const SM_HISHA: HuffmanCode = HuffmanCode { value: 0b111111, bit_length: 7 };

    pub const GM_FU: HuffmanCode = HuffmanCode { value: 0b100, bit_length: 3 };
    pub const GM_KYOU: HuffmanCode = HuffmanCode { value: 0b10001, bit_length: 5 };
    pub const GM_KEI: HuffmanCode = HuffmanCode { value: 0b10011, bit_length: 5 };
    pub const GM_GIN: HuffmanCode = HuffmanCode { value: 0b10101, bit_length: 5 };
    pub const GM_KIN: HuffmanCode = HuffmanCode { value: 0b10111, bit_length: 5 };
    pub const GM_KAKU: HuffmanCode = HuffmanCode { value: 0b1011111, bit_length: 7 };
    pub const GM_HISHA: HuffmanCode = HuffmanCode { value: 0b1111111, bit_length: 7 };

    const MAX_BIT_LENGTH_IN_MOCHIGOMA: u8 = 7;

    pub fn defined(&self) -> Result<bool,ReadError> {
        match *self {
            HuffmanCode::BLANK | HuffmanCode::S_FU | HuffmanCode::S_KYOU |
            HuffmanCode::S_KEI | HuffmanCode::S_GIN | HuffmanCode::S_KAKU |
            HuffmanCode::S_HISHA | HuffmanCode::S_KIN | HuffmanCode::S_FUN |
            HuffmanCode::S_KYOUN | HuffmanCode::S_KEIN | HuffmanCode::S_GINN |
            HuffmanCode::S_KAKUN | HuffmanCode::S_HISHAN |
            HuffmanCode::G_FU | HuffmanCode::G_KYOU |
            HuffmanCode::G_KEI | HuffmanCode::G_GIN | HuffmanCode::G_KAKU |
            HuffmanCode::G_HISHA | HuffmanCode::G_KIN | HuffmanCode::G_FUN |
            HuffmanCode::G_KYOUN | HuffmanCode::G_KEIN | HuffmanCode::G_GINN |
            HuffmanCode::G_KAKUN | HuffmanCode::G_HISHAN => {
                Ok(true)
            },
            HuffmanCode {
                bit_length: HuffmanCode::MAX_BIT_LENGTH_ON_BANMEN..=std::u8::MAX,
                ..
            } => Err(ReadError::OverMaxBitLength),
            _ => Ok(false)
        }
    }

    pub fn defined_mochigoma(&self) -> Result<bool,ReadError> {
        match *self {
            HuffmanCode::SM_FU | HuffmanCode::SM_KYOU |
            HuffmanCode::SM_KEI | HuffmanCode::SM_GIN | HuffmanCode::SM_KAKU |
            HuffmanCode::SM_HISHA | HuffmanCode::SM_KIN |
            HuffmanCode::GM_FU | HuffmanCode::GM_KYOU |
            HuffmanCode::GM_KEI | HuffmanCode::GM_GIN | HuffmanCode::GM_KAKU |
            HuffmanCode::GM_HISHA | HuffmanCode::GM_KIN => {
                Ok(true)
            },
            HuffmanCode {
                bit_length: HuffmanCode::MAX_BIT_LENGTH_IN_MOCHIGOMA..=std::u8::MAX,
                ..
            } => Err(ReadError::OverMaxBitLength),
            _ => Ok(false)
        }
    }
}
impl traits::TryFrom<&HuffmanCode> for KomaKind {
    type Error = ReadError;
    fn try_from(hc:&HuffmanCode) -> Result<KomaKind, Self::Error> {
        match hc {
            &HuffmanCode::S_FU => Ok(KomaKind::SFu),
            &HuffmanCode::S_KYOU => Ok(KomaKind::SKyou),
            &HuffmanCode::S_KEI => Ok(KomaKind::SKei),
            &HuffmanCode::S_GIN => Ok(KomaKind::SGin),
            &HuffmanCode::S_KIN => Ok(KomaKind::SKin),
            &HuffmanCode::S_KAKU => Ok(KomaKind::SKaku),
            &HuffmanCode::S_HISHA => Ok(KomaKind::SHisha),
            &HuffmanCode::S_FUN => Ok(KomaKind::SFuN),
            &HuffmanCode::S_KYOUN => Ok(KomaKind::SKyouN),
            &HuffmanCode::S_KEIN => Ok(KomaKind::SKeiN),
            &HuffmanCode::S_GINN => Ok(KomaKind::SGinN),
            &HuffmanCode::S_KAKUN => Ok(KomaKind::SKakuN),
            &HuffmanCode::S_HISHAN => Ok(KomaKind::SHishaN),
            &HuffmanCode::G_FU => Ok(KomaKind::GFu),
            &HuffmanCode::G_KYOU => Ok(KomaKind::GKyou),
            &HuffmanCode::G_KEI => Ok(KomaKind::GKei),
            &HuffmanCode::G_GIN => Ok(KomaKind::GGin),
            &HuffmanCode::G_KIN => Ok(KomaKind::GKin),
            &HuffmanCode::G_KAKU => Ok(KomaKind::GKaku),
            &HuffmanCode::G_HISHA => Ok(KomaKind::GHisha),
            &HuffmanCode::G_FUN => Ok(KomaKind::GFuN),
            &HuffmanCode::G_KYOUN => Ok(KomaKind::GKyouN),
            &HuffmanCode::G_KEIN => Ok(KomaKind::GKeiN),
            &HuffmanCode::G_GINN => Ok(KomaKind::GGinN),
            &HuffmanCode::G_KAKUN => Ok(KomaKind::GKakuN),
            &HuffmanCode::G_HISHAN => Ok(KomaKind::GHishaN),
            &HuffmanCode::BLANK => Ok(KomaKind::Blank),
            _ => Err(ReadError::Undefined)
        }
    }
}
impl traits::TryFrom<&HuffmanCode> for MochigomaKind {
    type Error = ReadError;
    fn try_from(hc:&HuffmanCode) -> Result<MochigomaKind, Self::Error> {
        match hc {
            &HuffmanCode::SM_FU | &HuffmanCode::GM_FU => Ok(MochigomaKind::Fu),
            &HuffmanCode::SM_KYOU | &HuffmanCode::GM_KYOU => Ok(MochigomaKind::Kyou),
            &HuffmanCode::SM_KEI | &HuffmanCode::GM_KEI => Ok(MochigomaKind::Kei),
            &HuffmanCode::SM_GIN | &HuffmanCode::GM_GIN => Ok(MochigomaKind::Gin),
            &HuffmanCode::SM_KAKU | &HuffmanCode::GM_KAKU => Ok(MochigomaKind::Kaku),
            &HuffmanCode::SM_HISHA | &HuffmanCode::GM_HISHA => Ok(MochigomaKind::Hisha),
            &HuffmanCode::SM_KIN | &HuffmanCode::GM_KIN => Ok(MochigomaKind::Kin),
            _ => Err(ReadError::Undefined)
        }
    }
}
impl traits::TryFrom<&HuffmanCode> for Teban {
    type Error = ReadError;
    fn try_from(hc: &HuffmanCode) -> Result<Teban, Self::Error> {
        match hc {
            &HuffmanCode::S_FU | &HuffmanCode::S_KYOU | &HuffmanCode::S_KEI |
            &HuffmanCode::S_GIN | &HuffmanCode::S_KIN | &HuffmanCode::S_KAKU |
            &HuffmanCode::S_HISHA | &HuffmanCode::S_FUN | &HuffmanCode::S_KYOUN |
            &HuffmanCode::S_KEIN | &HuffmanCode::S_GINN | &HuffmanCode::S_KAKUN |
            &HuffmanCode::S_HISHAN => {
                Ok(Teban::Sente)
            },
            &HuffmanCode::G_FU | &HuffmanCode::G_KYOU | &HuffmanCode::G_KEI |
            &HuffmanCode::G_GIN | &HuffmanCode::G_KIN | &HuffmanCode::G_KAKU |
            &HuffmanCode::G_HISHA | &HuffmanCode::G_FUN | &HuffmanCode::G_KYOUN |
            &HuffmanCode::G_KEIN | &HuffmanCode::G_GINN | &HuffmanCode::G_KAKUN |
            &HuffmanCode::G_HISHAN => {
                Ok(Teban::Gote)
            },
            &HuffmanCode::SM_FU | &HuffmanCode::SM_KYOU | &HuffmanCode::SM_KEI |
            &HuffmanCode::SM_GIN | &HuffmanCode::SM_KIN | &HuffmanCode::SM_KAKU |
            &HuffmanCode::SM_HISHA => {
                Ok(Teban::Sente)
            },
            &HuffmanCode::GM_FU | &HuffmanCode::GM_KYOU | &HuffmanCode::GM_KEI |
            &HuffmanCode::GM_GIN | &HuffmanCode::GM_KIN | &HuffmanCode::GM_KAKU |
            &HuffmanCode::GM_HISHA => {
                Ok(Teban::Gote)
            },
            _ => Err(ReadError::Undefined)
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Hcpe {
    pub buf: [u8; 32]
}
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub struct HuffmanCodedPosAndEval {
    pub packed: Hcpe,
    pub eval: i16,
    pub best_move16: u16,
    pub game_result: i8,
    pub padding: u8,
}
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub struct ExtendFields {
    pub eval: i16,
    pub best_move: BestMove,
    pub game_result: GameResult,
}