use usiagent::shogi::*;

use crate::BitStreamReader;
use super::super::error::*;
use super::super::traits;
use super::haffman_code::ExtendFields;
use crate::hcpe::haffman_code::{HuffmanCode, HuffmanCodedPosAndEval, Hcpe, GameResult};
use crate::traits::TryFrom;

const MOVE_NONE:u16 = 0;
const MOVE_NULL:u16 = 129;
const MOVE_PVS_END:u16 = 1 << 15;
const MOVE_PROMOTE:u16 = 1 << 14;

const DROP_FROM_ORIGN:u16 = 81;

const MOCHIGOMA_MAP:[MochigomaKind; 7] = [
    MochigomaKind::Fu,
    MochigomaKind::Kyou,
    MochigomaKind::Kei,
    MochigomaKind::Gin,
    MochigomaKind::Kaku,
    MochigomaKind::Hisha,
    MochigomaKind::Kin
];

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BestMove {
    None,
    Null,
    MovePVsEnd,
    MoveTo(u32,u32,u32,u32,bool),
    MovePut(MochigomaKind,u32,u32)
}
impl traits::TryFrom<u16> for BestMove {
    type Error = ReadError;
    fn try_from(value: u16) -> Result<BestMove,ReadError> {
        Ok(match value {
            MOVE_NONE => BestMove::None,
            MOVE_NULL => BestMove::Null,
            MOVE_PVS_END => BestMove::MovePVsEnd,
            v => {
                let from = (v >> 7) & 0x7f;

                if from > 80 {
                    let index = from - DROP_FROM_ORIGN;

                    if index >= 7 {
                        return Err(ReadError::InvalidFormat(String::from("piece kind is invalid.")))
                    }

                    let kind = MOCHIGOMA_MAP[index as usize];

                    let sq = v & 0x7f;

                    if sq > 80 {
                        return Err(ReadError::InvalidFormat(String::from("move put position is invalid.")))
                    }

                    let x = sq / 9;
                    let y = sq - 9 * x;

                    BestMove::MovePut(kind, x as u32, y as u32)
                } else {
                    let n = v & MOVE_PROMOTE != 0;

                    let sq = from;

                    if sq > 80 {
                        return Err(ReadError::InvalidFormat(String::from("move from position is invalid.")))
                    }

                    let sx = sq / 9;
                    let sy = sq - 9 * sx;

                    let sq = v & 0x7f;

                    if sq > 80 {
                        return Err(ReadError::InvalidFormat(String::from("move to position is invalid.")))
                    }

                    let dx = sq / 9;
                    let dy = sq - 9 * dx;

                    BestMove::MoveTo(sx as u32, sy as u32, dx as u32, dy as u32, n)
                }
            }
        })
    }
}
pub struct HcpeReader {
}
impl HcpeReader {
    pub fn new() -> HcpeReader {
        HcpeReader {
        }
    }

    fn read_raw_sfen_with_extended(&mut self, buf:Vec<u8>) -> Result<HuffmanCodedPosAndEval,ReadError> {
        if buf.len() != 38 {
            Err(ReadError::InvalidFormat(String::from("input size is incorrect.")))
        } else {
            let mut buf = buf;
            let remained = buf.split_off(32);

            let packed_sfen = buf;

            let mut buf = remained;

            let remained = buf.split_off(2);

            let score_value:i16 = buf[0] as i16 | (buf[1] as i16) << 8;

            let mut buf = remained;

            let remained = buf.split_off(2);

            let best_move16 = buf[0] as u16 | (buf[1] as u16) << 8;

            let mut buf = remained;

            let _ = buf.split_off(1);

            let game_result:i8 = buf[0] as i8;

            let mut sfen_buf = [0; 32];

            for i in 0..32 {
                sfen_buf[i] = packed_sfen[i];
            }

            Ok(HuffmanCodedPosAndEval {
                packed: Hcpe { buf: sfen_buf },
                eval: score_value,
                best_move16: best_move16,
                game_result: game_result,
                padding: 0
            })
        }
    }
}
impl traits::Reader<ExtendFields> for HcpeReader {
    fn read_sfen(&mut self, buf:&[u8]) -> Result<(Teban,Banmen,MochigomaCollections),ReadError> {
        if buf.len() != 32 {
            Err(ReadError::InvalidFormat(String::from("input size is incorrect.")))
        } else {
            let mut bs = BitStreamReader::new(buf);

            let mut banmen = Banmen([[KomaKind::Blank; 9]; 9]);
            let mut ms:Mochigoma = Mochigoma::new();
            let mut mg:Mochigoma = Mochigoma::new();

            let teban = if bs.get_bit_from_lsb()? == 0 {
                Teban::Sente
            } else {
                Teban::Gote
            };

            let sq = bs.get_bits_from_lsb(7)? as u32;
            let x = sq as usize / 9;
            let y = sq as usize - 9 * x;

            banmen.0[y][x] = KomaKind::SOu;

            let sq = bs.get_bits_from_lsb(7)? as u32;
            let x = sq as usize / 9;
            let y = sq as usize - 9 * x;

            banmen.0[y][x] = KomaKind::GOu;

            for x in 0..9 {
                for y in 0..9 {
                    if banmen.0[y][x] == KomaKind::SOu || banmen.0[y][x] == KomaKind::GOu {
                        continue;
                    }

                    let mut hc = HuffmanCode { value: 0, bit_length: 0 };

                    loop {
                        hc.value |= bs.get_bit_from_lsb()? << hc.bit_length;
                        hc.bit_length += 1;

                        match hc.defined() {
                            Ok(true) => {
                                let kind = KomaKind::try_from(&hc)?;

                                if banmen.0[y][x] == KomaKind::SOu || banmen.0[y][x] == KomaKind::GOu {
                                    break;
                                } else {
                                    banmen.0[y][x] = kind;
                                    break;
                                }
                            },
                            Ok(false) => (),
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    }
                }
            }

            while bs.get_cursor() < 256 {
                let mut hc = HuffmanCode { value: 0, bit_length: 0 };

                loop {
                    hc.value |= bs.get_bit_from_lsb()? << hc.bit_length;
                    hc.bit_length += 1;

                    match hc.defined_mochigoma() {
                        Ok(true) => {
                            let kind = MochigomaKind::try_from(&hc)?;
                            let teban = Teban::try_from(&hc)?;

                            if teban == Teban::Sente {
                                ms.put(kind);
                            } else {
                                mg.put(kind);
                            }
                            break;
                        },
                        Ok(false) => (),
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }
            }

            Ok((teban,banmen,MochigomaCollections::Pair(ms,mg)))
        }
    }

    fn read_sfen_with_extended(&mut self, buf:Vec<u8>) -> Result<((Teban,Banmen,MochigomaCollections),ExtendFields),ReadError> {
        let raw_packed_sfen_with_extended = self.read_raw_sfen_with_extended(buf)?;

        let game_result = match raw_packed_sfen_with_extended.game_result {
            0 => GameResult::Draw,
            1 => GameResult::SenteWin,
            2 => GameResult::GoteWin,
            r => {
                return Err(ReadError::InvalidFormat(format!("unknown game_result {}.",r)));
            }
        };

        let gamestate = self.read_sfen(&raw_packed_sfen_with_extended.packed.buf)?;

        Ok((gamestate,ExtendFields {
            eval: raw_packed_sfen_with_extended.eval,
            best_move: BestMove::try_from(raw_packed_sfen_with_extended.best_move16)?,
            game_result:  game_result
        }))
    }
}