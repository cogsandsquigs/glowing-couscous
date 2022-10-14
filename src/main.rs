pub mod fuck;

use self::fuck::obfusicate;
use clap::Parser;
use std::fs::File;
use std::io::prelude::*;

/// Simple program to greet a person
#[derive(Clone, Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The filename to parse the javascript from
    #[arg(short, long)]
    filename: String,

    /// The output file
    #[arg(short, long, default_value = "jsfuck.js")]
    output: String,
}

fn main() {
    let args = Args::parse();

    let mut file = File::open(args.filename.clone())
        .expect(format!("File `{}` not found", args.filename).as_str());

    let mut js = String::new();

    file.read_to_string(&mut &mut js)
        .expect("Error while reading file");

    let obfusicated = obfusicate(js.as_str());
    println!("{}", obfusicated);
}
