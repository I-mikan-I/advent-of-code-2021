use std::path::Path;

fn main() {
    let path = if let Some(x) = std::env::args().nth(1) {
        x
    } else {
        "2021/day-09/input-test".to_string()
    };
    let path = Path::new(&path);
    let contents = std::fs::read_to_string(path).unwrap();
    let heightmap: Vec<Vec<u32>> = contents
        .lines()
        .map(str::chars)
        .map(|chars| chars.map(|c| c.to_digit(10).unwrap()))
        .map(Iterator::collect::<Vec<u32>>)
        .collect();
    let res_01 = ex_01(&heightmap);
    let res_02 = ex_02(&heightmap);

    println!(
        "The danger level of low points is {}!\n\
    The danger level of the largest basins is {}!",
        res_01, res_02
    )
}

fn ex_01(heightmap: &[Vec<u32>]) -> usize {
    let minimums = find_minimums(heightmap);
    let sum = minimums
        .into_iter()
        .map(|(y, x)| heightmap[y][x] + 1)
        .sum::<u32>();
    sum as usize
}

fn ex_02(heightmap: &[Vec<u32>]) -> usize {
    let minimums = find_minimums(heightmap);
    let mut result: Vec<_> = minimums
        .into_iter()
        .map(|t| find_basin_size(t, heightmap, &mut Vec::new()))
        .collect();
    result.sort_by(|a, b| b.cmp(a));
    result[0] * result[1] * result[2]
}

fn find_minimums(heightmap: &[Vec<u32>]) -> Vec<(usize, usize)> {
    let mut minimums: Vec<(usize, usize)> = Vec::new();
    for (y, r) in heightmap.iter().enumerate() {
        for (x, c) in r.iter().enumerate() {
            let up = if y > 0 {
                heightmap.get(y - 1).map(|r_| r_.get(x)).flatten()
            } else {
                None
            };
            let down = heightmap.get(y + 1).map(|r_| r_.get(x)).flatten();
            let left = if x > 0 {
                heightmap.get(y).map(|r_| r_.get(x - 1)).flatten()
            } else {
                None
            };
            let right = heightmap.get(y).map(|r_| r_.get(x + 1)).flatten();
            let mut minimum = true;
            for o in [up, down, left, right] {
                match o {
                    Some(t) if t <= c => {
                        minimum = false;
                        break;
                    }
                    _ => (),
                }
            }
            if minimum {
                minimums.push((y as usize, x as usize))
            }
        }
    }
    minimums
}

macro_rules! get_if_larger {
    ($y: expr, $x:expr, $height:expr, $visited:expr, $heightmap:expr) => {
        $heightmap
            .get($y)
            .map(|r_| r_.get($x))
            .flatten()
            .filter(|&&val| val > $height)
            .map(|_| find_basin_size(($y, $x), $heightmap, $visited))
            .unwrap_or(0)
    };
}

fn find_basin_size(
    (y, x): (usize, usize),
    heightmap: &[Vec<u32>],
    visited: &mut Vec<(usize, usize)>,
) -> usize {
    let height = heightmap[y][x];
    if height > 8 {
        return 0;
    }
    visited.push((y, x));

    let up = if y > 0 && !visited.contains(&(y - 1, x)) {
        get_if_larger!(y - 1, x, height, visited, heightmap)
    } else {
        0
    };
    let down = if !visited.contains(&(y + 1, x)) {
        get_if_larger!(y + 1, x, height, visited, heightmap)
    } else {
        0
    };
    let left = if x > 0 && !visited.contains(&(y, x - 1)) {
        get_if_larger!(y, x - 1, height, visited, heightmap)
    } else {
        0
    };
    let right = if !visited.contains(&(y, x + 1)) {
        get_if_larger!(y, x + 1, height, visited, heightmap)
    } else {
        0
    };

    up + down + left + right + 1
}
