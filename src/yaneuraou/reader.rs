use usiagent::shogi::*;
use usiagent::event::GameEndState;

use crate::BitStreamReader;
use super::super::error::*;
use super::super::traits;
use super::haffman_code::ExtendFields;
use crate::yaneuraou::haffman_code::{HuffmanCode, PackedSfenWithExtended, PackedSfen};
use crate::traits::TryFrom;
use std::collections::HashMap;

const MOVE_NONE:u16 = 0;
const MOVE_NULL:u16 = (1 << 7) + 1;
const MOVE_RESIGN:u16 = (1 << 7) + 2;
const MOVE_WIN:u16 = (1 << 7) + 3;

const MOVE_DROP:u16 = 1 << 14;
const MOVE_PROMOTE:u16 = 1 << 15;

const MOCHIGOMA_MAP:[MochigomaKind; 7] = [
    MochigomaKind::Fu,
    MochigomaKind::Kyou,
    MochigomaKind::Kei,
    MochigomaKind::Gin,
    MochigomaKind::Kaku,
    MochigomaKind::Hisha,
    MochigomaKind::Kin
];

pub enum BeseMove {
    None,
    Null,
    Resign,
    Win,
    MoveTo(u32,u32,u32,u32,bool),
    MovePut(MochigomaKind,u32,u32)
}
impl traits::TryFrom<u16> for BeseMove {
    type Error = ReadError;
    fn try_from(value: u16) -> Result<BeseMove,ReadError> {
        Ok(match value {
            MOVE_NONE => BeseMove::None,
            MOVE_NULL => BeseMove::Null,
            MOVE_RESIGN => BeseMove::Resign,
            MOVE_WIN => BeseMove::Win,
            v => {
                if v & MOVE_DROP != 0 {
                    let index = (v >> 7) & 0x7f;

                    if index < 1 || index > 7 {
                        return Err(ReadError::InvalidFormat(String::from("piece kind is invalid.")))
                    }

                    let kind = MOCHIGOMA_MAP[index as usize - 1];

                    let sq = v & 0x7f;

                    let y = sq / 9;
                    let x = sq - 9 * y;

                    BeseMove::MovePut(kind,x as u32, y as u32)
                } else {
                    let n = v & MOVE_PROMOTE != 0;

                    let sq = (v >> 7) & 0x7f;
                    let sy = sq / 9;
                    let sx = sq - 9 * sy;

                    let sq = v & 0x7f;

                    let dy = sq / 9;
                    let dx = sq - 9 * dy;

                    BeseMove::MoveTo(sx as u32, sy as u32, dx as u32, dy as u32, n)
                }
            }
        })
    }
}
pub struct PackedSfenReader {
}
impl PackedSfenReader {
    pub fn new() -> PackedSfenReader {
        PackedSfenReader {
        }
    }


    fn read_raw_sfen_with_extended(&mut self, buf:Vec<u8>) -> Result<PackedSfenWithExtended,ReadError> {
        if buf.len() != 40 {
            Err(ReadError::InvalidFormat(String::from("input size is incorrect.")))
        } else {
            let mut buf = buf;
            let remained = buf.split_off(32);

            let packed_sfen = buf;

            let mut buf = remained;

            let remained = buf.split_off(2);

            let mut score_value:i16 = buf[0] as i16;
            score_value = score_value << 8 | buf[1] as i16;

            let mut buf = remained;

            let remained = buf.split_off(2);

            let mut best_move16 = buf[0] as u16;
            best_move16 = best_move16 << 8 | buf[1] as u16;

            let mut buf = remained;

            let remained = buf.split_off(2);

            let mut end_play = buf[0] as u16;
            end_play = end_play << 8 | buf[1] as u16;

            let mut buf = remained;

            let _ = buf.split_off(1);

            let game_result:i8 = buf[0] as i8;

            let mut sfen_buf = [0; 32];

            for i in 0..32 {
                sfen_buf[i] = packed_sfen[i];
            }

            Ok(PackedSfenWithExtended {
                packed: PackedSfen { buf: sfen_buf },
                value: score_value,
                best_move16: best_move16,
                end_ply: end_play,
                game_result: game_result,
                padding: 0
            })
        }
    }
}
impl<'a> traits::Reader<ExtendFields> for PackedSfenReader {
    fn read_sfen(&mut self, buf:&[u8]) -> Result<(Teban,Banmen,MochigomaCollections),ReadError> {
        if buf.len() != 32 {
            Err(ReadError::InvalidFormat(String::from("input size is incorrect.")))
        } else {
            let mut bs = BitStreamReader::new(buf);

            let mut banmen = Banmen([[KomaKind::Blank; 9]; 9]);
            let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();
            let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

            let teban = if bs.get_bit_from_lsb()? == 0 {
                Teban::Sente
            } else {
                Teban::Gote
            };

            let sq = bs.get_bits_from_lsb(7)? as u32;
            let y = sq as usize / 9;
            let x = sq as usize - 9 * y;

            banmen.0[y][x] = KomaKind::SOu;

            let sq = bs.get_bits_from_lsb(7)? as u32;
            let y = sq as usize / 9;
            let x = sq as usize - 9 * y;

            banmen.0[y][x] = KomaKind::GOu;

            for y in 0..9 {
                for x in 0..9 {
                    let mut hc = HuffmanCode { value: 0, bit_length: 0 };

                    loop {
                        hc.value |= bs.get_bit_from_lsb()? << hc.bit_length;
                        hc.bit_length += 1;

                        match hc.defined() {
                            Ok(true) if hc == HuffmanCode::BLANK => {
                                if banmen.0[y][x] == KomaKind::SOu || banmen.0[y][x] == KomaKind::GOu {
                                    break;
                                } else {
                                    banmen.0[y][x] = KomaKind::Blank;
                                    break;
                                }
                            },
                            Ok(true) => {
                                let nari = if hc != HuffmanCode::KIN && bs.get_bit_from_lsb()? == 1 {
                                    true
                                } else {
                                    false
                                };

                                let teban = if bs.get_bit_from_lsb()? == 0 {
                                    Teban::Sente
                                } else {
                                    Teban::Gote
                                };

                                let kind = KomaKind::try_from(&(teban,nari,&hc))?;

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
                            if hc != HuffmanCode::KIN {
                                let _ = bs.get_bit_from_lsb()?;
                            }

                            let teban = if bs.get_bit_from_lsb()? == 0 {
                                Teban::Sente
                            } else {
                                Teban::Gote
                            };

                            let kind = MochigomaKind::try_from(&hc)?;

                            if teban == Teban::Sente {
                                let c = ms.get(&kind).map(|&k| k).unwrap_or(0);

                                ms.insert(kind,c+1);
                            } else {
                                let c = mg.get(&kind).map(|&k| k).unwrap_or(0);

                                mg.insert(kind,c+1);
                            }
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

        let game_result = if raw_packed_sfen_with_extended.game_result == 1 {
            GameEndState::Win
        } else if raw_packed_sfen_with_extended.game_result == -1 {
            GameEndState::Lose
        } else {
            GameEndState::Draw
        };

        let gamestate = self.read_sfen(&raw_packed_sfen_with_extended.packed.buf)?;

        Ok((gamestate,ExtendFields {
            value: raw_packed_sfen_with_extended.value,
            best_move: BeseMove::try_from(raw_packed_sfen_with_extended.best_move16)?,
            end_ply: raw_packed_sfen_with_extended.end_ply,
            game_result:  game_result
        }))
    }
}