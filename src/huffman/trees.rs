use bit_vec::BitVec;
use std::collections::HashMap;

pub struct HuffmanTree {

}

impl HuffmanTree {
    pub fn get_mappings(weights: &HashMap<u8, u8>) -> (HashMap<u8, BitVec>, HashMap<BitVec, u8>) {
        let on_bits = HuffmanTree::get_mapping_on_bits(weights);
        let on_bytes = HuffmanTree::get_mapping_on_bytes(weights);
        (on_bits, on_bytes)
    }

    pub fn get_mapping_on_bits(_weights: &HashMap<u8, u8>) -> HashMap<u8, BitVec> {
        let mut mapping = HashMap::new();
        mapping.insert('a' as u8, BitVec::from_elem(4, false));  // 0001
        mapping.get_mut(&('a' as u8)).unwrap().set(3, true);
        mapping.insert('b' as u8, BitVec::from_elem(4, false));  // 0010
        mapping.get_mut(&('b' as u8)).unwrap().set(2, true);
        mapping.insert('c' as u8, BitVec::from_elem(4, false));  // 0011
        mapping.get_mut(&('c' as u8)).unwrap().set(2, true);
        mapping.get_mut(&('c' as u8)).unwrap().set(3, true);
        mapping.insert('~' as u8, BitVec::from_elem(3, false));  // 011
        mapping.get_mut(&('~' as u8)).unwrap().set(1, true);
        mapping.get_mut(&('~' as u8)).unwrap().set(2, true);

        mapping.insert('0' as u8, BitVec::from_elem(4, false));  // 1000
        mapping.get_mut(&('0' as u8)).unwrap().set(0, true);
        mapping.insert('1' as u8, BitVec::from_elem(4, false));  // 1001
        mapping.get_mut(&('1' as u8)).unwrap().set(0, true);
        mapping.get_mut(&('1' as u8)).unwrap().set(3, true);
        mapping.insert('2' as u8, BitVec::from_elem(4, false));  // 1010
        mapping.get_mut(&('2' as u8)).unwrap().set(0, true);
        mapping.get_mut(&('2' as u8)).unwrap().set(2, true);
        mapping.insert('3' as u8, BitVec::from_elem(4, false));  // 1011
        mapping.get_mut(&('3' as u8)).unwrap().set(0, true);
        mapping.get_mut(&('3' as u8)).unwrap().set(2, true);
        mapping.get_mut(&('3' as u8)).unwrap().set(3, true);
        mapping.insert('4' as u8, BitVec::from_elem(4, false));  // 1100
        mapping.get_mut(&('4' as u8)).unwrap().set(0, true);
        mapping.get_mut(&('4' as u8)).unwrap().set(1, true);
        mapping.insert('5' as u8, BitVec::from_elem(4, false));  // 1101
        mapping.get_mut(&('5' as u8)).unwrap().set(0, true);
        mapping.get_mut(&('5' as u8)).unwrap().set(1, true);
        mapping.get_mut(&('5' as u8)).unwrap().set(3, true);
        mapping.insert('6' as u8, BitVec::from_elem(4, false));  // 1110
        mapping.get_mut(&('6' as u8)).unwrap().set(0, true);
        mapping.get_mut(&('6' as u8)).unwrap().set(1, true);
        mapping.get_mut(&('6' as u8)).unwrap().set(2, true);
        mapping.insert('7' as u8, BitVec::from_elem(4, false));  // 1111
        mapping.get_mut(&('7' as u8)).unwrap().set(0, true);
        mapping.get_mut(&('7' as u8)).unwrap().set(1, true);
        mapping.get_mut(&('7' as u8)).unwrap().set(2, true);
        mapping.get_mut(&('7' as u8)).unwrap().set(3, true);
        mapping.insert('8' as u8, BitVec::from_elem(5, false));  // 01010
        mapping.get_mut(&('8' as u8)).unwrap().set(1, true);
        mapping.get_mut(&('8' as u8)).unwrap().set(3, true);
        mapping.insert('9' as u8, BitVec::from_elem(5, false));  // 01011
        mapping.get_mut(&('9' as u8)).unwrap().set(1, true);
        mapping.get_mut(&('9' as u8)).unwrap().set(3, true);
        mapping.get_mut(&('9' as u8)).unwrap().set(4, true);

        mapping
    } 

    pub fn get_mapping_on_bytes(weights: &HashMap<u8, u8>) -> HashMap<BitVec, u8> {
        let on_bits = HuffmanTree::get_mapping_on_bits(weights);
        on_bits.into_iter().map(|(byte, bit)| {
            (bit, byte)
        })
        .collect()
    } 
}

