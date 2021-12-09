use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/09.txt");

    let grid = char_grid(input);
    let neighborhood = grid.manhattan();

    let mut low_points = vec![];
    for (pos, v) in grid.grid_iter_index() {
        if neighborhood.get_all_neighbors(pos).any(|p| grid[p] <= *v) {
            continue;
        }
        low_points.push(pos);
    }

    let mut basins = vec![];
    for low_point in low_points {
        let mut basin = HashSet::new();
        basin.insert(low_point);
        let mut queue = vec![low_point];
        while let Some(pos) = queue.pop() {
            let v = grid[pos];
            for neighbor in neighborhood.get_all_neighbors(pos) {
                let neigh = grid[neighbor];
                if neigh != '9' && neigh > v && basin.insert(neighbor) {
                    queue.push(neighbor);
                }
            }
        }
        basins.push(basin.len());
    }
    basins.sort_unstable_by(|a, b| a.cmp(b).reverse());
    let res = basins[..3].iter().product::<usize>();
    pv!(res);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/09.txt");
    let grid = char_grid(input);
    let neighborhood = grid.manhattan();

    let mut sum = 0;
    for (pos, v) in grid.grid_iter_index() {
        if neighborhood.get_all_neighbors(pos).any(|p| grid[p] <= *v) {
            continue;
        }
        sum += parse_c(*v) + 1;
    }
    pv!(sum);
}
