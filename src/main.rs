use std::{collections::HashMap, env, hash::Hash};
use compression::lz77::implementation::LZ77;

#[macro_use]
extern crate fstrings;

#[derive(Hash, PartialEq, Eq)]
enum Params {
    WindowSize,
    BlockSize,
}


struct CompressionParams {
    command_line_aliases: HashMap<String, Params>,
    params: HashMap<Params, usize>,
}

impl CompressionParams {
    fn new() -> Self {
        let mut default_params: HashMap<Params, usize> = HashMap::new();
        default_params.insert(Params::WindowSize, 5);
        default_params.insert(Params::BlockSize, 3);

        let mut aliases: HashMap<String, Params> = HashMap::new();
        aliases.insert("-window_size".to_string(), Params::WindowSize);
        aliases.insert("-blocks_num".to_string(), Params::BlockSize);
        Self {
            command_line_aliases: aliases,
            params: default_params,
        }
    }

    fn update(&mut self, alias: &String, value: usize) {
        let param = &self.command_line_aliases.get(alias).expect(&self.give_help_message());
        if let Some(old_val) = self.params.get_mut(param) {
            *old_val = value;
        }
    }

    #[allow(dead_code)]
    fn get_param(&mut self, param: &Params) -> &usize {
        self.params.get(param).unwrap()
    }

    fn give_help_message(&self) -> String {
        let info = "INFO:";
        let sep = "-----------------------";
        let possible_options = "xd";
        let t1 = "Usage is: cargo run -- [-options value]";
        let t2 = &f!("Possible 'options' are {possible_options} and 'value' should be an integer");

        [info, sep, t1, t2, sep].join("\n")
    }
}

fn main() {
    let mut compression_params = CompressionParams::new();

    let args: Vec<String> = env::args().collect();
    for i in (1..args.len()-1).step_by(2) {
        let alias = &args[i];
        let value = &args[i+1].parse().expect(&compression_params.give_help_message());
        compression_params.update(alias, *value); 
    }

    let mut z = LZ77 {
        x: 5,
    };

    println!("{}", z.give_num());

}
