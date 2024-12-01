use std::fs;

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("day1/input").unwrap();
    let (left, right): (Vec<u64>, Vec<u64>) = input
        .lines()
        .filter_map(|x| x.split_whitespace().next_tuple())
        .map(|(l, r)| (l.parse::<u64>().unwrap(), r.parse::<u64>().unwrap()))
        .unzip();
    let total_distance = (left.into_iter().sorted())
        .zip(right.into_iter().sorted())
        .map(|(l, r)| l.abs_diff(r))
        .sum::<u64>();
    println!("{total_distance}");
}
