use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/01.txt");

    let parsed = input.lines().map(parse).to_vec();
    let sums = parsed.windows(3).map(|w| w.iter().sum::<isize>()).to_vec();
    let increased = sums
        .iter()
        .zip(sums.iter().skip(1))
        .filter(|(a, b)| b > a)
        .count();

    pv!(increased);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/01.txt");

    let parsed = input.lines().map(parse).to_vec();
    let increased = parsed
        .iter()
        .zip(parsed.iter().skip(1))
        .filter(|(a, b)| b > a)
        .count();

    pv!(increased);
}
