use bit_vec::BitVec;
use compression::huffman::{self, HuffmanTreeCreator};
use std::collections::HashMap;

#[macro_use]
mod utils;

test!(test_building_tree_from_weights_1, {
    let mut weights = HashMap::new();
    weights.insert(b'a', 1_u8);
    weights.insert(b'b', 2_u8);
    weights.insert(b'c', 3_u8);

    let (on_bits, on_bytes) = HuffmanTreeCreator::get_mappings(&weights);

    let mut expected_on_bits = HashMap::new();
    expected_on_bits.insert(b'a', BitVec::from_elem(3, false)); // 100
    expected_on_bits.get_mut(&b'a').unwrap().set(0, true);

    expected_on_bits.insert(b'b', BitVec::from_elem(3, false)); // 101
    expected_on_bits.get_mut(&b'b').unwrap().set(0, true);
    expected_on_bits.get_mut(&b'b').unwrap().set(2, true);

    expected_on_bits.insert(b'c', BitVec::from_elem(2, true)); // 11
    expected_on_bits.get_mut(&b'c').unwrap().set(0, true);

    let expected_on_bytes: HashMap<BitVec, u8> = expected_on_bits
        .clone()
        .into_iter()
        .map(|(bytes, bits)| (bits, bytes))
        .collect();

    assert_eq!(on_bits, expected_on_bits);
    assert_eq!(on_bytes, expected_on_bytes);
});

test!(test_building_tree_from_weights_2, {
    let mut weights = HashMap::new();
    weights.insert(b'a', 16_u8);
    weights.insert(b'b', 32_u8);
    weights.insert(b'c', 32_u8);
    weights.insert(b'd', 8_u8);
    weights.insert(b'e', 8_u8);

    let (on_bits, on_bytes) = HuffmanTreeCreator::get_mappings(&weights);

    let mut expected_on_bits = HashMap::new();
    expected_on_bits.insert(b'a', BitVec::from_elem(4, false)); // 1100
    expected_on_bits.get_mut(&b'a').unwrap().set(0, true);
    expected_on_bits.get_mut(&b'a').unwrap().set(1, true);

    expected_on_bits.insert(b'b', BitVec::from_elem(3, true)); // 111
    expected_on_bits.insert(b'c', BitVec::from_elem(2, false)); // 10
    expected_on_bits.get_mut(&b'c').unwrap().set(0, true);

    expected_on_bits.insert(b'd', BitVec::from_elem(5, false)); // 11010
    expected_on_bits.get_mut(&b'd').unwrap().set(0, true);
    expected_on_bits.get_mut(&b'd').unwrap().set(1, true);
    expected_on_bits.get_mut(&b'd').unwrap().set(3, true);

    expected_on_bits.insert(b'e', BitVec::from_elem(5, true)); // 11011
    expected_on_bits.get_mut(&b'e').unwrap().set(2, false);

    let expected_on_bytes: HashMap<BitVec, u8> = expected_on_bits
        .clone()
        .into_iter()
        .map(|(bytes, bits)| (bits, bytes))
        .collect();

    assert_eq!(on_bits, expected_on_bits);
    assert_eq!(on_bytes, expected_on_bytes);
});

test!(test_huffman_1, {
    let s: Vec<u8> = Vec::from("abcabcbabcbb");
    let compressed = huffman::HuffmanCompressor::compress(&s, false);
    let decompressed = huffman::HuffmanCompressor::decompress(&compressed);
    assert_eq!(s, decompressed);
});

test!(test_huffman_2, {
    let s: Vec<u8> =
        Vec::from("abcdefghijklmonprstuwvxyzABCDEFGHIJKLMNOPRSTUWVXYZ1234567890!@#$%^&*((_+");
    let compressed = huffman::HuffmanCompressor::compress(&s, false);
    println!("{}", compressed);
    let decompressed = huffman::HuffmanCompressor::decompress(&compressed);
    assert_eq!(s, decompressed);
});

test!(test_huffman_3, {
    let big_word = String::from_iter(['a'; 10000]);
    let s: Vec<u8> = Vec::from(big_word);
    let compressed = huffman::HuffmanCompressor::compress(&s, false);
    println!("{}", compressed);
    let decompressed = huffman::HuffmanCompressor::decompress(&compressed);
    assert_eq!(s, decompressed);
});

test!(test_huffman_4, {
    let s: Vec<u8> =
        Vec::from("abcdefghijklmonprstuwvxyzABCDEFGHIJKLMNOPRSTUWVXYZ1234567890!@#$%^&*((_+");
    let compressed = huffman::HuffmanCompressor::compress(&s, true);
    println!("{}", compressed);
    let decompressed = huffman::HuffmanCompressor::decompress(&compressed);
    assert_eq!(s, decompressed);
});
