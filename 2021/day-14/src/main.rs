use itertools::Itertools;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::alpha1;
use nom::combinator::map_res;
use nom::multi::many1;
use nom::{sequence, IResult};
use num::bigint::BigInt;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};
use std::path::Path;

fn main() {
    let path = if let Some(x) = std::env::args().nth(1) {
        x
    } else {
        "2021/day-14/input-test".to_string()
    };
    let path = Path::new(&path);
    let contents = std::fs::read_to_string(path).unwrap();
    let res_01 = calculate_steps(&contents, 10);
    let res_02 = calculate_steps(&contents, 40);
    println!(
        "Solution to part 1: {}!\n\
         Solution to part 2: {}!",
        res_01, res_02
    );
}

type Rule = ([char; 2], char);

// trailing white spaces parser macro
macro_rules! tws {
    ($t:expr) => {
        nom::sequence::terminated($t, nom::character::complete::multispace0)
    };
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    map_res::<_, _, _, _, &'static str, _, _>(
        sequence::tuple((tws!(alpha1), tws!(tag_no_case("->")), alpha1)),
        |(begin, _, end): (&str, &str, &str)| {
            Ok((
                begin
                    .chars()
                    .collect::<Vec<_>>()
                    .try_into()
                    .map_err(|_| "issue parsing lvalue of rule.")?,
                end.chars().next().ok_or("issue parsing rvalue of rule.")?,
            ))
        },
    )(input)
}

fn parse_rules(input: &str) -> IResult<&str, Vec<Rule>> {
    many1(tws!(parse_rule))(input)
}

fn calculate_steps(input: &str, steps: usize) -> BigInt {
    let mut input = input.lines();
    let mut polymer: VecDeque<_> = input.next().unwrap().trim().chars().collect();
    let input: String = input.skip(1).join("\n");
    // vec of all rules
    let rules = parse_rules(&input).unwrap_or_else(|_| panic!()).1;
    // current pairs per rule (if no matching rule is found, we omit the pair)
    let mut rule_count: HashMap<_, _> = rules.iter().map(|v| (v.0, BigInt::from(0))).collect();
    // current char count
    let mut char_count: HashMap<char, BigInt> = HashMap::new();

    //set up initial state
    for _ in 0..polymer.len() {
        let current = polymer.pop_front().unwrap();
        *char_count.entry(current).or_insert_with(|| 0.into()) += 1;
        if let Some(&next) = polymer.front() {
            *rule_count.get_mut(&[current, next]).unwrap() += 1;
        }
    }

    //clone data to use old state during one step
    let mut char_count_temp = char_count.clone();
    let mut rule_count_temp = rule_count.clone();
    for _ in 0..steps {
        // update next state for every rule..
        for key in rules.iter() {
            let key = key.0;
            if let Some(middle) = fuse_polymer((key[0], key[1]), &rules) {
                let pair_count = &rule_count[&key];
                if *pair_count < 1.into() {
                    continue;
                }
                *rule_count_temp.get_mut(&key).unwrap() -= pair_count;
                match char_count_temp.entry(middle) {
                    Entry::Vacant(e) => {
                        e.insert(1.into());
                    }
                    Entry::Occupied(mut e) => *e.get_mut() += pair_count,
                };
                if rule_count.contains_key(&[key[0], middle]) {
                    *rule_count_temp.get_mut(&[key[0], middle]).unwrap() += pair_count;
                }
                if rule_count.contains_key(&[middle, key[1]]) {
                    *rule_count_temp.get_mut(&[middle, key[1]]).unwrap() += pair_count;
                }
            }
        }
        char_count = char_count_temp.clone();
        rule_count = rule_count_temp.clone();
    }
    let (min, max) = match char_count.into_values().minmax() {
        itertools::MinMaxResult::MinMax(min, max) => (min, max),
        _ => panic!("no minimum and maximum found."),
    };
    max - min
}

fn fuse_polymer((left, right): (char, char), rules: &[Rule]) -> Option<char> {
    rules
        .iter()
        .find(|&rule| rule.0 == [left, right])
        .map(|result| result.1)
}
