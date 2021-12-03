use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/03.txt");

    let mut parsed = input
        .lines()
        .map(|s| usize::from_str_radix(s, 2).unwrap())
        .to_vec();

    let mut oxygen = 0;
    {
        let mut parsed = parsed.clone();
        for bit in (0..12).rev() {
            let mut count = 0;
            for i in parsed.iter() {
                if i & (1 << bit) != 0 {
                    count += 1;
                }
            }
            if count * 2 < parsed.len() {
                parsed.retain(|n| n & (1 << bit) == 0);
            } else {
                parsed.retain(|n| n & (1 << bit) != 0);
            }
            if parsed.len() == 1 {
                break;
            }
        }
        oxygen = parsed[0];
    }
    let mut co2 = 0;
    {
        let mut parsed = parsed.clone();
        for bit in (0..12).rev() {
            let mut count = 0;
            for i in parsed.iter() {
                if i & (1 << bit) != 0 {
                    count += 1;
                }
            }
            if count * 2 < parsed.len() {
                parsed.retain(|n| n & (1 << bit) != 0);
            } else {
                parsed.retain(|n| n & (1 << bit) == 0);
            }
            if parsed.len() == 1 {
                break;
            }
        }
        co2 = parsed[0];
    }
    let res = oxygen * co2;
    pv!(res);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/03.txt");

    let parsed = input
        .lines()
        .map(|s| usize::from_str_radix(s, 2).unwrap())
        .to_vec();
    let mut gamma = 0usize;
    let mut epsi = 0;
    for bit in 0..12 {
        let mut count = 0;
        for i in parsed.iter() {
            if i & (1 << bit) != 0 {
                count += 1;
            }
        }
        if count > parsed.len() / 2 {
            gamma |= 1 << bit;
        } else {
            epsi |= 1 << bit;
        }
    }
    let res = gamma * epsi;
    pv!(res);
}
