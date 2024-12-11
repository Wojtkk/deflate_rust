use compression::CompressionParams;
use compression::DeflateCompression;
use std::env;
use std::io;
use std::io::prelude::*;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
fn main() {
    let mut compression_params = CompressionParams::new();

    let args: Vec<String> = env::args().collect();
    for i in (1..args.len() - 1).step_by(2) {
        let alias = &args[i];
        let value = &args[i + 1]
            .parse()
            .unwrap_or_else(|_| panic!("{}", compression_params.give_help_message()));
        compression_params.update(alias, *value);
    }

    let mut deflate_compressor = DeflateCompression::new(&compression_params);

    let sep = compression::lz77::implementation::ResultEncoding::get_separator();
    println!(
        "Please type String, and press enter, but please avoid {} among the characters.",
        sep
    );
    println!("You can do it multiple times, if you want to stop type \"Bye\" and press enter.");

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let to_compress = line.unwrap();
        if to_compress == "Bye" {
            break;
        }

        let compreseed = deflate_compressor.deflate_compress(&to_compress);
        let decompressed = deflate_compressor.deflate_decompress(&compreseed);

        println!("---------------------");
        println!("To compress: {}", to_compress);
        println!("To compressed: {}", compreseed);
        println!("To decompressed: {}", decompressed);
        println!("\n");
    }
}
