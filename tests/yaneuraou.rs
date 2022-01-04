use packedsfen::yaneuraou::reader::PackedSfenReader;
use packedsfen::traits::Reader;
use packedsfen::yaneuraou::reader::BestMove;

use usiagent::shogi::{Teban, Banmen, MochigomaCollections, MochigomaKind};
use usiagent::rule::BANMEN_START_POS;
use usiagent::shogi::Mochigoma;

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
use packedsfen::error::ReadError;

#[test]
fn test_yaneuraou_read_sfen_teban_sente_initial_position() {
    let mut reader = PackedSfenReader::new();

    let input:[u8; 32] = [
        0b0101100_0,0b1_0100100,0b01_0_10001,0b001_0_0_0_10,0b000011_0_0,0b11_101011,
        0b01_101111,0b001_0_0_0_10,0b0011111_0,0b1_001011_0,0b01_0_10011,0b001_0_0_0_10,
        0b000111_0_0,0b1_0_101111,0b01_0_0_0_100,0b01111_0_00,0b0_0_1001_0_0,0b11_0_0001_0,
        0b001_0_1011,0b0001_0_0_0_1,0b1_001111_0,0b01_0_10011,0b001_0_0_0_10,0b000111_0_0,
        0b11_101011,0b01_100111,0b001_0_0_0_10,0b0111111_0,0b1_001011_0,0b01_0_10001,
        0b001_0_0_0_10,0b000011_0_0
    ];

    let (teban,banmen,mc) = reader.read_sfen(&input).unwrap();

    assert_eq!(teban,Teban::Sente);
    assert_eq!(banmen,BANMEN_START_POS);
    assert_eq!(mc,MochigomaCollections::Pair(Mochigoma::new(),Mochigoma::new()));
}

#[test]
fn test_yaneuraou_read_sfen_teban_gote_initial_position() {
    let mut reader = PackedSfenReader::new();

    let input:[u8; 32] = [
        0b0101100_1,0b1_0100100,0b01_0_10001,0b001_0_0_0_10,0b000011_0_0,0b11_101011,
        0b01_101111,0b001_0_0_0_10,0b0011111_0,0b1_001011_0,0b01_0_10011,0b001_0_0_0_10,
        0b000111_0_0,0b1_0_101111,0b01_0_0_0_100,0b01111_0_00,0b0_0_1001_0_0,0b11_0_0001_0,
        0b001_0_1011,0b0001_0_0_0_1,0b1_001111_0,0b01_0_10011,0b001_0_0_0_10,0b000111_0_0,
        0b11_101011,0b01_100111,0b001_0_0_0_10,0b0111111_0,0b1_001011_0,0b01_0_10001,
        0b001_0_0_0_10,0b000011_0_0
    ];

    let (teban,banmen,mc) = reader.read_sfen(&input).unwrap();

    assert_eq!(teban,Teban::Gote);
    assert_eq!(banmen,BANMEN_START_POS);
    assert_eq!(mc,MochigomaCollections::Pair(Mochigoma::new(),Mochigoma::new()));
}

#[test]
fn test_yaneuraou_read_sfen_teban_sente_sente_hisha_kaku_nari_and_one_nari() {
    let mut reader = PackedSfenReader::new();

    let input:[u8; 32] = [
        0b0101100_0,0b1_0100100,0b01_0_11001,0b001_0_0_0_11,0b000011_0_0,0b11_111011,
        0b01_101111,0b001_0_0_0_10,0b1011111_0,0b1_001011_0,0b01_0_11011,0b001_0_0_0_10,
        0b000111_0_0,0b1_0_101111,0b01_0_0_0_100,0b01111_0_00,0b0_0_1001_0_0,
        0b11_0_0001_0,0b001_0_1011,0b0001_0_0_0_1,0b1_001111_0,0b01_0_10011,
        0b001_0_0_0_10,0b010111_0_0,0b11_101011,0b01_100111,0b001_0_0_0_10,0b1111111_0,
        0b1_011011_0,0b01_0_10001,0b101_0_0_0_10,0b010011_0_0
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
    assert_eq!(mc,MochigomaCollections::Pair(Mochigoma::new(),Mochigoma::new()));
}


#[test]
fn test_yaneuraou_read_sfen_teban_gote_sente_hisha_kaku_nari_and_one_nari() {
    let mut reader = PackedSfenReader::new();

    let input:[u8; 32] = [
        0b0101100_1,0b1_0100100,0b01_0_11001,0b001_0_0_0_11,0b000011_0_0,0b11_111011,
        0b01_111111,0b001_0_0_0_10,0b0011111_0,0b1_001011_0,0b01_0_11011,0b001_0_0_0_10,
        0b000111_0_0,0b1_0_101111,0b01_0_0_0_100,0b01111_0_00,0b0_0_1001_0_0,0b11_0_0001_0,
        0b001_0_1011,0b0001_0_0_0_1,0b1_001111_0,0b01_0_10011,0b001_0_0_0_10,0b010111_0_0,
        0b11_101011,0b01_110111,0b001_0_0_0_10,0b0111111_0,0b1_011011_0,0b01_0_10001,
        0b101_0_0_0_10,0b010011_0_0,
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
    assert_eq!(mc,MochigomaCollections::Pair(Mochigoma::new(),Mochigoma::new()));
}

#[test]
fn test_yaneuraou_read_sfen_teban_sente_mochigoma_half() {
    let mut reader = PackedSfenReader::new();

    let input:[u8; 32] = [
        0b0101100_0,0b1_0100100,0b01_0_10001,0b0_0_0_0_0_0_10,0b11_101011,0b01_101111,
        0b0_0_0_0_0_0_10,0b1_0_100111,0b0_0_0_0_0_100,0b0_101111_0,0b0_0_0_0_1001,
        0b0_1001_0_0_0,0b1111_0_0_0_0,0b0_1001_0_10,0b111_0_0_0_0_0,0b1001_0_100,
        0b11_0_0_0_0_0_0,0b1111_1010,0b1001_1001,0b11_0_0_0_0_0_0,0b001_0_1000,
        0b0_0_0_0_0_0_0_1,0b000_010_00,0b10_000_010,0b0_010_000_0,0b1_00001_00,
        0b0101_0100,0b11_01101_0,0b01011_000,0b1_0001111,0b11_011111,0b00111_001
    ];

    let mut ms:Mochigoma = Mochigoma::new();

    ms.insert(MochigomaKind::Fu,9);
    ms.insert(MochigomaKind::Kyou,2);
    ms.insert(MochigomaKind::Kei,2);
    ms.insert(MochigomaKind::Gin,2);
    ms.insert(MochigomaKind::Kin,2);
    ms.insert(MochigomaKind::Hisha, 1);
    ms.insert(MochigomaKind::Kaku,1);

    let emc = MochigomaCollections::Pair(ms,Mochigoma::new());

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
fn test_yaneuraou_read_sfen_teban_gote_mochigoma_half() {
    let mut reader = PackedSfenReader::new();

    let input:[u8; 32] = [
        0b0101100_1,0b0_0100100,0b001_0_0_0_0_0,0b000011_0_0,0b01_0_0_0_0_0_0,
        0b011111_00,0b001011_00,0b01_0_0_0_0_0_0,0b00111_0_00,0b1_0_0_0_0_0_0_0,
        0b1111_0_000,0b1_0_0_0_0_0_00,0b0_0_0_0_0_000,0b1_0_0001_0_0,0b0_0_0_00111,
        0b0_0001_0_0_0,0b0_0_000111,0b0001_0_0_0_0,0b00111111,0b0_0_001011,
        0b0001_0_0_0_0,0b0_000011_0,0b100_110_10,0b10_100_110,0b0_110_100_1,
        0b1_10001_10,0b0101_1100,0b11_11101_1,0b11011_100,0b1_1001111,0b11_111111,
        0b10111_101
    ];

    let mut mg:Mochigoma = Mochigoma::new();

    mg.insert(MochigomaKind::Fu,9);
    mg.insert(MochigomaKind::Kyou,2);
    mg.insert(MochigomaKind::Kei,2);
    mg.insert(MochigomaKind::Gin,2);
    mg.insert(MochigomaKind::Kin,2);
    mg.insert(MochigomaKind::Hisha, 1);
    mg.insert(MochigomaKind::Kaku,1);

    let emc = MochigomaCollections::Pair(Mochigoma::new(),mg);

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
fn test_yaneuraou_read_sfen_teban_sente_mochigoma_full() {
    let mut reader = PackedSfenReader::new();

    let input:[u8; 32] = [
        0b0101100_0,0b0_0100100,
        0b00000000,0b00000000,0b00000000,0b00000000,0b00000000,0b00000000,0b00000000,0b00000000,
        0b00000000,0b00_000000,
        0b0_000_010_0,0b010_000_01,
        0b00_010_000,0b0_000_010_0,0b010_000_01,0b00_010_000,
        0b0001_010_0,0b01_01001_0,0b01001_000,0b101_00101,0b1_00101_01,0b0011_0110,
        0b11_01011_0,0b01011_000,0b111_00111,0b1_00111_00,0b1111_0011,
        0b01111_000,0b011111_01,0b0111111_0
    ];

    let mut ms:Mochigoma = Mochigoma::new();

    ms.insert(MochigomaKind::Fu,18);
    ms.insert(MochigomaKind::Kyou,4);
    ms.insert(MochigomaKind::Kei,4);
    ms.insert(MochigomaKind::Gin,4);
    ms.insert(MochigomaKind::Kin,4);
    ms.insert(MochigomaKind::Hisha, 2);
    ms.insert(MochigomaKind::Kaku,2);

    let mg:Mochigoma = Mochigoma::new();

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
fn test_yaneuraou_read_sfen_teban_gote_mochigoma_full() {
    let mut reader = PackedSfenReader::new();

    let input:[u8; 32] = [
        0b0101100_1,0b0_0100100,
        0b00000000,0b00000000,0b00000000,0b00000000,0b00000000,0b00000000,0b00000000,0b00000000,
        0b00000000,0b00_000000,
        0b0_100_110_1,0b110_100_11,
        0b00_110_100,0b0_100_110_1,0b110_100_11,0b00_110_100,
        0b0001_110_1,0b01_11001_1,0b11001_100,0b101_10101,0b1_10101_11,0b0011_1110,
        0b11_11011_1,0b11011_100,0b111_10111,0b1_10111_10,0b1111_1011,
        0b01111_100,0b011111_11,0b1111111_1
    ];

    let ms:Mochigoma = Mochigoma::new();

    let mut mg:Mochigoma = Mochigoma::new();

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
fn test_yaneuraou_read_sfen_teban_sente_mochigoma_half_and_half() {
    let mut reader = PackedSfenReader::new();

    let input:[u8; 32] = [
        0b0101100_0,0b0_0100100,
        0b00000000,0b00000000,0b00000000,0b00000000,0b00000000,0b00000000,0b00000000,0b00000000,
        0b00000000,0b00_000000,
        0b0_000_010_0,0b010_000_01,
        0b00_010_000,0b0_100_110_0,0b110_100_11,0b00_110_100,
        0b0001_110_1,0b01_01001_0,0b11001_100,0b101_00101,0b1_10101_01,0b0011_1110,
        0b11_01011_0,0b11011_100,0b111_00111,0b1_10111_00,0b1111_1011,
        0b01111_000,0b011111_11,0b1111111_0
    ];

    let mut ms:Mochigoma = Mochigoma::new();

    ms.insert(MochigomaKind::Fu,9);
    ms.insert(MochigomaKind::Kyou,2);
    ms.insert(MochigomaKind::Kei,2);
    ms.insert(MochigomaKind::Gin,2);
    ms.insert(MochigomaKind::Kin,2);
    ms.insert(MochigomaKind::Hisha, 1);
    ms.insert(MochigomaKind::Kaku,1);

    let mut mg:Mochigoma = Mochigoma::new();

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
fn test_yaneuraou_yaneuraou_read_sfen_teban_gote_mochigoma_half_and_half() {
    let mut reader = PackedSfenReader::new();

    let input:[u8; 32] = [
        0b0101100_1,0b0_0100100,
        0b00000000,0b00000000,0b00000000,0b00000000,0b00000000,0b00000000,0b00000000,0b00000000,
        0b00000000,0b00_000000,
        0b0_000_010_0,0b010_000_01,
        0b00_010_000,0b0_100_110_0,0b110_100_11,0b00_110_100,
        0b0001_110_1,0b01_01001_0,0b11001_100,0b101_00101,0b1_10101_01,0b0011_1110,
        0b11_01011_0,0b11011_100,0b111_00111,0b1_10111_00,0b1111_1011,
        0b01111_000,0b011111_11,0b1111111_0
    ];

    let mut ms:Mochigoma = Mochigoma::new();

    ms.insert(MochigomaKind::Fu,9);
    ms.insert(MochigomaKind::Kyou,2);
    ms.insert(MochigomaKind::Kei,2);
    ms.insert(MochigomaKind::Gin,2);
    ms.insert(MochigomaKind::Kin,2);
    ms.insert(MochigomaKind::Hisha, 1);
    ms.insert(MochigomaKind::Kaku,1);

    let mut mg:Mochigoma = Mochigoma::new();

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
fn test_yaneuraou_read_sfen_with_extended_test_score_value_is_min() {
    let mut reader = PackedSfenReader::new();

    let input:Vec<u8> = vec![
        0b0101100_0,0b1_0100100,0b01_0_10001,0b001_0_0_0_10,0b000011_0_0,0b11_101011,
        0b01_101111,0b001_0_0_0_10,0b0011111_0,0b1_001011_0,0b01_0_10011,0b001_0_0_0_10,
        0b000111_0_0,0b1_0_101111,0b01_0_0_0_100,0b01111_0_00,0b0_0_1001_0_0,0b11_0_0001_0,
        0b001_0_1011,0b0001_0_0_0_1,0b1_001111_0,0b01_0_10011,0b001_0_0_0_10,0b000111_0_0,
        0b11_101011,0b01_100111,0b001_0_0_0_10,0b0111111_0,0b1_001011_0,0b01_0_10001,
        0b001_0_0_0_10,0b000011_0_0,
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
    assert_eq!(mc,MochigomaCollections::Pair(Mochigoma::new(),Mochigoma::new()));
    assert_eq!(value,-1);
    assert_eq!(best_move,BestMove::None);
    assert_eq!(end_ply,0);
    assert_eq!(game_result,GameEndState::Win);
}

#[test]
fn test_yaneuraou_read_sfen_with_extended_test_score_value_is_max() {
    let mut reader = PackedSfenReader::new();

    let input:Vec<u8> = vec![
        0b0101100_0,0b1_0100100,0b01_0_10001,0b001_0_0_0_10,0b000011_0_0,0b11_101011,
        0b01_101111,0b001_0_0_0_10,0b0011111_0,0b1_001011_0,0b01_0_10011,0b001_0_0_0_10,
        0b000111_0_0,0b1_0_101111,0b01_0_0_0_100,0b01111_0_00,0b0_0_1001_0_0,0b11_0_0001_0,
        0b001_0_1011,0b0001_0_0_0_1,0b1_001111_0,0b01_0_10011,0b001_0_0_0_10,0b000111_0_0,
        0b11_101011,0b01_100111,0b001_0_0_0_10,0b0111111_0,0b1_001011_0,0b01_0_10001,
        0b001_0_0_0_10,0b000011_0_0,
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
    assert_eq!(mc,MochigomaCollections::Pair(Mochigoma::new(),Mochigoma::new()));
    assert_eq!(value,std::i16::MAX);
    assert_eq!(best_move,BestMove::None);
    assert_eq!(end_ply,0);
    assert_eq!(game_result,GameEndState::Win);
}

#[test]
fn test_yaneuraou_read_sfen_with_extended_test_bestmove_none() {
    let mut reader = PackedSfenReader::new();

    let input:Vec<u8> = vec![
        0b0101100_0,0b1_0100100,0b01_0_10001,0b001_0_0_0_10,0b000011_0_0,0b11_101011,
        0b01_101111,0b001_0_0_0_10,0b0011111_0,0b1_001011_0,0b01_0_10011,0b001_0_0_0_10,
        0b000111_0_0,0b1_0_101111,0b01_0_0_0_100,0b01111_0_00,0b0_0_1001_0_0,0b11_0_0001_0,
        0b001_0_1011,0b0001_0_0_0_1,0b1_001111_0,0b01_0_10011,0b001_0_0_0_10,0b000111_0_0,
        0b11_101011,0b01_100111,0b001_0_0_0_10,0b0111111_0,0b1_001011_0,0b01_0_10001,
        0b001_0_0_0_10,0b000011_0_0,
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
    assert_eq!(mc,MochigomaCollections::Pair(Mochigoma::new(),Mochigoma::new()));
    assert_eq!(value,std::i16::MAX);
    assert_eq!(best_move,BestMove::None);
    assert_eq!(end_ply,0);
    assert_eq!(game_result,GameEndState::Win);
}

#[test]
fn test_yaneuraou_read_sfen_with_extended_test_bestmove_null() {
    let mut reader = PackedSfenReader::new();

    let input:Vec<u8> = vec![
        0b0101100_0,0b1_0100100,0b01_0_10001,0b001_0_0_0_10,0b000011_0_0,0b11_101011,
        0b01_101111,0b001_0_0_0_10,0b0011111_0,0b1_001011_0,0b01_0_10011,0b001_0_0_0_10,
        0b000111_0_0,0b1_0_101111,0b01_0_0_0_100,0b01111_0_00,0b0_0_1001_0_0,0b11_0_0001_0,
        0b001_0_1011,0b0001_0_0_0_1,0b1_001111_0,0b01_0_10011,0b001_0_0_0_10,0b000111_0_0,
        0b11_101011,0b01_100111,0b001_0_0_0_10,0b0111111_0,0b1_001011_0,0b01_0_10001,
        0b001_0_0_0_10,0b000011_0_0,
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
    assert_eq!(mc,MochigomaCollections::Pair(Mochigoma::new(),Mochigoma::new()));
    assert_eq!(value,-1);
    assert_eq!(best_move,BestMove::Null);
    assert_eq!(end_ply,0);
    assert_eq!(game_result,GameEndState::Win);
}

#[test]
fn test_yaneuraou_read_sfen_with_extended_test_bestmove_resign() {
    let mut reader = PackedSfenReader::new();

    let input:Vec<u8> = vec![
        0b0101100_0,0b1_0100100,0b01_0_10001,0b001_0_0_0_10,0b000011_0_0,0b11_101011,
        0b01_101111,0b001_0_0_0_10,0b0011111_0,0b1_001011_0,0b01_0_10011,0b001_0_0_0_10,
        0b000111_0_0,0b1_0_101111,0b01_0_0_0_100,0b01111_0_00,0b0_0_1001_0_0,0b11_0_0001_0,
        0b001_0_1011,0b0001_0_0_0_1,0b1_001111_0,0b01_0_10011,0b001_0_0_0_10,0b000111_0_0,
        0b11_101011,0b01_100111,0b001_0_0_0_10,0b0111111_0,0b1_001011_0,0b01_0_10001,
        0b001_0_0_0_10,0b000011_0_0,
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
    assert_eq!(mc,MochigomaCollections::Pair(Mochigoma::new(),Mochigoma::new()));
    assert_eq!(value,-1);
    assert_eq!(best_move,BestMove::Resign);
    assert_eq!(end_ply,0);
    assert_eq!(game_result,GameEndState::Lose);
}

#[test]
fn test_yaneuraou_read_sfen_with_extended_test_bestmove_win() {
    let mut reader = PackedSfenReader::new();

    let input:Vec<u8> = vec![
        0b0101100_0,0b1_0100100,0b01_0_10001,0b001_0_0_0_10,0b000011_0_0,0b11_101011,
        0b01_101111,0b001_0_0_0_10,0b0011111_0,0b1_001011_0,0b01_0_10011,0b001_0_0_0_10,
        0b000111_0_0,0b1_0_101111,0b01_0_0_0_100,0b01111_0_00,0b0_0_1001_0_0,0b11_0_0001_0,
        0b001_0_1011,0b0001_0_0_0_1,0b1_001111_0,0b01_0_10011,0b001_0_0_0_10,0b000111_0_0,
        0b11_101011,0b01_100111,0b001_0_0_0_10,0b0111111_0,0b1_001011_0,0b01_0_10001,
        0b001_0_0_0_10,0b000011_0_0,
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
    assert_eq!(mc,MochigomaCollections::Pair(Mochigoma::new(),Mochigoma::new()));
    assert_eq!(value,-1);
    assert_eq!(best_move,BestMove::Win);
    assert_eq!(end_ply,0);
    assert_eq!(game_result,GameEndState::Win);
}

#[test]
fn test_yaneuraou_read_sfen_with_extended_test_bestmove_non_drop_non_promote() {
    let mut reader = PackedSfenReader::new();

    let inputs:Vec<Vec<u8>> = vec![
        vec![
            0b0101100_0,0b1_0100100,0b01_0_10001,0b001_0_0_0_10,0b000011_0_0,0b11_101011,
            0b01_101111,0b001_0_0_0_10,0b0011111_0,0b1_001011_0,0b01_0_10011,0b001_0_0_0_10,
            0b000111_0_0,0b1_0_101111,0b01_0_0_0_100,0b01111_0_00,0b0_0_1001_0_0,0b11_0_0001_0,
            0b001_0_1011,0b0001_0_0_0_1,0b1_001111_0,0b01_0_10011,0b001_0_0_0_10,0b000111_0_0,
            0b11_101011,0b01_100111,0b001_0_0_0_10,0b0111111_0,0b1_001011_0,0b01_0_10001,
            0b001_0_0_0_10,0b000011_0_0,
            0b11111111,0b11111111,0b0000000,0b00000100,0,0,1,0
        ],
        vec![
            0b0101100_0,0b1_0100100,0b01_0_10001,0b001_0_0_0_10,0b000011_0_0,0b11_101011,
            0b01_101111,0b001_0_0_0_10,0b0011111_0,0b1_001011_0,0b01_0_10011,0b001_0_0_0_10,
            0b000111_0_0,0b1_0_101111,0b01_0_0_0_100,0b01111_0_00,0b0_0_1001_0_0,0b11_0_0001_0,
            0b001_0_1011,0b0001_0_0_0_1,0b1_001111_0,0b01_0_10011,0b001_0_0_0_10,0b000111_0_0,
            0b11_101011,0b01_100111,0b001_0_0_0_10,0b0111111_0,0b1_001011_0,0b01_0_10001,
            0b001_0_0_0_10,0b000011_0_0,
            0b11111111,0b11111111,0b01001000,0b00101000,0,0,1,0
        ],
        vec![
            0b0101100_0,0b1_0100100,0b01_0_10001,0b001_0_0_0_10,0b000011_0_0,0b11_101011,
            0b01_101111,0b001_0_0_0_10,0b0011111_0,0b1_001011_0,0b01_0_10011,0b001_0_0_0_10,
            0b000111_0_0,0b1_0_101111,0b01_0_0_0_100,0b01111_0_00,0b0_0_1001_0_0,0b11_0_0001_0,
            0b001_0_1011,0b0001_0_0_0_1,0b1_001111_0,0b01_0_10011,0b001_0_0_0_10,0b000111_0_0,
            0b11_101011,0b01_100111,0b001_0_0_0_10,0b0111111_0,0b1_001011_0,0b01_0_10001,
            0b001_0_0_0_10,0b000011_0_0,
            0b11111111,0b11111111,0b00001000,0b00000000,0,0,1,0
        ],
        vec![
            0b0101100_0,0b1_0100100,0b01_0_10001,0b001_0_0_0_10,0b000011_0_0,0b11_101011,
            0b01_101111,0b001_0_0_0_10,0b0011111_0,0b1_001011_0,0b01_0_10011,0b001_0_0_0_10,
            0b000111_0_0,0b1_0_101111,0b01_0_0_0_100,0b01111_0_00,0b0_0_1001_0_0,0b11_0_0001_0,
            0b001_0_1011,0b0001_0_0_0_1,0b1_001111_0,0b01_0_10011,0b001_0_0_0_10,0b000111_0_0,
            0b11_101011,0b01_100111,0b001_0_0_0_10,0b0111111_0,0b1_001011_0,0b01_0_10001,
            0b001_0_0_0_10,0b000011_0_0,
            0b11111111,0b11111111,0b01010000,0b00100100,0,0,1,0
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
        assert_eq!(mc, MochigomaCollections::Pair(Mochigoma::new(), Mochigoma::new()));
        assert_eq!(value, -1);
        assert_eq!(best_move, answer_best_move);
        assert_eq!(end_ply, 0);
        assert_eq!(game_result, GameEndState::Win);
    }
}

#[test]
fn test_yaneuraou_read_sfen_with_extended_test_bestmove_non_drop_promote() {
    let mut reader = PackedSfenReader::new();

    let inputs:Vec<Vec<u8>> = vec![
        vec![
            0b0101100_0,0b1_0100100,0b01_0_10001,0b001_0_0_0_10,0b000011_0_0,0b11_101011,
            0b01_101111,0b001_0_0_0_10,0b0011111_0,0b1_001011_0,0b01_0_10011,0b001_0_0_0_10,
            0b000111_0_0,0b1_0_101111,0b01_0_0_0_100,0b01111_0_00,0b0_0_1001_0_0,0b11_0_0001_0,
            0b001_0_1011,0b0001_0_0_0_1,0b1_001111_0,0b01_0_10011,0b001_0_0_0_10,0b000111_0_0,
            0b11_101011,0b01_100111,0b001_0_0_0_10,0b0111111_0,0b1_001011_0,0b01_0_10001,
            0b001_0_0_0_10,0b000011_0_0,
            0b11111111,0b11111111,0b00000000,0b10000100,0,0,1,0
        ],
        vec![
            0b0101100_0,0b1_0100100,0b01_0_10001,0b001_0_0_0_10,0b000011_0_0,0b11_101011,
            0b01_101111,0b001_0_0_0_10,0b0011111_0,0b1_001011_0,0b01_0_10011,0b001_0_0_0_10,
            0b000111_0_0,0b1_0_101111,0b01_0_0_0_100,0b01111_0_00,0b0_0_1001_0_0,0b11_0_0001_0,
            0b001_0_1011,0b0001_0_0_0_1,0b1_001111_0,0b01_0_10011,0b001_0_0_0_10,0b000111_0_0,
            0b11_101011,0b01_100111,0b001_0_0_0_10,0b0111111_0,0b1_001011_0,0b01_0_10001,
            0b001_0_0_0_10,0b000011_0_0,
            0b11111111,0b11111111,0b01001000,0b10101000,0,0,1,0
        ],
        vec![
            0b0101100_0,0b1_0100100,0b01_0_10001,0b001_0_0_0_10,0b000011_0_0,0b11_101011,
            0b01_101111,0b001_0_0_0_10,0b0011111_0,0b1_001011_0,0b01_0_10011,0b001_0_0_0_10,
            0b000111_0_0,0b1_0_101111,0b01_0_0_0_100,0b01111_0_00,0b0_0_1001_0_0,0b11_0_0001_0,
            0b001_0_1011,0b0001_0_0_0_1,0b1_001111_0,0b01_0_10011,0b001_0_0_0_10,0b000111_0_0,
            0b11_101011,0b01_100111,0b001_0_0_0_10,0b0111111_0,0b1_001011_0,0b01_0_10001,
            0b001_0_0_0_10,0b000011_0_0,
            0b11111111,0b11111111,0b00001000,0b10000000,0,0,1,0
        ],
        vec![
            0b0101100_0,0b1_0100100,0b01_0_10001,0b001_0_0_0_10,0b000011_0_0,0b11_101011,
            0b01_101111,0b001_0_0_0_10,0b0011111_0,0b1_001011_0,0b01_0_10011,0b001_0_0_0_10,
            0b000111_0_0,0b1_0_101111,0b01_0_0_0_100,0b01111_0_00,0b0_0_1001_0_0,0b11_0_0001_0,
            0b001_0_1011,0b0001_0_0_0_1,0b1_001111_0,0b01_0_10011,0b001_0_0_0_10,0b000111_0_0,
            0b11_101011,0b01_100111,0b001_0_0_0_10,0b0111111_0,0b1_001011_0,0b01_0_10001,
            0b001_0_0_0_10,0b000011_0_0,
            0b11111111,0b11111111,0b01010000,0b10100100,0,0,1,0
        ],
    ];

    let answer_bestmoves:Vec<BestMove> = vec![
        BestMove::MoveTo(0,8,0,0,true),
        BestMove::MoveTo(8,8,8,0,true),
        BestMove::MoveTo(0,0,0,8,true),
        BestMove::MoveTo(8,0,8,8,true)
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
        assert_eq!(mc, MochigomaCollections::Pair(Mochigoma::new(), Mochigoma::new()));
        assert_eq!(value, -1);
        assert_eq!(best_move, answer_best_move);
        assert_eq!(end_ply, 0);
        assert_eq!(game_result, GameEndState::Win);
    }
}

#[test]
fn test_yaneuraou_read_sfen_with_extended_test_bestmove_drop() {
    let mut reader = PackedSfenReader::new();

    let inputs:Vec<Vec<u8>> = vec![
        vec![
            0b0101100_0,0b1_0100100,0b01_0_10001,0b001_0_0_0_10,0b000011_0_0,0b11_101011,
            0b01_101111,0b001_0_0_0_10,0b0011111_0,0b1_001011_0,0b01_0_10011,0b001_0_0_0_10,
            0b000111_0_0,0b1_0_101111,0b01_0_0_0_100,0b01111_0_00,0b0_0_1001_0_0,0b11_0_0001_0,
            0b001_0_1011,0b0001_0_0_0_1,0b1_001111_0,0b01_0_10011,0b001_0_0_0_10,0b000111_0_0,
            0b11_101011,0b01_100111,0b001_0_0_0_10,0b0111111_0,0b1_001011_0,0b01_0_10001,
            0b001_0_0_0_10,0b000011_0_0,
            0b11111111,0b11111111,0b10000000,0b01000000,0,0,1,0
        ],
        vec![
            0b0101100_0,0b1_0100100,0b01_0_10001,0b001_0_0_0_10,0b000011_0_0,0b11_101011,
            0b01_101111,0b001_0_0_0_10,0b0011111_0,0b1_001011_0,0b01_0_10011,0b001_0_0_0_10,
            0b000111_0_0,0b1_0_101111,0b01_0_0_0_100,0b01111_0_00,0b0_0_1001_0_0,0b11_0_0001_0,
            0b001_0_1011,0b0001_0_0_0_1,0b1_001111_0,0b01_0_10011,0b001_0_0_0_10,0b000111_0_0,
            0b11_101011,0b01_100111,0b001_0_0_0_10,0b0111111_0,0b1_001011_0,0b01_0_10001,
            0b001_0_0_0_10,0b000011_0_0,
            0b11111111,0b11111111,0b01001000,0b01000001,0,0,1,0
        ],
        vec![
            0b0101100_0,0b1_0100100,0b01_0_10001,0b001_0_0_0_10,0b000011_0_0,0b11_101011,
            0b01_101111,0b001_0_0_0_10,0b0011111_0,0b1_001011_0,0b01_0_10011,0b001_0_0_0_10,
            0b000111_0_0,0b1_0_101111,0b01_0_0_0_100,0b01111_0_00,0b0_0_1001_0_0,0b11_0_0001_0,
            0b001_0_1011,0b0001_0_0_0_1,0b1_001111_0,0b01_0_10011,0b001_0_0_0_10,0b000111_0_0,
            0b11_101011,0b01_100111,0b001_0_0_0_10,0b0111111_0,0b1_001011_0,0b01_0_10001,
            0b001_0_0_0_10,0b000011_0_0,
            0b11111111,0b11111111,0b10001000,0b01000001,0,0,1,0
        ],
        vec![
            0b0101100_0,0b1_0100100,0b01_0_10001,0b001_0_0_0_10,0b000011_0_0,0b11_101011,
            0b01_101111,0b001_0_0_0_10,0b0011111_0,0b1_001011_0,0b01_0_10011,0b001_0_0_0_10,
            0b000111_0_0,0b1_0_101111,0b01_0_0_0_100,0b01111_0_00,0b0_0_1001_0_0,0b11_0_0001_0,
            0b001_0_1011,0b0001_0_0_0_1,0b1_001111_0,0b01_0_10011,0b001_0_0_0_10,0b000111_0_0,
            0b11_101011,0b01_100111,0b001_0_0_0_10,0b0111111_0,0b1_001011_0,0b01_0_10001,
            0b001_0_0_0_10,0b000011_0_0,
            0b11111111,0b11111111,0b01010000,0b01000010,0,0,1,0
        ],
        vec![
            0b0101100_0,0b1_0100100,0b01_0_10001,0b001_0_0_0_10,0b000011_0_0,0b11_101011,
            0b01_101111,0b001_0_0_0_10,0b0011111_0,0b1_001011_0,0b01_0_10011,0b001_0_0_0_10,
            0b000111_0_0,0b1_0_101111,0b01_0_0_0_100,0b01111_0_00,0b0_0_1001_0_0,0b11_0_0001_0,
            0b001_0_1011,0b0001_0_0_0_1,0b1_001111_0,0b01_0_10011,0b001_0_0_0_10,0b000111_0_0,
            0b11_101011,0b01_100111,0b001_0_0_0_10,0b0111111_0,0b1_001011_0,0b01_0_10001,
            0b001_0_0_0_10,0b000011_0_0,
            0b11111111,0b11111111,0b10000000,0b01000010,0,0,1,0
        ],
        vec![
            0b0101100_0,0b1_0100100,0b01_0_10001,0b001_0_0_0_10,0b000011_0_0,0b11_101011,
            0b01_101111,0b001_0_0_0_10,0b0011111_0,0b1_001011_0,0b01_0_10011,0b001_0_0_0_10,
            0b000111_0_0,0b1_0_101111,0b01_0_0_0_100,0b01111_0_00,0b0_0_1001_0_0,0b11_0_0001_0,
            0b001_0_1011,0b0001_0_0_0_1,0b1_001111_0,0b01_0_10011,0b001_0_0_0_10,0b000111_0_0,
            0b11_101011,0b01_100111,0b001_0_0_0_10,0b0111111_0,0b1_001011_0,0b01_0_10001,
            0b001_0_0_0_10,0b000011_0_0,
            0b11111111,0b11111111,0b01001000,0b01000011,0,0,1,0
        ],
        vec![
            0b0101100_0,0b1_0100100,0b01_0_10001,0b001_0_0_0_10,0b000011_0_0,0b11_101011,
            0b01_101111,0b001_0_0_0_10,0b0011111_0,0b1_001011_0,0b01_0_10011,0b001_0_0_0_10,
            0b000111_0_0,0b1_0_101111,0b01_0_0_0_100,0b01111_0_00,0b0_0_1001_0_0,0b11_0_0001_0,
            0b001_0_1011,0b0001_0_0_0_1,0b1_001111_0,0b01_0_10011,0b001_0_0_0_10,0b000111_0_0,
            0b11_101011,0b01_100111,0b001_0_0_0_10,0b0111111_0,0b1_001011_0,0b01_0_10001,
            0b001_0_0_0_10,0b000011_0_0,
            0b11111111,0b11111111,0b10001000,0b01000011,0,0,1,0
        ],
    ];

    let answer_bestmoves:Vec<BestMove> = vec![
        BestMove::MovePut(MochigomaKind::Fu,0,0),
        BestMove::MovePut(MochigomaKind::Kyou,8,0),
        BestMove::MovePut(MochigomaKind::Kei,0,8),
        BestMove::MovePut(MochigomaKind::Gin,8,8),
        BestMove::MovePut(MochigomaKind::Kaku,0,0),
        BestMove::MovePut(MochigomaKind::Hisha,8,0),
        BestMove::MovePut(MochigomaKind::Kin,0,8)
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
        assert_eq!(mc, MochigomaCollections::Pair(Mochigoma::new(), Mochigoma::new()));
        assert_eq!(value, -1);
        assert_eq!(best_move, answer_best_move);
        assert_eq!(end_ply, 0);
        assert_eq!(game_result, GameEndState::Win);
    }
}

#[test]
fn test_yaneuraou_read_sfen_with_extended_test_bestmove_drop_outofrange() {
    let mut reader = PackedSfenReader::new();

    let inputs:Vec<Vec<u8>> = vec![
        vec![
            0b0101100_0,0b1_0100100,0b01_0_10001,0b001_0_0_0_10,0b000011_0_0,0b11_101011,
            0b01_101111,0b001_0_0_0_10,0b0011111_0,0b1_001011_0,0b01_0_10011,0b001_0_0_0_10,
            0b000111_0_0,0b1_0_101111,0b01_0_0_0_100,0b01111_0_00,0b0_0_1001_0_0,0b11_0_0001_0,
            0b001_0_1011,0b0001_0_0_0_1,0b1_001111_0,0b01_0_10011,0b001_0_0_0_10,0b000111_0_0,
            0b11_101011,0b01_100111,0b001_0_0_0_10,0b0111111_0,0b1_001011_0,0b01_0_10001,
            0b001_0_0_0_10,0b000011_0_0,
            0b11111111,0b11111111,0b00000000,0b01000000,0,0,1,0
        ],
        vec![
            0b0101100_0,0b1_0100100,0b01_0_10001,0b001_0_0_0_10,0b000011_0_0,0b11_101011,
            0b01_101111,0b001_0_0_0_10,0b0011111_0,0b1_001011_0,0b01_0_10011,0b001_0_0_0_10,
            0b000111_0_0,0b1_0_101111,0b01_0_0_0_100,0b01111_0_00,0b0_0_1001_0_0,0b11_0_0001_0,
            0b001_0_1011,0b0001_0_0_0_1,0b1_001111_0,0b01_0_10011,0b001_0_0_0_10,0b000111_0_0,
            0b11_101011,0b01_100111,0b001_0_0_0_10,0b0111111_0,0b1_001011_0,0b01_0_10001,
            0b001_0_0_0_10,0b000011_0_0,
            0b11111111,0b11111111,0b00001000,0b01000100,0,0,1,0
        ]
    ];

    for input in inputs.into_iter() {
        let r = reader.read_sfen_with_extended(input);

        assert_eq!(r, Err(ReadError::InvalidFormat(String::from("piece kind is invalid."))));
    }
}

#[test]
fn test_yaneuraou_read_sfen_with_extended_test_end_play_is_min() {
    let mut reader = PackedSfenReader::new();

    let input:Vec<u8> = vec![
        0b0101100_0,0b1_0100100,0b01_0_10001,0b001_0_0_0_10,0b000011_0_0,0b11_101011,
        0b01_101111,0b001_0_0_0_10,0b0011111_0,0b1_001011_0,0b01_0_10011,0b001_0_0_0_10,
        0b000111_0_0,0b1_0_101111,0b01_0_0_0_100,0b01111_0_00,0b0_0_1001_0_0,0b11_0_0001_0,
        0b001_0_1011,0b0001_0_0_0_1,0b1_001111_0,0b01_0_10011,0b001_0_0_0_10,0b000111_0_0,
        0b11_101011,0b01_100111,0b001_0_0_0_10,0b0111111_0,0b1_001011_0,0b01_0_10001,
        0b001_0_0_0_10,0b000011_0_0,
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
    assert_eq!(mc,MochigomaCollections::Pair(Mochigoma::new(),Mochigoma::new()));
    assert_eq!(value,-1);
    assert_eq!(best_move,BestMove::None);
    assert_eq!(end_ply,0);
    assert_eq!(game_result,GameEndState::Win);
}

#[test]
fn test_yaneuraou_read_sfen_with_extended_test_end_play_is_max() {
    let mut reader = PackedSfenReader::new();

    let input:Vec<u8> = vec![
        0b0101100_0,0b1_0100100,0b01_0_10001,0b001_0_0_0_10,0b000011_0_0,0b11_101011,
        0b01_101111,0b001_0_0_0_10,0b0011111_0,0b1_001011_0,0b01_0_10011,0b001_0_0_0_10,
        0b000111_0_0,0b1_0_101111,0b01_0_0_0_100,0b01111_0_00,0b0_0_1001_0_0,0b11_0_0001_0,
        0b001_0_1011,0b0001_0_0_0_1,0b1_001111_0,0b01_0_10011,0b001_0_0_0_10,0b000111_0_0,
        0b11_101011,0b01_100111,0b001_0_0_0_10,0b0111111_0,0b1_001011_0,0b01_0_10001,
        0b001_0_0_0_10,0b000011_0_0,
        0b11111111,0b11111111,0,0,0xFF,0xFF,1,0
    ];

    let ((teban,banmen,mc),ExtendFields {
        value,
        best_move,
        end_ply,
        game_result
    }) = reader.read_sfen_with_extended(input).unwrap();

    assert_eq!(teban,Teban::Sente);
    assert_eq!(banmen,BANMEN_START_POS);
    assert_eq!(mc,MochigomaCollections::Pair(Mochigoma::new(),Mochigoma::new()));
    assert_eq!(value,-1);
    assert_eq!(best_move,BestMove::None);
    assert_eq!(end_ply,std::u16::MAX);
    assert_eq!(game_result,GameEndState::Win);
}

#[test]
fn test_yaneuraou_read_sfen_with_extended_test_game_result_is_win() {
    let mut reader = PackedSfenReader::new();

    let input:Vec<u8> = vec![
        0b0101100_0,0b1_0100100,0b01_0_10001,0b001_0_0_0_10,0b000011_0_0,0b11_101011,
        0b01_101111,0b001_0_0_0_10,0b0011111_0,0b1_001011_0,0b01_0_10011,0b001_0_0_0_10,
        0b000111_0_0,0b1_0_101111,0b01_0_0_0_100,0b01111_0_00,0b0_0_1001_0_0,0b11_0_0001_0,
        0b001_0_1011,0b0001_0_0_0_1,0b1_001111_0,0b01_0_10011,0b001_0_0_0_10,0b000111_0_0,
        0b11_101011,0b01_100111,0b001_0_0_0_10,0b0111111_0,0b1_001011_0,0b01_0_10001,
        0b001_0_0_0_10,0b000011_0_0,
        0b11111111,0b11111111,0,0,0xFF,0xFF,1,0
    ];

    let ((teban,banmen,mc),ExtendFields {
        value,
        best_move,
        end_ply,
        game_result
    }) = reader.read_sfen_with_extended(input).unwrap();

    assert_eq!(teban,Teban::Sente);
    assert_eq!(banmen,BANMEN_START_POS);
    assert_eq!(mc,MochigomaCollections::Pair(Mochigoma::new(),Mochigoma::new()));
    assert_eq!(value,-1);
    assert_eq!(best_move,BestMove::None);
    assert_eq!(end_ply,std::u16::MAX);
    assert_eq!(game_result,GameEndState::Win);
}

#[test]
fn test_yaneuraou_read_sfen_with_extended_test_game_result_is_lose() {
    let mut reader = PackedSfenReader::new();

    let input:Vec<u8> = vec![
        0b0101100_0,0b1_0100100,0b01_0_10001,0b001_0_0_0_10,0b000011_0_0,0b11_101011,
        0b01_101111,0b001_0_0_0_10,0b0011111_0,0b1_001011_0,0b01_0_10011,0b001_0_0_0_10,
        0b000111_0_0,0b1_0_101111,0b01_0_0_0_100,0b01111_0_00,0b0_0_1001_0_0,0b11_0_0001_0,
        0b001_0_1011,0b0001_0_0_0_1,0b1_001111_0,0b01_0_10011,0b001_0_0_0_10,0b000111_0_0,
        0b11_101011,0b01_100111,0b001_0_0_0_10,0b0111111_0,0b1_001011_0,0b01_0_10001,
        0b001_0_0_0_10,0b000011_0_0,
        0b11111111,0b11111111,0,0,0xFF,0xFF,0xFF,0
    ];

    let ((teban,banmen,mc),ExtendFields {
        value,
        best_move,
        end_ply,
        game_result
    }) = reader.read_sfen_with_extended(input).unwrap();

    assert_eq!(teban,Teban::Sente);
    assert_eq!(banmen,BANMEN_START_POS);
    assert_eq!(mc,MochigomaCollections::Pair(Mochigoma::new(),Mochigoma::new()));
    assert_eq!(value,-1);
    assert_eq!(best_move,BestMove::None);
    assert_eq!(end_ply,std::u16::MAX);
    assert_eq!(game_result,GameEndState::Lose);
}

#[test]
fn test_yaneuraou_read_sfen_with_extended_test_game_result_is_draw() {
    let mut reader = PackedSfenReader::new();

    let input:Vec<u8> = vec![
        0b0101100_0,0b1_0100100,0b01_0_10001,0b001_0_0_0_10,0b000011_0_0,0b11_101011,
        0b01_101111,0b001_0_0_0_10,0b0011111_0,0b1_001011_0,0b01_0_10011,0b001_0_0_0_10,
        0b000111_0_0,0b1_0_101111,0b01_0_0_0_100,0b01111_0_00,0b0_0_1001_0_0,0b11_0_0001_0,
        0b001_0_1011,0b0001_0_0_0_1,0b1_001111_0,0b01_0_10011,0b001_0_0_0_10,0b000111_0_0,
        0b11_101011,0b01_100111,0b001_0_0_0_10,0b0111111_0,0b1_001011_0,0b01_0_10001,
        0b001_0_0_0_10,0b000011_0_0,
        0b11111111,0b11111111,0,0,0xFF,0xFF,0,0
    ];

    let ((teban,banmen,mc),ExtendFields {
        value,
        best_move,
        end_ply,
        game_result
    }) = reader.read_sfen_with_extended(input).unwrap();

    assert_eq!(teban,Teban::Sente);
    assert_eq!(banmen,BANMEN_START_POS);
    assert_eq!(mc,MochigomaCollections::Pair(Mochigoma::new(),Mochigoma::new()));
    assert_eq!(value,-1);
    assert_eq!(best_move,BestMove::None);
    assert_eq!(end_ply,std::u16::MAX);
    assert_eq!(game_result,GameEndState::Draw);
}

#[test]
fn test_yaneuraou_read_sfen_with_extended_test_buffer_size_incorrect() {
    let mut reader = PackedSfenReader::new();

    let inputs = vec![
        (0..41).map(|_| 0).collect::<Vec<u8>>(),
        (0..39).map(|_| 0).collect::<Vec<u8>>(),
        vec![]
    ];


    for input in inputs.into_iter() {
        let r = reader.read_sfen_with_extended(input);

        assert_eq!(r, Err(ReadError::InvalidFormat(String::from("input size is incorrect."))));
    }
}