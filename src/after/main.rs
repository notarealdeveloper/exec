use std::env;
use std::io;
use std::path;
use std::fs;
use regex::Regex;

fn print_usage_and_exit() {
    let argv: Vec<String> = env::args().collect();
    let prog = path::Path::new(&argv[0]).file_name().unwrap();
    eprintln!("usage: {:?} <pattern> [file]", prog);
    std::process::exit(1);
}

fn process_lines<R: io::BufRead>(reader: R, re: &Regex) {
    let mut on = false;

    for line in reader.lines() {
        let s = line.unwrap();
        if on {
            println!("{}", s);
        }
        if re.is_match(&s) {
            on = true;
        }
    }
}

fn main() {
    let argv: Vec<String> = env::args().collect();

    let pattern = &argv[1];
    let re = Regex::new(pattern).unwrap();

    match argv.len() {

        1 => print_usage_and_exit(),

        2 => {
            let stdin = io::stdin();
            let reader = stdin.lock();
            process_lines(reader, &re);
        },

        3 => {
            let file = fs::File::open(&argv[2]).unwrap();
            let reader = io::BufReader::new(file);
            process_lines(reader, &re);
        },

        _ => print_usage_and_exit(),
    };
}
