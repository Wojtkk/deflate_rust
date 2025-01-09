use bit_vec::BitVec;
//use compression::huffman::HuffmanCompressor;
use compression::huffman::HuffmanTree;
use std::collections::HashMap;

#[test]
fn test_building_tree_from_weights() {
    let weights = HashMap::new();
    weights.insert('a' as u8, 4 as u8);
    weights.insert('b' as u8, 11 as u8);
    weights.insert('c' as u8, 12 as u8);
    weights.insert('d' as u8, 5 as u8);
    weights.insert('d' as u8, 1 as u8);
    weights.insert('d' as u8, 1 as u8);

    let (on_bits, on_bytes)  = HuffmanTree::get_mappings(&weights);

    let expected_on_bits = HashMap::new();
    let expected_on_bytes = HashMap::new();
    expected_on_bits.insert(BitVec, v)
}

