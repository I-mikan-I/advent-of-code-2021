use std::path::Path;
use std::str::FromStr;

fn main() {
    let path = if let Some(x) = std::env::args().nth(1) {
        x
    } else {
        "2021/day-07/input-test".to_string()
    };
    let path = Path::new(&path);
    let contents = std::fs::read_to_string(path).unwrap();
    let numbers: Vec<_> = contents
        .split(',')
        .map(str::trim)
        .map(|n| usize::from_str(n).unwrap())
        .collect();

    let ex_01 = find_minimal_fuel_path(&numbers, |x1, x2| (x1 as i64 - x2 as i64).abs() as usize);
    let ex_02 = find_minimal_fuel_path(&numbers, gauss);

    println!(
        "Solution to challenge 1: {}\n\
              Solution to challenge 2: {}",
        ex_01, ex_02
    );
}

fn find_minimal_fuel_path(positions: &[usize], consumption_fn: fn(usize, usize) -> usize) -> usize {
    let mut current_fuel = usize::MAX;
    let upper_bound = *positions.iter().max().unwrap();
    for x in 0..=upper_bound {
        let next_fuel = positions
            .iter()
            .fold(0_usize, |agg, &num| agg + consumption_fn(x, num));
        if next_fuel > current_fuel {
            break;
        }
        current_fuel = next_fuel;
    }
    current_fuel
}

fn gauss(position: usize, goal: usize) -> usize {
    let diff = (position as i64 - goal as i64).abs() as usize;
    (diff * (diff + 1)) / 2
}
