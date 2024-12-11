const MAX_BIT_LENGTH_OF_CHAR: usize = 9;

use std::cmp::max;
use std::collections::HashMap;

use crate::utils::bitvec_ext::BitVecSlice;
use bit_vec::BitVec;

use super::predef_codes;

const DEFAULT_PREDEF_VALUE: usize = 1;

pub struct HuffmanCodes {
    mapping_on_bits: HashMap<char, BitVec>,
    mapping_on_chars: HashMap<BitVec, char>,
}

impl HuffmanCodes {
    pub fn new_predefined() -> Self {
        HuffmanCodes {
            mapping_on_bits: HuffmanCodes::get_predefined_mapping_on_bits(),
            mapping_on_chars: HuffmanCodes::get_predefined_mapping_on_chars(),
        }
    }

    pub fn new_calc_on_text(_text: &str) -> Self {
        HuffmanCodes::new_predefined() // TODO
    }

    fn get_predefined_mapping_on_bits() -> HashMap<char, BitVec> {
        predef_codes::mapping_on_bits()
    }

    fn get_predefined_mapping_on_chars() -> HashMap<BitVec, char> {
        let mapping_on_bits = HuffmanCodes::get_predefined_mapping_on_bits();

        let mut reversed_map: HashMap<BitVec, char> = HashMap::new();
        for (key, value) in mapping_on_bits {
            reversed_map.insert(value, key);
        }

        reversed_map
    }

    pub fn map_on_bits(&self, c: char) -> BitVec {
        self.mapping_on_bits[&c].clone()
    }

    pub fn match_bits(&self, bits: &BitVec) -> (Option<char>, usize) {
        let mut prefix = BitVec::new();
        for (i, bit) in bits.iter().enumerate() {
            prefix.push(bit);
            if let Some(&c) = self.mapping_on_chars.get(&prefix) {
                return (Some(c), i);
            }
        }

        (None, 0)
    }
}
pub struct HuffmanCompressor {
    predefined: bool,
    huffman_codes: Option<HuffmanCodes>,
}

impl HuffmanCompressor {
    pub fn new(predefined: Option<usize>) -> Self {
        HuffmanCompressor {
            predefined: predefined.unwrap_or(DEFAULT_PREDEF_VALUE) > 0,
            huffman_codes: None,
        }
    }

    pub fn compress(&mut self, text: &str) -> BitVec {
        if self.predefined {
            self.huffman_codes = Some(HuffmanCodes::new_predefined())
        } else {
            self.huffman_codes = Some(HuffmanCodes::new_calc_on_text(text))
        }

        let huffman_codes = self
            .huffman_codes
            .as_ref()
            .expect("Huffman codes should be Some() at this moment");

        let mut bits = BitVec::new();
        for c in text.chars() {
            let char_bits = huffman_codes.map_on_bits(c);
            bits.extend(char_bits);
        }

        bits
    }

    pub fn decompress(&self, bits: &mut BitVec) -> String {
        let mut index: usize = 0;
        let mut chars: Vec<char> = Vec::new();
        while index < bits.len() {
            let end: usize = max(index + MAX_BIT_LENGTH_OF_CHAR, bits.len());
            let slice = bits.slice(index, end);
            let (c, increment) = self
                .huffman_codes
                .as_ref()
                .expect("Decompressing without huffman codes set!")
                .match_bits(&slice);
            chars.push(c.expect(
                "Bits in decompressed sequence do not match any char for current huffman codes.",
            ));
            index += increment;
        }

        chars.iter().collect()
    }
}
