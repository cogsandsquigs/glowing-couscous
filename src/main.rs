pub mod fuck;

use crate::fuck::obfusicate;

fn main() {
    println!("{}", obfusicate("console.log(\"hello world!\")"));
}
