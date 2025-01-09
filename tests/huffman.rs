use bit_vec::BitVec;
//use compression::huffman::HuffmanCompressor;
use compression::huffman::{self, HuffmanTreeCreator};
use std::collections::HashMap;

#[test]
fn test_building_tree_from_weights_1() {
    let mut weights = HashMap::new();
    weights.insert('a' as u8, 1 as u8);
    weights.insert('b' as u8, 2 as u8);
    weights.insert('c' as u8, 3 as u8);

    let (on_bits, on_bytes)  = HuffmanTreeCreator::get_mappings(&weights);

    let mut expected_on_bits = HashMap::new();

    // 'a' becomes 100 (prepend 1 to 00)
    expected_on_bits.insert('a' as u8, BitVec::from_elem(3, false)); // 100
    expected_on_bits.get_mut(&('a' as u8)).unwrap().set(0, true);

    // 'b' becomes 101 (prepend 1 to 01)
    expected_on_bits.insert('b' as u8, BitVec::from_elem(3, false)); // 101
    expected_on_bits.get_mut(&('b' as u8)).unwrap().set(0, true);
    expected_on_bits.get_mut(&('b' as u8)).unwrap().set(2, true);

    // 'c' becomes 11 (prepend 1 to 1)
    expected_on_bits.insert('c' as u8, BitVec::from_elem(2, true)); // 11
    expected_on_bits.get_mut(&('c' as u8)).unwrap().set(0, true);

    let expected_on_bytes: HashMap<BitVec, u8> = expected_on_bits.clone().into_iter().map(|(bytes, bits)| {
        (bits, bytes)
    })
    .collect();

    assert_eq!(on_bits, expected_on_bits);
    assert_eq!(on_bytes, expected_on_bytes);
}

#[test]
fn test_building_tree_from_weights_2() {
    let mut weights = HashMap::new();
    weights.insert('a' as u8, 16 as u8);
    weights.insert('b' as u8, 32 as u8);
    weights.insert('c' as u8, 32 as u8);
    weights.insert('d' as u8, 8 as u8);
    weights.insert('e' as u8, 8 as u8);

    let (on_bits, on_bytes)  = HuffmanTreeCreator::get_mappings(&weights);

    let mut expected_on_bits = HashMap::new();

    expected_on_bits.insert('a' as u8, BitVec::from_elem(4, false)); // 1100
    expected_on_bits.get_mut(&('a' as u8)).unwrap().set(0, true);
    expected_on_bits.get_mut(&('a' as u8)).unwrap().set(1, true);
    
    expected_on_bits.insert('b' as u8, BitVec::from_elem(3, true)); // 111
    
    expected_on_bits.insert('c' as u8, BitVec::from_elem(2, false)); // 10
    expected_on_bits.get_mut(&('c' as u8)).unwrap().set(0, true);
    
    expected_on_bits.insert('d' as u8, BitVec::from_elem(5, false)); // 11010
    expected_on_bits.get_mut(&('d' as u8)).unwrap().set(0, true);
    expected_on_bits.get_mut(&('d' as u8)).unwrap().set(1, true);
    expected_on_bits.get_mut(&('d' as u8)).unwrap().set(3, true);
    
    expected_on_bits.insert('e' as u8, BitVec::from_elem(5, true)); // 11011
    expected_on_bits.get_mut(&('e' as u8)).unwrap().set(2, false);
    
    let expected_on_bytes: HashMap<BitVec, u8> = expected_on_bits.clone().into_iter().map(|(bytes, bits)| {
        (bits, bytes)
    })
    .collect();

    assert_eq!(on_bits, expected_on_bits);
    assert_eq!(on_bytes, expected_on_bytes);
}

#[test]
fn test_huffman_1() {
    let s: Vec<u8> = Vec::from("abcabcbabcbb");
    let compressed = huffman::HuffmanCompressor::compress(&s, false);
    let decompressed = huffman::HuffmanCompressor::decompress(&compressed);
    assert_eq!(s, decompressed);
}

#[test]
fn test_huffman_2() {
    let s: Vec<u8> = Vec::from("abcdefghijklmonprstuwvxyzABCDEFGHIJKLMNOPRSTUWVXYZ1234567890!@#$%^&*((_+");
    let compressed = huffman::HuffmanCompressor::compress(&s, false);
    println!("{}", compressed);
    let decompressed = huffman::HuffmanCompressor::decompress(&compressed);
    assert_eq!(s, decompressed);
}

#[test]
fn test_huffman_3() {
    let big_word = String::from_iter(['a'; 10000]);
    let s: Vec<u8> = Vec::from(big_word);
    let compressed = huffman::HuffmanCompressor::compress(&s, false);
    println!("{}", compressed);
    let decompressed = huffman::HuffmanCompressor::decompress(&compressed);
    assert_eq!(s, decompressed);
}
