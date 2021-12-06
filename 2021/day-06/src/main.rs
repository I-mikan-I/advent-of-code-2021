use std::path::Path;
use std::str::FromStr;

fn main() {
    let path = if let Some(x) = std::env::args().nth(1) {
        x
    } else {
        "2021/day-06/input-test".to_string()
    };
    let path = Path::new(&path);
    let contents = std::fs::read_to_string(path).unwrap();
    let numbers: Vec<_> = contents
        .split(',')
        .into_iter()
        .map(str::trim)
        .map(|n| u8::from_str(n).unwrap())
        .collect();

    println!(
        "Number of lanternfish after 80 days: {}.\n\
        Number of lanternfish after 256 days: {}.",
        fish_simulation(&numbers, 80),
        fish_simulation(&numbers, 256)
    );
}

fn fish_simulation(contents: &[u8], days: usize) -> usize {
    let mut current_state = [0_usize; 9];
    let mut next_state = current_state.clone();

    for i in contents {
        current_state[*i as usize] += 1;
    }
    for _ in 0..days {
        for k in 0..current_state.len() {
            let next_counter = k as i32 - 1;
            if next_counter < 0 {
                next_state[next_state.len() - 1] += current_state[k];
                next_state[next_state.len() - 3] += current_state[k];
            } else {
                next_state[next_counter as usize] += current_state[k];
            }
        }
        current_state = next_state;
        next_state = [0_usize; 9];
    }
    return current_state.into_iter().sum();
}
