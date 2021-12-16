use crate::Packet::Operator;
use itertools::Itertools;
use nom::bytes::complete::take;
use nom::character::complete::char;
use nom::combinator::map_res;
use nom::error::ErrorKind::Fail;
use nom::error::FromExternalError;
use nom::multi::many1;
use nom::sequence::preceded;
use nom::IResult;
use std::path::Path;

#[derive(Debug)]
enum Packet {
    Literal(usize),
    Operator,
}

fn parse_operator(input: &str) -> IResult<&str, Packet> {
    let (input, len_type_id) = map_res(take(1_usize), |bit| {
        if bit == "1" {
            std::result::Result::<bool, ()>::Ok(true)
        } else {
            std::result::Result::<bool, ()>::Ok(false)
        }
    })(input)?;
    if len_type_id {
        let (input, sub_packets) = map_res(take(11_usize), |s| usize::from_str_radix(s, 2))(input)?;

        Ok((input, Operator))
    } else {
        let (input, length) = map_res(take(15_usize), |s| usize::from_str_radix(s, 2))(input)?;
        Ok((input, Operator))
    }
}

fn parse_literal(input: &str) -> IResult<&str, Packet> {
    let (input, begin) = many1(preceded(char('0'), take(3_usize)))(input)?;
    let (input, end) = preceded(char('1'), take(3_usize))(input)?;
    let result = begin.into_iter().chain(std::iter::once(end)).join("");
    let result = usize::from_str_radix(&result, 2).unwrap();
    Ok((input, Packet::Literal(result)))
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    let (input, version) = map_res(take(3_usize), |s| u8::from_str_radix(s, 2))(input)?;
    let (input, type_id) = map_res(take(3_usize), |s| u8::from_str_radix(s, 2))(input)?;
    println!("Version: {}", version);
    if type_id == 4 {
        parse_literal(input)
    } else {
        parse_operator(input)
    }
}

fn parse_packets(input: &str) -> IResult<&str, Vec<Packet>> {
    many1(parse_packet)(input)
}

fn main() {
    let path = if let Some(x) = std::env::args().nth(1) {
        x
    } else {
        "2021/day-15/input-test".to_string()
    };
    let path = Path::new(&path);
    let contents = std::fs::read_to_string(path).unwrap();
    ex_01(&contents);
}

fn ex_01(input: &str) {
    let hex: Vec<u8> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(16).unwrap() as u8)
        .collect();

    let binary = hex.into_iter().map(|num| format!("{:0>4b}\n", num)).join("");
    let packets = parse_packets(&binary);
    println!("{}", binary);
    println!("{:?}", packets.unwrap_or_else(|_| panic!()));
}
