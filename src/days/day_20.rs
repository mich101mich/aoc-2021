use crate::utils::*;

#[rustfmt::skip]
const MOORE: [(isize, isize); 9] = [(-1, -1), (0, -1), (1, -1), (-1, 0), (0, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/20.txt");

    let mut grid = hashtag_grid(input);
    grid.remove(0);
    grid.remove(0);
    let (w, h) = grid.size();

    let iterations = 50;

    let mut grid: Grid<bool> = std::iter::repeat_with(|| vec![false; w])
        .take(iterations + 1)
        .chain(grid.iter().cloned())
        .chain(std::iter::repeat_with(|| vec![false; w]).take(iterations + 1))
        .to_vec()
        .into();
    grid.iter_mut().for_each(|row| {
        *row = std::iter::repeat(false)
            .take(iterations + 1)
            .chain(row.iter().copied())
            .chain(std::iter::repeat(false).take(iterations + 1))
            .collect();
    });
    let (w, h) = grid.size();
    let neigh = grid.moore();

    let lut = hashtag_line(input.lines().next().unwrap());

    let mut border = false;

    let mut new_grid = grid.clone();

    for _ in 0..iterations {
        new_grid.grid_iter_mut_index().for_each(|((x, y), v)| {
            let n = MOORE
                .iter()
                .map(|(dx, dy)| (x as isize + dx, y as isize + dy))
                .map(|(x, y)| {
                    if x < 0 || y < 0 || x == w as isize || y == h as isize {
                        border
                    } else {
                        grid[y as usize][x as usize]
                    }
                })
                .fold(0, |acc, x| (acc << 1) | (x as usize));
            *v = lut[n];
        });
        std::mem::swap(&mut grid, &mut new_grid);

        border = if border { lut[511] } else { lut[0] };
    }

    let count = grid.count();
    pv!(count);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/20.txt");

    let mut grid = hashtag_grid(input);
    grid.remove(0);
    grid.remove(0);
    let (w, h) = grid.size();
    grid.insert(0, vec![false; w]);
    grid.insert(0, vec![false; w]);
    grid.insert(0, vec![false; w]);
    grid.extend(vec![vec![false; w]; 3]);
    grid.iter_mut().for_each(|row| {
        row.insert(0, false);
        row.insert(0, false);
        row.insert(0, false);
        row.extend(vec![false; 3]);
    });
    let (w, h) = grid.size();
    let neigh = grid.moore();

    let lut = hashtag_line(input.lines().next().unwrap());

    let mut border = false;

    let mut new_grid = grid.clone();

    for _ in 0..2 {
        new_grid.grid_iter_mut_index().for_each(|((x, y), v)| {
            let n = MOORE
                .iter()
                .map(|(dx, dy)| (x as isize + dx, y as isize + dy))
                .map(|(x, y)| {
                    if x < 0 || y < 0 || x == w as isize || y == h as isize {
                        border
                    } else {
                        grid[y as usize][x as usize]
                    }
                })
                .fold(0, |acc, x| (acc << 1) | (x as usize));
            *v = lut[n];
        });
        std::mem::swap(&mut grid, &mut new_grid);

        border = if border { lut[511] } else { lut[0] };
    }

    let count = grid.count();
    pv!(count);
}
