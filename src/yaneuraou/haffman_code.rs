use usiagent::shogi::*;
use usiagent::event::GameEndState;

use super::super::error::*;
use super::super::traits;
use super::reader::BestMove;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HuffmanCode {
    pub value: u8,
    pub bit_length: u8,
}

impl HuffmanCode {
    pub const BLANK: HuffmanCode = HuffmanCode{ value: 0b0, bit_length: 1 };
    pub const FU: HuffmanCode = HuffmanCode{ value: 0b1, bit_length: 2 };
    pub const KYOU: HuffmanCode = HuffmanCode{ value: 0b11, bit_length: 4 };
    pub const KEI: HuffmanCode = HuffmanCode{ value: 0b1011, bit_length: 4 };
    pub const GIN: HuffmanCode = HuffmanCode{ value: 0b111, bit_length: 4 };
    pub const KAKU: HuffmanCode = HuffmanCode{ value: 0b1_1111, bit_length: 6 };
    pub const HISHA: HuffmanCode = HuffmanCode{ value: 0b11_1111, bit_length: 6 };
    pub const KIN: HuffmanCode = HuffmanCode{ value: 0b1111, bit_length: 5 };
    const MAX_BIT_LENGTH_ON_BANMEN: u8 = 6;

    pub const M_FU: HuffmanCode = HuffmanCode{ value: 0b0, bit_length: 1 };
    pub const M_KYOU: HuffmanCode = HuffmanCode{ value: 0b1, bit_length: 3 };
    pub const M_KEI: HuffmanCode = HuffmanCode{ value: 0b101, bit_length: 3 };
    pub const M_GIN: HuffmanCode = HuffmanCode{ value: 0b11, bit_length: 3 };
    pub const M_KAKU: HuffmanCode = HuffmanCode{ value: 0b1_111, bit_length: 5 };
    pub const M_HISHA: HuffmanCode = HuffmanCode{ value: 0b11_111, bit_length: 5 };
    pub const M_KIN: HuffmanCode = HuffmanCode{ value: 0b111, bit_length: 4 };
    const MAX_BIT_LENGTH_IN_MOCHIGOMA: u8 = 5;

    pub fn defined(&self) -> Result<bool,ReadError> {
        match *self {
            HuffmanCode::BLANK | HuffmanCode::FU | HuffmanCode::KYOU |
            HuffmanCode::KEI | HuffmanCode::GIN | HuffmanCode::KAKU |
            HuffmanCode::HISHA | HuffmanCode::KIN => {
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
            HuffmanCode::M_FU | HuffmanCode::M_KYOU |
            HuffmanCode::M_KEI | HuffmanCode::M_GIN | HuffmanCode::M_KAKU |
            HuffmanCode::M_HISHA | HuffmanCode::M_KIN => {
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
type TebanAndKomaKindAndNariFlag<'a> = (Teban, bool, &'a HuffmanCode);
impl<'a> traits::TryFrom<&'a TebanAndKomaKindAndNariFlag<'a>> for KomaKind {
    type Error = ReadError;
    fn try_from(src:&'a TebanAndKomaKindAndNariFlag) -> Result<KomaKind, Self::Error> {
        match *src {
            (Teban::Sente,false,&HuffmanCode::FU) => Ok(KomaKind::SFu),
            (Teban::Sente,false,&HuffmanCode::KYOU) => Ok(KomaKind::SKyou),
            (Teban::Sente,false,&HuffmanCode::KEI) => Ok(KomaKind::SKei),
            (Teban::Sente,false,&HuffmanCode::GIN) => Ok(KomaKind::SGin),
            (Teban::Sente,false,&HuffmanCode::KIN) => Ok(KomaKind::SKin),
            (Teban::Sente,false,&HuffmanCode::KAKU) => Ok(KomaKind::SKaku),
            (Teban::Sente,false,&HuffmanCode::HISHA) => Ok(KomaKind::SHisha),
            (Teban::Sente,true,&HuffmanCode::FU) => Ok(KomaKind::SFuN),
            (Teban::Sente,true,&HuffmanCode::KYOU) => Ok(KomaKind::SKyouN),
            (Teban::Sente,true,&HuffmanCode::KEI) => Ok(KomaKind::SKeiN),
            (Teban::Sente,true,&HuffmanCode::GIN) => Ok(KomaKind::SGinN),
            (Teban::Sente,true,&HuffmanCode::KAKU) => Ok(KomaKind::SKakuN),
            (Teban::Sente,true,&HuffmanCode::HISHA) => Ok(KomaKind::SHishaN),
            (Teban::Gote,false,&HuffmanCode::FU) => Ok(KomaKind::GFu),
            (Teban::Gote,false,&HuffmanCode::KYOU) => Ok(KomaKind::GKyou),
            (Teban::Gote,false,&HuffmanCode::KEI) => Ok(KomaKind::GKei),
            (Teban::Gote,false,&HuffmanCode::GIN) => Ok(KomaKind::GGin),
            (Teban::Gote,false,&HuffmanCode::KIN) => Ok(KomaKind::GKin),
            (Teban::Gote,false,&HuffmanCode::KAKU) => Ok(KomaKind::GKaku),
            (Teban::Gote,false,&HuffmanCode::HISHA) => Ok(KomaKind::GHisha),
            (Teban::Gote,true,&HuffmanCode::FU) => Ok(KomaKind::GFuN),
            (Teban::Gote,true,&HuffmanCode::KYOU) => Ok(KomaKind::GKyouN),
            (Teban::Gote,true,&HuffmanCode::KEI) => Ok(KomaKind::GKeiN),
            (Teban::Gote,true,&HuffmanCode::GIN) => Ok(KomaKind::GGinN),
            (Teban::Gote,true,&HuffmanCode::KAKU) => Ok(KomaKind::GKakuN),
            (Teban::Gote,true,&HuffmanCode::HISHA) => Ok(KomaKind::GHishaN),
            (_,_,&HuffmanCode::BLANK) => Ok(KomaKind::Blank),
            _ => Err(ReadError::Undefined)
        }
    }
}
impl traits::TryFrom<&HuffmanCode> for MochigomaKind {
    type Error = ReadError;
    fn try_from(hc:&HuffmanCode) -> Result<MochigomaKind, Self::Error> {
        match *hc {
            HuffmanCode::M_FU => Ok(MochigomaKind::Fu),
            HuffmanCode::M_KYOU => Ok(MochigomaKind::Kyou),
            HuffmanCode::M_KEI => Ok(MochigomaKind::Kei),
            HuffmanCode::M_GIN => Ok(MochigomaKind::Gin),
            HuffmanCode::M_KAKU => Ok(MochigomaKind::Kaku),
            HuffmanCode::M_HISHA => Ok(MochigomaKind::Hisha),
            HuffmanCode::M_KIN => Ok(MochigomaKind::Kin),
            _ => Err(ReadError::Undefined)
        }
    }
}
pub struct PackedSfen {
    pub buf: [u8; 32]
}
#[repr(C)]
pub struct PackedSfenWithExtended {
    pub packed: PackedSfen,
    pub value: i16,
    pub best_move16: u16,
    pub end_ply: u16,
    pub game_result: i8,
    pub padding: u8,
}
#[repr(C)]
pub struct ExtendFields {
    pub value: i16,
    pub best_move: BestMove,
    pub end_ply: u16,
    pub game_result: GameEndState,
}
