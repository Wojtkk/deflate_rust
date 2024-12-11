use compression::lz77::hashes::HashTable;
use compression::lz77::LZ77Compressor;

#[test]
fn hashes() {
    let s = String::from("abcabcbabcbb");  
    let hash_table = HashTable::new(&s, Some(1));

    assert_eq!(hash_table.get_hash(0, 1), hash_table.get_hash(0, 1));
    assert_eq!(hash_table.get_hash(0, 2), hash_table.get_hash(3, 5));
    assert_eq!(hash_table.get_hash(3, 6), hash_table.get_hash(7, 10));

    assert_ne!(hash_table.get_hash(0, 2), hash_table.get_hash(0, 3));
    assert_ne!(hash_table.get_hash(1, 2), hash_table.get_hash(0, 1));
    assert_ne!(hash_table.get_hash(1, 10), hash_table.get_hash(0, 10));
} 

#[test]
fn compression1() {
    let s_org = String::from("AbcdefghAbcdefghAbcdefgh");
    let compressor_instance = LZ77Compressor::new(Some(6), Some(10));
    let s_compr = compressor_instance.compress(&s_org);
    println!("{}", s_compr);
    let decompress = compressor_instance.decompress(&s_compr);
    println!("{}", decompress);
    assert!(false);

}

#[test]
fn compression2() {
    let s_org = String::from("aaaaaaaaaabbbbbbbbbb");
    let compressor_instance = LZ77Compressor::new(Some(8), Some(6));
    let s_compr = compressor_instance.compress(&s_org);
    println!("{}", s_compr);
    let decompress = compressor_instance.decompress(&s_compr);
    println!("{}", decompress);
    assert!(false);
}

#[test]
fn compression3() {
    let s_org = String::from("aaaaaabbbbbbccccccaaabbbccc");
    let compressor_instance = LZ77Compressor::new(Some(6), Some(6));
    let s_compr = compressor_instance.compress(&s_org);
    println!("{}", s_compr);
    println!("{}", s_compr.to_string());
    let s_decompr = compressor_instance.decompress(&s_compr);
    println!("{}", s_decompr);
    assert!(false);
}

#[test]
fn compression4() {
    let s_org = String::from("aaaaaabbbbbbccccccaaabbbccc");
    let compressor_instance = LZ77Compressor::new(None, Some(7));
    let s_compr = compressor_instance.compress(&s_org);
    println!("{}", s_compr);
    println!("{}", s_compr.to_string());
    let s_decompr = compressor_instance.decompress(&s_compr);
    println!("{}", s_decompr);
    assert!(false);
}