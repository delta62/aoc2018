use std::env;
use std::fs::File;
use std::io::Read;

mod d01;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        panic!("Usage: $0 prog infile");
    }

    let prog = args[1].as_str();
    let mut input_file = File::open(&args[2]).unwrap();
    let mut input_bytes: Vec<u8> = Vec::new();
    input_file.read_to_end(&mut input_bytes).unwrap();
    let input = String::from_utf8(input_bytes).unwrap();

    match prog {
        "d01_1" => {
            let result = d01::freq(input.as_str());
            println!("{}", result);
        },
        "d01_2" => {
            let result = d01::dup(input.as_str());
            println!("{}", result);
        },
        _ => panic!("Unknown program"),
    }
}
