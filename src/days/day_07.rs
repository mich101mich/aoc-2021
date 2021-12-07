use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/07.txt");

    fn count_fuel(crabs: &[isize], target: isize) -> isize {
        crabs
            .iter()
            .map(|i| (i - target).abs())
            .map(|n| n * (n + 1) / 2)
            .sum::<isize>()
    }
    let parsed = comma_values::<isize>(input);
    let mut target = parsed.iter().sum::<isize>() / parsed.len() as isize;
    let mut cost = count_fuel(&parsed, target);
    loop {
        let lower = count_fuel(&parsed, target - 1);
        let upper = count_fuel(&parsed, target + 1);
        if lower < cost {
            target -= 1;
            cost = lower;
        } else if upper < cost {
            target += 1;
            cost = upper;
        } else {
            break;
        }
    }
    pv!(target, cost);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/07.txt");

    fn count_fuel(crabs: &[isize], target: isize) -> isize {
        crabs.iter().map(|i| (i - target).abs()).sum::<isize>()
    }
    let parsed = comma_values::<isize>(input);
    let mut target = parsed.iter().sum::<isize>() / parsed.len() as isize;
    let mut cost = count_fuel(&parsed, target);
    loop {
        let lower = count_fuel(&parsed, target - 1);
        let upper = count_fuel(&parsed, target + 1);
        if lower < cost {
            target -= 1;
            cost = lower;
        } else if upper < cost {
            target += 1;
            cost = upper;
        } else {
            break;
        }
    }
    pv!(target, cost);
}
