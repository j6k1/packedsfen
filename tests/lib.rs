use packedsfen::yaneuraou::reader::PackedSfenReader;
use packedsfen::traits::Reader;
use usiagent::shogi::{Teban, Banmen, MochigomaCollections};
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
