use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Neg, Sub};
use std::path::Path;
use std::str::FromStr;

fn main() {
    let path = if let Some(x) = std::env::args().nth(1) {
        x
    } else {
        "2021/day-19/input-test".to_string()
    };
    let path = Path::new(&path);
    let contents = std::fs::read_to_string(path).unwrap();
    let mut scanners: Vec<_> = contents
        .split("\n\n")
        .map(|str| Scanner::from_str(str).unwrap())
        .collect();
    let mut found = vec![scanners.remove(0)];

    while !scanners.is_empty() {
        let mut to_add = Vec::new();
        'outer: for s0 in &mut found {
            for s1 in 0..scanners.len() {
                if s0.orient(scanners.get_mut(s1).unwrap()) {
                    let tmp = scanners.remove(s1);
                    println!("============> FIXATED <===========\n{}\n{:?}", &tmp.name, &tmp.offset);
                    to_add.push(tmp);
                    break 'outer;
                }
            }
        }
        found.append(&mut to_add);
    }
    let complete = found
        .iter_mut()
        .reduce(|agg, left| {
            agg.merge(left);
            agg
        })
        .unwrap();
    println!("beacon count: {}.", complete.view.len());
    let max_distance = found
        .iter()
        .map(|scanner| scanner.offset)
        .permutations(2)
        .map(|v| v[0].manhattan(&v[1]))
        .max()
        .unwrap();
    println!("max distance: {}.", max_distance);
}

type Alignment = (i32, i32, i32);

const ALIGNMENTS: [Alignment; 24] = [
    (1, 2, 3),
    (1, -3, 2),
    (1, -2, -3),
    (1, 3, -2),
    (-1, -2, 3),
    (-1, -3, -2),
    (-1, 2, -3),
    (-1, 3, 2),
    (2, -1, 3),
    (2, -3, -1),
    (2, 1, -3),
    (2, 3, 1),
    (-2, 1, 3),
    (-2, -3, 1),
    (-2, -1, -3),
    (-2, 3, -1),
    (3, 2, -1),
    (3, 1, 2),
    (3, -2, 1),
    (3, -1, -2),
    (-3, 2, 1),
    (-3, -1, 2),
    (-3, -2, -1),
    (-3, 1, -2),
];

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Beacon {
    x: i32,
    y: i32,
    z: i32,
}

impl Beacon {
    fn manhattan(&self, other: &Beacon) -> usize {
        [self.x - other.x, self.y - other.y, self.z - other.z]
            .into_iter()
            .map(i32::abs)
            .map(|n| n as usize)
            .sum()
    }
}

impl Neg for Beacon {
    type Output = Beacon;

    fn neg(self) -> Self::Output {
        Beacon {
            x: self.x.neg(),
            y: self.y.neg(),
            z: self.z.neg(),
        }
    }
}

impl<'a> Neg for &'a Beacon {
    type Output = Beacon;

    fn neg(self) -> Self::Output {
        Beacon {
            x: self.x.neg(),
            y: self.y.neg(),
            z: self.z.neg(),
        }
    }
}

impl Sub for Beacon {
    type Output = Beacon;

    fn sub(self, rhs: Self) -> Self::Output {
        Beacon {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<'a> Sub for &'a Beacon {
    type Output = Beacon;

    fn sub(self, rhs: Self) -> Self::Output {
        Beacon {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Add for Beacon {
    type Output = Beacon;
    fn add(self, rhs: Self) -> Self::Output {
        Beacon {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<'a> Add for &'a Beacon {
    type Output = Beacon;
    fn add(self, rhs: Self) -> Self::Output {
        Beacon {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl FromStr for Beacon {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [x, y, z]: [i32; 3] = s
            .trim()
            .split(',')
            .map(i32::from_str)
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .map_err(|_| "wrong coordinate length.")?;
        Ok(Self { x, y, z })
    }
}

#[derive(Debug, Clone)]
struct Scanner {
    name: String,
    beacons: Vec<Beacon>,
    view: Vec<Beacon>,
    alignment: Alignment,
    offset: Beacon,
}

impl Scanner {
    fn align(&mut self, to: Alignment) {
        self.alignment = to;
        self.view = self
            .beacons
            .iter()
            .map(|beacon| {
                let mut x = match self.alignment.0 {
                    -1 | 1 => beacon.x,
                    -2 | 2 => beacon.y,
                    -3 | 3 => beacon.z,
                    _ => panic!("unknown alignment."),
                };
                x *= self.alignment.0 / self.alignment.0.abs();
                let mut y = match self.alignment.1 {
                    -1 | 1 => beacon.x,
                    -2 | 2 => beacon.y,
                    -3 | 3 => beacon.z,
                    _ => panic!("unknown alignment."),
                };
                y *= self.alignment.1 / self.alignment.1.abs();
                let mut z = match self.alignment.2 {
                    -1 | 1 => beacon.x,
                    -2 | 2 => beacon.y,
                    -3 | 3 => beacon.z,
                    _ => panic!("unknown alignment."),
                };
                z *= self.alignment.2 / self.alignment.2.abs();
                Beacon { x, y, z }
            })
            .collect()
    }

    fn orient(&mut self, other: &mut Scanner) -> bool {
        for align in ALIGNMENTS {
            other.align(align);
            for beacon in &self.view {
                for beacon_o in &other.view {
                    let ref_vec_self: Vec<_> = self.view.iter().map(|b| b.sub(beacon)).collect();
                    #[allow(clippy::needless_collect)]
                    let ref_vec_other: Vec<_> =
                        other.view.iter().map(|b| b.sub(beacon_o)).collect();

                    let mut matches = 0;
                    for ref_beacon in &ref_vec_self {
                        if ref_vec_other.contains(ref_beacon) {
                            matches += 1;
                        }
                        if matches >= 12 {
                            other.offset = *beacon + self.offset - *beacon_o;
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
    fn merge(&mut self, other: &Scanner) {
        for beacon in &other.view {
            let beacon = beacon + &other.offset;
            if !self.beacons.contains(&beacon) {
                self.beacons.push(beacon);
            }
            self.align(self.alignment);
        }
    }
}

impl Display for Scanner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "--- {} ---", self.name)?;
        writeln!(f, "offset: {:?}", self.offset)?;
        for beacon in &self.view {
            writeln!(f, "{},{},{}", beacon.x, beacon.y, beacon.z)?;
        }
        writeln!(f,)
    }
}

impl FromStr for Scanner {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.lines().map(str::trim);
        let name = iter
            .next()
            .ok_or("no name provided.")?
            .strip_prefix("--- ")
            .ok_or("no prefix.")?
            .strip_suffix(" ---")
            .ok_or("no suffix.")?;
        let beacons = iter
            .map(|beacon| beacon.parse())
            .collect::<Result<Vec<Beacon>, _>>()?;
        let mut result = Self {
            name: name.to_string(),
            beacons,
            view: Vec::new(),
            alignment: ALIGNMENTS[0],
            offset: Beacon { x: 0, y: 0, z: 0 },
        };
        result.align(ALIGNMENTS[0]);
        Ok(result)
    }
}
