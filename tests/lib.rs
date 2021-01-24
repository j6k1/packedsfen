use packedsfen::yaneuraou::reader::PackedSfenReader;

#[test]
fn test_read_sfen() {
    let mut reader = PackedSfenReader::new();

    let inputs:Vec<Vec<u8>> = vec![
        [0b001101_10,0b1101_0111, 0b01_011111,0b0_11111_01,0b1101_1011,0b01_001101,
          0b0_0111110,0b1_00000_11,
          0b111101_0_0,0b0101_0101,0b0101_0101,0b0101_0101,0b0101_0101,0b0101_0000,
          0b00000_000,0b00000_00,0b00000000,
          0b0100_0100,0b0100_0100,0b0100_0100,0b0100_0100,0b0100_0_011,0b11100_000,0b00_111111,
          0b00_0_00110,0b0_1011000,0b011100_11,0b110_0_0111,0b00_101100,0b001100_00
        ].into_vec().into_iter().map(|bits| bits as u8).collect()
    ];
}