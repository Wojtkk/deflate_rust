const MAX_BIT_LENGTH_OF_CHAR: usize = 8;
const HALF_ASCII_SZ: u8 = 64;
const ASCII_SZ: u8 = 64;
const NUM_OF_BITS_IN_BYTE: u8 = 8;

use std::cmp::min;
use std::collections::HashMap;
use std::mem;

use bit_vec::BitVec;
use itertools::Itertools;

use super::{utils, trees::HuffmanTree, weights::WeightsCalculator};

pub struct HuffmanCodes {
    symbols_weights: HashMap<u8, u8>,
    symbols_num: Option<u8>, 
    mapping_on_bits: HashMap<u8, BitVec>,
    mapping_on_bytes: HashMap<BitVec, u8>,
}

impl HuffmanCodes {
    pub fn new_predefined(bytes: &Vec<u8>) -> Self {
        let weights = WeightsCalculator::default();
        let (on_bits, on_bytes) = HuffmanTree::get_mappings(&weights);
        HuffmanCodes {
            symbols_weights: weights, 
            symbols_num: Some(utils::calc_distinct_symbols_num(bytes)), 
            mapping_on_bits: on_bits,
            mapping_on_bytes: on_bytes, 
        }
    }

    pub fn new_calc_on_bytes(bytes: &Vec<u8>) -> Self {
        let weights = WeightsCalculator::fitted_to_text(bytes);
        let (on_bits, on_bytes) = HuffmanTree::get_mappings(&weights);
        HuffmanCodes {
            symbols_weights: weights,
            symbols_num: Some(utils::calc_distinct_symbols_num(bytes)),
            mapping_on_bits: on_bits, 
            mapping_on_bytes: on_bytes, 
        }
    }

    pub fn new_from_compression_result(compression_result_bits: &BitVec) -> Self {
        let weights = HuffmanCodes::extract_weights_from_compression_result(compression_result_bits);
        let (on_bits, on_bytes) = HuffmanTree::get_mappings(&weights);
        HuffmanCodes {
            symbols_weights: weights,
            symbols_num: None, 
            mapping_on_bits: on_bits, 
            mapping_on_bytes: on_bytes, 
        }
    }

    fn extract_weights_from_compression_result(compression_result_bits: &BitVec) -> HashMap<u8, u8> {
        let version_bit =compression_result_bits[0]; 

        let bytes = compression_result_bits.to_bytes();
        let encoding_weights_space: usize = utils::set_last_bit(bytes[0], false) as usize + 1;

        if version_bit {
            (1..encoding_weights_space).step_by(2).map(|i| {
                (bytes[i], bytes[i+1])
            })
            .collect()
        }
        else {
            (1..encoding_weights_space).map(|i| {
               (i as u8, bytes[i]) 
            })
            .collect()
        }
    }

    pub fn append_weights_encoding_tree(&self, bits: BitVec) ->  BitVec {
        let symbols_num_in_seq = self.symbols_num.unwrap();
        println!("xdd {}", symbols_num_in_seq);

        let mut result = BitVec::new();
        let (version_bit, encoding_bytes_space): (_, u8) = if symbols_num_in_seq > HALF_ASCII_SZ {
            (false, ASCII_SZ)
        } else {
            (true, symbols_num_in_seq * 2)
        };

        let info_byte = utils::set_last_bit(encoding_bytes_space, version_bit);
        println!("{}", info_byte);
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

    pub fn remove_weights_enconding(bits: &BitVec) -> BitVec {
        let bytes = bits.to_bytes();
        let mut encoding_weights_space: usize = utils::set_last_bit(bytes[0], false) as usize + 1; 
        println!("{}", encoding_weights_space);
        encoding_weights_space *= NUM_OF_BITS_IN_BYTE as usize;
        println!("{}", encoding_weights_space);
        (encoding_weights_space..bits.len()).map(|i|{
            bits.get(i).unwrap()
        })
        .collect()
    }

    pub fn map_on_bits(&self, c: u8) -> BitVec {
        println!("{}", c);
        self.mapping_on_bits[&c].clone()
    }

    pub fn match_bits(&self, bits: &BitVec) -> (Option<u8>, usize) {
        let mut prefix = BitVec::new();
        for (i, bit) in bits.iter().enumerate() {
            prefix.push(bit);
            if let Some(&c) = self.mapping_on_bytes.get(&prefix) {
                return (Some(c), i + 1);
            }
        }


        for (key, value) in &self.mapping_on_bytes {
            println!("{}: {} \n", key, value);
        }
        println!("{}", prefix);
        (None, 0)
    }
}
pub struct HuffmanCompressor;

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

    pub fn decompress(bits: &BitVec) -> Vec<u8> {
        let huffman_codes = HuffmanCodes::new_from_compression_result(bits);
        println!("{}", bits);
        let bits = HuffmanCodes::remove_weights_enconding(bits);  
        println!("{}", bits);
        let mut index: usize = 0;
        let mut chars: Vec<u8> = Vec::new();
        while index < bits.len() {
            let end: usize = min(index + MAX_BIT_LENGTH_OF_CHAR, bits.len());
            let slice: BitVec = (index..end).map(|i| {
                bits.get(i).unwrap()
            })
            .collect();

            println!("xdd {}", index);
            let (c, increment) = huffman_codes.match_bits(&slice);
            println!("{}", c.unwrap());
            chars.push(c.expect("Bits in decompressed sequence do not match any char for current huffman codes."));
            index += increment;
        }

        chars.into_iter().collect()
    }
}
