use std::collections::HashSet;
use std::io::{BufRead, BufReader};

fn main() {
    let nums: HashSet<u32> = BufReader::new(std::io::stdin())
        .lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .collect();

    for num in nums.iter() {
        if nums.get(&(2020 - num)).is_some() {
            println!("Pair: {} + {}", num, 2020 - num);
            println!("Product: {}", num * (2020 - num));
            return;
        }
    }
}
