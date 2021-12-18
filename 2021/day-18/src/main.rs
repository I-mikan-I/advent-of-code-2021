use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::ops::{Add, Deref};
use std::path::Path;
use std::str::FromStr;

use nom::character::complete::{char, digit1};
use nom::combinator::map_res;
use nom::sequence::{delimited, separated_pair};
use nom::{Finish, IResult};

fn main() {
    let path = if let Some(x) = std::env::args().nth(1) {
        x
    } else {
        "2021/day-18/input-test".to_string()
    };
    let path = Path::new(&path);
    let contents = std::fs::read_to_string(path).unwrap();
    let numbers: Vec<_> = contents
        .lines()
        .map(|s| {
            parse_number(s)
                .finish()
                .unwrap_or_else(|e: nom::error::Error<&str>| panic!("{:?}", e))
                .1
        })
        .collect();
    let res_01 = ex_01(&numbers);
    let res_02 = ex_02(&numbers);
    println!(
        "Solution to part 1: {}!\n\
    Solution to part 2: {}!",
        res_01, res_02
    );
}

#[derive(Debug, Clone)]
enum Number {
    Regular(i32),
    Pair {
        left: Box<Number>,
        right: Box<Number>,
    },
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Regular(val) => write!(f, "{}", val),
            Number::Pair { left, right } => write!(f, "[{},{}]", left, right),
        }
    }
}

impl Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        Number::Pair {
            left: Box::new(self),
            right: Box::new(rhs),
        }
    }
}

fn parse_regular(input: &str) -> IResult<&str, Number> {
    map_res::<_, _, _, _, ParseIntError, _, _>(digit1, |res| {
        Ok(Number::Regular(i32::from_str(res)?))
    })(input)
}

fn parse_pair(input: &str) -> IResult<&str, Number> {
    delimited(
        char('['),
        separated_pair(parse_number, char(','), parse_number),
        char(']'),
    )(input)
    .map(|(input, (left, right))| {
        (
            input,
            Number::Pair {
                left: Box::new(left),
                right: Box::new(right),
            },
        )
    })
}

fn parse_number(input: &str) -> IResult<&str, Number> {
    if let Ok(inner) = parse_regular(input) {
        Ok(inner)
    } else {
        parse_pair(input)
    }
}

fn ex_01(input: &[Number]) -> usize {
    let result = input
        .iter()
        .cloned()
        .reduce(|last, mut next| {
            reduce_split(&mut next);
            let mut new = last + next;
            reduce_split(&mut new);
            new
        })
        .unwrap();
    magnitude(&result)
}

fn ex_02(input: &[Number]) -> usize {
    input
        .iter()
        .cloned()
        .permutations(2)
        .map(|mut vec| {
            let mut res = vec.remove(0) + vec.remove(0);
            reduce_split(&mut res);
            res
        })
        .map(|number| magnitude(&number))
        .max()
        .unwrap()
}

fn magnitude(number: &Number) -> usize {
    match number {
        Number::Regular(val) => *val as usize,
        Number::Pair { left, right } => 3 * magnitude(left) + 2 * magnitude(right),
    }
}

fn reduce_split(number: &mut Number) {
    let mut reduced = false;
    let mut did_split = true;
    while reduced || did_split {
        reduced = reduce(number, 0).1;
        while reduced {
            reduced = reduce(number, 0).1;
        }
        did_split = split(number);
    }
}

fn split(number: &mut Number) -> bool {
    match number {
        Number::Regular(val) if *val >= 10 => {
            *number = Number::Pair {
                left: Box::new(Number::Regular(*val / 2)),
                right: Box::new(Number::Regular(*val / 2 + *val % 2)),
            };
            true
        }
        Number::Pair { left, right } => {
            if split(left) {
                true
            } else {
                split(right)
            }
        }
        _ => false,
    }
}

fn reduce(number: &mut Number, depth: usize) -> ((Option<i32>, Option<i32>), bool) {
    if depth == 4 {
        let left_p;
        let right_p;
        match number {
            Number::Pair { left, right } => match [(*left).deref(), (*right).deref()] {
                [Number::Regular(vall), Number::Regular(valr)] => {
                    left_p = Some(*vall);
                    right_p = Some(*valr);
                }
                _ => panic!("reached depth 4 but further pairs found."),
            },
            Number::Regular(_) => return ((None, None), false),
        }
        *number = Number::Regular(0);
        ((left_p, right_p), true)
    } else {
        match number {
            Number::Pair { left, right } => {
                let res_left = reduce(left, depth + 1);
                if let ((None, None), flag) = res_left {
                    if flag {
                        res_left
                    } else {
                        let res_right = reduce(right, depth + 1);
                        if let ((None, None), _) = res_right {
                            res_right
                        } else {
                            if let Some(left_val) = res_right.0 .0 {
                                update_right(left, left_val)
                            }
                            ((None, res_right.0 .1), res_right.1)
                        }
                    }
                } else {
                    if let Some(right_val) = res_left.0 .1 {
                        update_left(right, right_val)
                    }
                    ((res_left.0 .0, None), res_left.1)
                }
            }
            _ => ((None, None), false),
        }
    }
}

fn update_right(number: &mut Number, with: i32) {
    match number {
        Number::Pair { right, .. } => update_right(right, with),
        Number::Regular(val) => *val += with,
    }
}

fn update_left(number: &mut Number, with: i32) {
    match number {
        Number::Pair { left, .. } => update_left(left, with),
        Number::Regular(val) => *val += with,
    }
}
