use compression::lz77::hashes::HashTable;
use compression::lz77::LZ77Compressor;
use compression::lz77::implementation::ResultEncodingVec;

#[test]
fn hashes() {
    let s = String::from("abcabcbabcbb");  
    let hash_table = HashTable::new(&s, Some(1));

    assert_eq!(hash_table.get_hash(0, 1), hash_table.get_hash(0, 1));
    assert_eq!(hash_table.get_hash(0, 2), hash_table.get_hash(3, 5));
    assert_eq!(hash_table.get_hash(3, 6), hash_table.get_hash(7, 10));

    assert_ne!(hash_table.get_hash(0, 2), hash_table.get_hash(0, 3));
    assert_ne!(hash_table.get_hash(1, 2), hash_table.get_hash(0, 1));
    assert_ne!(hash_table.get_hash(0, 10), hash_table.get_hash(0, 10));
} 

fn compression() {
    let s = String::from("AbcdeAbcdeAbcde");

}