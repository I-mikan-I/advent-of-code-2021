use itertools::Itertools;
use std::cmp::{max, min};
use std::str::FromStr;

fn main() {
    let p1 = std::env::args()
        .nth(1)
        .map(|s| usize::from_str(&s).ok())
        .flatten()
        .expect("Please provide player 1's position as the first argument");
    let p2 = std::env::args()
        .nth(2)
        .map(|s| usize::from_str(&s).ok())
        .flatten()
        .expect("Please provide player 2's position as the first argument");
    let res_01 = ex_01(p1, p2);
    let res_02 = ex_02(p1, p2);
    println!(
        "Solution to part 1: {}!\n\
    Solution to part 2: {}!",
        res_01, res_02
    );
}

struct Die(usize);

impl Die {
    fn new() -> Die {
        Die(0)
    }
    fn roll(&mut self) -> usize {
        self.0 += 1;
        self.0
    }
}

fn ex_01(mut player1: usize, mut player2: usize) -> usize {
    const BOARD_SIZE: usize = 10;
    const GOAL: usize = 1000;
    let mut die = Die::new();
    let mut p1_score = 0;
    let mut p2_score = 0;

    let mut rotation = true;
    while p1_score < GOAL && p2_score < GOAL {
        let (player, score) = if rotation {
            (&mut player1, &mut p1_score)
        } else {
            (&mut player2, &mut p2_score)
        };
        let rolls = std::iter::repeat_with(|| die.roll()).take(3).sum::<usize>();
        *player = (*player - 1 + rolls) % BOARD_SIZE + 1;
        *score += *player;
        rotation = !rotation;
    }
    min(p1_score, p2_score) * die.0
}

fn ex_02(player1: usize, player2: usize) -> usize {
    const BOARD_SIZE: usize = 10;
    const GOAL: usize = 21;
    let mut p1_wins = 0_usize;
    let mut p2_wins = 0_usize;

    let mut scores = [[[[0; BOARD_SIZE]; BOARD_SIZE]; GOAL]; GOAL];
    scores[0][0][player1 - 1][player2 - 1] = 1;

    let mut rotation = true;
    let rolls: [usize; 27] = (1..=3)
        .cartesian_product(1..=3)
        .cartesian_product(1..=3)
        .map(|((f, s), t)| f + s + t)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    loop {
        let mut scores_t = scores;
        for i in 0..GOAL {
            for k in 0..GOAL {
                for j in 0..BOARD_SIZE {
                    let new_positions = rolls
                        .iter()
                        .map(|roll| (j + roll) % BOARD_SIZE)
                        .collect::<Vec<_>>();
                    for l in 0..BOARD_SIZE {
                        for &np in &new_positions {
                            if rotation {
                                let new_score = i + np + 1;
                                if new_score >= GOAL {
                                    p1_wins += scores[i][k][j][l];
                                } else {
                                    scores_t[new_score][k][np][l] += scores[i][k][j][l];
                                }
                            } else {
                                let new_score = k + np + 1;
                                if new_score >= GOAL {
                                    p2_wins += scores[i][k][l][j];
                                } else {
                                    scores_t[i][new_score][l][np] += scores[i][k][l][j];
                                }
                            }
                        }
                        if rotation {
                            scores_t[i][k][j][l] -= scores[i][k][j][l];
                        } else {
                            scores_t[i][k][l][j] -= scores[i][k][l][j];
                        }
                    }
                }
            }
        }
        rotation = !rotation;
        if scores_t == scores {
            break;
        }
        scores = scores_t;
    }
    max(p1_wins, p2_wins)
}
