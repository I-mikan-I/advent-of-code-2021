use std::fs::read_to_string;
use std::path::Path;
use std::str::FromStr;

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let path = Path::new(&path);
    let contents = read_to_string(path).unwrap();
    let numbers: Vec<_> = contents
        .split_whitespace()
        .map(|num| i32::from_str(num).unwrap())
        .collect();
    let result = ex_02(numbers);
    println!("Result: {}", result);
}

fn ex_02(numbers: Vec<i32>) -> usize {
    let mut sum = 0;
    let mut window = &numbers[0..3];
    for n in 3..numbers.len() {
        let last: i32 = window.iter().sum();
        window = &numbers[n - 2..n + 1];
        let next = window.iter().sum();
        if last < next {
            sum += 1;
        }
    }
    sum
}

fn ex_01(numbers: Vec<i32>) -> usize {
    let mut iter = numbers.into_iter();
    let mut last = iter.next().unwrap();
    iter.fold(0, |agg, num| {
        if num > last {
            last = num;
            agg + 1
        } else {
            last = num;
            agg
        }
    })
}
