use nom::bytes::complete::{tag, take, take_until};
use nom::character::complete::i32;
use nom::combinator::{map_res, opt};
use nom::multi::{many1, many_till};
use nom::sequence::preceded;
use nom::{Finish, IResult};
use std::path::Path;

macro_rules! ws {
    ($t:expr) => {
        nom::sequence::delimited(
            nom::character::complete::multispace0,
            $t,
            nom::character::complete::multispace0,
        )
    };
}

fn parse_fragment(input: &str) -> IResult<&str, ProgramFragment> {
    let (input, pop) = map_res(
        many_till(take(1_usize), preceded(tag("div z "), i32)),
        |(_, pop)| match pop {
            1 => std::result::Result::<bool, String>::Ok(false),
            26 => Ok(true),
            _ => Err(format!("Error during deserialization: input: {}", input)),
        },
    )(input)?;
    let (input, test_offset) = ws!(preceded(tag("add x "), i32))(input)?;
    let (input, _) = many_till(take(1_usize), tag("add y w"))(input)?;
    let (input, value_offset) = ws!(preceded(tag("add y "), i32))(input)?;
    let (input, _) = opt(ws!(take_until("inp w")))(input)?;
    Ok((
        input,
        ProgramFragment {
            does_pop: pop,
            write_offset: value_offset,
            test_offset,
        },
    ))
}

fn parse_fragments(input: &str) -> IResult<&str, Vec<ProgramFragment>> {
    many1(parse_fragment)(input)
}

fn main() {
    let path = if let Some(x) = std::env::args().nth(1) {
        x
    } else {
        "2021/day-24/input".to_string()
    };
    let path = Path::new(&path);
    let contents = std::fs::read_to_string(path).unwrap();
    let (max, min) = challenge(&contents);
    println!(
        "Highest legal model number: {}!\n\
    Lowest legal model number: {}!",
        max, min
    );
}

fn challenge(input: &str) -> (usize, usize) {
    let fragments = parse_fragments(input)
        .finish()
        .unwrap_or_else(|e: nom::error::Error<&str>| panic!("{}", e))
        .1;
    let mut number_max = [9; 14];
    let mut number_min = [1; 14];

    let indices: Vec<_> = fragments
        .iter()
        .enumerate()
        .filter_map(|(i, f)| if f.does_pop { Some(i) } else { None })
        .collect();

    for i in indices {
        let fragment = &fragments[i];
        let mut k = i - 1;
        let mut to_move = 1;
        let mut pair;
        loop {
            to_move -= 1;
            pair = &fragments[k];
            if pair.does_pop {
                to_move += 2;
            }
            if to_move == 0 {
                break;
            }
            k -= 1;
        }
        let i_min;
        let k_min;
        let i_max;
        let k_max;
        let goal = pair.write_offset + fragment.test_offset;
        if goal > 0 {
            i_max = 9;
            k_max = 9 - goal;
            k_min = 1;
            i_min = goal + 1;
        } else {
            k_max = 9;
            i_max = 9 + goal;
            k_min = goal.abs() + 1;
            i_min = 1;
        }
        number_min[i] = i_min as u8;
        number_min[k] = k_min as u8;
        number_max[i] = i_max as u8;
        number_max[k] = k_max as u8;
    }
    let result_min = number_min
        .into_iter()
        .fold(0_usize, |agg, new| agg * 10 + new as usize);
    let result_max = number_max
        .into_iter()
        .fold(0_usize, |agg, new| agg * 10 + new as usize);
    (result_max, result_min)
}

struct ProgramFragment {
    test_offset: i32,
    write_offset: i32,
    does_pop: bool,
}
