use compression::CompressionParams;
use compression::DeflateCompression;

#[macro_use]
mod utils;

test!(test_deflate_1, {
    let s = String::from("abcabcbabcbbhjklijhga789!");
    let mut compressor = DeflateCompression::new(&CompressionParams::new());
    let compressed = compressor.deflate_compress(&s);
    let decompressed = compressor.deflate_decompress(&compressed);
    assert_eq!(s, decompressed);
});
