use::std::collections::HashMap;
use std::u8;
use std::cmp::max;
pub struct WeightsCalculator {

}

impl WeightsCalculator {
    pub fn default() -> HashMap<u8, u8> {
        // temporary solution
        let mut mapping = HashMap::new();
        mapping.insert('a' as u8, 0);
        mapping.insert('b' as u8, 1); 
        mapping.insert('c' as u8, 2);
        mapping.insert('~' as u8, 3);

        mapping
    }

    pub fn fitted_to_text(bytes: &Vec<u8>) -> HashMap<u8, u8> {
        let mut symbol_counter: HashMap<&u8, usize> = HashMap::new(); 
        bytes.into_iter().for_each(|s| {
            match symbol_counter.get(s) {
                Some(count) => { symbol_counter.insert(s, count + 1); },
                None => { symbol_counter.insert(s, 1); }
            }
        });

        let count_max = symbol_counter.values().max().unwrap(); 
        let scaler = count_max / u8::MAX as usize + 1;

        symbol_counter.into_iter().map(|(symbol, occurences)| {
            (*symbol, max((occurences / scaler) as u8, 1))
        })
        .collect()
    }
}
       