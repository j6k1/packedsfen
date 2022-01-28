use packedsfen::hcpe::reader::HcpeReader;
use packedsfen::traits::Reader;
use packedsfen::hcpe::reader::BestMove;
use packedsfen::hcpe::haffman_code::GameResult;
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
use packedsfen::hcpe::haffman_code::ExtendFields;
use packedsfen::error::ReadError;

#[test]
fn test_hcpe_read_sfen_teban_sente_initial_position() {
    let mut reader = HcpeReader::new();

    let input:[u8; 32] = [
        0b0101100_0,0b1_0100100,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,0b11_010111,0b01_011111,
        0b001_0_0_0_01,0b0011111_0,0b1_000111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,
        0b1_0_101111,0b01_0_0_0_010,0b01111_0_00,0b0_0_0101_0_0,0b11_0_0001_0,0b101_0_1011,
        0b0001_0_0_0_0,0b1_001111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,0b11_010111,
        0b01_010111,0b001_0_0_0_01,0b0111111_0,0b1_000111_0,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0
    ];

    let (teban,banmen,mc) = reader.read_sfen(&input).unwrap();

    assert_eq!(teban,Teban::Sente);
    assert_eq!(banmen,BANMEN_START_POS);
    assert_eq!(mc,MochigomaCollections::Pair(Mochigoma::new(),Mochigoma::new()));
}

#[test]
fn test_hcpe_read_sfen_teban_gote_initial_position() {
    let mut reader = HcpeReader::new();

    let input:[u8; 32] = [
        0b0101100_1,0b1_0100100,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,0b11_010111,0b01_011111,
        0b001_0_0_0_01,0b0011111_0,0b1_000111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,0b1_0_101111,
        0b01_0_0_0_010,0b01111_0_00,0b0_0_0101_0_0,0b11_0_0001_0,0b101_0_1011,0b0001_0_0_0_0,0b1_001111_0,
        0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,0b11_010111,0b01_010111,0b001_0_0_0_01,0b0111111_0,
        0b1_000111_0,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0
    ];

    let (teban,banmen,mc) = reader.read_sfen(&input).unwrap();

    assert_eq!(teban,Teban::Gote);
    assert_eq!(banmen,BANMEN_START_POS);
    assert_eq!(mc,MochigomaCollections::Pair(Mochigoma::new(),Mochigoma::new()));
}

#[test]
fn test_hcpe_read_sfen_teban_sente_sente_hisha_kaku_nari_and_one_nari() {
    let mut reader = HcpeReader::new();

    let input:[u8; 32] = [
        0b0101100_0,0b1_0100100,0b01_0_11001,0b001_0_0_0_11,0b000011_0_0,0b11_110111,0b01_011111,0b001_0_0_0_01,
        0b0011111_0,0b1_000111_1,0b01_0_11101,0b001_0_0_0_01,0b001011_0_0,0b1_0_101111,0b01_0_0_0_010,
        0b01111_0_00,0b0_0_0101_0_0,0b11_0_0001_0,0b101_0_1011,0b0001_0_0_0_0,0b1_001111_0,0b01_0_01101,
        0b001_0_0_0_01,0b101011_0_0,0b11_010111,0b01_010111,0b001_0_0_0_01,0b0111111_0,0b1_100111_1,
        0b01_0_01001,0b001_0_0_0_01,0b100011_0_1
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
fn test_hcpe_read_sfen_teban_gote_sente_hisha_kaku_nari_and_one_nari() {
    let mut reader = HcpeReader::new();

    let input:[u8; 32] = [
        0b0101100_1,0b1_0100100,0b01_0_11001,0b001_0_0_0_11,0b000011_0_0,0b11_110111,0b01_111111,
        0b001_0_0_0_01,0b0011111_0,0b1_000111_0,0b01_0_11101,0b001_0_0_0_01,0b001011_0_0,0b1_0_101111,
        0b01_0_0_0_010,0b01111_0_00,0b0_0_0101_0_0,0b11_0_0001_0,0b101_0_1011,0b0001_0_0_0_0,0b1_001111_0,
        0b01_0_01101,0b001_0_0_0_01,0b101011_0_0,0b11_010111,0b01_110111,0b001_0_0_0_01,0b0111111_0,
        0b1_100111_0,0b01_0_01001,0b001_0_0_0_01,0b100011_0_1
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
fn test_hcpe_read_sfen_teban_sente_mochigoma_half() {
    let mut reader = HcpeReader::new();

    let input:[u8; 32] = [
        0b0101100_0,0b1_0100100,0b01_0_01001,0b0_0_0_0_0_0_01,0b11_010111,0b01_011111,0b0_0_0_0_0_0_01,
        0b1_0_011011,0b0_0_0_0_0_010,0b0_101111_0,0b0_0_0_0_0101,0b0_0101_0_0_0,0b1111_0_0_0_0,0b0_0101_0_10,
        0b011_0_0_0_0_0,0b0101_0_011,0b11_0_0_0_0_0_0,0b1111_0101,0b0101_0101,0b11_0_0_0_0_0_0,0b101_0_0100,
        0b1_0_0_0_0_0_0_0,0b11_011111,0b111_00111,0b1_00111_00,0b0101_0010,0b11_00011_0,0b00001_000,0b000_00001,
        0b00_000_000,0b0_000_000_0,0b000_000_00
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
fn test_hcpe_read_sfen_teban_gote_mochigoma_half() {
    let mut reader = HcpeReader::new();

    let input:[u8; 32] = [
        0b0101100_1,0b0_0100100,0b00000000,0b00000000,0b00000000,0b00000000,0b00000000,0b00000000,
        0b0001_0000,0b0001_0001,0b0001_0001,0b0001_0001,0b0001_0001,0b0011111_0,
        0b11_00000_0,0b1_0_001111,0b011_00001,0b00111_001,0b1_001111_0,0b111_00111,0b01011_000,
        0b0_000011_0,
        0b110_100_11,0b10_110_100,0b0_100_110_1,0b1_10001_11,0b0101_1100,0b11_11101_1,0b10011_110,
        0b111_10111,0b101111_10,0b1111111_1
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
fn test_hcpe_read_sfen_teban_sente_mochigoma_full() {
    let mut reader = HcpeReader::new();

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
fn test_hcpe_read_sfen_teban_gote_mochigoma_full() {
    let mut reader = HcpeReader::new();

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
fn test_hcpe_read_sfen_teban_sente_mochigoma_half_and_half() {
    let mut reader = HcpeReader::new();

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
fn test_hcpe_read_sfen_teban_gote_mochigoma_half_and_half() {
    let mut reader = HcpeReader::new();

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
fn test_hcpe_read_sfen_with_extended_test_score_value_is_min() {
    let mut reader = HcpeReader::new();

    let input:Vec<u8> = vec![
        0b0101100_0,0b1_0100100,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,0b11_010111,0b01_011111,
        0b001_0_0_0_01,0b0011111_0,0b1_000111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,
        0b1_0_101111,0b01_0_0_0_010,0b01111_0_00,0b0_0_0101_0_0,0b11_0_0001_0,0b101_0_1011,
        0b0001_0_0_0_0,0b1_001111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,0b11_010111,
        0b01_010111,0b001_0_0_0_01,0b0111111_0,0b1_000111_0,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,
        0b11111111,0b11111111,0,0,1,0
    ];

    let ((teban,banmen,mc),ExtendFields {
        eval,
        best_move,
        game_result
    }) = reader.read_sfen_with_extended(input).unwrap();

    assert_eq!(teban,Teban::Sente);
    assert_eq!(banmen,BANMEN_START_POS);
    assert_eq!(mc,MochigomaCollections::Pair(Mochigoma::new(),Mochigoma::new()));
    assert_eq!(eval,-1);
    assert_eq!(best_move,BestMove::None);
    assert_eq!(game_result,GameResult::SenteWin);
}

#[test]
fn test_hcpe_read_sfen_with_extended_test_score_value_is_max() {
    let mut reader = HcpeReader::new();

    let input:Vec<u8> = vec![
        0b0101100_0,0b1_0100100,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,0b11_010111,0b01_011111,
        0b001_0_0_0_01,0b0011111_0,0b1_000111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,
        0b1_0_101111,0b01_0_0_0_010,0b01111_0_00,0b0_0_0101_0_0,0b11_0_0001_0,0b101_0_1011,
        0b0001_0_0_0_0,0b1_001111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,0b11_010111,
        0b01_010111,0b001_0_0_0_01,0b0111111_0,0b1_000111_0,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,
        0b11111111,0b01111111,0,0,1,0
    ];

    let ((teban,banmen,mc),ExtendFields {
        eval,
        best_move,
        game_result
    }) = reader.read_sfen_with_extended(input).unwrap();

    assert_eq!(teban,Teban::Sente);
    assert_eq!(banmen,BANMEN_START_POS);
    assert_eq!(mc,MochigomaCollections::Pair(Mochigoma::new(),Mochigoma::new()));
    assert_eq!(eval,std::i16::MAX);
    assert_eq!(best_move,BestMove::None);
    assert_eq!(game_result,GameResult::SenteWin);
}

#[test]
fn test_hcpe_read_sfen_with_extended_test_bestmove_none() {
    let mut reader = HcpeReader::new();

    let input:Vec<u8> = vec![
        0b0101100_0,0b1_0100100,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,0b11_010111,0b01_011111,
        0b001_0_0_0_01,0b0011111_0,0b1_000111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,
        0b1_0_101111,0b01_0_0_0_010,0b01111_0_00,0b0_0_0101_0_0,0b11_0_0001_0,0b101_0_1011,
        0b0001_0_0_0_0,0b1_001111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,0b11_010111,
        0b01_010111,0b001_0_0_0_01,0b0111111_0,0b1_000111_0,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,
        0b11111111,0b01111111,0,0,1,0
    ];

    let ((teban,banmen,mc),ExtendFields {
        eval,
        best_move,
        game_result
    }) = reader.read_sfen_with_extended(input).unwrap();

    assert_eq!(teban,Teban::Sente);
    assert_eq!(banmen,BANMEN_START_POS);
    assert_eq!(mc,MochigomaCollections::Pair(Mochigoma::new(),Mochigoma::new()));
    assert_eq!(eval,std::i16::MAX);
    assert_eq!(best_move,BestMove::None);
    assert_eq!(game_result,GameResult::SenteWin);
}

#[test]
fn test_hcpe_read_sfen_with_extended_test_bestmove_null() {
    let mut reader = HcpeReader::new();

    let input:Vec<u8> = vec![
        0b0101100_0,0b1_0100100,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,0b11_010111,0b01_011111,
        0b001_0_0_0_01,0b0011111_0,0b1_000111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,
        0b1_0_101111,0b01_0_0_0_010,0b01111_0_00,0b0_0_0101_0_0,0b11_0_0001_0,0b101_0_1011,
        0b0001_0_0_0_0,0b1_001111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,0b11_010111,
        0b01_010111,0b001_0_0_0_01,0b0111111_0,0b1_000111_0,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,
        0b11111111,0b11111111,(1 << 7) + 1,0,1,0
    ];

    let ((teban,banmen,mc),ExtendFields {
        eval,
        best_move,
        game_result
    }) = reader.read_sfen_with_extended(input).unwrap();

    assert_eq!(teban,Teban::Sente);
    assert_eq!(banmen,BANMEN_START_POS);
    assert_eq!(mc,MochigomaCollections::Pair(Mochigoma::new(),Mochigoma::new()));
    assert_eq!(eval,-1);
    assert_eq!(best_move,BestMove::Null);
    assert_eq!(game_result,GameResult::SenteWin);
}
#[test]
fn test_hcpe_read_sfen_with_extended_test_bestmove_non_drop_non_promote() {
    let mut reader = HcpeReader::new();

    let inputs:Vec<Vec<u8>> = vec![
        vec![
            0b0101100_0,0b1_0100100,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,0b11_010111,0b01_011111,
            0b001_0_0_0_01,0b0011111_0,0b1_000111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,
            0b1_0_101111,0b01_0_0_0_010,0b01111_0_00,0b0_0_0101_0_0,0b11_0_0001_0,0b101_0_1011,
            0b0001_0_0_0_0,0b1_001111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,0b11_010111,
            0b01_010111,0b001_0_0_0_01,0b0111111_0,0b1_000111_0,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,
            0b11111111,0b11111111,0b0000000,0b00000100,1,0
        ],
        vec![
            0b0101100_0,0b1_0100100,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,0b11_010111,0b01_011111,
            0b001_0_0_0_01,0b0011111_0,0b1_000111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,
            0b1_0_101111,0b01_0_0_0_010,0b01111_0_00,0b0_0_0101_0_0,0b11_0_0001_0,0b101_0_1011,
            0b0001_0_0_0_0,0b1_001111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,0b11_010111,
            0b01_010111,0b001_0_0_0_01,0b0111111_0,0b1_000111_0,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,
            0b11111111,0b11111111,0b01001000,0b00101000,1,0
        ],
        vec![
            0b0101100_0,0b1_0100100,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,0b11_010111,0b01_011111,
            0b001_0_0_0_01,0b0011111_0,0b1_000111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,
            0b1_0_101111,0b01_0_0_0_010,0b01111_0_00,0b0_0_0101_0_0,0b11_0_0001_0,0b101_0_1011,
            0b0001_0_0_0_0,0b1_001111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,0b11_010111,
            0b01_010111,0b001_0_0_0_01,0b0111111_0,0b1_000111_0,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,
            0b11111111,0b11111111,0b00001000,0b00000000,1,0
        ],
        vec![
            0b0101100_0,0b1_0100100,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,0b11_010111,0b01_011111,
            0b001_0_0_0_01,0b0011111_0,0b1_000111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,
            0b1_0_101111,0b01_0_0_0_010,0b01111_0_00,0b0_0_0101_0_0,0b11_0_0001_0,0b101_0_1011,
            0b0001_0_0_0_0,0b1_001111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,0b11_010111,
            0b01_010111,0b001_0_0_0_01,0b0111111_0,0b1_000111_0,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,
            0b11111111,0b11111111,0b01010000,0b00100100,1,0
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
            eval,
            best_move,
            game_result
        }) = reader.read_sfen_with_extended(input).unwrap();

        assert_eq!(teban, Teban::Sente);
        assert_eq!(banmen, BANMEN_START_POS);
        assert_eq!(mc, MochigomaCollections::Pair(Mochigoma::new(), Mochigoma::new()));
        assert_eq!(eval, -1);
        assert_eq!(best_move, answer_best_move);
        assert_eq!(game_result, GameResult::SenteWin);
    }
}

#[test]
fn test_hcpe_read_sfen_with_extended_test_bestmove_non_drop_promote() {
    let mut reader = HcpeReader::new();

    let inputs:Vec<Vec<u8>> = vec![
        vec![
            0b0101100_0,0b1_0100100,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,0b11_010111,0b01_011111,
            0b001_0_0_0_01,0b0011111_0,0b1_000111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,
            0b1_0_101111,0b01_0_0_0_010,0b01111_0_00,0b0_0_0101_0_0,0b11_0_0001_0,0b101_0_1011,
            0b0001_0_0_0_0,0b1_001111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,0b11_010111,
            0b01_010111,0b001_0_0_0_01,0b0111111_0,0b1_000111_0,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,
            0b11111111,0b11111111,0b00000000,0b01000100,1,0
        ],
        vec![
            0b0101100_0,0b1_0100100,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,0b11_010111,0b01_011111,
            0b001_0_0_0_01,0b0011111_0,0b1_000111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,
            0b1_0_101111,0b01_0_0_0_010,0b01111_0_00,0b0_0_0101_0_0,0b11_0_0001_0,0b101_0_1011,
            0b0001_0_0_0_0,0b1_001111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,0b11_010111,
            0b01_010111,0b001_0_0_0_01,0b0111111_0,0b1_000111_0,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,
            0b11111111,0b11111111,0b01001000,0b01101000,1,0
        ],
        vec![
            0b0101100_0,0b1_0100100,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,0b11_010111,0b01_011111,
            0b001_0_0_0_01,0b0011111_0,0b1_000111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,
            0b1_0_101111,0b01_0_0_0_010,0b01111_0_00,0b0_0_0101_0_0,0b11_0_0001_0,0b101_0_1011,
            0b0001_0_0_0_0,0b1_001111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,0b11_010111,
            0b01_010111,0b001_0_0_0_01,0b0111111_0,0b1_000111_0,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,
            0b11111111,0b11111111,0b00001000,0b01000000,1,0
        ],
        vec![
            0b0101100_0,0b1_0100100,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,0b11_010111,0b01_011111,
            0b001_0_0_0_01,0b0011111_0,0b1_000111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,
            0b1_0_101111,0b01_0_0_0_010,0b01111_0_00,0b0_0_0101_0_0,0b11_0_0001_0,0b101_0_1011,
            0b0001_0_0_0_0,0b1_001111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,0b11_010111,
            0b01_010111,0b001_0_0_0_01,0b0111111_0,0b1_000111_0,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,
            0b11111111,0b11111111,0b01010000,0b01100100,1,0
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
            eval,
            best_move,
            game_result
        }) = reader.read_sfen_with_extended(input).unwrap();

        assert_eq!(teban, Teban::Sente);
        assert_eq!(banmen, BANMEN_START_POS);
        assert_eq!(mc, MochigomaCollections::Pair(Mochigoma::new(), Mochigoma::new()));
        assert_eq!(eval, -1);
        assert_eq!(best_move, answer_best_move);
        assert_eq!(game_result, GameResult::SenteWin);
    }
}

#[test]
fn test_hcpe_read_sfen_with_extended_test_bestmove_drop() {
    let mut reader = HcpeReader::new();

    let inputs:Vec<Vec<u8>> = vec![
        vec![
            0b0101100_0,0b1_0100100,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,0b11_010111,0b01_011111,
            0b001_0_0_0_01,0b0011111_0,0b1_000111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,
            0b1_0_101111,0b01_0_0_0_010,0b01111_0_00,0b0_0_0101_0_0,0b11_0_0001_0,0b101_0_1011,
            0b0001_0_0_0_0,0b1_001111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,0b11_010111,
            0b01_010111,0b001_0_0_0_01,0b0111111_0,0b1_000111_0,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,
            0b11111111,0b11111111,0b10000000,0b00101000,1,0
        ],
        vec![
            0b0101100_0,0b1_0100100,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,0b11_010111,0b01_011111,
            0b001_0_0_0_01,0b0011111_0,0b1_000111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,
            0b1_0_101111,0b01_0_0_0_010,0b01111_0_00,0b0_0_0101_0_0,0b11_0_0001_0,0b101_0_1011,
            0b0001_0_0_0_0,0b1_001111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,0b11_010111,
            0b01_010111,0b001_0_0_0_01,0b0111111_0,0b1_000111_0,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,
            0b11111111,0b11111111,0b01001000,0b00101001,1,0
        ],
        vec![
            0b0101100_0,0b1_0100100,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,0b11_010111,0b01_011111,
            0b001_0_0_0_01,0b0011111_0,0b1_000111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,
            0b1_0_101111,0b01_0_0_0_010,0b01111_0_00,0b0_0_0101_0_0,0b11_0_0001_0,0b101_0_1011,
            0b0001_0_0_0_0,0b1_001111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,0b11_010111,
            0b01_010111,0b001_0_0_0_01,0b0111111_0,0b1_000111_0,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,
            0b11111111,0b11111111,0b10001000,0b00101001,1,0
        ],
        vec![
            0b0101100_0,0b1_0100100,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,0b11_010111,0b01_011111,
            0b001_0_0_0_01,0b0011111_0,0b1_000111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,
            0b1_0_101111,0b01_0_0_0_010,0b01111_0_00,0b0_0_0101_0_0,0b11_0_0001_0,0b101_0_1011,
            0b0001_0_0_0_0,0b1_001111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,0b11_010111,
            0b01_010111,0b001_0_0_0_01,0b0111111_0,0b1_000111_0,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,
            0b11111111,0b11111111,0b01010000,0b00101010,1,0
        ],
        vec![
            0b0101100_0,0b1_0100100,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,0b11_010111,0b01_011111,
            0b001_0_0_0_01,0b0011111_0,0b1_000111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,
            0b1_0_101111,0b01_0_0_0_010,0b01111_0_00,0b0_0_0101_0_0,0b11_0_0001_0,0b101_0_1011,
            0b0001_0_0_0_0,0b1_001111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,0b11_010111,
            0b01_010111,0b001_0_0_0_01,0b0111111_0,0b1_000111_0,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,
            0b11111111,0b11111111,0b10000000,0b00101010,1,0
        ],
        vec![
            0b0101100_0,0b1_0100100,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,0b11_010111,0b01_011111,
            0b001_0_0_0_01,0b0011111_0,0b1_000111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,
            0b1_0_101111,0b01_0_0_0_010,0b01111_0_00,0b0_0_0101_0_0,0b11_0_0001_0,0b101_0_1011,
            0b0001_0_0_0_0,0b1_001111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,0b11_010111,
            0b01_010111,0b001_0_0_0_01,0b0111111_0,0b1_000111_0,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,
            0b11111111,0b11111111,0b01001000,0b00101011,1,0
        ],
        vec![
            0b0101100_0,0b1_0100100,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,0b11_010111,0b01_011111,
            0b001_0_0_0_01,0b0011111_0,0b1_000111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,
            0b1_0_101111,0b01_0_0_0_010,0b01111_0_00,0b0_0_0101_0_0,0b11_0_0001_0,0b101_0_1011,
            0b0001_0_0_0_0,0b1_001111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,0b11_010111,
            0b01_010111,0b001_0_0_0_01,0b0111111_0,0b1_000111_0,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,
            0b11111111,0b11111111,0b10001000,0b00101011,1,0
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
            eval,
            best_move,
            game_result
        }) = reader.read_sfen_with_extended(input).unwrap();

        assert_eq!(teban, Teban::Sente);
        assert_eq!(banmen, BANMEN_START_POS);
        assert_eq!(mc, MochigomaCollections::Pair(Mochigoma::new(), Mochigoma::new()));
        assert_eq!(eval, -1);
        assert_eq!(best_move, answer_best_move);
        assert_eq!(game_result, GameResult::SenteWin);
    }
}

#[test]
fn test_hcpe_read_sfen_with_extended_test_bestmove_drop_outofrange() {
    let mut reader = HcpeReader::new();

    let inputs:Vec<Vec<u8>> = vec![
        vec![
            0b0101100_0,0b1_0100100,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,0b11_010111,0b01_011111,
            0b001_0_0_0_01,0b0011111_0,0b1_000111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,
            0b1_0_101111,0b01_0_0_0_010,0b01111_0_00,0b0_0_0101_0_0,0b11_0_0001_0,0b101_0_1011,
            0b0001_0_0_0_0,0b1_001111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,0b11_010111,
            0b01_010111,0b001_0_0_0_01,0b0111111_0,0b1_000111_0,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,
            0b11111111,0b11111111,0b00000000,0b00101100,1,0
        ],
        vec![
            0b0101100_0,0b1_0100100,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,0b11_010111,0b01_011111,
            0b001_0_0_0_01,0b0011111_0,0b1_000111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,
            0b1_0_101111,0b01_0_0_0_010,0b01111_0_00,0b0_0_0101_0_0,0b11_0_0001_0,0b101_0_1011,
            0b0001_0_0_0_0,0b1_001111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,0b11_010111,
            0b01_010111,0b001_0_0_0_01,0b0111111_0,0b1_000111_0,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,
            0b11111111,0b11111111,0b11010001,0b00101000,1,0
        ]
    ];

    let answer = ["piece kind is invalid.","move put position is invalid."];

    for (input,&err) in inputs.into_iter().zip(&answer) {
        let r = reader.read_sfen_with_extended(input);

        assert_eq!(r, Err(ReadError::InvalidFormat(String::from(err))));
    }
}

#[test]
fn test_hcpe_read_sfen_with_extended_test_bestmove_non_drop_outofrange() {
    let mut reader = HcpeReader::new();

    let input:Vec<u8> = vec![
        0b0101100_0,0b1_0100100,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,0b11_010111,0b01_011111,
        0b001_0_0_0_01,0b0011111_0,0b1_000111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,
        0b1_0_101111,0b01_0_0_0_010,0b01111_0_00,0b0_0_0101_0_0,0b11_0_0001_0,0b101_0_1011,
        0b0001_0_0_0_0,0b1_001111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,0b11_010111,
        0b01_010111,0b001_0_0_0_01,0b0111111_0,0b1_000111_0,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,
        0b11111111,0b01010001,0b01010001,0b00000000,1,0
    ];

    let r = reader.read_sfen_with_extended(input);

    assert_eq!(r, Err(ReadError::InvalidFormat(String::from("move to position is invalid."))));
}

#[test]
fn test_hcpe_read_sfen_with_extended_test_game_result_is_sente_win() {
    let mut reader = HcpeReader::new();

    let input:Vec<u8> = vec![
        0b0101100_0,0b1_0100100,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,0b11_010111,0b01_011111,
        0b001_0_0_0_01,0b0011111_0,0b1_000111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,
        0b1_0_101111,0b01_0_0_0_010,0b01111_0_00,0b0_0_0101_0_0,0b11_0_0001_0,0b101_0_1011,
        0b0001_0_0_0_0,0b1_001111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,0b11_010111,
        0b01_010111,0b001_0_0_0_01,0b0111111_0,0b1_000111_0,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,
        0b11111111,0b11111111,0,0,1,0
    ];

    let ((teban,banmen,mc),ExtendFields {
        eval,
        best_move,
        game_result
    }) = reader.read_sfen_with_extended(input).unwrap();

    assert_eq!(teban,Teban::Sente);
    assert_eq!(banmen,BANMEN_START_POS);
    assert_eq!(mc,MochigomaCollections::Pair(Mochigoma::new(),Mochigoma::new()));
    assert_eq!(eval,-1);
    assert_eq!(best_move,BestMove::None);
    assert_eq!(game_result,GameResult::SenteWin);
}

#[test]
fn test_hcpe_read_sfen_with_extended_test_game_result_is_gote_win() {
    let mut reader = HcpeReader::new();

    let input:Vec<u8> = vec![
        0b0101100_0,0b1_0100100,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,0b11_010111,0b01_011111,
        0b001_0_0_0_01,0b0011111_0,0b1_000111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,
        0b1_0_101111,0b01_0_0_0_010,0b01111_0_00,0b0_0_0101_0_0,0b11_0_0001_0,0b101_0_1011,
        0b0001_0_0_0_0,0b1_001111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,0b11_010111,
        0b01_010111,0b001_0_0_0_01,0b0111111_0,0b1_000111_0,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,
        0b11111111,0b11111111,0,0,2,0
    ];

    let ((teban,banmen,mc),ExtendFields {
        eval,
        best_move,
        game_result
    }) = reader.read_sfen_with_extended(input).unwrap();

    assert_eq!(teban,Teban::Sente);
    assert_eq!(banmen,BANMEN_START_POS);
    assert_eq!(mc,MochigomaCollections::Pair(Mochigoma::new(),Mochigoma::new()));
    assert_eq!(eval,-1);
    assert_eq!(best_move,BestMove::None);
    assert_eq!(game_result,GameResult::GoteWin);
}

#[test]
fn test_hcpe_read_sfen_with_extended_test_game_result_is_draw() {
    let mut reader = HcpeReader::new();

    let input:Vec<u8> = vec![
        0b0101100_0,0b1_0100100,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,0b11_010111,0b01_011111,
        0b001_0_0_0_01,0b0011111_0,0b1_000111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,
        0b1_0_101111,0b01_0_0_0_010,0b01111_0_00,0b0_0_0101_0_0,0b11_0_0001_0,0b101_0_1011,
        0b0001_0_0_0_0,0b1_001111_0,0b01_0_01101,0b001_0_0_0_01,0b001011_0_0,0b11_010111,
        0b01_010111,0b001_0_0_0_01,0b0111111_0,0b1_000111_0,0b01_0_01001,0b001_0_0_0_01,0b000011_0_0,
        0b11111111,0b11111111,0,0,0,0
    ];

    let ((teban,banmen,mc),ExtendFields {
        eval,
        best_move,
        game_result
    }) = reader.read_sfen_with_extended(input).unwrap();

    assert_eq!(teban,Teban::Sente);
    assert_eq!(banmen,BANMEN_START_POS);
    assert_eq!(mc,MochigomaCollections::Pair(Mochigoma::new(),Mochigoma::new()));
    assert_eq!(eval,-1);
    assert_eq!(best_move,BestMove::None);
    assert_eq!(game_result,GameResult::Draw);
}

#[test]
fn test_hcpe_read_sfen_with_extended_test_buffer_size_incorrect() {
    let mut reader = HcpeReader::new();

    let inputs = vec![
        (0..39).map(|_| 0).collect::<Vec<u8>>(),
        (0..37).map(|_| 0).collect::<Vec<u8>>(),
        vec![]
    ];


    for input in inputs.into_iter() {
        let r = reader.read_sfen_with_extended(input);

        assert_eq!(r, Err(ReadError::InvalidFormat(String::from("input size is incorrect."))));
    }
}