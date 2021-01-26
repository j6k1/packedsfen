use packedsfen::yaneuraou::reader::PackedSfenReader;
use packedsfen::traits::Reader;
use usiagent::shogi::{Teban, MochigomaCollections};
use usiagent::rule::BANMEN_START_POS;
use std::collections::HashMap;

#[test]
fn test_read_sfen_teban_sente_initial_position() {
    let mut reader = PackedSfenReader::new();

    let input:[u8; 32] = [
        0b1001100_0,0b1_0000100,0b011_10001,0b00111_101,0b1_101111_1,0b111_10111,0b01011_100,0b0_100011_1,
        0b10111111,0b111_00000,0b01_0_10011,0b01_1001_10,0b01_1001_10,0b01_1001_10,0b01_1001_10,0b000000_10,
        0b00000000,0b00000000,0b001_00000,
        0b001_0001_0,0b001_0001_0,0b001_0001_0,0b001_0001_0,0b011111_0_0,0b1_00000_00,
        0b0_0011111,0b11_000011,0b0111_0010,0b001111_00,0b11_001111,0b1011_0001,0b0000_1100
    ];

    let (teban,banmen,mc) = reader.read_sfen(&input).unwrap();

    assert_eq!(teban,Teban::Sente);
    assert_eq!(banmen,BANMEN_START_POS);
    assert_eq!(mc,MochigomaCollections::Pair(HashMap::new(),HashMap::new()));
}