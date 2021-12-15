use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::path::Path;

fn main() {
    let path = if let Some(x) = std::env::args().nth(1) {
        x
    } else {
        "2021/day-15/input-test".to_string()
    };
    let path = Path::new(&path);
    let contents = std::fs::read_to_string(path).unwrap();
    let res_01 = search_grid(&parse_grid(&contents));
    let res_02 = search_grid(&make_grid_02(&contents));
    println!(
        "Solution to part 1: {}!\n\
    Solution to part 2: {}!",
        res_01, res_02
    )
}

struct DistanceTup((usize, usize), usize);

impl Eq for DistanceTup {}

impl PartialEq<Self> for DistanceTup {
    fn eq(&self, other: &Self) -> bool {
        other.1 == self.1
    }
}

impl PartialOrd<Self> for DistanceTup {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.1.partial_cmp(&self.1)
    }
}

impl Ord for DistanceTup {
    fn cmp(&self, other: &Self) -> Ordering {
        other.1.cmp(&self.1)
    }
}

fn make_grid_02(input: &str) -> Vec<Vec<u32>> {
    let mut grid = parse_grid(input);
    let old_end = grid.len();
    for i in 1..5 {
        for r in grid.iter_mut().take(old_end) {
            let mut new_part: Vec<_> = r
                .iter()
                .take(old_end)
                .map(|v| (v + i - 1) % 9 + 1)
                .collect();
            r.append(&mut new_part);
        }
    }
    for i in 0..4_u32 {
        for r in 0..old_end {
            let new_row: Vec<_> = grid[r + i as usize * old_end]
                .iter()
                .map(|v| v % 9 + 1)
                .collect();
            grid.push(new_row);
        }
    }
    grid
}

fn parse_grid(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn search_grid(grid: &[Vec<u32>]) -> usize {
    let mut queue: BinaryHeap<DistanceTup> = BinaryHeap::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut distances: HashMap<(usize, usize), usize> = HashMap::new();
    queue.push(DistanceTup((0_usize, 0_usize), 0));
    distances.insert((0_usize, 0_usize), 0);

    let end: (usize, usize) = (grid.len() - 1, grid[0].len() - 1);
    loop {
        let next = queue.pop();
        let next = match next {
            Some(next) => next,
            None => continue,
        };
        for cords in [(-1_i32, 0_i32), (1, 0), (0, -1), (0, 1)] {
            if let Ok(y) = usize::try_from(next.0 .0 as i32 + cords.0) {
                if let Ok(x) = usize::try_from(next.0 .1 as i32 + cords.1) {
                    if !visited.contains(&(y, x)) {
                        if let Some(&old_dis) = grid.get(y).map(|r| r.get(x)).flatten() {
                            let new_distance = next.1 + old_dis as usize;
                            let new_distance = match distances.get(&(y, x)) {
                                Some(&val) if val <= new_distance => continue,
                                _ => new_distance,
                            };
                            distances.insert((y, x), new_distance);
                            queue.push(DistanceTup((y, x), new_distance))
                        }
                    }
                }
            }
        }
        visited.insert(next.0);
        if next.0 == end {
            break;
        }
    }
    distances[&end]
}
