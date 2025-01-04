pub mod huffman;
pub mod lz77;
pub mod utils;

use core::fmt;
use std::collections::HashMap;

use fstrings::{format_args_f, format_f};
use huffman::HuffmanCompressor;
use lz77::LZ77Compressor;

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum Params {
    WindowSize,
    MaxBlockSize,
    CodesPredef,
}

impl Params {
    pub fn explain(&self) -> &str {
        match self {
            Params::WindowSize => "Length of the interval in which in case of identical words occurence, the later one will be compressed.",
            Params::MaxBlockSize => "Max length of word we will spot in sliding window.",
            Params::CodesPredef => "If 1 then huffman codes will be predefined, otherwise we will calculate it according to the given text.",
        }
    }
}

pub struct HelpDisplayer<'a> {
    command_line_aliases: &'a HashMap<String, Params>,
}

impl<'a> HelpDisplayer<'a> {
    pub fn new(compression_params: &'a CompressionParams) -> Self {
        HelpDisplayer {
            command_line_aliases: &compression_params.command_line_aliases,
        }
    }
}

impl<'a> fmt::Display for HelpDisplayer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let info = "INFO:";
        let sep = "-----------------------\n";
        let possible_options: String = self
            .command_line_aliases
            .clone()
            .into_iter()
            .map(|(k, v)| k + ": " + v.explain() + "\n")
            .collect();
        let t1 = "Usage is: cargo run -- [options value]";
        let t2 =
            &format_f!("Possible 'options' are:\n{possible_options}    Value should be an integer");

        let message = [info, sep, t1, t2, sep].join("\n");
        write!(f, "{}", message)
    }
}

pub struct CompressionParams {
    command_line_aliases: HashMap<String, Params>,
    params: HashMap<Params, Option<usize>>,
}

impl Default for CompressionParams {
    fn default() -> Self {
        Self::new()
    }
}

impl CompressionParams {
    pub fn new() -> Self {
        let mut default_params: HashMap<Params, Option<usize>> = HashMap::new();
        default_params.insert(Params::WindowSize, None);
        default_params.insert(Params::MaxBlockSize, None);
        default_params.insert(Params::CodesPredef, None);

        let mut aliases: HashMap<String, Params> = HashMap::new();
        aliases.insert("-window_size".to_string(), Params::WindowSize);
        aliases.insert("-max_len_of_block".to_string(), Params::MaxBlockSize);
        aliases.insert("-codes_predef".to_string(), Params::CodesPredef);
        Self {
            command_line_aliases: aliases,
            params: default_params,
        }
    }

    pub fn update(&mut self, alias: &String, value: usize) {
        let param = &self
            .command_line_aliases
            .get(alias)
            .unwrap_or_else(|| panic!("{}", HelpDisplayer::new(self)));
        if let Some(old_val) = self.params.get_mut(param) {
            *old_val = Some(value);
        }
    }

    #[allow(dead_code)]
    pub fn get_param(&self, param: &Params) -> Option<usize> {
        *self.params.get(param).unwrap()
    }
}

#[warn(dead_code)]
pub struct DeflateCompression {
    lz77_compressor: LZ77Compressor,
    _huffman_compressor: HuffmanCompressor,
}

impl DeflateCompression {
    pub fn new(compression_params: &CompressionParams) -> Self {
        let window_size = compression_params.get_param(&Params::WindowSize);
        let max_block_size = compression_params.get_param(&Params::MaxBlockSize);
        let predefined_codes = compression_params.get_param(&Params::CodesPredef);
        DeflateCompression {
            lz77_compressor: LZ77Compressor::new(window_size, max_block_size),
            _huffman_compressor: HuffmanCompressor::new(predefined_codes),
        }
    }

    pub fn deflate_compress(&mut self, text: &String) -> Vec<u8> {
        let text = Vec::from(text.as_bytes());

        self.lz77_compressor.compress(&text)
        //self.huffman_compressor.compress(&lz77_output)
        // This would be the second part :DD
    }

    pub fn deflate_decompress(&self, text: &[u8]) -> String {
        let decompressed_bytes = self.lz77_compressor.decompress(text);
        String::from_utf8(decompressed_bytes).unwrap()
        // self.lz77_compressor.decompress(&huffman_decompressed)
        // This would be the second part :DD
    }
}
