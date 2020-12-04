use std::collections::HashSet;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), &'static str> {
    let nums: HashSet<u32> = BufReader::new(std::io::stdin())
        .lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .collect();

    let pair = get_pair(&nums, 2020)?;
    let triple = get_triple(&nums, 2020)?;

    println!(
        "Pair: {:?}\nProduct: {}\n",
        pair,
        pair.iter().product::<u32>()
    );
    println!(
        "Triple: {:?}\nProduct: {}\n",
        triple,
        triple.iter().product::<u32>()
    );

    Ok(())
}

fn get_pair(nums: &HashSet<u32>, target: u32) -> Result<[u32; 2], &'static str> {
    for num in nums.iter() {
        if let Some(pair) = target.checked_sub(*num).and_then(|pair| nums.get(&pair)) {
            return Ok([*num, *pair]);
        }
    }
    Err("no such pair exists")
}

fn get_triple(nums: &HashSet<u32>, target: u32) -> Result<[u32; 3], &'static str> {
    for num in nums.iter() {
        if let Ok(matching_pair) = get_pair(nums, target - *num) {
            return Ok([*num, matching_pair[0], matching_pair[1]]);
        }
    }

    Err("no such triple exists")
}
