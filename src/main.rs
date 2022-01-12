#![allow(unused)]

use std::fs::{create_dir_all, File};
use std::io;
use std::io::prelude::*;
use std::path::Path;

mod lib;
use lib::cli::*;
use lib::proc::*;

fn main() {
    let app = Cli::new();
    let args = app.parse();
    let input = Path::new(args.value_of("input").unwrap_or("STDIN"));

    let mut output = String::new();
    if input == Path::new("STDIN") {
        io::stdin().read_to_string(&mut output).unwrap();
        // println!("{}", output);
    } else {
        let mut file = File::open(input).unwrap();
        file.read_to_string(&mut output).unwrap();
        // println!("{}", output);
    }

    upload_to_pastebin(&output, &args);
}
