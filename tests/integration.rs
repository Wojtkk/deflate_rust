use compression::DeflateCompression;
use compression::CompressionParams;

#[test]
fn test_deflate_1() {
    let s = String::from("abcabcbabcbbhjklijhga789!");
    let mut compressor = DeflateCompression::new(&CompressionParams::new());
    let compressed = compressor.deflate_compress(&s);
    let decompressed = compressor.deflate_decompress(&compressed);
    assert_eq!(s, decompressed);
}
