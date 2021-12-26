use std::cmp::{max, min};
use std::fmt::{Display, Formatter};
use std::path::Path;
use Amphipod::*;

fn main() {
    let path = if let Some(x) = std::env::args().nth(1) {
        x
    } else {
        "2021/day-23/input".to_string()
    };
    let path = Path::new(&path);
    let contents = std::fs::read_to_string(path).unwrap();
    let contents: Vec<_> = contents
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();
    let row_1 = [
        contents[2][3],
        contents[2][5],
        contents[2][7],
        contents[2][9],
    ]
    .into_iter()
    .map(|c| Amphipod::new(c).expect("Illegal input contents"))
    .collect::<Vec<_>>()
    .try_into()
    .unwrap();
    let row_2 = [
        contents[3][3],
        contents[3][5],
        contents[3][7],
        contents[3][9],
    ]
    .into_iter()
    .map(|c| Amphipod::new(c).expect("Illegal input contents"))
    .collect::<Vec<_>>()
    .try_into()
    .unwrap();
    let (res_01, res_02) = (ex_01(row_1, row_2), ex_02(row_1, row_2));
    println!(
        "Solution to challenge 1: {}!\n\
    Solution to challenge 2: {}!",
        res_01, res_02
    );
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Amphipod {
    A,
    B,
    C,
    D,
}

impl Amphipod {
    fn new(c: char) -> Result<Self, ()> {
        match c {
            'A' => Ok(A),
            'B' => Ok(B),
            'C' => Ok(C),
            'D' => Ok(D),
            _ => Err(()),
        }
    }
    fn energy(&self) -> u64 {
        match self {
            A => 1,
            B => 10,
            C => 100,
            D => 1000,
        }
    }
    fn entrance(&self) -> usize {
        match self {
            A => 2,
            B => 4,
            C => 6,
            D => 8,
        }
    }
}

impl Display for Amphipod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            A => 'A',
            B => 'B',
            C => 'C',
            D => 'D',
        };
        write!(f, "{}", c)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Burrow<const N: usize> {
    first: [Option<Amphipod>; N],
    second: [Option<Amphipod>; N],
    third: [Option<Amphipod>; N],
    fourth: [Option<Amphipod>; N],
    hallway: [Option<Amphipod>; 11],
}

impl<const N: usize> Display for Burrow<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "#############")?;
        write!(f, "#")?;
        for pod in self.hallway {
            match pod {
                Some(v) => write!(f, "{}", v)?,
                None => write!(f, ".")?,
            }
        }
        writeln!(f, "#")?;
        for i in 0..N {
            if i == 0 {
                write!(f, "###")?;
            } else {
                write!(f, "  #")?;
            }
            for pod in [self.first[i], self.second[i], self.third[i], self.fourth[i]] {
                match pod {
                    Some(v) => write!(f, "{}", v)?,
                    None => write!(f, ".")?,
                }
                write!(f, "#")?;
            }
            if i == 0 {
                write!(f, "##")?;
            }
            writeln!(f)?;
        }
        write!(f, "  #########")
    }
}

fn ex_01(row_1: [Amphipod; 4], row_2: [Amphipod; 4]) -> u64 {
    let mut burrow = Burrow {
        first: [Some(row_1[0]), Some(row_2[0])],
        second: [Some(row_1[1]), Some(row_2[1])],
        third: [Some(row_1[2]), Some(row_2[2])],
        fourth: [Some(row_1[3]), Some(row_2[3])],
        hallway: [None; 11],
    };
    best_solution(&mut burrow).unwrap()
}

fn ex_02(row_1: [Amphipod; 4], row_2: [Amphipod; 4]) -> u64 {
    let mut burrow = Burrow {
        first: [Some(row_1[0]), Some(D), Some(D), Some(row_2[0])],
        second: [Some(row_1[1]), Some(C), Some(B), Some(row_2[1])],
        third: [Some(row_1[2]), Some(B), Some(A), Some(row_2[2])],
        fourth: [Some(row_1[3]), Some(A), Some(C), Some(row_2[3])],
        hallway: [None; 11],
    };
    best_solution(&mut burrow).unwrap()
}

fn best_solution<const N: usize>(burrow: &mut Burrow<N>) -> Option<u64> {
    let mut energy = 0;
    let mut active = true;
    // move into side rooms if possible
    while active {
        active = false;
        for i in 0..burrow.hallway.len() {
            let pod = burrow.hallway[i];
            if pod.is_none() {
                continue;
            }
            let pod_ = pod.unwrap();
            let target = match pod_ {
                A => &mut burrow.first,
                B => &mut burrow.second,
                C => &mut burrow.third,
                D => &mut burrow.fourth,
            };
            if target[0].is_none()
                && target[1..]
                    .iter()
                    .map(|opt| opt.unwrap_or(pod_))
                    .all(|b| b == pod_)
            {
                if burrow.hallway[min(i + 1, pod_.entrance())
                    ..=max(i as i32 - 1, pod_.entrance() as i32) as usize]
                    .iter()
                    .any(|opt| opt.is_some())
                {
                    continue;
                }
                for i in 0..=target.len() {
                    energy += pod_.energy();
                    if i == target.len() || target[i].is_some() {
                        target[i - 1] = Some(pod_);
                        break;
                    }
                }
                active = true;
                let used = (max(i, pod_.entrance()) - min(i, pod_.entrance()) - 1)
                    * pod_.energy() as usize;
                energy += used as u64;
                burrow.hallway[i] = None;
            }
        }
    }
    let side_rooms = [burrow.first, burrow.second, burrow.third, burrow.fourth];
    if side_rooms
        .iter()
        .zip([A, B, C, D])
        .all(|(side, goal)| side.iter().all(|&opt| opt == Some(goal)))
    {
        return Some(energy);
    }
    let mut possible_moves = Vec::new();
    // generate all possible ways to move an amphipod out of a side room.
    for (mut side_room, goal) in side_rooms.into_iter().zip([A, B, C, D]) {
        let mut energy = energy;
        let should_move = !(side_room
            .iter()
            .map(|opt| opt.unwrap_or(goal))
            .all(|b| b == goal));
        if !should_move {
            continue;
        }
        for i in 0..side_room.len() {
            if let Some(v) = side_room[i] {
                side_room[i] = None;
                side_room[0] = Some(v);
                energy += v.energy() * i as u64;
                break;
            }
        }
        let energy_add = side_room[0].unwrap().energy();
        let start = goal.entrance();
        let (mut left_active, mut right_active) = (true, true);
        // for each positions in the hallway to move to (excluding in front of an entrance)
        for i in 0..9 {
            energy += energy_add;
            let left = start as i32 - i as i32;
            let right = start as i32 + i as i32;
            if left < 0 || burrow.hallway[left as usize].is_some() {
                left_active = false;
            }
            if right > 10 || burrow.hallway[right as usize].is_some() {
                right_active = false;
            }
            if !left_active && !right_active {
                break;
            }
            let mut burrow1 = burrow.clone();
            let mut modified_side_room = side_room;
            modified_side_room[0] = None;
            match goal {
                A => {
                    burrow1.first = modified_side_room;
                }
                B => {
                    burrow1.second = modified_side_room;
                }
                C => {
                    burrow1.third = modified_side_room;
                }
                D => {
                    burrow1.fourth = modified_side_room;
                }
            }
            let mut burrow2 = burrow1.clone();
            if left_active && ![2_i32, 4, 6, 8].contains(&left) {
                let mut hallway1 = burrow.hallway;
                hallway1[left as usize] = side_room[0];
                burrow1.hallway = hallway1;
                possible_moves.push((energy, burrow1));
            }
            if right_active && ![2_i32, 4, 6, 8].contains(&right) {
                let mut hallway2 = burrow.hallway;
                hallway2[right as usize] = side_room[0];
                burrow2.hallway = hallway2;
                possible_moves.push((energy, burrow2));
            }
        }
    }
    // find solution with minimum energy
    possible_moves
        .into_iter()
        .filter_map(|(energy, mut burrow)| {
            best_solution(&mut burrow).map(|energy_| energy_ + energy)
        })
        .min()
}
