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

fn run_compression_test_case(s_org: String, expected_compr: String, compressor: LZ77Compressor) {
    let s_compr = compressor.compress(&s_org);
    let s_decompr = compressor.decompress(&s_compr);
    assert_eq!(s_compr, expected_compr);
    assert_eq!(s_org, s_decompr);
}

#[test]
fn compression1() {
    let s_org = String::from("AbcdefghAbcdefghAbcdefgh");
    let s_compr_expected = String::from("AbcdefghAbcdefghAbcdefgh");
    let compressor_instance = LZ77Compressor::new(Some(6), Some(10));
    run_compression_test_case(s_org, s_compr_expected, compressor_instance);
}

#[test]
fn compression2() {
    let s_org = String::from("aaaaaaaaaabbbbbbbbbb");
    let s_compr_expected = String::from("aaaa~1~6~bbbb~1~6~");
    let compressor_instance = LZ77Compressor::new(Some(8), Some(6));
    run_compression_test_case(s_org, s_compr_expected, compressor_instance);
}

#[test]
fn compression3() {
    let s_org = String::from("aaaaaabbbbbbccccccaaabbbccc");
    let s_compr_expected = String::from("aaaaaabbbbbbccccccaaabbbccc");
    let compressor_instance = LZ77Compressor::new(Some(6), Some(6));
    run_compression_test_case(s_org, s_compr_expected, compressor_instance);
}

#[test]
fn compression4() {
    let s_org = String::from("aaaaaabbbbbbccccccaaabbbccc");
    let s_compr_expected = String::from("aaaaaabbbbbbccccccaaa~12~6~");
    let compressor_instance = LZ77Compressor::new(None, Some(7));
    run_compression_test_case(s_org, s_compr_expected, compressor_instance);
}

#[test]
fn compression5() {
    let s_org = String::from("aaaaaabbbbbabbababbbbbbbbbbaaaaaaaaabbbbbbabbbbbababababab");
    let s_compr_expected = String::from("aaaaaabbbbbabbab~11~6~~16~6~a~1~7~~15~7~~6~7~a~2~7~");
    let compressor_instance = LZ77Compressor::new(None, Some(7));
    run_compression_test_case(s_org, s_compr_expected, compressor_instance);
}

#[test]
fn compression6() {
    let big_word = String::from_iter(['a'; 100000]);
    let s_org = big_word.clone();
    let s_compr_expected = big_word;
    let compressor_instance = LZ77Compressor::new(Some(5), Some(5));
    run_compression_test_case(s_org, s_compr_expected, compressor_instance);
}

#[test]
fn compression7() {
    let big_word = String::from_iter(['a'; 100000]);
    let s_org = big_word.clone();
    let compressor_instance = LZ77Compressor::new(Some(5), Some(5));
    let s_compr = compressor_instance.compress(&s_org);
    let s_decompr = compressor_instance.decompress(&s_compr);
    assert_eq!(s_org, s_decompr);
}
