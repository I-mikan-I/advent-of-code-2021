use std::path::Path;

fn main() {
    let path = if let Some(x) = std::env::args().nth(1) {
        x
    } else {
        "2021/day-25/input-test".to_string()
    };
    let path = Path::new(&path);
    let contents = std::fs::read_to_string(path).unwrap();
    let res = challenge(&contents);
    println!("Result of day-25: {}!", res);
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum SeaCucumber {
    Right,
    Down,
}

impl SeaCucumber {
    fn forward(
        &self,
        (x, y): (usize, usize),
        current: &[Vec<Option<SeaCucumber>>],
        next: &mut [Vec<Option<SeaCucumber>>],
    ) -> bool {
        match self {
            SeaCucumber::Right => {
                let old = y;
                let y = if y < current[0].len() - 1 { y + 1 } else { 0 };
                if current[x][y].is_none() {
                    next[x][y] = Some(SeaCucumber::Right);
                    next[x][old] = None;
                    true
                } else {
                    false
                }
            }
            SeaCucumber::Down => {
                let old = x;
                let x = if x < current.len() - 1 { x + 1 } else { 0 };
                if current[x][y].is_none() {
                    next[x][y] = Some(SeaCucumber::Down);
                    next[old][y] = None;
                    true
                } else {
                    false
                }
            }
        }
    }
}

fn challenge(input: &str) -> usize {
    let mut map: Vec<_> = input
        .lines()
        .map(|l| {
            l.trim()
                .chars()
                .map(|c| match c {
                    '.' => None,
                    '>' => Some(SeaCucumber::Right),
                    'v' => Some(SeaCucumber::Down),
                    _ => panic!(),
                })
                .collect::<Vec<_>>()
        })
        .collect();
    let mut map_clone = map.clone();
    let mut steps = 0;
    let mut changed = true;
    while changed {
        changed = false;
        steps += 1;
        for rotation in [true, false] {
            for (x, row) in map.iter().enumerate() {
                for (y, field) in row.iter().enumerate() {
                    match field {
                        Some(v @ SeaCucumber::Right) if rotation => {
                            changed = v.forward((x, y), &map, &mut map_clone) || changed;
                        }
                        Some(v @ SeaCucumber::Down) if !rotation => {
                            changed = v.forward((x, y), &map, &mut map_clone) || changed;
                        }
                        _ => (),
                    }
                }
            }
            map = map_clone.clone();
        }
        map = map_clone.clone();
    }
    steps
}
