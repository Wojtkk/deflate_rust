use std::collections::HashMap;
use bit_vec::BitVec;

pub fn mapping_on_bits() -> HashMap<char, BitVec> {
    let mut mapping = HashMap::new();
    mapping.insert('a', BitVec::from_elem(1,true));
    mapping.insert('a', BitVec::from_elem(1,false));
    
    mapping
}


