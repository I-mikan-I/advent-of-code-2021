use if_chain::if_chain;
use itertools::Itertools;
use std::path::Path;

const GRID_SIZE: usize = 10;

fn main() {
    let path = if let Some(x) = std::env::args().nth(1) {
        x
    } else {
        "2021/day-11/input-test".to_string()
    };
    let path = Path::new(&path);
    let contents = std::fs::read_to_string(path).unwrap();
    let mut grid: [[usize; GRID_SIZE]; GRID_SIZE] = contents
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|char| char.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let (res_01, res_02) = (ex_01(&mut grid.clone()), ex_02(&mut grid));
    println!(
        "Solution to part 1: {}!\n\
        Solution to part 2: {}!",
        res_01, res_02
    );
}

fn ex_01(grid: &mut [[usize; GRID_SIZE]; GRID_SIZE]) -> usize {
    let mut flashes = 0;
    for _ in 1..=100 {
        step(grid, &mut flashes);
    }
    flashes
}

fn ex_02(grid: &mut [[usize; GRID_SIZE]; GRID_SIZE]) -> usize {
    let mut step_counter = 0;
    loop {
        step_counter += 1;
        step(grid, &mut 0);
        if grid
            .iter()
            .flat_map(<&[usize; GRID_SIZE]>::into_iter)
            .all(|&v| v == 0)
        {
            break;
        }
    }
    step_counter
}

fn step(grid: &mut [[usize; GRID_SIZE]; GRID_SIZE], flashes: &mut usize) {
    grid.iter_mut()
        .flat_map(|row| row.iter_mut())
        .for_each(|v| *v += 1);
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            try_flash(grid, (y, x), flashes);
        }
    }
}

fn try_flash(
    grid: &mut [[usize; GRID_SIZE]; GRID_SIZE],
    (y, x): (usize, usize),
    counter: &mut usize,
) {
    let elem = grid[y][x];
    if elem > 9 {
        grid[y][x] = 0;
        *counter += 1;
        (-1_i32..=1)
            .cartesian_product(-1_i32..=1)
            .for_each(|(y_, x_)| {
                if_chain!(
                    if let Ok(y_) = usize::try_from(y as i32 + y_);
                    if let Ok(x_) = usize::try_from(x as i32 + x_);
                    if let Some(to_incr) = grid.get_mut(y_).map(|row| row.get_mut(x_)).flatten();
                    then {
                        *to_incr += (*to_incr > 0) as usize;
                        try_flash(grid, (y_, x_), counter);
                    }
                )
            });
    }
}
