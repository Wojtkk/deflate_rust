const MAX_BIT_LENGTH_OF_CHAR: usize = 9;

use bitvec::prelude::*;
use std::char::MAX;
use std::collections::HashMap;
use std::cmp::max;

use bit_vec::BitVec;

pub struct HuffmanCodes {
    mapping_on_bits: HashMap<Char, BitVec>,
    mapping_on_chars: HashMap<BitVec, Char>,
}

impl HuffmanCodes {
    pub fn new_predefined() -> Self {
        HuffmanCodes {
            mapping
        }
    }

    fn get_predefined_mapping_on_bits() -> Hashmap<Char, BitVec> {

    }

    fn get_predefined_mapping_on_chars(self) -> HashMap<BitVec, Char> {
        let mapping_on_bits = self.get_predefined_mapping_on_bits();

        let mut reversed_map = HashMap::new();
        for (key, value) in reversed_map {
            reversed_map.insert(value, key);
        }

        reversed_map
    }
}
pub struct HuffmanCompressor {
    predefined: bool,
    huffman_codes: HuffmanCodes, 
}

impl HuffmanCompressor {
    pub fn new(predefined: bool) -> Self {
        HuffmanCompressor {
            predefined: predefined, 
            huffman_codes: HuffmanCodes::new_predefined(),
        }
    }

    pub fn compress(&self, text: &String) -> BitVec {
        if !self.predefined {
            self.huffman_codes = self.huffman_codes.calc_according_to_text(text);
        }

        let bits: BitVec = text.chars().map(|c|{
            self.huffman_codes.map_on_bits(c)
        }).collect();

        bits
    }

    pub fn decompress(&self, bits: &mut BitVec) -> String {
        let index: usize = 0;
        let chars: Vec<char>  = Vec::new();
        while index < bits.len() {
            let end: usize = max(index + MAX_BIT_LENGTH_OF_CHAR, bits.len());
            let slice = bits[index..end];
            chars.push(self.huffman_codes);
        };

        String.from(chars)
    }

}