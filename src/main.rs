use std::env;

use compression::CompressionParams;

fn main() {
    let mut compression_params = CompressionParams::new();

    let args: Vec<String> = env::args().collect();
    for i in (1..args.len()-1).step_by(2) {
        let alias = &args[i];
        let value = &args[i+1].parse().expect(&compression_params.give_help_message());
        compression_params.update(alias, *value); 
    }



}
