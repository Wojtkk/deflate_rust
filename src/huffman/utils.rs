
pub fn calc_distinct_symbols_num(bytes: &Vec<u8>) -> usize {
    let mut bytes = bytes.clone();
    bytes.dedup();
    bytes.len()
}

pub fn set_last_bit(x: u8, b: bool) -> u8 {
    let mask = !(1 << 7);
    let flag = (b as u8) << 7;
    x & mask | flag
}
