pub fn calc_distinct_symbols_num(bytes: &[u8]) -> u8 {
    let mut bytes = bytes.to_owned();
    bytes.sort();
    bytes.dedup();
    bytes.len() as u8
}
