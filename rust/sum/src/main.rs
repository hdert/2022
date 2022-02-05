use std::env::{args, Args};

fn main() {
    let mut args: Args = args();
    let num: u64 = args.nth(1).unwrap().parse().unwrap();
    let answer = sum(num);

    println!("The sum of all positive numbers up to, and including {num} is {answer}.")
}

fn sum(num: u64) -> u64 {
    if num == 0 {
        return 0;
    }
    sum(num - 1) + num
}
