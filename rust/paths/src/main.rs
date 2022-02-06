use std::env::args;

fn main() {
    let mut args = args();
    let w: usize = args.nth(1).unwrap().parse().unwrap();
    let h: usize = args.next().unwrap().parse().unwrap();
    let answer = sum_paths(w, h);

    println!("There are {answer} possible paths for you grid of {w} x {h} cells.")
}

fn sum_paths(w: usize, h: usize) -> usize {
    if w == 1 || h == 1 {
        return 1;
    }
    sum_paths(w, h - 1) + sum_paths(w - 1, h)
}
