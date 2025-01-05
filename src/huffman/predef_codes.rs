use bit_vec::BitVec;
use std::collections::HashMap;

pub fn mapping_on_bits() -> HashMap<u8, BitVec> {
    let mut mapping = HashMap::new();
    mapping.insert('a' as u8, BitVec::from_bytes(&[0, 0]));
    mapping.insert('b' as u8, BitVec::from_bytes(&[0, 1]));
    mapping.insert('c' as u8, BitVec::from_bytes(&[1, 0]));
    mapping.insert('~' as u8, BitVec::from_bytes(&[1, 1]));

    mapping
}
