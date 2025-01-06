const MAX_BIT_LENGTH_OF_CHAR: usize = 9;
const DEFAULT_PREDEF_VALUE: usize = 1;
const HALF_ASCII_SZ: u8 = 64;
const ASCII_SZ: u8 = 64;

use std::cmp::max;
use std::collections::HashMap;

use crate::utils::bitvec_ext::BitVecSlice;
use bit_vec::BitVec;
use itertools::Itertools;

use super::{utils, trees::HuffmanTree, weights::WeightsCalculator};

pub struct HuffmanCodes {
    symbols_weights: HashMap<u8, u8>,
    symbols_num: usize, 
    mapping_on_bits: Option<HashMap<u8, BitVec>>,
    mapping_on_bytes: Option<HashMap<BitVec, u8>>,
}

impl HuffmanCodes {
    pub fn new_predefined(bytes: &Vec<u8>) -> Self {
        let weights = WeightsCalculator::default();
        HuffmanCodes {
            symbols_weights: weights, 
            symbols_num: utils::calc_distinct_symbols_num(bytes), 
            mapping_on_bits: Some(HuffmanTree::get_mapping_on_bits(weights)),
            mapping_on_bytes: None, 
        }
    }

    pub fn new_calc_on_bytes(bytes: &Vec<u8>) -> Self {
        let weights = WeightsCalculator::fitted_to_text(bytes);
        HuffmanCodes {
            symbols_weights: weights,
            mapping_on_bits: Some(HuffmanTree::get_mapping_on_bits(weights)),
            mapping_on_bytes: None, 
        }
    }

    pub fn new_from_compression_result(compression_result_bits: &BitVec) -> Self {
        let weights = HuffmanCodes::extract_weights_from_compression_result(compression_result_bits);
        HuffmanCodes {
            symbols_weights: weights,
            mapping_on_bits: None, 
            mapping_on_bytes: Some(HuffmanTree::get_mapping_on_bits(weights)), 
        }
    }

    fn extract_weights_from_compression_result(compression_result_bits: &BitVec) -> HashMap<u8, u8> {
        let version_bit =compression_result_bits[0]; 

        let bytes = compression_result_bits.to_bytes();
        let encoding_weights_space: usize = HuffmanCodes::set_last_bit(bytes[0], false) as usize;

        if version_bit {
            (1..encoding_weights_space+1).step_by(2).map(|i| {
                (bytes[i], bytes[i+1])
            })
            .collect()
        }
        else {
            (1..encoding_weights_space+1).map(|i|) {
               (i, bytes[i]) 
            }
            .collect()
        }
    }

    pub fn append_weights_encoding_tree(&self, bits: BitVec) ->  BitVec {
        let map = self.mapping_on_bits.clone().unwrap();
        let symbols_num_in_seq = map.len() as u8;

        let mut result = BitVec::new();
        let (version_bit, encoding_bytes_space): (_, u8) = if symbols_num_in_seq > HALF_ASCII_SZ {
            (false, ASCII_SZ)
        } else {
            (true, symbols_num_in_seq * 2)
        };

        let info_byte = HuffmanCodes::set_last_bit(encoding_bytes_space, version_bit);
        result.extend(BitVec::from_bytes(&[info_byte]));

        let weights: Vec<u8> = if symbols_num_in_seq > HALF_ASCII_SZ {
            (0..ASCII_SZ).map(|i| {
                *(self.symbols_weights.get(&i)).unwrap_or(&0)
            })
            .collect()
        } else {
            self.symbols_weights.clone().into_iter().map(|(symbol, weight)| {
                vec![symbol, weight]
            })
            .concat()
        };

        result.extend(BitVec::from_bytes(&weights));   
        result.extend(bits);

        result
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
            true => HuffmanCodes::new_predefined(ascii_bytes),
            false => HuffmanCodes::new_calc_on_bytes(ascii_bytes)
        };

        let mut encoded_input = BitVec::new();
        for c in ascii_bytes {
            let char_bits = huffman_codes.map_on_bits(*c);
            encoded_input.extend(char_bits);
        }

        huffman_codes.append_weights_encoding_tree(encoded_input)
    }

    pub fn decompress(bits: &mut BitVec) -> Vec<u8> {
        let huffman_codes = HuffmanCodes::new_from_compression_result(bits);
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
