use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, i32};
use nom::combinator::map_res;
use nom::multi::{many1, many_m_n};
use nom::sequence::tuple;
use nom::{Finish, IResult};
use std::cmp::{max, min};
use std::path::Path;

macro_rules! ws {
    ($t:expr) => {
        nom::sequence::delimited(
            nom::character::complete::multispace0,
            $t,
            nom::character::complete::multispace0,
        )
    };
}

fn main() {
    let path = if let Some(x) = std::env::args().nth(1) {
        x
    } else {
        "2021/day-22/input-test".to_string()
    };
    let path = Path::new(&path);
    let contents = std::fs::read_to_string(path).unwrap();
    let cuboids = many1(ws!(parse_cuboid))(&contents[..])
        .finish()
        .unwrap_or_else(|e: nom::error::Error<&str>| panic!("{}", e))
        .1;
    let res_01 = ex_01(cuboids.clone());
    let res_02 = ex_02(cuboids);
    println!(
        "Solution to day 1: {}!\n\
    Solution to day 2: {}!",
        res_01, res_02
    );
}

type CuboidRange = (i32, i32);
#[derive(Debug, Copy, Clone)]
struct Cuboid {
    on: bool,
    x: CuboidRange,
    y: CuboidRange,
    z: CuboidRange,
}

impl Cuboid {
    fn split(self, other: &Cuboid) -> Vec<Cuboid> {
        let begin = vec![self];
        if !self.touches(other) {
            return begin;
        }
        let access_mut: [fn(&mut Cuboid) -> &mut CuboidRange; 3] =
            [|c| &mut c.x, |c| &mut c.y, |c| &mut c.z];
        let access_imut: [fn(&Cuboid) -> CuboidRange; 3] = [|c| c.x, |c| c.y, |c| c.z];

        access_mut
            .into_iter()
            .zip(access_imut.into_iter())
            .fold(begin, |agg, (m, im)| {
                agg.into_iter()
                    .flat_map(|mut part| {
                        if im(other).0 >= im(&part).0 && im(other).0 <= im(&part).1 {
                            let mut child = part;
                            m(&mut part).1 = im(other).0 - 1;
                            m(&mut child).0 = im(&part).1 + 1;
                            [Some(part), Some(child)]
                        } else {
                            [Some(part), None]
                        }
                    })
                    .flatten()
                    .flat_map(|mut part| {
                        if im(other).1 >= im(&part).0 && im(other).1 <= im(&part).1 {
                            let mut child = part;
                            m(&mut part).1 = im(other).1;
                            m(&mut child).0 = im(&part).1 + 1;
                            [Some(part), Some(child)]
                        } else {
                            [Some(part), None]
                        }
                    })
                    .flatten()
                    .collect()
            })
    }
    fn contains(&self, other: &Cuboid) -> bool {
        [self.x, self.y, self.z]
            .into_iter()
            .zip([other.x, other.y, other.z].into_iter())
            .all(|(s, o)| o.0 >= s.0 && o.1 <= s.1)
    }
    fn touches(&self, other: &Cuboid) -> bool {
        [self.x, self.y, self.z]
            .into_iter()
            .zip([other.x, other.y, other.z].into_iter())
            .all(|(s, o)| o.0 <= s.1 && o.1 >= s.0)
    }
}

fn parse_range(cord: char) -> impl Fn(&str) -> IResult<&str, CuboidRange> {
    move |input| {
        let (input, _) = char(cord)(input)?;
        let (input, _) = char('=')(input)?;
        let (input, (fst, _, snd)) = tuple((i32, tag(".."), i32))(input)?;
        let (input, _) = many_m_n(0, 1, char(','))(input)?;
        Ok((input, (min(fst, snd), max(fst, snd))))
    }
}

fn parse_cuboid(input: &str) -> IResult<&str, Cuboid> {
    let (input, on) = ws!(map_res(alt((tag("on"), tag("off"))), |result| {
        std::result::Result::<_, ()>::Ok(result == "on")
    }))(input)?;
    let (input, x) = parse_range('x')(input)?;
    let (input, y) = parse_range('y')(input)?;
    let (input, z) = parse_range('z')(input)?;
    Ok((input, Cuboid { on, x, y, z }))
}

fn ex_01(mut cuboids: Vec<Cuboid>) -> usize {
    cuboids.reverse();
    let mut reactor = [[[false; 101]; 101]; 101];
    for z in 0..reactor.len() {
        for y in 0..reactor.len() {
            for x in 0..reactor.len() {
                let x_ = x as i32 - 50;
                let y_ = y as i32 - 50;
                let z_ = z as i32 - 50;
                reactor[z][y][x] = cuboids
                    .iter()
                    .filter(|cuboid| {
                        cuboid.x.0 <= x_
                            && cuboid.x.1 >= x_
                            && cuboid.y.0 <= y_
                            && cuboid.y.1 >= y_
                            && cuboid.z.0 <= z_
                            && cuboid.z.1 >= z_
                    })
                    .map(|cuboid| cuboid.on)
                    .next()
                    .unwrap_or(false);
            }
        }
    }
    reactor.iter().flatten().flatten().filter(|v| **v).count()
}

fn ex_02(cuboids: Vec<Cuboid>) -> u128 {
    let active = cuboids
        .into_iter()
        .fold(Vec::new(), |vec, cuboid| add(cuboid, vec));
    active
        .into_iter()
        .map(|cb| {
            (cb.x.1 - cb.x.0 + 1) as u128
                * (cb.y.1 - cb.y.0 + 1) as u128
                * (cb.z.1 - cb.z.0 + 1) as u128
        })
        .sum::<u128>()
}

fn add(new: Cuboid, others: Vec<Cuboid>) -> Vec<Cuboid> {
    let to_split = others;
    let with = new;
    let mut result: Vec<_> = to_split
        .into_iter()
        .flat_map(|ts| ts.split(&with))
        .filter(|child| !(child.x.1 < child.x.0 || child.y.1 < child.y.0 || child.z.1 < child.z.0))
        .filter(|child| !new.contains(child))
        .collect();
    if new.on {
        result.push(new);
    }
    result
}
