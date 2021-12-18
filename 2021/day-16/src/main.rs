use crate::Packet::Operator;
use itertools::Itertools;
use nom::bytes::complete::take;
use nom::character::complete::char;
use nom::combinator::map_res;
use nom::multi::{many0, many_m_n};
use nom::sequence::preceded;
use nom::{Finish, IResult};
use std::path::Path;

#[derive(Debug)]
enum Packet {
    Literal(usize),
    Operator(Vec<PacketWrapper>),
}

#[derive(Debug)]
struct PacketWrapper(u8, u8, Packet);

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
        let (input, rest) = many_m_n(sub_packets, sub_packets, parse_packet)(input)?;
        Ok((input, Operator(rest)))
    } else {
        let (input, length) = map_res(take(15_usize), |s| usize::from_str_radix(s, 2))(input)?;
        let (consumed, rest) = many0(parse_packet)(&input[..length])?;
        Ok((&input[length - consumed.len()..], Operator(rest)))
    }
}

fn parse_literal(input: &str) -> IResult<&str, Packet> {
    let (input, begin) = many0(preceded(char('1'), take(4_usize)))(input)?;
    let (input, end) = preceded(char('0'), take(4_usize))(input)?;
    let result = begin.into_iter().chain(std::iter::once(end)).join("");
    let result = usize::from_str_radix(&result, 2).unwrap();
    Ok((input, Packet::Literal(result)))
}

fn parse_packet(input: &str) -> IResult<&str, PacketWrapper> {
    let (input, version) = map_res(take(3_usize), |s| u8::from_str_radix(s, 2))(input)?;
    let (input, type_id) = map_res(take(3_usize), |s| u8::from_str_radix(s, 2))(input)?;
    if type_id == 4 {
        nom::combinator::map(parse_literal, |packet| {
            PacketWrapper(version, type_id, packet)
        })(input)
    } else {
        nom::combinator::map(parse_operator, |packet| {
            PacketWrapper(version, type_id, packet)
        })(input)
    }
}

fn main() {
    let path = if let Some(x) = std::env::args().nth(1) {
        x
    } else {
        "2021/day-16/input-test".to_string()
    };
    let path = Path::new(&path);
    let contents = std::fs::read_to_string(path).unwrap();
    ex_01(&contents);
}

fn ex_01(input: &str) {
    let hex_iter = input.trim().chars().map(|c| c.to_digit(16).unwrap() as u8);

    let binary = hex_iter.map(|num| format!("{:0>4b}", num)).join("");
    let packets = parse_packet(&binary)
        .finish()
        .unwrap_or_else(|e: nom::error::Error<&str>| panic!("{:?}", e))
        .1;
    let version_sum = packet_sum(&packets);
    println!("version sum: {}", version_sum);
    let result = calculate(&packets);
    println!("result: {}", result);
}

fn packet_sum(packet: &PacketWrapper) -> usize {
    match &packet.2 {
        Packet::Literal(_) => packet.0 as usize,
        Packet::Operator(vec) => packet.0 as usize + vec.iter().map(packet_sum).sum::<usize>(),
    }
}

fn calculate(packet: &PacketWrapper) -> usize {
    match &packet {
        PacketWrapper(_, _, Packet::Literal(val)) => *val,
        PacketWrapper(_, 0, Packet::Operator(vec)) => vec.iter().map(calculate).sum::<usize>(),
        PacketWrapper(_, 1, Packet::Operator(vec)) => vec.iter().map(calculate).product::<usize>(),
        PacketWrapper(_, 2, Packet::Operator(vec)) => vec.iter().map(calculate).min().unwrap(),
        PacketWrapper(_, 3, Packet::Operator(vec)) => vec.iter().map(calculate).max().unwrap(),
        PacketWrapper(_, 5, Packet::Operator(vec)) => {
            (calculate(&vec[0]) > calculate(&vec[1])) as usize
        }
        PacketWrapper(_, 6, Packet::Operator(vec)) => {
            (calculate(&vec[0]) < calculate(&vec[1])) as usize
        }
        PacketWrapper(_, 7, Packet::Operator(vec)) => {
            (calculate(&vec[0]) == calculate(&vec[1])) as usize
        }
        _ => panic!("unknown packet"),
    }
}
