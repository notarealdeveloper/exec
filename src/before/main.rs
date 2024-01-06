use std::env;
use std::io;
use std::path;
use regex::Regex;

fn main() {
    let argv: Vec<String> = env::args().collect();

    if argv.len() < 2 {
        let prog = path::Path::new(&argv[0]).file_name().unwrap();
        eprintln!("usage: {:?} <pattern>", prog);
        std::process::exit(1);
    }

    let mut on = true;
    let re = Regex::new(&argv[1]).unwrap();

    for line in io::stdin().lines() {
        let s = line.unwrap();
        if re.is_match(&s) {
            on = false;
        }
        if on {
            println!("{}", s);
        }
    }
}
