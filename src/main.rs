use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

mod lib;
use lib::cli::*;
use lib::proc::*;

fn main() {
    let app = Cli::new();
    let args = app.parse();
    let input = args.value_of("input").unwrap_or("-");

    let mut output = String::new();
    if input == "-" {
        io::stdin().read_to_string(&mut output).unwrap();
    } else {
        if Path::new(input).is_file() {
            let mut file = File::open(input).unwrap();
            file.read_to_string(&mut output).unwrap();
        } else {
            output = input.to_string();
        }
    }

    let copybin_url = upload_to_pastebin(&output, &args);
    match copybin_url {
        Ok(url) => println!("{}", url),
        _ => println!("Error uploading to pastebin"),
    }
}
