use packedsfen::yaneuraou::reader::PackedSfenReader;
use packedsfen::traits::Reader;
use packedsfen::yaneuraou::reader::BestMove;

use usiagent::shogi::{Teban, Banmen, MochigomaCollections, MochigomaKind};
use usiagent::rule::BANMEN_START_POS;
use std::collections::HashMap;

use usiagent::shogi::KomaKind::{
    SFu,
    SKyou,
    SKei,
    SGin,
    SKin,
    SKaku,
    SHisha,
    SOu,
    SFuN,
    SKyouN,
    SKeiN,
    SGinN,
    SKakuN,
    SHishaN,
    GFu,
    GKyou,
    GKei,
    GGin,
    GKin,
    GKaku,
    GHisha,
    GOu,
    GFuN,
    GKyouN,
    GKeiN,
    GGinN,
    GKakuN,
    GHishaN,
    Blank
};
use packedsfen::yaneuraou::haffman_code::ExtendFields;
use usiagent::event::GameEndState;

#[test]
fn test_read_sfen_teban_sente_initial_position() {
    let mut reader = PackedSfenReader::new();

    let input:[u8; 32] = [
        0b1001100_0,0b1_0000100,0b011_10001,0b00111_101,0b1_101111_1,0b111_10111,0b01011_100,0b0_100011_1,
        0b10111111,0b111_00000,0b01_0_10011,0b01_1001_10,0b01_1001_10,0b01_1001_10,0b01_1001_10,0b000000_10,
        0b00000000,0b00000000,0b001_00000,
        0b001_0001_0,0b001_0001_0,0b001_0001_0,0b001_0001_0,0b011111_0_0,0b1_00000_00,
        0b0_0011111,0b11_000011,0b0111_0010,0b001111_00,0b11_001111,0b1011_0001,0b000011_00
    ];

    let (teban,banmen,mc) = reader.read_sfen(&input).unwrap();

    assert_eq!(teban,Teban::Sente);
    assert_eq!(banmen,BANMEN_START_POS);
    assert_eq!(mc,MochigomaCollections::Pair(HashMap::new(),HashMap::new()));
}

#[test]
fn test_read_sfen_teban_gote_initial_position() {
    let mut reader = PackedSfenReader::new();

    let input:[u8; 32] = [
        0b1001100_1,0b1_0000100,0b011_10001,0b00111_101,0b1_101111_1,0b111_10111,0b01011_100,0b0_100011_1,
        0b10111111,0b111_00000,0b01_0_10011,0b01_1001_10,0b01_1001_10,0b01_1001_10,0b01_1001_10,0b000000_10,
        0b00000000,0b00000000,0b001_00000,
        0b001_0001_0,0b001_0001_0,0b001_0001_0,0b001_0001_0,0b011111_0_0,0b1_00000_00,
        0b0_0011111,0b11_000011,0b0111_0010,0b001111_00,0b11_001111,0b1011_0001,0b000011_00
    ];

    let (teban,banmen,mc) = reader.read_sfen(&input).unwrap();

    assert_eq!(teban,Teban::Gote);
    assert_eq!(banmen,BANMEN_START_POS);
    assert_eq!(mc,MochigomaCollections::Pair(HashMap::new(),HashMap::new()));
}

#[test]
fn test_read_sfen_teban_sente_sente_hisha_kaku_nari_and_one_nari() {
    let mut reader = PackedSfenReader::new();

    let input:[u8; 32] = [
        0b1001100_0,0b1_0000100,0b011_11001,0b10111_111,0b1_101111_1,0b111_10111,0b01011_100,0b0_100011_1,
        0b10111111,0b111_00000,0b01_0_10011,0b01_1001_11,0b01_1001_10,0b01_1001_10,0b01_1001_10,0b000000_10,
        0b00000000,0b00000000,0b001_00000,
        0b001_0001_0,0b001_0001_0,0b001_0001_0,0b101_0001_0,0b011111_0_0,0b1_00000_01,
        0b0_0111111,0b11_000011,0b0111_0010,0b001111_00,0b11_001111,0b1011_0101,0b010011_01
    ];

    let (teban,banmen,mc) = reader.read_sfen(&input).unwrap();

    assert_eq!(teban,Teban::Sente);
    assert_eq!(banmen,Banmen([
        [GKyouN, GKeiN, GGinN, GKin, GOu, GKin, GGin, GKei, GKyou],
        [Blank, GHisha, Blank, Blank, Blank, Blank, Blank, GKaku, Blank],
        [GFuN, GFu, GFu, GFu, GFu, GFu, GFu, GFu, GFu],
        [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
        [SFu, SFu, SFu, SFu, SFu, SFu, SFu, SFu, SFuN],
        [Blank, SKakuN, Blank, Blank, Blank, Blank, Blank, SHishaN, Blank],
        [SKyou, SKei, SGin, SKin, SOu, SKin, SGinN, SKeiN, SKyouN]
    ]));
    assert_eq!(mc,MochigomaCollections::Pair(HashMap::new(),HashMap::new()));
}


#[test]
fn test_read_sfen_teban_gote_sente_hisha_kaku_nari_and_one_nari() {
    let mut reader = PackedSfenReader::new();

    let input:[u8; 32] = [
        0b1001100_1,0b1_0000100,0b011_11001,0b10111_111,0b1_101111_1,0b111_10111,0b01011_100,0b0_100011_1,
        0b11111111,0b111_00000,0b01_0_11011,0b01_1001_11,0b01_1001_10,0b01_1001_10,0b01_1001_10,0b000000_10,
        0b00000000,0b00000000,0b001_00000,
        0b001_0001_0,0b001_0001_0,0b001_0001_0,0b101_0001_0,0b011111_0_0,0b1_00000_00,
        0b0_0011111,0b11_000011,0b0111_0010,0b001111_00,0b11_001111,0b1011_0101,0b010011_01
    ];

    let (teban,banmen,mc) = reader.read_sfen(&input).unwrap();

    assert_eq!(teban,Teban::Gote);
    assert_eq!(banmen,Banmen([
        [GKyouN, GKeiN, GGinN, GKin, GOu, GKin, GGin, GKei, GKyou],
        [Blank, GHishaN, Blank, Blank, Blank, Blank, Blank, GKakuN, Blank],
        [GFuN, GFu, GFu, GFu, GFu, GFu, GFu, GFu, GFu],
        [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
        [SFu, SFu, SFu, SFu, SFu, SFu, SFu, SFu, SFuN],
        [Blank, SKaku, Blank, Blank, Blank, Blank, Blank, SHisha, Blank],
        [SKyou, SKei, SGin, SKin, SOu, SKin, SGinN, SKeiN, SKyouN]
    ]));
    assert_eq!(mc,MochigomaCollections::Pair(HashMap::new(),HashMap::new()));
}

#[test]
fn test_read_sfen_teban_sente_mochigoma_half() {
    let mut reader = PackedSfenReader::new();

    let input:[u8; 32] = [
        0b1001100_0,0b1_0000100,0b011_10001,0b00111_101,0b1_101111_1,0b111_10111,0b01011_100,0b0_100011_1,
        0b10111111,0b111_00000,0b01_0_10011,0b01_1001_10,0b01_1001_10,0b01_1001_10,0b01_1001_10,0b000000_10,
        0b00000000,0b00000000,0b00000000,0b00000000,0b00000000,0b0_0000000,0b000_010_00,0b10_000_010,
        0b0_010_000_0,0b1_00001_00,0b0101_0100,0b11_01101_0,0b01011_000,0b111_00111,0b011111_00,0b0001111_0
    ];

    let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();

    ms.insert(MochigomaKind::Fu,9);
    ms.insert(MochigomaKind::Kyou,2);
    ms.insert(MochigomaKind::Kei,2);
    ms.insert(MochigomaKind::Gin,2);
    ms.insert(MochigomaKind::Kin,2);
    ms.insert(MochigomaKind::Hisha, 1);
    ms.insert(MochigomaKind::Kaku,1);

    let emc = MochigomaCollections::Pair(ms,HashMap::new());

    let (teban,banmen,mc) = reader.read_sfen(&input).unwrap();

    assert_eq!(teban,Teban::Sente);
    assert_eq!(banmen,Banmen([
        [GKyou, GKei, GGin, GKin, GOu, GKin, GGin, GKei, GKyou],
        [Blank, GHisha, Blank, Blank, Blank, Blank, Blank, GKaku, Blank],
        [GFu, GFu, GFu, GFu, GFu, GFu, GFu, GFu, GFu],
        [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, SOu, Blank, Blank, Blank, Blank]
    ]));
    assert_eq!(mc,emc);
}

#[test]
fn test_read_sfen_teban_gote_mochigoma_half() {
    let mut reader = PackedSfenReader::new();

    let input:[u8; 32] = [
        0b1001100_1,0b0_0000100,0b00000000,0b00000000,0b00000000,0b00000000,0b00000000,0b00000000,
        0b0001_0000,0b0001_0001,0b0001_0001,0b0001_0001,0b0001_0001,0b0011111_0,
        0b11_00000_0,0b1_0_001111,0b011_00001,0b00111_001,0b1_001111_0,0b111_00111,0b01011_000,
        0b0_000011_0,
        0b110_100_11,0b10_110_100,0b0_100_110_1,0b1_10001_11,0b0101_1100,0b11_11101_1,0b10011_110,
        0b111_10111,0b101111_10,0b1111111_1
    ];

    let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

    mg.insert(MochigomaKind::Fu,9);
    mg.insert(MochigomaKind::Kyou,2);
    mg.insert(MochigomaKind::Kei,2);
    mg.insert(MochigomaKind::Gin,2);
    mg.insert(MochigomaKind::Kin,2);
    mg.insert(MochigomaKind::Hisha, 1);
    mg.insert(MochigomaKind::Kaku,1);

    let emc = MochigomaCollections::Pair(HashMap::new(),mg);

    let (teban,banmen,mc) = reader.read_sfen(&input).unwrap();

    assert_eq!(teban,Teban::Gote);
    assert_eq!(banmen,Banmen([
        [Blank, Blank, Blank, Blank, GOu, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
        [SFu, SFu, SFu, SFu, SFu, SFu, SFu, SFu, SFu],
        [Blank, SKaku, Blank, Blank, Blank, Blank, Blank, SHisha, Blank],
        [SKyou, SKei, SGin, SKin, SOu, SKin, SGin, SKei, SKyou]
    ]));
    assert_eq!(mc,emc);
}

#[test]
fn test_read_sfen_teban_sente_mochigoma_full() {
    let mut reader = PackedSfenReader::new();

    let input:[u8; 32] = [
        0b1001100_0,0b0_0000100,
        0b00000000,0b00000000,0b00000000,0b00000000,0b00000000,0b00000000,0b00000000,0b00000000,
        0b00000000,0b00_000000,
        0b0_000_010_0,0b010_000_01,
        0b00_010_000,0b0_000_010_0,0b010_000_01,0b00_010_000,
        0b0001_010_0,0b01_01001_0,0b01001_000,0b101_00101,0b1_00101_01,0b0011_0110,
        0b11_01011_0,0b01011_000,0b111_00111,0b1_00111_00,0b1111_0011,
        0b01111_000,0b011111_01,0b0111111_0
    ];

    let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();

    ms.insert(MochigomaKind::Fu,18);
    ms.insert(MochigomaKind::Kyou,4);
    ms.insert(MochigomaKind::Kei,4);
    ms.insert(MochigomaKind::Gin,4);
    ms.insert(MochigomaKind::Kin,4);
    ms.insert(MochigomaKind::Hisha, 2);
    ms.insert(MochigomaKind::Kaku,2);

    let mg:HashMap<MochigomaKind,u32> = HashMap::new();

    let emc = MochigomaCollections::Pair(ms,mg);

    let (teban,banmen,mc) = reader.read_sfen(&input).unwrap();

    assert_eq!(teban,Teban::Sente);
    assert_eq!(banmen,Banmen([
        [Blank, Blank, Blank, Blank, GOu, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, SOu, Blank, Blank, Blank, Blank]
    ]));
    assert_eq!(mc,emc);
}

#[test]
fn test_read_sfen_teban_gote_mochigoma_full() {
    let mut reader = PackedSfenReader::new();

    let input:[u8; 32] = [
        0b1001100_1,0b0_0000100,
        0b00000000,0b00000000,0b00000000,0b00000000,0b00000000,0b00000000,0b00000000,0b00000000,
        0b00000000,0b00_000000,
        0b0_100_110_1,0b110_100_11,
        0b00_110_100,0b0_100_110_1,0b110_100_11,0b00_110_100,
        0b0001_110_1,0b01_11001_1,0b11001_100,0b101_10101,0b1_10101_11,0b0011_1110,
        0b11_11011_1,0b11011_100,0b111_10111,0b1_10111_10,0b1111_1011,
        0b01111_100,0b011111_11,0b1111111_1
    ];

    let ms:HashMap<MochigomaKind,u32> = HashMap::new();

    let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

    mg.insert(MochigomaKind::Fu,18);
    mg.insert(MochigomaKind::Kyou,4);
    mg.insert(MochigomaKind::Kei,4);
    mg.insert(MochigomaKind::Gin,4);
    mg.insert(MochigomaKind::Kin,4);
    mg.insert(MochigomaKind::Hisha, 2);
    mg.insert(MochigomaKind::Kaku,2);

    let emc = MochigomaCollections::Pair(ms,mg);

    let (teban,banmen,mc) = reader.read_sfen(&input).unwrap();

    assert_eq!(teban,Teban::Gote);
    assert_eq!(banmen,Banmen([
        [Blank, Blank, Blank, Blank, GOu, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, SOu, Blank, Blank, Blank, Blank]
    ]));
    assert_eq!(mc,emc);
}

#[test]
fn test_read_sfen_teban_sente_mochigoma_half_and_half() {
    let mut reader = PackedSfenReader::new();

    let input:[u8; 32] = [
        0b1001100_0,0b0_0000100,
        0b00000000,0b00000000,0b00000000,0b00000000,0b00000000,0b00000000,0b00000000,0b00000000,
        0b00000000,0b00_000000,
        0b0_000_010_0,0b010_000_01,
        0b00_010_000,0b0_100_110_0,0b110_100_11,0b00_110_100,
        0b0001_110_1,0b01_01001_0,0b11001_100,0b101_00101,0b1_10101_01,0b0011_1110,
        0b11_01011_0,0b11011_100,0b111_00111,0b1_10111_00,0b1111_1011,
        0b01111_000,0b011111_11,0b1111111_0
    ];

    let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();

    ms.insert(MochigomaKind::Fu,9);
    ms.insert(MochigomaKind::Kyou,2);
    ms.insert(MochigomaKind::Kei,2);
    ms.insert(MochigomaKind::Gin,2);
    ms.insert(MochigomaKind::Kin,2);
    ms.insert(MochigomaKind::Hisha, 1);
    ms.insert(MochigomaKind::Kaku,1);

    let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

    mg.insert(MochigomaKind::Fu,9);
    mg.insert(MochigomaKind::Kyou,2);
    mg.insert(MochigomaKind::Kei,2);
    mg.insert(MochigomaKind::Gin,2);
    mg.insert(MochigomaKind::Kin,2);
    mg.insert(MochigomaKind::Hisha, 1);
    mg.insert(MochigomaKind::Kaku,1);

    let emc = MochigomaCollections::Pair(ms,mg);

    let (teban,banmen,mc) = reader.read_sfen(&input).unwrap();

    assert_eq!(teban,Teban::Sente);
    assert_eq!(banmen,Banmen([
        [Blank, Blank, Blank, Blank, GOu, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, SOu, Blank, Blank, Blank, Blank]
    ]));
    assert_eq!(mc,emc);
}

#[test]
fn test_read_sfen_teban_gote_mochigoma_half_and_half() {
    let mut reader = PackedSfenReader::new();

    let input:[u8; 32] = [
        0b1001100_1,0b0_0000100,
        0b00000000,0b00000000,0b00000000,0b00000000,0b00000000,0b00000000,0b00000000,0b00000000,
        0b00000000,0b00_000000,
        0b0_000_010_0,0b010_000_01,
        0b00_010_000,0b0_100_110_0,0b110_100_11,0b00_110_100,
        0b0001_110_1,0b01_01001_0,0b11001_100,0b101_00101,0b1_10101_01,0b0011_1110,
        0b11_01011_0,0b11011_100,0b111_00111,0b1_10111_00,0b1111_1011,
        0b01111_000,0b011111_11,0b1111111_0
    ];

    let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();

    ms.insert(MochigomaKind::Fu,9);
    ms.insert(MochigomaKind::Kyou,2);
    ms.insert(MochigomaKind::Kei,2);
    ms.insert(MochigomaKind::Gin,2);
    ms.insert(MochigomaKind::Kin,2);
    ms.insert(MochigomaKind::Hisha, 1);
    ms.insert(MochigomaKind::Kaku,1);

    let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

    mg.insert(MochigomaKind::Fu,9);
    mg.insert(MochigomaKind::Kyou,2);
    mg.insert(MochigomaKind::Kei,2);
    mg.insert(MochigomaKind::Gin,2);
    mg.insert(MochigomaKind::Kin,2);
    mg.insert(MochigomaKind::Hisha, 1);
    mg.insert(MochigomaKind::Kaku,1);

    let emc = MochigomaCollections::Pair(ms,mg);

    let (teban,banmen,mc) = reader.read_sfen(&input).unwrap();

    assert_eq!(teban,Teban::Gote);
    assert_eq!(banmen,Banmen([
        [Blank, Blank, Blank, Blank, GOu, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
        [Blank, Blank, Blank, Blank, SOu, Blank, Blank, Blank, Blank]
    ]));
    assert_eq!(mc,emc);
}

#[test]
fn test_read_sfen_with_extended_test_score_value_is_min() {
    let mut reader = PackedSfenReader::new();

    let input:Vec<u8> = vec![
        0b1001100_0,0b1_0000100,0b011_10001,0b00111_101,0b1_101111_1,0b111_10111,0b01011_100,0b0_100011_1,
        0b10111111,0b111_00000,0b01_0_10011,0b01_1001_10,0b01_1001_10,0b01_1001_10,0b01_1001_10,0b000000_10,
        0b00000000,0b00000000,0b001_00000,
        0b001_0001_0,0b001_0001_0,0b001_0001_0,0b001_0001_0,0b011111_0_0,0b1_00000_00,
        0b0_0011111,0b11_000011,0b0111_0010,0b001111_00,0b11_001111,0b1011_0001,0b000011_00,
        0b11111111,0b11111111,0,0,0,0,1,0
    ];

    let ((teban,banmen,mc),ExtendFields {
        value,
        best_move,
        end_ply,
        game_result
    }) = reader.read_sfen_with_extended(input).unwrap();

    assert_eq!(teban,Teban::Sente);
    assert_eq!(banmen,BANMEN_START_POS);
    assert_eq!(mc,MochigomaCollections::Pair(HashMap::new(),HashMap::new()));
    assert_eq!(value,-1);
    assert_eq!(best_move,BestMove::None);
    assert_eq!(end_ply,0);
    assert_eq!(game_result,GameEndState::Win);
}


#[test]
fn test_read_sfen_with_extended_test_score_value_is_max() {
    let mut reader = PackedSfenReader::new();

    let input:Vec<u8> = vec![
        0b1001100_0,0b1_0000100,0b011_10001,0b00111_101,0b1_101111_1,0b111_10111,0b01011_100,0b0_100011_1,
        0b10111111,0b111_00000,0b01_0_10011,0b01_1001_10,0b01_1001_10,0b01_1001_10,0b01_1001_10,0b000000_10,
        0b00000000,0b00000000,0b001_00000,
        0b001_0001_0,0b001_0001_0,0b001_0001_0,0b001_0001_0,0b011111_0_0,0b1_00000_00,
        0b0_0011111,0b11_000011,0b0111_0010,0b001111_00,0b11_001111,0b1011_0001,0b000011_00,
        0b11111111,0b01111111,0,0,0,0,1,0
    ];

    let ((teban,banmen,mc),ExtendFields {
        value,
        best_move,
        end_ply,
        game_result
    }) = reader.read_sfen_with_extended(input).unwrap();

    assert_eq!(teban,Teban::Sente);
    assert_eq!(banmen,BANMEN_START_POS);
    assert_eq!(mc,MochigomaCollections::Pair(HashMap::new(),HashMap::new()));
    assert_eq!(value,std::i16::MAX);
    assert_eq!(best_move,BestMove::None);
    assert_eq!(end_ply,0);
    assert_eq!(game_result,GameEndState::Win);
}

#[test]
fn test_read_sfen_with_extended_test_bestmove_null() {
    let mut reader = PackedSfenReader::new();

    let input:Vec<u8> = vec![
        0b1001100_0,0b1_0000100,0b011_10001,0b00111_101,0b1_101111_1,0b111_10111,0b01011_100,0b0_100011_1,
        0b10111111,0b111_00000,0b01_0_10011,0b01_1001_10,0b01_1001_10,0b01_1001_10,0b01_1001_10,0b000000_10,
        0b00000000,0b00000000,0b001_00000,
        0b001_0001_0,0b001_0001_0,0b001_0001_0,0b001_0001_0,0b011111_0_0,0b1_00000_00,
        0b0_0011111,0b11_000011,0b0111_0010,0b001111_00,0b11_001111,0b1011_0001,0b000011_00,
        0b11111111,0b11111111,(1 << 7) + 1,0,0,0,1,0
    ];

    let ((teban,banmen,mc),ExtendFields {
        value,
        best_move,
        end_ply,
        game_result
    }) = reader.read_sfen_with_extended(input).unwrap();

    assert_eq!(teban,Teban::Sente);
    assert_eq!(banmen,BANMEN_START_POS);
    assert_eq!(mc,MochigomaCollections::Pair(HashMap::new(),HashMap::new()));
    assert_eq!(value,-1);
    assert_eq!(best_move,BestMove::Null);
    assert_eq!(end_ply,0);
    assert_eq!(game_result,GameEndState::Win);
}

#[test]
fn test_read_sfen_with_extended_test_bestmove_resign() {
    let mut reader = PackedSfenReader::new();

    let input:Vec<u8> = vec![
        0b1001100_0,0b1_0000100,0b011_10001,0b00111_101,0b1_101111_1,0b111_10111,0b01011_100,0b0_100011_1,
        0b10111111,0b111_00000,0b01_0_10011,0b01_1001_10,0b01_1001_10,0b01_1001_10,0b01_1001_10,0b000000_10,
        0b00000000,0b00000000,0b001_00000,
        0b001_0001_0,0b001_0001_0,0b001_0001_0,0b001_0001_0,0b011111_0_0,0b1_00000_00,
        0b0_0011111,0b11_000011,0b0111_0010,0b001111_00,0b11_001111,0b1011_0001,0b000011_00,
        0b11111111,0b11111111,(1 << 7) + 2,0,0,0,0xFF,0
    ];

    let ((teban,banmen,mc),ExtendFields {
        value,
        best_move,
        end_ply,
        game_result
    }) = reader.read_sfen_with_extended(input).unwrap();

    assert_eq!(teban,Teban::Sente);
    assert_eq!(banmen,BANMEN_START_POS);
    assert_eq!(mc,MochigomaCollections::Pair(HashMap::new(),HashMap::new()));
    assert_eq!(value,-1);
    assert_eq!(best_move,BestMove::Resign);
    assert_eq!(end_ply,0);
    assert_eq!(game_result,GameEndState::Lose);
}

#[test]
fn test_read_sfen_with_extended_test_bestmove_win() {
    let mut reader = PackedSfenReader::new();

    let input:Vec<u8> = vec![
        0b1001100_0,0b1_0000100,0b011_10001,0b00111_101,0b1_101111_1,0b111_10111,0b01011_100,0b0_100011_1,
        0b10111111,0b111_00000,0b01_0_10011,0b01_1001_10,0b01_1001_10,0b01_1001_10,0b01_1001_10,0b000000_10,
        0b00000000,0b00000000,0b001_00000,
        0b001_0001_0,0b001_0001_0,0b001_0001_0,0b001_0001_0,0b011111_0_0,0b1_00000_00,
        0b0_0011111,0b11_000011,0b0111_0010,0b001111_00,0b11_001111,0b1011_0001,0b000011_00,
        0b11111111,0b11111111,(1 << 7) + 3,0,0,0,1,0
    ];

    let ((teban,banmen,mc),ExtendFields {
        value,
        best_move,
        end_ply,
        game_result
    }) = reader.read_sfen_with_extended(input).unwrap();

    assert_eq!(teban,Teban::Sente);
    assert_eq!(banmen,BANMEN_START_POS);
    assert_eq!(mc,MochigomaCollections::Pair(HashMap::new(),HashMap::new()));
    assert_eq!(value,-1);
    assert_eq!(best_move,BestMove::Win);
    assert_eq!(end_ply,0);
    assert_eq!(game_result,GameEndState::Win);
}


#[test]
fn test_read_sfen_with_extended_test_bestmove_non_drop_non_promote() {
    let mut reader = PackedSfenReader::new();

    let inputs:Vec<Vec<u8>> = vec![
        vec![
            0b1001100_0,0b1_0000100,0b011_10001,0b00111_101,0b1_101111_1,0b111_10111,0b01011_100,0b0_100011_1,
            0b10111111,0b111_00000,0b01_0_10011,0b01_1001_10,0b01_1001_10,0b01_1001_10,0b01_1001_10,0b000000_10,
            0b00000000,0b00000000,0b001_00000,
            0b001_0001_0,0b001_0001_0,0b001_0001_0,0b001_0001_0,0b011111_0_0,0b1_00000_00,
            0b0_0011111,0b11_000011,0b0111_0010,0b001111_00,0b11_001111,0b1011_0001,0b000011_00,
            0b11111111,0b11111111,0b00000000,0b00100100,0,0,1,0
        ],
        vec![
            0b1001100_0,0b1_0000100,0b011_10001,0b00111_101,0b1_101111_1,0b111_10111,0b01011_100,0b0_100011_1,
            0b10111111,0b111_00000,0b01_0_10011,0b01_1001_10,0b01_1001_10,0b01_1001_10,0b01_1001_10,0b000000_10,
            0b00000000,0b00000000,0b001_00000,
            0b001_0001_0,0b001_0001_0,0b001_0001_0,0b001_0001_0,0b011111_0_0,0b1_00000_00,
            0b0_0011111,0b11_000011,0b0111_0010,0b001111_00,0b11_001111,0b1011_0001,0b000011_00,
            0b11111111,0b11111111,0b00001000,0b00101000,0,0,1,0
        ],
        vec![
            0b1001100_0,0b1_0000100,0b011_10001,0b00111_101,0b1_101111_1,0b111_10111,0b01011_100,0b0_100011_1,
            0b10111111,0b111_00000,0b01_0_10011,0b01_1001_10,0b01_1001_10,0b01_1001_10,0b01_1001_10,0b000000_10,
            0b00000000,0b00000000,0b001_00000,
            0b001_0001_0,0b001_0001_0,0b001_0001_0,0b001_0001_0,0b011111_0_0,0b1_00000_00,
            0b0_0011111,0b11_000011,0b0111_0010,0b001111_00,0b11_001111,0b1011_0001,0b000011_00,
            0b11111111,0b11111111,0b01001000,0b00000000,0,0,1,0
        ],
        vec![
            0b1001100_0,0b1_0000100,0b011_10001,0b00111_101,0b1_101111_1,0b111_10111,0b01011_100,0b0_100011_1,
            0b10111111,0b111_00000,0b01_0_10011,0b01_1001_10,0b01_1001_10,0b01_1001_10,0b01_1001_10,0b000000_10,
            0b00000000,0b00000000,0b001_00000,
            0b001_0001_0,0b001_0001_0,0b001_0001_0,0b001_0001_0,0b011111_0_0,0b1_00000_00,
            0b0_0011111,0b11_000011,0b0111_0010,0b001111_00,0b11_001111,0b1011_0001,0b000011_00,
            0b11111111,0b11111111,0b1010000,0b00000100,0,0,1,0
        ],
    ];

    let answer_bestmoves:Vec<BestMove> = vec![
        BestMove::MoveTo(0,8,0,0,false),
        BestMove::MoveTo(8,8,8,0,false),
        BestMove::MoveTo(0,0,0,8,false),
        BestMove::MoveTo(8,0,8,8,false)
    ];

    for (input,answer_best_move) in inputs.into_iter().zip(answer_bestmoves.into_iter()) {
        let ((teban, banmen, mc), ExtendFields {
            value,
            best_move,
            end_ply,
            game_result
        }) = reader.read_sfen_with_extended(input).unwrap();

        assert_eq!(teban, Teban::Sente);
        assert_eq!(banmen, BANMEN_START_POS);
        assert_eq!(mc, MochigomaCollections::Pair(HashMap::new(), HashMap::new()));
        assert_eq!(value, -1);
        assert_eq!(best_move, answer_best_move);
        assert_eq!(end_ply, 0);
        assert_eq!(game_result, GameEndState::Win);
    }
}
