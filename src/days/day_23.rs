use crate::utils::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Amphipod {
    pos: Point,
    name: usize,
}

#[allow(unused)]
fn draw_state(pods: &[Amphipod], grid: &Grid<bool>) {
    let positions = pods.iter().map(|p| (p.pos, p.name)).to_map();
    for y in 0..grid.h() {
        for x in 0..grid.w() {
            if let Some(&name) = positions.get(&(x, y)) {
                print!("{}", (b'A' + name as u8) as char);
            } else {
                print!("{}", if grid[(x, y)] { '#' } else { ' ' });
            }
        }
        println!();
    }
    println!();
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/23.txt");

    let input = {
        let mut lines = input.lines().to_vec();
        lines.insert(3, "  #D#C#B#A#");
        lines.insert(4, "  #D#B#A#C#");
        lines.join("\n")
    };

    let energy = [1, 10, 100, 1000];

    let mut grid = hashtag_grid(&input);
    let neigh = grid.manhattan();
    let (w, h) = grid.size();
    grid.iter_mut().for_each(|row| row.resize(w, false));

    let mut letters = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| {
                ('A'..='D').contains(&c).then(|| {
                    (Amphipod {
                        pos: (x, y),
                        name: (c as u8 - b'A') as usize,
                    })
                })
            })
        })
        .to_vec();
    letters.sort_by(|a, b| a.name.cmp(&b.name));
    let mut pods = [Amphipod::default(); 16];
    for (p, l) in pods.iter_mut().zip(letters.iter()) {
        *p = *l;
    }

    let hallway_y = 1;
    let room_x = letters.iter().map(|p| p.pos.0).to_set();
    let all_empty = grid
        .grid_iter_index()
        .filter(|(_, v)| !*v)
        .map(|(p, _)| p)
        .to_vec();

    let mut goal = [Amphipod::default(); 16];
    let mut target_room_x = room_x.iter().copied().to_vec();
    target_room_x.sort_unstable();

    let paths = a_star_search(
        |pods, out| {
            let positions = pods.iter().map(|p| p.pos).to_set();

            let mut room_pods = HashMap::new();
            for p in pods {
                if p.pos.1 > hallway_y {
                    room_pods
                        .entry(p.pos.0)
                        .or_insert_with(HashSet::new)
                        .insert(p.name);
                }
            }

            if positions.iter().all(|p| p.1 > hallway_y) && room_pods.values().all(|r| r.len() == 1)
            {
                out.push((goal, 0));
                return;
            }

            for (i, Amphipod { pos, name }) in pods.iter().copied().enumerate() {
                let in_room = pos.1 > hallway_y;
                let target_room = target_room_x[name];
                if in_room && pos.0 == target_room && room_pods[&target_room].len() == 1 {
                    continue;
                }
                let pod_cost = energy[name];
                let mut reachable = dijkstra_search(
                    |pos, out| {
                        out.extend(
                            neigh
                                .get_all_neighbors(pos)
                                .filter(|p| !grid[p])
                                .filter(|p| !positions.contains(p)),
                        );
                    },
                    pos,
                    &all_empty,
                );
                for (next_pos, path) in reachable {
                    if path.cost == 0 {
                        continue;
                    }
                    let mut exclusive = false;
                    if next_pos.1 == hallway_y {
                        if !in_room || room_x.contains(&next_pos.0) {
                            continue;
                        }
                    } else {
                        if next_pos.0 != target_room {
                            continue;
                        }
                        if let Some(other) = room_pods.get(&next_pos.0) {
                            if other.iter().any(|&n| n != name) {
                                continue;
                            }
                            if (next_pos.1 + 1..=hallway_y + 4)
                                .any(|y| !positions.contains(&(next_pos.0, y)))
                            {
                                // empty spot below us
                                continue;
                            }
                        } else if next_pos.1 != hallway_y + 4 {
                            continue;
                        }
                        // can move into a room
                        out.clear();
                        exclusive = true;
                    }
                    let cost = path.cost * pod_cost;
                    let mut new_state = pods;
                    new_state[i].pos = next_pos;

                    out.push((new_state, cost));
                    if exclusive {
                        return;
                    }
                }
            }
        },
        pods,
        goal,
        |p| {
            p.iter()
                .filter(|p| p.pos.0 != target_room_x[p.name])
                .map(|p| energy[p.name] * 2)
                .sum()
        },
    );

    let path = paths.unwrap();
    pv!(path.cost);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/23.txt");

    let energy = [1, 10, 100, 1000];

    let mut grid = hashtag_grid(input);
    let neigh = grid.manhattan();
    let (w, h) = grid.size();
    grid.iter_mut().for_each(|row| row.resize(w, false));

    let mut letters = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| {
                ('A'..='D').contains(&c).then(|| {
                    (Amphipod {
                        pos: (x, y),
                        name: (c as u8 - b'A') as usize,
                    })
                })
            })
        })
        .to_vec();
    letters.sort_by(|a, b| a.name.cmp(&b.name));
    let mut pods = [Amphipod::default(); 8];
    for (p, l) in pods.iter_mut().zip(letters.iter()) {
        *p = *l;
    }

    let room_x = letters.iter().map(|p| p.pos.0).to_set();
    let room_y = {
        let set = letters.iter().map(|p| p.pos.1).to_set();
        let mut iter = set.iter();
        [*set.iter().min().unwrap(), *set.iter().max().unwrap()]
    };
    let hallway_y = 1;
    let all_empty = grid
        .grid_iter_index()
        .filter(|(_, v)| !*v)
        .map(|(p, _)| p)
        .to_vec();
    let mut target_room_x = room_x.iter().copied().to_vec();
    target_room_x.sort_unstable();

    let mut goal = [Amphipod::default(); 8];

    let path = a_star_search(
        |pods, out| {
            let positions = pods.iter().map(|p| p.pos).to_set();

            let mut room_pods = HashMap::new();
            for p in pods {
                if p.pos.1 > hallway_y {
                    room_pods
                        .entry(p.pos.0)
                        .or_insert_with(HashSet::new)
                        .insert(p.name);
                }
            }

            if positions.iter().all(|p| p.1 > hallway_y) && room_pods.values().all(|r| r.len() == 1)
            {
                out.push((goal, 0));
                return;
            }

            for (i, Amphipod { pos, name }) in pods.iter().copied().enumerate() {
                let in_room = pos.1 > hallway_y;
                let target_room = target_room_x[name];
                if in_room && pos.0 == target_room && room_pods[&target_room].len() == 1 {
                    continue;
                }
                let pod_cost = energy[name];
                let mut reachable = dijkstra_search(
                    |pos, out| {
                        out.extend(
                            neigh
                                .get_all_neighbors(pos)
                                .filter(|p| !grid[p])
                                .filter(|p| !positions.contains(p)),
                        );
                    },
                    pos,
                    &all_empty,
                );
                for (next_pos, path) in reachable {
                    if path.cost == 0 {
                        continue;
                    }
                    let mut exclusive = false;
                    if next_pos.1 == hallway_y {
                        if !in_room || room_x.contains(&next_pos.0) {
                            continue;
                        }
                    } else {
                        if next_pos.0 != target_room {
                            continue;
                        }
                        if let Some(other) = room_pods.get(&next_pos.0) {
                            if other.iter().any(|&n| n != name) {
                                continue;
                            }
                        } else if next_pos.1 != hallway_y + 2 {
                            continue;
                        }
                        // can move into a room
                        out.clear();
                        exclusive = true;
                    }
                    let cost = path.cost * pod_cost;
                    let mut new_state = pods;
                    new_state[i].pos = next_pos;

                    out.push((new_state, cost));
                    if exclusive {
                        return;
                    }
                }
            }
        },
        pods,
        goal,
        |p| {
            p.iter()
                .filter(|p| p.pos.0 != target_room_x[p.name])
                .map(|p| energy[p.name] * 2)
                .sum()
        },
    );

    let path = path.unwrap();
    pv!(path.cost);
}
