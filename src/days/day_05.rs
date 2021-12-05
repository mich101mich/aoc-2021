use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/05.txt");

    let parsed = input
        .lines()
        .map(|l| scanf!(l, "{},{} -> {},{}", isize, isize, isize, isize).unwrap())
        .to_vec();

    let mut grid = Grid::new_clone((1000, 1000), 0usize);

    for (x1, y1, x2, y2) in parsed {
        if x1 == x2 {
            for y in y1.min(y2)..=y1.max(y2) {
                grid[(x1, y)] += 1;
            }
        } else if y1 == y2 {
            for x in x1.min(x2)..=x1.max(x2) {
                grid[(x, y1)] += 1;
            }
        } else {
            let x_dir = (x2 - x1).signum();
            let y_dir = (y2 - y1).signum();
            let diff = (y2 - y1).abs();
            for d in 0..=diff {
                grid[(x1 + x_dir * d, y1 + y_dir * d)] += 1;
            }
        }
    }
    let count = grid.grid_iter().filter(|v| **v >= 2).count();
    pv!(count);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/05.txt");

    let parsed = input
        .lines()
        .map(|l| scanf!(l, "{},{} -> {},{}", isize, isize, isize, isize).unwrap())
        .to_vec();

    let mut grid = Grid::new_clone((1000, 1000), 0usize);

    for (x1, y1, x2, y2) in parsed {
        if x1 == x2 {
            for y in y1.min(y2)..=y1.max(y2) {
                grid[(x1, y)] += 1;
            }
        } else if y1 == y2 {
            for x in x1.min(x2)..=x1.max(x2) {
                grid[(x, y1)] += 1;
            }
        }
    }
    let count = grid.grid_iter().filter(|v| **v >= 2).count();
    pv!(count);
}
