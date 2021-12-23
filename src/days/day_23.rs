use crate::utils::*;

const ENERGY: [usize; 4] = [1, 10, 100, 1000];

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
    let mut grid = hashtag_grid(&input);
    let (w, h) = grid.size();
    grid.iter_mut().for_each(|row| row.resize(w, false));

    const HALLWAY_POS: [Point; 7] = [(1, 1), (2, 1), (4, 1), (6, 1), (8, 1), (10, 1), (11, 1)];
    const HALLWAY_Y: usize = 1;
    const ROOM_POS_X: [usize; 4] = [3, 5, 7, 9];
    const ROOM_POS_Y: [usize; 4] = [5, 4, 3, 2];

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    struct State {
        hallway: [Option<usize>; 7],
        rooms: [[Option<usize>; 4]; 4],
    }
    impl State {
        fn can_walk(&self, start: usize, end: usize) -> bool {
            let range = start.min(end) + 1..start.max(end);
            self.hallway
                .iter()
                .zip(HALLWAY_POS.iter())
                .all(|(h, (x, _))| h.is_none() || !range.contains(x))
        }
        fn draw(&self, grid: &Grid<bool>) {
            let mut special = HashMap::new();
            for (hx, pod) in self.hallway.iter().enumerate() {
                if let Some(name) = pod {
                    special.insert(HALLWAY_POS[hx], *name);
                }
            }
            for (r, room) in self.rooms.iter().enumerate() {
                for (i, pod) in room.iter().enumerate() {
                    if let Some(name) = pod {
                        special.insert((ROOM_POS_X[r], ROOM_POS_Y[i]), *name);
                    }
                }
            }
            for y in 0..grid.h() {
                for x in 0..grid.w() {
                    if let Some(&name) = special.get(&(x, y)) {
                        print!("{}", (b'A' + name as u8) as char);
                    } else {
                        print!("{}", if grid[(x, y)] { '#' } else { ' ' });
                    }
                }
                println!();
            }
            println!();
        }
    }

    let mut start = State::default();
    for (i, line) in input.lines().skip(2).take(4).enumerate() {
        let line = &line[3..];
        let mut iter = line.split('#');
        for (c, r) in iter.zip(&mut start.rooms) {
            let c = c.chars().next().unwrap() as u8 - b'A';
            r[i] = Some(c as usize);
        }
    }
    start.rooms.iter_mut().for_each(|r| r.reverse());
    let mut goal = State::default();
    for (i, room) in goal.rooms.iter_mut().enumerate() {
        room.fill(Some(i));
    }

    let path = a_star_search(
        |state, out| {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
            struct Room {
                last: Option<usize>,
                last_i: usize,
                last_y: usize,
                next: usize,
                next_y: usize,
                dirty: bool,
                unaccessible: bool,
                x: usize,
            }
            let mut rooms = [Room::default(); 4];
            for ((r, src), dest) in state.rooms.iter().enumerate().zip(&mut rooms) {
                let full = src[3].is_some();
                let mut dirty = false;
                for (i, pod) in src.iter().enumerate() {
                    if let Some(pod) = *pod {
                        dest.last = Some(pod);
                        dest.last_i = i;
                        dest.last_y = ROOM_POS_Y[i];
                        dest.dirty = dest.dirty || pod != r;
                    } else {
                        dest.next = i;
                        dest.next_y = ROOM_POS_Y[i];
                        break;
                    }
                }
                dest.unaccessible = full || dest.dirty;
                dest.x = ROOM_POS_X[r];
            }

            // hallway -> room
            for (hx, pod) in state.hallway.iter().enumerate() {
                if let Some(pod) = *pod {
                    let target = rooms[pod];
                    if target.unaccessible {
                        continue;
                    }
                    let pos = HALLWAY_POS[hx];
                    if !state.can_walk(pos.0, target.x) {
                        continue;
                    }
                    let mut new_state = state;
                    new_state.hallway[hx] = None;
                    new_state.rooms[pod][target.next] = Some(pod);
                    let cost = manhattan(pos, (target.x, target.next_y));
                    out.push((new_state, cost * ENERGY[pod]));
                    return;
                }
            }

            // room -> room
            for (i, room) in rooms.iter().enumerate() {
                let pod = match room.last {
                    Some(pod) if pod != i => pod,
                    _ => continue,
                };
                let target = rooms[pod];
                if target.unaccessible || !state.can_walk(room.x, target.x) {
                    continue;
                }
                let mut new_state = state;
                new_state.rooms[i][room.last_i] = None;
                new_state.rooms[pod][target.next] = Some(pod);
                let hallway_stop = (room.x, HALLWAY_Y);
                let cost = manhattan((room.x, room.last_y), hallway_stop)
                    + manhattan(hallway_stop, (target.x, target.next_y));
                out.push((new_state, cost * ENERGY[pod]));
                return;
            }

            // room -> hallway
            for (i, room) in rooms.iter().enumerate() {
                let pod = match room.last {
                    Some(pod) => pod,
                    _ => continue,
                };
                if pod != i && !room.dirty {
                    continue;
                }
                let targets_left = state.hallway[..=i + 1]
                    .iter()
                    .enumerate()
                    .rev()
                    .take_while(|(_, h)| h.is_none());
                let targets_right = state
                    .hallway
                    .iter()
                    .enumerate()
                    .skip(i + 2)
                    .take_while(|(_, h)| h.is_none());
                for (hx, _) in targets_left.chain(targets_right) {
                    let mut new_state = state;
                    new_state.hallway[hx] = Some(pod);
                    new_state.rooms[i][room.last_i] = None;
                    let cost = manhattan((room.x, room.last_y), HALLWAY_POS[hx]);
                    out.push((new_state, cost * ENERGY[pod]));
                }
            }
        },
        start,
        goal,
        |_| 0,
    );

    let path = path.unwrap();
    pv!(path.cost);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/23.txt");

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
                let pod_cost = ENERGY[name];
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
                .map(|p| ENERGY[p.name] * 2)
                .sum()
        },
    );

    let path = path.unwrap();
    pv!(path.cost);
}
