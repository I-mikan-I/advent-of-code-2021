use std::iter::Sum;
use std::ops::Add;
use std::path::Path;

const BIT_SIZE: usize = 12;

/* ==========> BINARY TYPE <========== */

struct Binary {
    counts: [(usize, usize); BIT_SIZE],
}

impl Binary {
    fn gamma(&self) -> usize {
        self.counts.iter().rev().copied().fold(0, |gamma, count| {
            let bit = match count {
                (zeroes, ones) if zeroes > ones => 0,
                _ => 1,
            };
            let mut gamma = gamma << 1;
            gamma |= bit;
            gamma
        })
    }
    fn epsilon(&self) -> usize {
        Self::gamma(&Binary {
            counts: self
                .counts
                .iter()
                .copied()
                .map(|(l, r)| (r + if r == l { 1 } else { 0 }, l))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        })
    }
}

impl From<usize> for Binary {
    fn from(num: usize) -> Self {
        let mut num = num;
        let mut contents: [u8; BIT_SIZE] = [0; BIT_SIZE];
        for i in contents.iter_mut() {
            *i = (num & 1) as u8;
            num >>= 1;
        }
        let counts = contents
            .iter()
            .map(|&n| if n == 0 { (1, 0) } else { (0, 1) })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Self { counts }
    }
}
impl From<&usize> for Binary {
    fn from(num: &usize) -> Self {
        <Self as From<usize>>::from(*num)
    }
}

impl Add for Binary {
    type Output = Binary;

    fn add(self, rhs: Self) -> Self::Output {
        let counts_new = self
            .counts
            .into_iter()
            .zip(rhs.counts.into_iter())
            .map(|((lhs_z, lhs_o), (rhs_z, rhs_o))| (lhs_z + rhs_z, lhs_o + rhs_o))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Self { counts: counts_new }
    }
}

impl Sum for Binary {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|bin, bin2| bin + bin2).unwrap()
    }
}

/* ==========> CHALLENGE <========== */

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let path = Path::new(&path);
    let contents = std::fs::read_to_string(path).unwrap();
    let numbers: Vec<usize> = contents
        .split_whitespace()
        .map(|s| usize::from_str_radix(s, 2).unwrap())
        .collect();
    let result = ex_02(numbers.iter().copied());
    println!("result: {}", result)
}

fn ex_01(nums: impl Iterator<Item = usize>) -> usize {
    let result: Binary = nums.map(Into::into).sum();
    Binary::gamma(&result) * Binary::epsilon(&result)
}

fn ex_02(nums: impl Iterator<Item = usize>) -> usize {
    let copy: Vec<_> = nums.collect();
    let oxygen = oxygen(&copy);
    let scrubber = scrubber(&copy);
    println!("oxygen: {}, scrubber: {}", oxygen, scrubber);
    oxygen * scrubber
}

fn scrubber(nums: &[usize]) -> usize {
    filter_vitals(nums, Binary::epsilon)
}

fn oxygen(nums: &[usize]) -> usize {
    filter_vitals(nums, Binary::gamma)
}

fn filter_vitals(nums: &[usize], agg_filter: impl Fn(&Binary) -> usize) -> usize {
    let mut nums = Vec::from(nums);
    for i in (0..BIT_SIZE).rev() {
        let gamma = agg_filter(
            &nums
                .iter()
                .map(Into::into)
                .sum::<Binary>(),
        );
        let filter_bit = gamma & (1 << i);
        nums = nums
            .into_iter()
            .filter(|&num| (num & (1 << i)) ^ filter_bit == 0)
            .collect();
        if nums.len() == 1 {
            return nums[0];
        }
    }
    panic!("no number found")
}
