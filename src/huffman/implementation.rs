const MAX_BIT_LENGTH_OF_CHAR: usize = 9;

use std::cmp::max;
use std::collections::HashMap;

use crate::utils::bitvec_ext::BitVecSlice;
use bit_vec::BitVec;

use super::predef_codes;
use super::trees::HuffmanTree;

const DEFAULT_PREDEF_VALUE: usize = 1;

pub struct HuffmanCodes {
    mapping_on_bits: Option<HashMap<u8, BitVec>>,
    mapping_on_bytes: Option<HashMap<BitVec, u8>>,
}

impl HuffmanCodes {
    pub fn new_predefined() -> Self {
        HuffmanCodes {
            mapping_on_bits: Some(HuffmanTree::get_predefined_mapping_on_bits()),
            mapping_on_bytes: None, 
        }
    }

    pub fn new_calc_on_bytes(bytes: &Vec<u8>) -> Self {
        HuffmanCodes {
            mapping_on_bits: Some(HuffmanTree::get_mapping_from_text(bytes)),
            mapping_on_bytes: None, InitialClientState
        }
    }

    pub fn extract_from_compression_result(compression_result_bits: &BitVec) -> Self {
        HuffmanCodes {
            mapping_on_bits: None, 
            mapping_on_bytes: Some(HuffmanCodes::_extract_from_compression_result(compression_result_bits)), 
        }
    }

    fn _extract_from_compression_result(compression_result_bits: &BitVec) -> HashMap<BitVec, u8> {
        
    }

    pub fn append_encoded_tree(&self, bits: BitVec) ->  BitVec {
        // to begin with tomorrow lol    
    }

    pub fn map_on_bits(&self, c: u8) -> BitVec {
        let map = self.mapping_on_bits.as_ref().unwrap();
        map[&c].clone()
    }

    pub fn match_bits(&self, bits: &BitVec) -> (Option<u8>, usize) {
        let mut prefix = BitVec::new();
        let map = self.mapping_on_bytes.as_ref().unwrap();
        for (i, bit) in bits.iter().enumerate() {
            prefix.push(bit);
            if let Some(&c) = map.get(&prefix) {
                return (Some(c), i);
            }
        }

        (None, 0)
    }
}
pub struct HuffmanCompressor {
    predefined: bool,
}

impl HuffmanCompressor {
    pub fn compress(ascii_bytes: &Vec<u8>, predefined_codes: bool) -> BitVec {
        let huffman_codes = match predefined_codes {
            true => HuffmanCodes::new_predefined(),
            false => HuffmanCodes::new_calc_on_bytes(ascii_bytes)
        };

        let mut encoded_input = BitVec::new();
        for c in ascii_bytes {
            let char_bits = huffman_codes.map_on_bits(*c);
            encoded_input.extend(char_bits);
        }

        huffman_codes.append_encoded_tree(encoded_input)
    }

    pub fn decompress(bits: &mut BitVec) -> Vec<u8> {
        let huffman_codes = HuffmanCodes::extract_from_compression_result(bits);
        let mut index: usize = 0;
        let mut chars: Vec<u8> = Vec::new();
        while index < bits.len() {
            let end: usize = max(index + MAX_BIT_LENGTH_OF_CHAR, bits.len());
            let slice = bits.slice(index, end);
            let (c, increment) = huffman_codes.match_bits(&slice);
            chars.push(c.expect("Bits in decompressed sequence do not match any char for current huffman codes."));
            index += increment;
        }

        chars.into_iter().collect()
    }
}
