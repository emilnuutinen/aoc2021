use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").expect("File not found");
    let mut prev_val = 1000;
    let mut result = 0;

    for line in data.lines() {
        let val: u16 = line.parse::<u16>().unwrap();

        if prev_val < val {
            result += 1;
        }
        prev_val = val;
    }

    println!("{}", result);
}
