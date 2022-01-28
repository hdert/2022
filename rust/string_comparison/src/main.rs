use std::env::{args, Args};

fn main() {
    let mut args: Args = args();
    let first_string: String = args.nth(1).unwrap();
    let second_string: String = args.next().unwrap();

    if first_string.to_lowercase() == second_string.to_lowercase() {
        println!("Strings are equal");
    } else {
        println!("Strings are unequal");
    }
}
