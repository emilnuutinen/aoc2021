use std::fs;

enum Direction {
    Forward,
    Down,
    Up,
    None,
}

struct Command {
    direction: Direction,
    value: i32,
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("File not found");
    let mut hor: i32 = 0;
    let mut ver: i32 = 0;
    let mut line: i32 = 0;

    let commands: Vec<Command> = data
        .lines()
        .map(|line| {
            let line_values: Vec<&str> = line.split_whitespace().collect();
            let direction = match line_values[0] {
                "forward" => Direction::Forward,
                "down" => Direction::Down,
                "up" => Direction::Up,
                _ => Direction::None,
            };
            let value = line_values[1].parse::<i32>().unwrap();

            Command { direction, value }
        })
        .collect();

    for command in commands {
        line += 1;
        match command.direction {
            Direction::Forward => hor += command.value,
            Direction::Down => ver += command.value,
            Direction::Up => ver -= command.value,
            Direction::None => println!("Invalid command on line {}", line),
        }
    }

    println!("{}", ver * hor);
}
