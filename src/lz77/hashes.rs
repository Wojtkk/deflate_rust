use std::cmp::min;

const DEFAULT_NUM_OF_HASH_LAYERS: usize = 2;
const MAX_REASONABLE_NUM_OF_HASH_LAYERS: usize = 4;

pub struct HashTable<'a> {
    single_hashes: Vec<SingleHashTable<'a>>,
}

impl<'a> HashTable<'a> {
    pub fn new(ascii_bytes: &'a Vec<u8>, num_of_hash_layers: Option<usize>) -> Self {
        let modulos = Vec::from_iter([1000000007, 100000009, 1000000021, 100000033]);
        let primes = Vec::from_iter([27, 29, 31, 37]);
        let hash_num = num_of_hash_layers.unwrap_or(DEFAULT_NUM_OF_HASH_LAYERS);
        let hash_num = min(hash_num, MAX_REASONABLE_NUM_OF_HASH_LAYERS);

        HashTable {
            single_hashes: Vec::from_iter(
                (0..hash_num).map(|i| SingleHashTable::new(ascii_bytes, modulos[i], primes[i])),
            ),
        }
    }

    pub fn get_hash(&self, start: usize, stop: usize) -> Hash {
        Hash(Vec::from_iter(
            self.single_hashes
                .iter()
                .map(|single_hash| single_hash.get_hash(start, stop)),
        ))
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Hash(Vec<usize>);

pub struct SingleHashTable<'a> {
    text: &'a Vec<u8>,
    pref: Vec<u32>,
    pow: Vec<u32>,
    modulo: u64,
}

impl<'a> SingleHashTable<'a> {
    pub fn new(ascii_bytes: &'a Vec<u8>, m: u64, p: u64) -> Self {
        let pows = SingleHashTable::calc_pow(ascii_bytes.len(), m, p);
        let prefs = SingleHashTable::calc_pref(ascii_bytes, &pows, m);
        SingleHashTable {
            text: ascii_bytes,
            pref: prefs,
            pow: pows,
            modulo: m,
        }
    }

    fn calc_pow(size: usize, m: u64, p: u64) -> Vec<u32> {
        let mut x: u64 = 1;
        (0..size)
            .map(|_| {
                x = (x * p) % m;
                x as u32
            })
            .collect()
    }

    fn calc_pref(ascii_bytes: &Vec<u8>, pows: &[u32], m: u64) -> Vec<u32> {
        let mut x: u64 = 0;
        pows.iter()
            .zip(ascii_bytes)
            .map(|(&p, c)| {
                x = (x + p as u64 * (*c as u64)) % m;
                x as u32
            })
            .collect()
    }

    pub fn get_hash(&self, start: usize, stop: usize) -> usize {
        let to_subtract = match start {
            0 => 0,
            _ => self.pref[start - 1],
        };

        let interval: u64 =
            (self.modulo + self.pref[stop] as u64 - to_subtract as u64) % self.modulo;
        let shift = self.text.len() - start - 1;
        let shifted_interval = (interval * self.pow[shift] as u64) % self.modulo;
        shifted_interval as usize
    }
}
