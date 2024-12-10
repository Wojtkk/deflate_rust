pub mod lz77;
pub mod huffman;
pub mod utils;

use std::collections::HashMap;

use bit_vec::BitVec;
use huffman::{HuffmanCodes, HuffmanCompressor};
use lz77::LZ77Compressor;

#[macro_use]
extern crate fstrings;

#[derive(Hash, PartialEq, Eq)]
pub enum Params {
    WindowSize,
    BlockSize,
    CodesPredef,
}

pub struct CompressionParams {
    command_line_aliases: HashMap<String, Params>,
    params: HashMap<Params, usize>,
}

impl CompressionParams {
    pub fn new() -> Self {
        let mut default_params: HashMap<Params, usize> = HashMap::new();
        default_params.insert(Params::WindowSize, 5);
        default_params.insert(Params::BlockSize, 3);

        let mut aliases: HashMap<String, Params> = HashMap::new();
        aliases.insert("-window_size".to_string(), Params::WindowSize);
        aliases.insert("-blocks_num".to_string(), Params::BlockSize);
        aliases.insert("-codes_predef".to_string(), Params::CodesPredef);
        Self {
            command_line_aliases: aliases,
            params: default_params,
        }
    }

    pub fn update(&mut self, alias: &String, value: usize) {
        let param = &self.command_line_aliases.get(alias).expect(&self.give_help_message());
        if let Some(old_val) = self.params.get_mut(param) {
            *old_val = value;
        }
    }

    #[allow(dead_code)]
    pub fn get_param(&self, param: &Params) -> usize {
        *self.params.get(param).unwrap()
    }

    pub fn give_help_message(&self) -> String {
        let info = "INFO:";
        let sep = "-----------------------";
        let possible_options = "xd";
        let t1 = "Usage is: cargo run -- [-options value]";
        let t2 = &f!("Possible 'options' are {possible_options} and 'value' should be an integer");

        [info, sep, t1, t2, sep].join("\n")
    }
}

struct DeflateCompression<'a> {
    compression_params: &'a CompressionParams,
    lz77_compressor: LZ77Compressor,
    huffman_compressor: HuffmanCompressor, 
}

impl<'a> DeflateCompression<'a> {
    pub fn new(compression_params: &'a CompressionParams) -> Self {
        let predefined_codes = compression_params.get_param(&Params::CodesPredef) > 0;
        DeflateCompression {
            compression_params: compression_params,
            lz77_compressor: LZ77Compressor::new(Some(compression_params.get_param(&Params::WindowSize)), None),
            huffman_compressor: HuffmanCompressor::new(predefined_codes)
        }
    }

    pub fn deflate_compress(&mut self, text: &String) -> BitVec {
        let lz77_output: String = self.lz77_compressor.compress(text);
        self.huffman_compressor.compress(&lz77_output)
    }

    pub fn deflate_decompress(&self, mut bytes: BitVec) -> String {
        let huffman_decompressed = self.huffman_compressor.decompress(&mut bytes);
        self.lz77_compressor.decompress(&huffman_decompressed)
    }
}

