use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/13.txt");

    let mut iter = input.lines();
    let points = iter
        .by_ref()
        .take_while(|s| !s.is_empty())
        .map(|l| scanf!(l, "{},{}", usize, usize).unwrap())
        .to_vec();

    let (mut w, mut h) = points
        .iter()
        .fold((0, 0), |(w, h), (x, y)| (w.max(*x + 1), h.max(*y + 1)));

    let mut grid = Grid::new_clone((w, h), false);
    for p in points {
        grid[p] = true;
    }

    for (axis, pos) in iter.filter_map(|s| scanf!(s, "fold along {}={}", char, usize)) {
        if axis == 'x' {
            for y in 0..h {
                for x in 0..pos {
                    let v = grid[(w - 1 - x, y)];
                    let target = &mut grid[(x, y)];
                    *target = *target || v;
                }
            }
            w = pos;
        } else {
            for x in 0..w {
                for y in 0..pos {
                    let v = grid[(x, h - 1 - y)];
                    let target = &mut grid[(x, y)];
                    *target = *target || v;
                }
            }
            h = pos;
        }
        grid.trim_to((w, h));
    }
    grid.print('#', ' ');
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/13.txt");

    let mut iter = input.lines();
    let points = iter
        .by_ref()
        .take_while(|s| !s.is_empty())
        .map(|l| scanf!(l, "{},{}", usize, usize).unwrap())
        .to_vec();

    let (mut w, mut h) = points
        .iter()
        .fold((0, 0), |(w, h), (x, y)| (w.max(*x + 1), h.max(*y + 1)));

    let mut grid = Grid::new_clone((w, h), false);
    for p in points {
        grid[p] = true;
    }

    for (axis, pos) in iter.take(1).filter_map(|s| scanf!(s, "fold along {}={}", char, usize)) {
        if axis == 'x' {
            for y in 0..h {
                for x in 0..pos {
                    let v = grid[(w - 1 - x, y)];
                    let target = &mut grid[(x, y)];
                    *target = *target || v;
                }
            }
            w = pos;
        } else {
            for x in 0..w {
                for y in 0..pos {
                    let v = grid[(x, h - 1 - y)];
                    let target = &mut grid[(x, y)];
                    *target = *target || v;
                }
            }
            h = pos;
        }
        grid.trim_to((w, h));
    }
    let count = grid.count();
    pv!(count);
}
