use crate::utils::*;

fn is_empty(r: (isize, isize)) -> bool {
    r.1 < r.0
}
fn len(r: (isize, isize)) -> usize {
    let len = r.1 - r.0 + 1;
    len.max(0) as usize
}

#[derive(Debug, Clone, Copy)]
struct Cube {
    x: (isize, isize),
    y: (isize, isize),
    z: (isize, isize),
}
impl Cube {
    fn new(x: (isize, isize), y: (isize, isize), z: (isize, isize)) -> Self {
        Cube { x, y, z }
    }
    fn is_empty(&self) -> bool {
        is_empty(self.x) || is_empty(self.y) || is_empty(self.z)
    }
    fn len(&self) -> usize {
        len(self.x) * len(self.y) * len(self.z)
    }
    fn intersection(&self, other: &Self) -> Option<Self> {
        let ret = Cube::new(
            (self.x.0.max(other.x.0), self.x.1.min(other.x.1)),
            (self.y.0.max(other.y.0), self.y.1.min(other.y.1)),
            (self.z.0.max(other.z.0), self.z.1.min(other.z.1)),
        );
        (!ret.is_empty()).then(|| ret)
    }
    fn intersects(&self, other: &Self) -> bool {
        self.x.0 <= other.x.1
            && self.x.1 >= other.x.0
            && self.y.0 <= other.y.1
            && self.y.1 >= other.y.0
            && self.z.0 <= other.z.1
            && self.z.1 >= other.z.0
    }
    fn shatter_into(&self, other: &Self, out: &mut Vec<Cube>) {
        let (mut x, mut y, mut z) = (self.x, self.y, self.z);

        let pre_x = (self.x.0, other.x.0 - 1);
        if !is_empty(pre_x) {
            out.push(Cube::new(pre_x, y, z));
            x.0 = other.x.0;
        }
        let post_x = (other.x.1 + 1, self.x.1);
        if !is_empty(post_x) {
            out.push(Cube::new(post_x, y, z));
            x.1 = other.x.1;
        }

        let pre_y = (self.y.0, other.y.0 - 1);
        if !is_empty(pre_y) {
            out.push(Cube::new(x, pre_y, z));
            y.0 = other.y.0;
        }
        let post_y = (other.y.1 + 1, self.y.1);
        if !is_empty(post_y) {
            out.push(Cube::new(x, post_y, z));
            y.1 = other.y.1;
        }

        let pre_z = (self.z.0, other.z.0 - 1);
        if !is_empty(pre_z) {
            out.push(Cube::new(x, y, pre_z));
            z.0 = other.z.0;
        }
        let post_z = (other.z.1 + 1, self.z.1);
        if !is_empty(post_z) {
            out.push(Cube::new(x, y, post_z));
            z.1 = other.z.1;
        }
    }
}
impl RegexRepresentation for Cube {
    const REGEX: &'static str = r"x=-?\d+..-?\d+,y=-?\d+..-?\d+,z=-?\d+..-?\d+";
}
impl std::str::FromStr for Cube {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = scanf!(
            s,
            "x={}..{},y={}..{},z={}..{}",
            isize,
            isize,
            isize,
            isize,
            isize,
            isize
        )
        .unwrap();
        Ok(Cube::new(
            (caps.0, caps.1),
            (caps.2, caps.3),
            (caps.4, caps.5),
        ))
    }
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/22.txt");

    let parsed = input
        .lines()
        .map(|l| {
            scanf!(
                l,
                "{} x={}..{},y={}..{},z={}..{}",
                String,
                isize,
                isize,
                isize,
                isize,
                isize,
                isize
            )
            .unwrap()
        })
        .map(|(on_off, x0, x1, y0, y1, z0, z1)| {
            (on_off == "on", Cube::new((x0, x1), (y0, y1), (z0, z1)))
        })
        .to_vec();

    let mut cubes = vec![];
    let mut new_cubes = vec![];
    for (on, cube) in parsed.iter().cloned() {
        for other in cubes.drain(..) {
            if cube.intersects(&other) {
                other.shatter_into(&cube, &mut new_cubes);
            } else {
                new_cubes.push(other);
            }
        }
        if on {
            new_cubes.push(cube);
        }
        std::mem::swap(&mut cubes, &mut new_cubes);
    }
    let count = cubes.iter().map(Cube::len).sum::<usize>();
    pv!(count);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/22.txt");

    let area = Cube::new((-50, 50), (-50, 50), (-50, 50));
    let parsed = input
        .lines()
        .map(|l| scanf!(l, "{} {}", String, Cube).unwrap())
        .filter_map(|(on_off, cube)| cube.intersection(&area).map(|c| (on_off == "on", c)))
        .to_vec();

    let mut cubes = HashSet::new();
    for (on, cube) in parsed.iter().copied() {
        for x in cube.x.0..=cube.x.1 {
            for y in cube.y.0..=cube.y.1 {
                for z in cube.z.0..=cube.z.1 {
                    if on {
                        cubes.insert((x, y, z));
                    } else {
                        cubes.remove(&(x, y, z));
                    }
                }
            }
        }
    }
    pv!(cubes.len());
}
