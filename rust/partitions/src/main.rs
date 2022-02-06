use std::env::args;

fn main() {
    let mut args = args();

    let individuals: usize = args.nth(1).unwrap().parse().unwrap();
    let max_group_size: usize = args.next().unwrap().parse().unwrap();

    // let individuals = 7;
    // let max_group_size = 4;

    let answer = new_new_partition(individuals, max_group_size);

    println!(
        "With {individuals} individuals and a maximum group size of {max_group_size} there {}",
        if answer == 1 {
            format!("is {} unique partition", answer)
        } else {
            format!("are {} unique partitions.", answer)
        }
    );
}

// fn partition(individuals: usize, max_group_size: usize) -> usize {
//     if individuals == 1 {
//         return 1;
//     } else if max_group_size == 1 {
//         return 1;
//     } else if individuals > max_group_size {
//         return partition(individuals - max_group_size, max_group_size)
//             + partition(max_group_size, max_group_size);
//     }
//     partition(individuals - 1, max_group_size) + 1
// }

// fn new_partition(individuals: usize, max_group_size: usize) -> usize {
//     let mut partitions = 0;
//     if individuals == 1 || max_group_size == 1 {
//         return 1;
//     } else if individuals > max_group_size {
//         partitions += new_partition(individuals - max_group_size, max_group_size);
//     }
//     new_partition(individuals - 1, max_group_size) + partitions
// }

fn new_new_partition(individuals: usize, max_group_size: usize) -> usize {
    let mut partitions = 0;
    if individuals == 0 {
        return 1;
    }
    if max_group_size == 0 {
        return 0;
    }
    if individuals >= max_group_size {
        partitions += new_new_partition(individuals - max_group_size, max_group_size);
    }

    new_new_partition(individuals, max_group_size - 1) + partitions
}
