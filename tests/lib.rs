use packedsfen::yaneuraou::reader::PackedSfenReader;
use packedsfen::traits::Reader;
use usiagent::shogi::{Teban, MochigomaCollections};
use usiagent::rule::BANMEN_START_POS;
use std::collections::HashMap;

#[test]
fn test_read_sfen_teban_sente_initial_position() {
    let mut reader = PackedSfenReader::new();

    let input:[u8; 32] = [
        0b0_0001000,0b0_1001100,0b01101_101,0b101_01110,0b1_011111_1,0b11111_011,0b101_10110,0b1_001101_0,
        0b11111101,0b00000_011,0b11101_0_01,0b01_0101_01,0b01_0101_01,0b01_0101_01,0b01_0101_01,0b01_000000,
        0b00000000,0b00000000,0b00000_010,
        0b0_0100_010,0b0_0100_010,0b0_0100_010,0b0_0100_010,0b0_0_011111,0b00_00000_1,
        0b1111100_0,0b001100_10,0b1100_0111,0b00_011110,0b11110_011,0b100_1011,0b00_001100
    ];

    let (teban,banmen,mc) = reader.read_sfen(&input).unwrap();

    assert_eq!(teban,Teban::Sente);
    assert_eq!(banmen,BANMEN_START_POS);
    assert_eq!(mc,MochigomaCollections::Pair(HashMap::new(),HashMap::new()));
}