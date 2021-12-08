use std::path::Path;
use std::str::FromStr;

fn main() {
    let path = if let Some(x) = std::env::args().nth(1) {
        x
    } else {
        "2021/day-08/input-test".to_string()
    };
    let path = Path::new(&path);
    let contents = std::fs::read_to_string(path).unwrap();
    let outputs: Vec<_> = contents.lines().map(str::trim).collect();
    let res_01 = ex_01(&outputs);
    let res_02 = ex_02(&outputs);
    println!(
        "Result to part 1: {}!\n\
        Result to part 2: {}!",
        res_01, res_02
    );
}

/* ================> EX_01 <================ */
fn ex_01(lines: &[&str]) -> usize {
    lines
        .iter()
        .flat_map(|line| line.split('|').nth(1).unwrap().split_whitespace())
        .filter(|&str| correct_length(str))
        .count()
}

fn correct_length(s: &str) -> bool {
    s.len() == 2 || s.len() == 4 || s.len() == 3 || s.len() == 7
}

/* ================> EX_02 <================ */

#[derive(Eq)]
struct SegmentCollection {
    segments: Vec<char>,
}

impl PartialEq<Self> for SegmentCollection {
    fn eq(&self, other: &Self) -> bool {
        self.segments.len() == other.segments.len()
            && self.intersection(other) == self.segments.len()
    }
}

impl FromStr for SegmentCollection {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(SegmentCollection::new(s))
    }
}

impl SegmentCollection {
    fn new(s: &str) -> Self {
        let segments = s.chars().filter(|c| !c.is_whitespace()).collect();
        Self { segments }
    }
    fn intersection(&self, other: &Self) -> usize {
        other
            .segments
            .iter()
            .filter(|&c| self.segments.contains(c))
            .count()
    }
    fn len(&self) -> usize {
        self.segments.len()
    }
}

macro_rules! rest_to_resolved {
    ($k:expr, $i:expr, $resolved:expr, $rest:expr) => {
        $resolved[$k] = Some($rest.swap_remove($i))
    };
}

// this has gotten a bit long and messy, but it works...
fn ex_02(lines: &[&str]) -> usize {
    let mut sum: usize = 0;
    for &v in lines {
        let [input, output]: [&str; 2] = v.split('|').collect::<Vec<_>>().try_into().unwrap();
        let mut resolved = [None, None, None, None, None, None, None, None, None, None];
        let mut rest = Vec::with_capacity(10);
        input
            .split_whitespace()
            .map(|s| SegmentCollection::from_str(s).unwrap())
            .for_each(|sc| rest.push(sc));
        let output: Vec<_> = output
            .split_whitespace()
            .map(|s| SegmentCollection::from_str(s).unwrap())
            .collect();

        while resolved.contains(&None) {
            for i in 0..rest.len() {
                // we need to make this check because we remove the SegmentCollection from the rest.
                if i >= rest.len() {
                    break;
                }
                let sc = &rest[i];
                match sc.len() {
                    2 => rest_to_resolved!(1, i, resolved, rest),
                    3 => rest_to_resolved!(7, i, resolved, rest),
                    4 => rest_to_resolved!(4, i, resolved, rest),
                    7 => rest_to_resolved!(8, i, resolved, rest),
                    6 => {
                        if let Some(scrutinizer) = resolved[1].as_ref() {
                            match sc.intersection(scrutinizer) {
                                x if x < 2 => rest_to_resolved!(6, i, resolved, rest),
                                _ if resolved[4].is_some() => {
                                    if resolved[4].as_ref().unwrap().intersection(sc) == 4 {
                                        rest_to_resolved!(9, i, resolved, rest);
                                    } else {
                                        rest_to_resolved!(0, i, resolved, rest);
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                    5 => {
                        if let Some(scrutinizer) = resolved[1].as_ref() {
                            match sc.intersection(scrutinizer) {
                                x if x >= 2 => rest_to_resolved!(3, i, resolved, rest),
                                _ if resolved[4].is_some() => {
                                    if resolved[4].as_ref().unwrap().intersection(sc) == 3 {
                                        rest_to_resolved!(5, i, resolved, rest);
                                    } else {
                                        rest_to_resolved!(2, i, resolved, rest);
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                    _ => panic!("unknown segment size"),
                }
            }
        }

        let mut output_final = 0_usize;
        for o in output {
            let display_number = resolved
                .iter()
                .enumerate()
                .find(|(_, sc)| &o == sc.as_ref().unwrap())
                .unwrap()
                .0;
            output_final *= 10;
            output_final += display_number;
        }
        sum += output_final;
    }
    sum
}
