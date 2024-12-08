use std::{cmp::max, collections::HashMap};

use super::hashes::{HashTable, HashPair};

const DEFAULT_WINDOW_SIZE: usize = 32768;
const DEFAULT_MAX_LEN_TO_REDUCE: usize = 181; 
const DEFAULT_LEN_TRESHOLD: usize = 6;

#[derive(Clone)]
enum ResultEncoding {
    Ascii(char),
    Reference(usize, usize)
}

impl ResultEncoding {
    pub fn to_string(&self) -> String {
        match self {
            ResultEncoding::Ascii(c) => c.to_string(),
            ResultEncoding::Reference(d, l) => {
                let sep = ResultEncoding::get_separator().to_string();
                [sep.clone(), l.to_string(), sep.clone(), d.to_string(), sep].concat()
            } 
        }     
    }

    fn get_separator() -> char {
        let sep_num: u32 = 129;
        char::from_u32(sep_num).expect("There is no char corresponding to given number!")
    }

    pub fn len_treshold() -> usize {
        DEFAULT_LEN_TRESHOLD
    }
}

struct ResultEncodingVec {
    vec: Vec<ResultEncoding> 
}

impl ResultEncodingVec {
    pub fn new() -> Self {
        ResultEncodingVec{vec: Vec::new()}
    }

    pub fn push(&mut self, elem: ResultEncoding) -> () {
        self.vec.push(elem); 
    }

    pub fn from_string(str: &str) -> Self {
        let (mut i, mut res, sep) = (0, ResultEncodingVec::new(), ResultEncoding::get_separator());
        while i < str.len() {
            let c = char::from(str.as_bytes()[i]);
            if c == sep {
                let (enc_reference, processed_size) = ResultEncodingVec::parse_reference(str, i);
                res.push(enc_reference);
                i += processed_size;
            } else {
                res.push(ResultEncoding::Ascii(c));
            }
            i += 1;
        }

        res 
    }

    fn parse_reference(str: &str, start_index: usize) -> (ResultEncoding, usize) {
        let sep = ResultEncoding::get_separator();
        let mut sep_counter = 0; 
        let mut i = start_index;
        let (mut dist, mut len) = (String::new(), String::new());
        loop {
            let c = char::from(str.as_bytes()[i]);
            if c == sep {
                sep_counter += 1;
            } 
            match sep_counter {
                1 => dist.push(c),
                2 => len.push(c),
                3 => break,
                _ => {},
            };
            i += 1;
        }

        let reference = ResultEncoding::Reference(dist.parse().unwrap(), len.parse().unwrap());
        (reference, i - start_index)
    }


    pub fn to_string(&self) -> String {
        self.vec.clone().into_iter().map(|x| {x.to_string()}).collect()
    }
}

struct SlidingWindow<'a> {
    text: &'a String,
    max_len_to_reduce: usize,
    window_size: usize,
    hashes: HashTable,
    subwords: Vec<HashMap<HashPair, usize>>,
    partial_result: Vec<(usize, usize)>
}

impl<'a> SlidingWindow<'a> {
    pub fn new(text: &'a String, window_size: usize, max_len_to_reduce: Option<usize>) -> Self {
        let max_len = max_len_to_reduce.unwrap_or(SlidingWindow::sqrt_usize(window_size));
        SlidingWindow {
            max_len_to_reduce: max_len, 
            window_size: window_size,
            text: text,
            hashes: HashTable::new(text),
            subwords: Vec::from_iter((0..max_len).into_iter().map(|_| {HashMap::new()})),
            partial_result: Vec::new() 
        }
    }

    fn sqrt_usize(n: usize) -> usize {
        (n as f64).sqrt() as usize
    }

    pub fn run(&mut self) -> () {
        for i in 0..self.text.len() {
            self.remove_old_subwords(i);
            let mut new_subwords_descending= self.extract_new_subwords(i);
            self.update_result(&new_subwords_descending, i);
            new_subwords_descending.reverse();
            let new_subwords_aescending = new_subwords_descending; 
            self.add_new_subwords(new_subwords_aescending);
        }
    }

    fn extract_new_subwords(&self, index: usize) -> Vec<(HashPair, usize)> {
        Vec::from_iter(
            (max(0, index-self.max_len_to_reduce)..index)
            .into_iter()
            .map(|i| {
                let h = self.hashes.get_hash(i, index);
                (h, i)
            })
        )
    }
    
    fn update_result(&mut self, new_subwords_desc: &Vec<(HashPair, usize)>, index: usize) -> () {
        let all_sub_and_new_sub  = self.subwords.clone().into_iter().zip(new_subwords_desc.into_iter());
        for (length, (bucket, subword)) in all_sub_and_new_sub.enumerate() {
            if let Some(same_word_start_index) = bucket.get(&subword.0) {
                let distance = index - same_word_start_index;
                self.partial_result.push((distance, length));
            } else {
                self.partial_result.push((0, 1));
            }
        }
    }

    fn add_new_subwords(&mut self, new_subwords: Vec<(HashPair, usize)>) -> () {
        for (mut bucket, subword) in self.subwords.clone().into_iter().zip(new_subwords.into_iter()) {
            if !bucket.contains_key(&subword.0) {
                bucket.insert(subword.0, subword.1);
            }
        }
    }

    fn remove_old_subwords(&mut self, curr_index: usize) -> () {
        let first_char_index = curr_index - self.window_size; 
        (1..self.max_len_to_reduce)
        .map(|i| {self.hashes.get_hash(first_char_index, first_char_index+i)})
        .zip(self.subwords.clone())
        .map(|(h, mut subwords_set)| {
            subwords_set.remove(&h);
        });
    }

    pub fn get_result(&self) -> String {
        let mut result = ResultEncodingVec::new();
        let mut i: usize = self.partial_result.len();
        while i >= 0 {
            let (dist, len) = self.partial_result[i];
            if len >= ResultEncoding::len_treshold() {
                result.push(ResultEncoding::Reference(dist, len));
            } else {
                result.push(ResultEncoding::Ascii(char::from(self.text.as_bytes()[i])));
            }
            i -= len;
        }

        result.to_string()
    }

}

pub struct LZ77Compressor {
    window_size: usize,
    max_len_to_reduce: Option<usize> 
}

impl LZ77Compressor {
    pub fn new(window_size: Option<usize>, max_len_to_reduce: Option<usize>) -> Self {
        LZ77Compressor {
            window_size: window_size.unwrap_or(DEFAULT_WINDOW_SIZE),
            max_len_to_reduce: max_len_to_reduce
        }
    }

    pub fn compress(&self, str: &String) -> String {
        let mut sw = SlidingWindow::new(str, self.window_size, None);
        sw.run();
        sw.get_result()
    }

    pub fn decompress(&self, str: &String) -> String {
        let encoded_result = ResultEncodingVec::from_string(str);
        encoded_result.to_string()
    } 
}