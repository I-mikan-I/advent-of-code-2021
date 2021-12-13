use itertools::Itertools;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

fn main() {
    let path = if let Some(x) = std::env::args().nth(1) {
        x
    } else {
        "2021/day-13/input-test".to_string()
    };
    let path = Path::new(&path);
    let contents = std::fs::read_to_string(path).unwrap();
    challenge(&contents);
}

fn challenge(input: &str) {
    let mut paper: Vec<Vec<bool>> = Vec::new();
    let mut lines = BufReader::new(input.as_bytes());
    let mut line = String::new();
    while let Ok(bytes) = lines.read_line(&mut line) {
        if bytes == 0 {
            break;
        }
        if line.trim().is_empty() {
            break;
        }
        let (x, y) = line
            .trim()
            .split(',')
            .map(|c| usize::from_str(c).unwrap())
            .collect_tuple::<(usize, usize)>()
            .unwrap();

        for _ in paper.len()..=y {
            paper.push(Vec::new());
        }
        for _ in paper[y].len()..=x {
            paper[y].push(false);
        }
        paper[y][x] = true;
        line.clear();
    }
    let mut step: usize = 0;
    while let Ok(bytes) = lines.read_line(&mut line) {
        step += 1;
        let command = line.trim();
        if bytes == 0 {
            break;
        }
        if command.is_empty() {
            continue;
        }
        let command = command.split(" along ").nth(1).unwrap();
        match command.as_bytes() {
            [b'y', b'=', rest @ ..] => fold_up(
                &mut paper,
                usize::from_str(std::str::from_utf8(rest).unwrap()).unwrap(),
            ),
            [b'x', b'=', rest @ ..] => fold_left(
                &mut paper,
                usize::from_str(std::str::from_utf8(rest).unwrap()).unwrap(),
            ),
            _ => panic!("unknown command."),
        };
        if step == 1 {
            let dots_visible = paper
                .iter()
                .flat_map(|row| row.iter())
                .filter(|b| **b)
                .count();
            println!("dots after first fold: {}!", dots_visible);
        }
        line.clear();
    }

    for row in paper.iter() {
        for &dot in row {
            let symbol = if dot { '#' } else { '.' };
            print!(" {}", symbol);
        }
        println!();
    }
}

fn fold_up(paper: &mut Vec<Vec<bool>>, line: usize) {
    for y in 0..line + 1 {
        if paper.len() > line + y {
            paper[line - y] = zip_booleans(
                paper[line - y].iter().copied(),
                paper[line + y].iter().copied(),
            );
        } else {
            break;
        }
    }
    paper.truncate(line);
}

fn fold_left(paper: &mut [Vec<bool>], line: usize) {
    for y in 0..paper.len() {
        let paper = &mut *paper;
        if paper[y].len() < line + 1 {
            continue;
        }
        paper[y] = zip_booleans(
            paper[y][..line].iter().rev().copied(),
            paper[y][line + 1..].iter().copied(),
        );
        paper[y].truncate(line);
    }
}

fn zip_booleans<T, E>(iter1: T, iter2: E) -> Vec<bool>
where
    T: Iterator<Item = bool>,
    E: Iterator<Item = bool>,
{
    use itertools::EitherOrBoth;
    iter1
        .zip_longest(iter2)
        .map(|zipped| {
            matches!(
                zipped,
                EitherOrBoth::Both(true, _)
                    | EitherOrBoth::Both(_, true)
                    | EitherOrBoth::Left(true)
                    | EitherOrBoth::Right(true)
            )
        })
        .collect()
}
