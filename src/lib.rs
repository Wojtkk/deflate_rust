pub mod huffman;
pub mod lz77;
pub mod utils;
use bit_vec::BitVec;

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
    ApplyHuffman,
    ApplyLZ77,
}

impl Params {
    pub fn explain(&self) -> &str {
        match self {
            Params::WindowSize => "Length of the interval in which in case of identical words occurence, the later one will be compressed.",
            Params::MaxBlockSize => "Max length of word we will spot in sliding window.",
            Params::CodesPredef => "If 0 then huffman codes won't be predefined, otherwise we will calculate it according to the given text.",
            Params::ApplyHuffman => "If 0 then huffman algorithm is not applied in compression, otherwise it is",
            Params::ApplyLZ77 => "If 0 then lz77 algorithm is not applied in compression, otherwise it is"
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
        default_params.insert(Params::CodesPredef, Some(0));
        default_params.insert(Params::ApplyHuffman, Some(1));
        default_params.insert(Params::ApplyLZ77, Some(1));

        let mut aliases: HashMap<String, Params> = HashMap::new();
        aliases.insert("-window_size".to_string(), Params::WindowSize);
        aliases.insert("-max_len_of_block".to_string(), Params::MaxBlockSize);
        aliases.insert("-codes_predef".to_string(), Params::CodesPredef);
        aliases.insert("-huff".to_string(), Params::ApplyHuffman);
        aliases.insert("-lz77".to_string(), Params::ApplyLZ77);
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
    apply_huffman: bool,
    huffman_codes_predefined: bool,
    apply_lz77: bool,
}

impl DeflateCompression {
    pub fn new(compression_params: &CompressionParams) -> Self {
        let window_size = compression_params.get_param(&Params::WindowSize);
        let max_block_size = compression_params.get_param(&Params::MaxBlockSize);
        let predefined_codes = compression_params
            .get_param(&Params::CodesPredef)
            .unwrap_or(0)
            > 0;
        let apply_huffman = compression_params
            .get_param(&Params::ApplyHuffman)
            .unwrap_or(1)
            > 0;
        let apply_lz77 = compression_params
            .get_param(&Params::ApplyLZ77)
            .unwrap_or(1)
            > 0;
        DeflateCompression {
            lz77_compressor: LZ77Compressor::new(window_size, max_block_size),
            apply_huffman,
            huffman_codes_predefined: predefined_codes,
            apply_lz77,
        }
    }

    pub fn deflate_compress(&mut self, text: &String) -> utils::TypeOr<BitVec, Vec<u8>> {
        let mut result = Vec::from(text.as_bytes());
        if self.apply_lz77 {
            result = self.lz77_compressor.compress(&result);
        }

        if self.apply_huffman {
            return utils::TypeOr::Left(HuffmanCompressor::compress(
                &result,
                self.huffman_codes_predefined,
            ));
        }
        utils::TypeOr::Right(result)
    }

    pub fn deflate_decompress(&self, seq: &utils::TypeOr<BitVec, Vec<u8>>) -> String {
        let result = match seq {
            //utils::TypeOr::Left(bits) => self.huffman_compressor.decompress(bits),
            utils::TypeOr::Left(bits) => HuffmanCompressor::decompress(bits),
            utils::TypeOr::Right(bytes) => bytes.clone(),
        };

        if self.apply_lz77 {
            return String::from_utf8(self.lz77_compressor.decompress(&result)).unwrap();
        }
        String::from_utf8(result).unwrap()
    }
}
