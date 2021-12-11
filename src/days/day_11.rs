use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/11.txt");

    let mut grid = digit_grid(input);
    let neighborhood = grid.moore();

    for step in 1.. {
        let mut count = 0;
        let mut increase = grid.grid_index_iter().to_vec();
        while let Some(pos) = increase.pop() {
            let v = &mut grid[pos];
            *v += 1;
            if *v == 10 {
                count += 1;
                increase.extend(neighborhood.get_all_neighbors(pos));
            }
        }
        grid.grid_iter_mut().for_each(|v| {
            if *v > 9 {
                *v = 0
            }
        });
        if count == 100 {
            pv!(step);
            break;
        }
    }
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/11.txt");

    let mut grid = digit_grid(input);
    let neighborhood = grid.moore();

    let mut count = 0;
    for _ in 0..100 {
        let mut increase = grid.grid_index_iter().to_vec();
        while let Some(pos) = increase.pop() {
            let v = &mut grid[pos];
            *v += 1;
            if *v == 10 {
                count += 1;
                increase.extend(neighborhood.get_all_neighbors(pos));
            }
        }
        grid.grid_iter_mut().for_each(|v| {
            if *v > 9 {
                *v = 0
            }
        });
    }
    pv!(count);
}
