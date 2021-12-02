use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let path = Path::new(&path);
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let (depth, distance) = ex_02(reader.lines().filter_map(|wrapped| wrapped.ok()));

    println!(
        "depth: {} distance: {} product: {}",
        depth,
        distance,
        depth * distance
    );
}

fn ex_01<T: Iterator<Item = R>, R: AsRef<str>>(lines: T) -> (i32, i32) {
    let mut depth = 0_i32;
    let mut distance = 0_i32;
    for line in lines {
        let line = line.as_ref();
        let mut line_iter = line.split_whitespace();
        let command = line_iter.next().unwrap();
        let value = i32::from_str(line_iter.next().unwrap()).unwrap();

        match command {
            "forward" => distance += value,
            "down" => depth += value,
            "up" => depth -= value,
            _ => panic!("unknown command"),
        };
    }
    (depth, distance)
}

fn ex_02<T: Iterator<Item = R>, R: AsRef<str>>(lines: T) -> (i32, i32) {
    let mut aim = 0;
    let mut depth = 0_i32;
    let mut distance = 0_i32;
    for line in lines {
        let line = line.as_ref();
        let mut line_iter = line.split_whitespace();
        let command = line_iter.next().unwrap();
        let value = i32::from_str(line_iter.next().unwrap()).unwrap();

        match command {
            "forward" => {
                distance += value;
                depth += value * aim;
            }
            "down" => aim += value,
            "up" => aim -= value,
            _ => panic!("unknown command"),
        };
    }
    (depth, distance)
}
