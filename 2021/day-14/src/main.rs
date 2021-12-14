use nom::bytes::complete::{tag_no_case, take_while};
use nom::character::complete::{alpha1, multispace0};
use nom::character::is_alphabetic;
use nom::combinator::map_res;
use nom::error::make_error;
use nom::error::ErrorKind::Char;
use nom::sequence::terminated;
use nom::{bytes::complete, combinator, sequence, IResult};
use std::collections::VecDeque;
use std::fmt::{Debug, Display, Formatter};
use std::path::Path;
use nom::multi::many1;
use itertools::Itertools;

fn main() {
    let path = if let Some(x) = std::env::args().nth(1) {
        x
    } else {
        "2021/day-14/input-test".to_string()
    };
    let path = Path::new(&path);
    let contents = std::fs::read_to_string(path).unwrap();
    ex_01(&contents);
}

type Rule = ([char; 2], char);

struct MyErr;

impl Debug for MyErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyErr")
    }
}

impl Display for MyErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyErr")
    }
}

impl std::error::Error for MyErr {}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    map_res::<_, _, _, _, MyErr, _, _>(
        sequence::tuple((
            terminated(alpha1, multispace0),
            terminated(tag_no_case("->"), multispace0),
            alpha1,
        )),
        |(begin, _, end): (&str, &str, &str)| {
            Ok((
                begin
                    .chars()
                    .collect::<Vec<_>>()
                    .try_into()
                    .map_err(|_| MyErr)?,
                end.chars().next().ok_or(MyErr)?,
            ))
        },
    )(input)
}

fn parse_rules(input: &str) -> IResult<&str, Vec<Rule>> {
    many1(terminated(parse_rule, multispace0))(input)
}

fn ex_01(input: &str) {
    // let polymer = VecDeque::new();
    // let polymer_next = VecDeque::new();
    let input: String = input.lines().skip(2).join("");
    let res = parse_rules(&input).unwrap_or(("", vec![]));
    let res = res.1;
    println!("{:?}", res);
}
