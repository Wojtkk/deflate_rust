use itertools::Itertools;

use super::hashes::{Hash, HashTable};
use std::{
    cmp::min,
    collections::{HashMap, VecDeque},
};

const DEFAULT_WINDOW_SIZE: usize = 32768;
const DEFAULT_LEN_TRESHOLD: usize = 6;
const DEFAULT_ASCII_NUM_OF_SEPARATOR: u8 = 126;

#[derive(Clone, Debug)]
pub enum ResultEncoding {
    Ascii(u8),
    Reference(usize, usize),
}

impl ResultEncoding {
    pub fn get_separator() -> u8 {
        DEFAULT_ASCII_NUM_OF_SEPARATOR
    }

    pub fn len_treshold() -> usize {
        DEFAULT_LEN_TRESHOLD
    }

    pub fn to_ascii_bytes(&self) -> Vec<u8> {
        let s = match self {
            ResultEncoding::Ascii(c) => vec![*c],
            ResultEncoding::Reference(d, l) => {
                let sep = vec![ResultEncoding::get_separator()];
                [
                    sep.clone(),
                    d.to_string().as_bytes().to_vec(),
                    sep.clone(),
                    l.to_string().as_bytes().to_vec(),
                    sep,
                ]
                .concat()
                .to_vec()
            }
        };

        s
    }
}

#[derive(Debug)]
pub struct ResultEncodingVec {
    vec: Vec<ResultEncoding>,
}

impl Default for ResultEncodingVec {
    fn default() -> Self {
        Self::new()
    }
}

impl ResultEncodingVec {
    pub fn new() -> Self {
        ResultEncodingVec { vec: Vec::new() }
    }

    pub fn push(&mut self, elem: ResultEncoding) {
        self.vec.push(elem);
    }

    pub fn reverse(&mut self) {
        self.vec.reverse();
    }

    pub fn from_ascii_bytes(ascii_bytes: &[u8]) -> Self {
        let (mut i, mut res, sep) = (0, ResultEncodingVec::new(), ResultEncoding::get_separator());
        while i < ascii_bytes.len() {
            let c = ascii_bytes[i];
            if c == sep {
                let (enc_reference, processed_size) =
                    ResultEncodingVec::parse_reference(ascii_bytes, i);
                res.push(enc_reference);
                i += processed_size;
            } else {
                res.push(ResultEncoding::Ascii(c));
                i += 1;
            }
        }

        res
    }

    pub fn expand(&self) -> Vec<u8> {
        let mut s = Vec::new();
        self.vec
            .clone()
            .into_iter()
            .map(|e| match e {
                ResultEncoding::Ascii(c) => {
                    s.push(c);
                    vec![c]
                }
                ResultEncoding::Reference(d, l) => {
                    let i = s.len() - d;
                    (i..i + l)
                        .map(|j| {
                            s.push(s[j]);
                            s[j]
                        })
                        .collect()
                }
            })
            .concat()
    }

    fn parse_reference(ascii_bytes: &[u8], start_index: usize) -> (ResultEncoding, usize) {
        let sep = ResultEncoding::get_separator();
        let mut sep_counter = 0;
        let mut i = start_index;
        let (mut dist, mut len) = (String::new(), String::new());
        loop {
            let c = ascii_bytes[i];
            if c == sep {
                sep_counter += 1;
                if sep_counter == 3 {
                    break;
                }
                i += 1;
                continue;
            }
            match sep_counter {
                1 => dist.push(c as char),
                2 => len.push(c as char),
                _ => break,
            };
            i += 1;
        }

        let reference = ResultEncoding::Reference(dist.parse().unwrap(), len.parse().unwrap());
        (reference, i - start_index + 1)
    }
}

struct SlidingWindow<'a> {
    text: &'a Vec<u8>,
    max_len_to_reduce: usize,
    window_size: usize,
    hashes: HashTable<'a>,
    subwords: Vec<HashMap<Hash, VecDeque<usize>>>,
    partial_result: Vec<(usize, usize)>,
}

impl<'a> SlidingWindow<'a> {
    pub fn new(text: &'a Vec<u8>, window_size: usize, max_len_to_reduce: Option<usize>) -> Self {
        let ws = min(window_size, text.len());
        let max_len = min(
            max_len_to_reduce.unwrap_or(SlidingWindow::sqrt_usize(window_size)),
            ws,
        );
        SlidingWindow {
            max_len_to_reduce: max_len,
            window_size: ws,
            text,
            hashes: HashTable::new(text, None),
            subwords: Vec::from_iter((0..max_len).map(|_| HashMap::new())),
            partial_result: Vec::new(),
        }
    }

    fn sqrt_usize(n: usize) -> usize {
        (n as f64).sqrt() as usize
    }

    pub fn run(&mut self) {
        for i in 0..self.text.len() {
            self.remove_old_subwords(i);
            let mut new_subwords_descending = self.extract_new_subwords(i);
            self.update_result(&new_subwords_descending, i);
            new_subwords_descending.reverse();
            let new_subwords_aescending = new_subwords_descending;
            self.add_new_subwords(new_subwords_aescending);
        }
    }

    fn remove_old_subwords(&mut self, curr_index: usize) {
        if curr_index >= self.window_size {
            let start = curr_index - self.window_size;
            for i in 0..self.max_len_to_reduce {
                if start + i > self.text.len() {
                    break;
                }

                let h = self.hashes.get_hash(start, start + i);
                if let Some(positions) = self.subwords[i].get_mut(&h) {
                    positions.pop_back().unwrap();
                    if positions.is_empty() {
                        self.subwords[i].remove(&h);
                    }
                }
            }
        }
    }

    fn extract_new_subwords(&self, index: usize) -> Vec<(Hash, usize)> {
        let start = if index >= self.max_len_to_reduce {
            index - self.max_len_to_reduce + 1
        } else {
            0
        };
        Vec::from_iter((start..index + 1).map(|i| {
            let h = self.hashes.get_hash(i, index);
            (h, i)
        }))
    }

    fn update_result(&mut self, new_subwords_desc: &[(Hash, usize)], index: usize) {
        for (i, subword) in new_subwords_desc.iter().enumerate() {
            let subword_length = new_subwords_desc.len() - i;
            let bucket = &self.subwords[subword_length - 1];
            if let Some(positions) = bucket.get(&subword.0) {
                let distance = index - positions[0] - subword_length + 1;
                self.partial_result.push((distance, subword_length));
                return;
            }
        }

        self.partial_result.push((0, 1));
    }

    fn add_new_subwords(&mut self, new_subwords: Vec<(Hash, usize)>) {
        for (i, (h, position)) in new_subwords.into_iter().enumerate() {
            if let Some(positions) = self.subwords[i].get_mut(&h) {
                positions.push_front(position);
            } else {
                self.subwords[i].insert(h, VecDeque::from_iter([position]));
            }
        }
    }

    pub fn get_result(&self) -> Vec<u8> {
        let mut result = ResultEncodingVec::new();
        let mut i = self.partial_result.len() - 1;
        loop {
            let (dist, len) = self.partial_result[i];
            if len >= ResultEncoding::len_treshold() {
                result.push(ResultEncoding::Reference(dist, len));
                if len > i {
                    break;
                }
                i -= len;
            } else {
                result.push(ResultEncoding::Ascii(self.text[i]));
                if i == 0 {
                    break;
                }
                i -= 1;
            }
        }

        result.reverse();
        result.vec.into_iter().map(|x| x.to_ascii_bytes()).concat()
    }
}

pub struct LZ77Compressor {
    window_size: usize,
    max_len_to_reduce: Option<usize>,
}

impl LZ77Compressor {
    pub fn new(window_size: Option<usize>, max_len_to_reduce: Option<usize>) -> Self {
        LZ77Compressor {
            window_size: window_size.unwrap_or(DEFAULT_WINDOW_SIZE),
            max_len_to_reduce,
        }
    }

    pub fn compress(&self, ascii_bytes: &Vec<u8>) -> Vec<u8> {
        let mut sw = SlidingWindow::new(ascii_bytes, self.window_size, self.max_len_to_reduce);
        sw.run();
        sw.get_result()
    }

    pub fn decompress(&self, ascii_bytes: &[u8]) -> Vec<u8> {
        let encoded_result = ResultEncodingVec::from_ascii_bytes(ascii_bytes);
        encoded_result.expand()
    }
}
