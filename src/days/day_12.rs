use crate::utils::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cave {
    name: &'static str,
    big: bool,
}
impl Cave {
    fn new(name: &'static str) -> Self {
        let big = name.chars().next().unwrap().is_uppercase();
        Cave { name, big }
    }
    fn is_goal(&self) -> bool {
        self.name == "end"
    }
    fn is_special(&self) -> bool {
        matches!(self.name, "start" | "end")
    }
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/12.txt");

    let parsed = input.lines().map(|l| {
        let mut s = l.split('-');
        (s.next().unwrap(), s.next().unwrap())
    });

    let mut connected = HashMap::new();
    for (a, b) in parsed {
        let a = Cave::new(a);
        let b = Cave::new(b);
        connected.entry(a).or_insert_with(HashSet::new).insert(b);
        connected.entry(b).or_insert_with(HashSet::new).insert(a);
    }

    fn count_paths(
        pos: Cave,
        visited: &mut HashSet<Cave>,
        visited_twice: bool,
        connected: &HashMap<Cave, HashSet<Cave>>,
    ) -> usize {
        let mut paths = 0;
        for next in connected.get(&pos).unwrap() {
            if next.is_goal() {
                paths += 1;
                continue;
            }
            if next.big {
                paths += count_paths(*next, visited, visited_twice, connected);
            } else if !visited.contains(next) {
                visited.insert(*next);
                paths += count_paths(*next, visited, visited_twice, connected);
                visited.remove(next);
            } else if !visited_twice && !next.is_special() {
                paths += count_paths(*next, visited, true, connected);
            }
        }
        paths
    }

    let start = Cave::new("start");
    let mut visited = HashSet::new();
    visited.insert(start);
    let count = count_paths(start, &mut visited, false, &connected);
    pv!(count);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/12.txt");

    let parsed = input.lines().map(|l| {
        let mut s = l.split('-');
        (s.next().unwrap(), s.next().unwrap())
    });

    let mut connected = HashMap::new();
    for (a, b) in parsed {
        let a = Cave::new(a);
        let b = Cave::new(b);
        connected.entry(a).or_insert_with(HashSet::new).insert(b);
        connected.entry(b).or_insert_with(HashSet::new).insert(a);
    }

    fn count_paths(
        pos: Cave,
        visited: &mut HashSet<Cave>,
        connected: &HashMap<Cave, HashSet<Cave>>,
    ) -> usize {
        let mut paths = 0;
        for next in connected.get(&pos).unwrap() {
            if next.is_goal() {
                paths += 1;
                continue;
            }
            if next.big || !visited.contains(next) {
                let inserted = visited.insert(*next);
                paths += count_paths(*next, visited, connected);
                if inserted {
                    visited.remove(next);
                }
            }
        }
        paths
    }

    let start = Cave::new("start");
    let mut visited = HashSet::new();
    visited.insert(start);
    let count = count_paths(start, &mut visited, &connected);
    pv!(count);
}
