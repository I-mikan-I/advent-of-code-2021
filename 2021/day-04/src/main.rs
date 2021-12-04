use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

use self::MaybeMarked::*;

const BOARD_SIZE: usize = 5;

#[derive(Debug, Copy, Clone)]
enum MaybeMarked {
    Unmarked(u8),
    Marked(u8),
}

#[derive(Debug, Clone)]
struct BingoBoard {
    fields: [[MaybeMarked; BOARD_SIZE]; BOARD_SIZE],
    row_count: [usize; BOARD_SIZE],
    column_count: [usize; BOARD_SIZE],
}

impl BingoBoard {
    fn mark(&mut self, num: u8) {
        for (y, row) in self.fields.iter_mut().enumerate() {
            for (x, field) in row.iter_mut().enumerate() {
                *field = match *field {
                    Unmarked(val) if val == num => {
                        self.row_count[x] += 1;
                        self.column_count[y] += 1;
                        Marked(val)
                    }
                    x => x,
                }
            }
        }
    }
    fn has_won(&self) -> bool {
        self.row_count
            .iter()
            .chain(self.column_count.iter())
            .any(|&count| count == BOARD_SIZE)
    }
    fn score(&self) -> Option<usize> {
        if !self.has_won() {
            None
        } else {
            Some(
                self.fields
                    .iter()
                    .flatten()
                    .map(|marked| match *marked {
                        Unmarked(val) => val as usize,
                        _ => 0,
                    })
                    .sum::<usize>(),
            )
        }
    }
}

impl FromStr for BingoBoard {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields = s
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                line.split_whitespace()
                    .map(|num_str| MaybeMarked::Unmarked(u8::from_str(num_str).unwrap()))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| "Issue during BingoBoard parsing occured")?;
        Ok(Self {
            fields,
            row_count: [0; BOARD_SIZE],
            column_count: [0; BOARD_SIZE],
        })
    }
}

fn main() {
    let path = if let Some(x) = std::env::args().nth(1) {
        x
    } else {
        "2021/day-04/input-test".to_string()
    };
    let path = Path::new(&path);
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut commands = String::new();
    reader
        .read_line(&mut commands)
        .expect("could not read command line");
    let commands = commands
        .split(',')
        .map(str::trim)
        .map(u8::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let boards = construct_boards(reader);

    let win_message = if let Some(score) = ex_01(&commands, &boards) {
        format!("We found a winner! score: {}", score)
    } else {
        "Nobody won this time...".to_string()
    };
    let lose_message = if let Some(score) = ex_02(&commands, &boards) {
        format!("We found a loser! score: {}", score)
    } else {
        "Nobody seems to have lost...".to_string()
    };

    println!("{}\n{}", win_message, lose_message);
}

fn ex_01(commands: &[u8], boards: &[BingoBoard]) -> Option<usize> {
    let mut boards = Vec::from(boards);
    for &c in commands {
        boards.iter_mut().for_each(|board| board.mark(c));
        let winner = boards.iter().find(|board| board.has_won());
        if let Some(winner) = winner {
            return winner.score().map(|s| s * c as usize);
        }
    }
    None
}

fn ex_02(commands: &[u8], boards: &[BingoBoard]) -> Option<usize> {
    let mut boards = Vec::from(boards);
    for &c in commands {
        boards = boards
            .into_iter()
            .filter(|board| !board.has_won())
            .collect();
        boards.iter_mut().for_each(|board| board.mark(c));
        if boards.len() == 1 && boards[0].has_won() {
            let loser = &boards[0];
            return loser.score().map(|s| s * c as usize);
        }
    }
    None
}

fn construct_boards(boards_reader: impl BufRead) -> Vec<BingoBoard> {
    let mut boards: Vec<BingoBoard> = vec![];
    let mut string_buffer = String::new();

    for line in boards_reader
        .lines()
        .chain(std::iter::once(Ok("\n".to_string())))
    {
        let line = match line {
            Ok(line) => line,
            _ => continue,
        };
        if line.trim().is_empty() && !string_buffer.is_empty() {
            boards.push(BingoBoard::from_str(&string_buffer).unwrap());
            string_buffer.clear();
        }
        string_buffer.push_str(&line);
        string_buffer.push('\n');
    }
    boards
}
