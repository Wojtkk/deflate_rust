
pub fn calc_distinct_symbols_num(bytes: &Vec<u8>) -> u8 {
    let mut bytes = bytes.clone();
    bytes.sort();
    bytes.dedup();
    bytes.len() as u8
}

pub fn set_last_bit(x: u8, b: bool) -> u8 {
    let mask = !(1 << 7);
    let flag = (b as u8) << 7;
    x & mask | flag
}
