use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/15.txt");

    let input_grid = digit_grid(input);
    let mut grid = input_grid.clone();
    for y in 0..grid.h() {
        let mut row = &mut grid[y];
        for repeat in 1..5 {
            row.extend(input_grid[y].iter().map(|x| (x + repeat - 1) % 9 + 1));
        }
    }
    assert_eq!(grid.w(), input_grid.w() * 5);
    let input_grid = grid.clone();
    for repeat in 1..5 {
        grid.extend(
            input_grid
                .iter()
                .map(|row| row.iter().map(|x| (x + repeat - 1) % 9 + 1).to_vec()),
        );
    }
    assert_eq!(grid.h(), input_grid.h() * 5);

    let goal = (grid.w() - 1, grid.h() - 1);
    let neighborhood = grid.manhattan();
    let path = a_star_search(
        |pos, out| out.extend(neighborhood.get_all_neighbors(pos).map(|p| (p, grid[p]))),
        (0, 0),
        goal,
        |p| neighborhood.heuristic(p, goal),
    )
    .unwrap();
    pv!(path.cost);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/15.txt");

    let mut grid = digit_grid(input);
    let goal = (grid.w() - 1, grid.h() - 1);
    let neighborhood = grid.manhattan();
    let path = a_star_search(
        |pos, out| out.extend(neighborhood.get_all_neighbors(pos).map(|p| (p, grid[p]))),
        (0, 0),
        goal,
        |p| neighborhood.heuristic(p, goal),
    )
    .unwrap();
    pv!(path.cost);
}
