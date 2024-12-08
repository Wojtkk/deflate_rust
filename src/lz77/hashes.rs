use std::array;

pub enum HashConstants {
    M1 = 
    PowerBase = 27,
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct HashPair((usize, usize));

pub struct HashTable {
    h: Vec<HashPair>,
    pow: Vec<(usize, usize)>,
    shift: usize,
    m: [u64; 2],
    p: u64,
}

impl HashTable {
    pub fn new(str: &String) -> Self {
        HashTable {
            h: Vec::new(),
            pow: Vec::new(), 
            shift: power_index_shift, 
            m: [HashConstants::],
            p: HashConstants::PowerBase as u64,
        }
    }

    pub fn get_hash(&self, start: usize, stop: usize) -> HashPair {

    }

    fn calc_pows(&mut self) {
        single
    }

    fn calc_single_pows(&self, modulo: us) -> Vec<usize> {

    }
}


