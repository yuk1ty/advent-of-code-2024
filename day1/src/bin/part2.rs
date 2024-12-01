use std::fs;

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("day1/input").unwrap();
    let (left, right): (Vec<u64>, Vec<u64>) = input
        .lines()
        .filter_map(|x| x.split_whitespace().next_tuple())
        .map(|(l, r)| (l.parse::<u64>().unwrap(), r.parse::<u64>().unwrap()))
        .unzip();
    let similarity_score = left
        .into_iter()
        .map(|l| right.iter().filter(|r| l == **r).count())
        .sum::<usize>();
    println!("{similarity_score}");
}
