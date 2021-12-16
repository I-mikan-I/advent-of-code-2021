use crate::Packet::Operator;
use itertools::Itertools;
use nom::bytes::complete::{take, take_while};
use nom::character::complete::char;
use nom::combinator::map_res;
use nom::error::ErrorKind::Fail;
use nom::error::FromExternalError;
use nom::multi::{many0, many1, many_m_n};
use nom::sequence::{preceded, terminated};
use nom::IResult;
use std::path::Path;

#[derive(Debug)]
enum Packet {
    Literal(usize),
    Operator(Vec<Packet>),
}

static mut VER_COUNT: usize = 0;

fn parse_operator(input: &str) -> IResult<&str, Packet> {
    println!("got operator: {}", input);
    let (input, len_type_id) = map_res(take(1_usize), |bit| {
        if bit == "1" {
            std::result::Result::<bool, ()>::Ok(true)
        } else {
            std::result::Result::<bool, ()>::Ok(false)
        }
    })(input)?;
    if len_type_id {
        let (input, sub_packets) = map_res(take(11_usize), |s| usize::from_str_radix(s, 2))(input)?;
        let (input, rest) = many_m_n(sub_packets, sub_packets, parse_packet)(input)?;
        Ok((input, Operator(rest)))
    } else {
        let (input, length) = map_res(take(15_usize), |s| usize::from_str_radix(s, 2))(input)?;
        let (consumed, rest) = many0(parse_packet)(&input[..length])?;
        Ok((&input[length - consumed.len()..], Operator(rest)))
    }
}

fn parse_literal(input: &str) -> IResult<&str, Packet> {
    println!("got literal: {}", input);
    let (input, begin) = many0(preceded(char('1'), take(4_usize)))(input)?;
    let (input, end) = preceded(char('0'), take(4_usize))(input)?;
    let result = begin.into_iter().chain(std::iter::once(end)).join("");
    println!("got result: {:?}", result);
    let result = usize::from_str_radix(&result, 2).unwrap();
    println!("got number: {}", result);
    Ok((input, Packet::Literal(result)))
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    println!("got packet: {}", input);
    let (input, version) = map_res(take(3_usize), |s| u8::from_str_radix(s, 2))(input)?;
    let (input, type_id) = map_res(take(3_usize), |s| u8::from_str_radix(s, 2))(input)?;
    println!("Version: {}", version);
    unsafe {
        VER_COUNT += version as usize;
    }
    if type_id == 4 {
        let res = parse_literal(input);
        res
    } else {
        //let res = terminated(parse_operator, take_while::<_, &str, _>(|c| c == '0'))(input);
        let res = parse_operator(input);
        res
    }
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

    let binary = hex.into_iter().map(|num| format!("{:0>4b}", num)).join("");
    //let binary = "00111000000000000110111101000101001010010001001000000000";
    let packets = parse_packet(&binary);
    println!("{}", binary);
    println!("{:?}", packets.unwrap_or_else(|_| panic!()));
    unsafe { println!("{}", VER_COUNT) }
}
