use std::cmp;
use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").expect("File not found");
    let mut prev_val = 1000;
    let mut result_a = 0;

    let numbers: Vec<u16> = data
        .lines()
        .map(|line| line.parse::<u16>().unwrap())
        .collect();

    for line in data.lines() {
        let val: u16 = line.parse::<u16>().unwrap();

        if prev_val < val {
            result_a += 1;
        }
        prev_val = val;
    }

    println!("{}", result_a);

    let mut prev_sum = 1000;
    let mut result_b = 0;

    for index in 0..numbers.len() {
        let ref slice = &numbers[index..cmp::min(index + 3, numbers.len())];
        let sum: u16 = slice.iter().sum();

        if prev_sum < sum {
            result_b += 1;
        }
        prev_sum = sum;
    }
    println!("{}", result_b);
}
