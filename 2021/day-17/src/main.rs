use nom::bytes::complete::take;
use nom::character::complete::i32;
use nom::multi::many_till;
use nom::{Finish, IResult};
use std::cmp::{max, min};
use std::ops::Neg;
use std::path::Path;

fn parse_range(input: &str) -> IResult<&str, (i32, i32)> {
    let (input, (_, begin)) = many_till(take(1_usize), i32)(input)?;
    let (input, (_, end)) = many_till(take(1_usize), i32)(input)?;
    Ok((input, (begin, end)))
}

fn main() {
    let path = if let Some(x) = std::env::args().nth(1) {
        x
    } else {
        "2021/day-17/input".to_string()
    };
    let path = Path::new(&path);
    let contents = std::fs::read_to_string(path).unwrap();
    let (input, x_range) = parse_range(&contents)
        .finish()
        .unwrap_or_else(|e: nom::error::Error<&str>| panic!("{}", e));
    let (_, y_range) = parse_range(input)
        .finish()
        .unwrap_or_else(|e: nom::error::Error<&str>| panic!("{}", e));

    let res_01 = ex_01(y_range.0);
    let res_02 = ex_02(x_range, y_range);
    println!(
        "Solution to challenge 1: {}!\n\
    Solution to challenge 2: {}!",
        res_01, res_02
    );
}

fn ex_01(y_min: i32) -> i32 {
    let launch_velocity = y_min.abs() - 1;
    launch_velocity * (launch_velocity + 1) / 2
}

fn ex_02(x_range: (i32, i32), y_range: (i32, i32)) -> usize {
    let x_tries = 0..=x_range.1;
    let y_tries = y_range.0..(y_range.0.neg());
    let mut possible_values: usize = 0;
    let mut found_x = Vec::new();

    for x_ in x_tries {
        let mut sum = 0;
        for x in 0..=x_ {
            sum += x;
            if sum >= x_range.0 && sum <= x_range.1 {
                found_x.push(x_);
                break;
            }
        }
    }
    for mut y in y_tries {
        let mut sum = 0;
        let mut steps = 0;
        let mut found_x = found_x.clone();
        loop {
            steps += 1;
            sum += y;
            if sum >= y_range.0 && sum <= y_range.1 {
                let possible_values_tmp: Vec<_> = found_x
                    .iter()
                    .map(|x| {
                        if steps == 1 {
                            (*x, *x)
                        } else {
                            (*x, min(steps, *x) * (max(x - steps + 1, 1) + x) / 2)
                        }
                    })
                    .filter(|(_, res)| res >= &x_range.0 && res <= &x_range.1)
                    .map(|(x, _)| x)
                    .collect();
                found_x = found_x
                    .into_iter()
                    .filter(|x| !possible_values_tmp.contains(x))
                    .collect();
                possible_values += possible_values_tmp.len();
            }
            if sum <= y_range.0 {
                break;
            }
            y -= 1;
        }
    }

    possible_values
}
