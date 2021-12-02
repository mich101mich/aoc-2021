use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/02.txt");

    let parsed = input
        .lines()
        .map(|l| scanf!(l, "{} {}", String, isize).unwrap())
        .map(|(s, n)| (Dir::from_str(&s).unwrap_or(Dir::Right), n));

    let mut pos = (0, 0isize);
    let mut aim = 0;
    for (d, n) in parsed {
        match d {
            Dir::Down => aim += n,
            Dir::Up => aim -= n,
            Dir::Right => {
                pos.1 += n;
                pos.0 += n * aim;
            }
            _ => panic!(),
        }
    }
    pv!(pos.0 * pos.1);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/02.txt");

    let parsed = input
        .lines()
        .map(|l| scanf!(l, "{} {}", String, isize).unwrap())
        .map(|(s, n)| (Dir::from_str(&s).unwrap_or(Dir::Right), n));

    let mut pos = (0, 0isize);
    for (d, n) in parsed {
        for _ in 0..n {
            pos += d;
        }
    }
    pv!(pos.0 * pos.1);
}
