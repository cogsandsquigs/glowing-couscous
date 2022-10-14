pub mod fuck;

use self::fuck::obfusicate;
use clap::Parser;
use std::fs::File;
use std::io::prelude::*;

/// Translates a regular javascript program into a JSFuck program,
/// a valid (!) program that only uses the characters +, !, [, ],
/// (, and ).
#[derive(Clone, Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The filename to parse the javascript from
    filename: String,

    /// The output file
    #[arg(short, long, default_value = "fuck.js")]
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

    let mut output = File::create(args.output.clone())
        .expect(format!("Error while creating file `{}`", args.output).as_str());

    output
        .write(obfusicated.as_bytes())
        .expect(format!("Error while writing to file `{}`", args.output).as_str());
}
