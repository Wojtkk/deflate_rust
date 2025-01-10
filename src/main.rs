use compression::utils::TypeOr;
use compression::CompressionParams;
use compression::DeflateCompression;
use compression::HelpDisplayer;
use std::env;
use std::io;
use std::io::prelude::*;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
fn main() {
    let mut compression_params = CompressionParams::new();

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        for i in (1..args.len() - 1).step_by(2) {
            let alias = &args[i];
            let value = &args[i + 1]
                .parse()
                .unwrap_or_else(|_| panic!("{}", HelpDisplayer::new(&compression_params)));
            compression_params.update(alias, *value);
        }
    } else {
        println!("{}", HelpDisplayer::new(&compression_params));
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
        let compressed = match compreseed {
            TypeOr::Left(bits) => bits.to_string(),
            TypeOr::Right(bytes) => String::from_utf8(bytes).unwrap(),
        };

        println!("---------------------");
        println!("To compress: {}", to_compress);
        println!("To compressed: {}", compressed);
        println!("To decompressed: {}", decompressed);
        println!("\n");
    }
}
