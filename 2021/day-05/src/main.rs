use std::collections::HashMap;
use std::path::Path;
use std::str::FromStr;

fn main() {
    let path = if let Some(x) = std::env::args().nth(1) {
        x
    } else {
        "2021/day-05/input-test".to_string()
    };
    let path = Path::new(&path);
    let contents = std::fs::read_to_string(path).unwrap();
    ex_01(&contents);
    ex_02(&contents);
}

fn ex_01(input: &str) {
    let result = map_vents(input, |_, _, _| (Box::new(0..0), Box::new(0..0)));
    println!("Ignoring diagonals, there are {} dangerous areas!", result)
}

fn ex_02(input: &str) {
    let result = map_vents(input, diagonal_generator);
    println!("Ignoring diagonals, there are {} dangerous areas!", result)
}
type CoordinateGenerator =
    fn([i32; 2], [i32; 2], bool) -> (Box<dyn Iterator<Item = i32>>, Box<dyn Iterator<Item = i32>>);
fn map_vents(input: &str, diagonal_generator: CoordinateGenerator) -> usize {
    let mut counts: HashMap<(i32, i32), i32> = HashMap::new();
    let endpoints: Vec<_> = input
        .lines()
        .map(|line| {
            line.split("->")
                .map(str::trim)
                .map(|tuple| {
                    match &tuple
                        .split(',')
                        .map(|num| i32::from_str(num).unwrap())
                        .collect::<Vec<_>>()[..]
                    {
                        [left, right] => (*left, *right),
                        _ => panic!("no match for tuple"),
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();

    for endpoint in endpoints {
        let (start, end) = match endpoint[..] {
            [start, end] => (start, end),
            _ => continue,
        };
        let x_values = [start.0, end.0];
        let y_values = [start.1, end.1];
        let (iter, other_iter): (Box<dyn Iterator<Item = i32>>, Box<dyn Iterator<Item = i32>>) =
            if start.0 == end.0 {
                (
                    Box::new(std::iter::repeat(start.0)),
                    Box::new(
                        y_values.into_iter().min().unwrap()..=y_values.into_iter().max().unwrap(),
                    ),
                )
            } else if start.1 == end.1 {
                (
                    Box::new(
                        x_values.into_iter().min().unwrap()..=x_values.into_iter().max().unwrap(),
                    ),
                    Box::new(std::iter::repeat(start.1)),
                )
            } else {
                diagonal_generator(
                    x_values,
                    y_values,
                    (start.0 - end.0) * (start.1 - end.1) < 0,
                )
            };

        for field in iter.zip(other_iter) {
            let old_count = *counts.get(&field).unwrap_or(&0);
            counts.insert(field, old_count + 1);
        }
    }
    counts.values().filter(|&&n| n >= 2).count()
}

fn diagonal_generator(
    x_values: [i32; 2],
    y_values: [i32; 2],
    reverse: bool,
) -> (Box<dyn Iterator<Item = i32>>, Box<dyn Iterator<Item = i32>>) {
    let iter = x_values.into_iter().min().unwrap()..=x_values.into_iter().max().unwrap();
    let mut _other_iter_tmp =
        y_values.into_iter().min().unwrap()..=y_values.into_iter().max().unwrap();
    let other_iter: Box<dyn Iterator<Item = i32>> = if reverse {
        Box::new(_other_iter_tmp.rev())
    } else {
        Box::new(_other_iter_tmp)
    };
    (Box::new(iter), other_iter)
}
