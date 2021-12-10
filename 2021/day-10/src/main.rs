use crate::ChunkDelimiter::{
    AnBegin, AnClose, BrBegin, BrClose, CrBegin, CrClose, SqBegin, SqClose,
};
use std::path::Path;
use std::str::FromStr;

fn main() {
    let path = if let Some(x) = std::env::args().nth(1) {
        x
    } else {
        "2021/day-10/input-test".to_string()
    };
    let path = Path::new(&path);
    let contents = std::fs::read_to_string(path).unwrap();
    let (res_01, res_02) = solve_challenges(&contents);

    println!(
        "The syntax score is: {}!\n\
    The autocompletion score is: {}!",
        res_01, res_02
    );
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum ChunkDelimiter {
    BrBegin,
    BrClose,
    CrBegin,
    CrClose,
    SqBegin,
    SqClose,
    AnBegin,
    AnClose,
}

impl ChunkDelimiter {
    fn partner(&self) -> ChunkDelimiter {
        match *self {
            BrBegin => BrClose,
            BrClose => BrBegin,
            CrBegin => CrClose,
            CrClose => CrBegin,
            SqBegin => SqClose,
            SqClose => SqBegin,
            AnBegin => AnClose,
            AnClose => AnBegin,
        }
    }
    fn score_error(&self) -> usize {
        match *self {
            BrClose => 3,
            SqClose => 57,
            CrClose => 1197,
            AnClose => 25137,
            _ => panic!("illegal character: character must be a closing delimiter"),
        }
    }
    fn score_incomplete(&self) -> usize {
        match *self {
            BrBegin => 1,
            SqBegin => 2,
            CrBegin => 3,
            AnBegin => 4,
            _ => panic!("illegal character: character must be an opening delimiter"),
        }
    }
}

impl FromStr for ChunkDelimiter {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "(" => Ok(BrBegin),
            ")" => Ok(BrClose),
            "{" => Ok(CrBegin),
            "}" => Ok(CrClose),
            "[" => Ok(SqBegin),
            "]" => Ok(SqClose),
            "<" => Ok(AnBegin),
            ">" => Ok(AnClose),
            _ => Err(()),
        }
    }
}

fn solve_challenges(lines: &str) -> (usize, usize) {
    let mut scores_incomplete = Vec::new();
    let mut score_error: usize = 0;
    'line_loop: for line in lines.lines() {
        let mut stack: Vec<ChunkDelimiter> = Vec::with_capacity(100);
        for c in line.chars() {
            let delim = ChunkDelimiter::from_str(&c.to_string()).expect("unknown character");
            let to_pop = match delim {
                BrClose | CrClose | SqClose | AnClose => Some(delim.partner()),
                _ => None,
            };
            if let Some(delim_pop) = to_pop {
                if let Some(popped) = stack.pop() {
                    if delim_pop == popped {
                        continue;
                    }
                }
                score_error += delim.score_error();
                continue 'line_loop;
            }
            stack.push(delim);
        }
        let mut score: usize = 0;
        for cd in stack.into_iter().rev() {
            score *= 5;
            score += cd.score_incomplete();
        }
        scores_incomplete.push(score);
    }
    scores_incomplete.sort_unstable();
    (score_error, scores_incomplete[scores_incomplete.len() / 2])
}
