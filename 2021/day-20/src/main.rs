use std::path::Path;

fn main() {
    let path = if let Some(x) = std::env::args().nth(1) {
        x
    } else {
        "2021/day-20/input".to_string()
    };
    let path = Path::new(&path);
    let contents = std::fs::read_to_string(path).unwrap();
    let res_1 = challenge(&contents, 2);
    let res_2 = challenge(&contents, 50);
    println!(
        "Solution to part 1: {}!\n\
    Solution to part 2: {}!",
        res_1, res_2
    );
}

fn generate_row(k: usize, row: &[u8], rim: u8) -> [u8; 3] {
    match k {
        k if k >= 1 && k <= row.len() - 2 => row[k - 1..=k + 1].try_into().unwrap(),
        k if k >= 1 => [row[k - 1], row[k], rim],
        k if k <= row.len() - 2 => [rim, row[k], row[k + 1]],
        _ => panic!(),
    }
}

fn challenge(input: &str, iterations: usize) -> usize {
    let mut lines = input.lines().peekable();
    let decoder: Vec<u8> = lines
        .next()
        .unwrap()
        .chars()
        .map(|char| match char {
            '.' => 0,
            '#' => 1,
            _ => panic!(),
        })
        .collect::<Vec<_>>();
    lines.next();
    let cols = lines.peek().unwrap().len();
    let mut image = Vec::with_capacity(iterations * 2);
    image.resize(iterations, vec![0; iterations * 2 + cols]);
    for (i, line) in lines.enumerate() {
        image.push(vec![0; iterations * 2 + cols]);
        for (k, v) in line.chars().enumerate() {
            image[i + iterations][k + iterations] = match v {
                '.' => 0,
                '#' => 1,
                _ => panic!(),
            };
        }
    }
    image.resize(image.len() + iterations, vec![0; iterations * 2 + cols]);
    let mut image_new = image.clone();
    let mut rim = 0;
    let (mut i, mut k);
    let (mut ib, mut kb) = (image.len() - iterations, image[0].len() - iterations);
    for n in 1..=iterations {
        let begin = iterations - n;
        image[begin].fill(rim);
        image[ib].fill(rim);
        i = begin;
        k = i;
        for row in &mut image {
            row[k] = rim;
            row[kb] = rim;
        }
        while i <= ib {
            while k <= kb {
                let row1: [u8; 3] = generate_row(k + n - iterations, &image[i][begin..kb + 1], rim);
                let row0 = if i > begin {
                    generate_row(k + n - iterations, &image[i - 1][begin..kb + 1], rim)
                } else {
                    [rim, rim, rim]
                };
                let row2 = if i < image.len() - 1 - begin {
                    generate_row(k + n - iterations, &image[i + 1][begin..kb + 1], rim)
                } else {
                    [rim, rim, rim]
                };
                let index = generate_index([row0, row1, row2].into_iter().flatten());
                image_new[i][k] = decoder[index];
                k += 1;
            }
            k = iterations - n;
            i += 1;
        }
        ib += 1;
        kb += 1;
        rim = decoder[generate_index([rim; 9].into_iter())];
        std::mem::swap(&mut image, &mut image_new);
    }
    image
        .into_iter()
        .flatten()
        .map(|byte| byte as usize)
        .sum::<usize>()
}

fn generate_index(iter: impl Iterator<Item = u8>) -> usize {
    iter.fold(0_usize, |agg, new| {
        let mut agg = agg << 1;
        agg += new as usize;
        agg
    })
}
