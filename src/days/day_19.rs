use crate::utils::*;

use cgmath::prelude::*;
type Vec3 = cgmath::Vector3<isize>;
type Mat3 = cgmath::Matrix3<f32>;
type Pose = (Vec3, Mat3);

fn manhattan(a: Vec3, b: Vec3) -> isize {
    (a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs()
}
fn moore(a: Vec3, b: Vec3) -> isize {
    (a.x - b.x)
        .abs()
        .max((a.y - b.y).abs())
        .max((a.z - b.z).abs())
}
fn i2f(v: Vec3) -> cgmath::Vector3<f32> {
    cgmath::Vector3::new(v.x as f32, v.y as f32, v.z as f32)
}
fn f2i(v: cgmath::Vector3<f32>) -> Vec3 {
    Vec3::new(v.x as isize, v.y as isize, v.z as isize)
}
fn get_all_rotations() -> Vec<Mat3> {
    let mut all_rotations = vec![];
    let sign_axis_combos = (0..3).flat_map(|i| [(i, -1.0), (i, 1.0)]);
    for (x_axis, x_sign) in sign_axis_combos.clone() {
        let mut x = cgmath::Vector3::zero();
        x[x_axis] = x_sign;
        for (y_axis, y_sign) in sign_axis_combos.clone() {
            if y_axis == x_axis {
                continue;
            }
            let mut y = cgmath::Vector3::zero();
            y[y_axis] = y_sign;
            let z = x.cross(y);
            all_rotations.push(Mat3::from_cols(x, y, z));
        }
    }
    all_rotations
}

#[derive(Debug, Clone)]
struct Scanner {
    pose: Option<Pose>,
    beacons: Vec<Vec3>,
    dist: HashMap<isize, Vec<(usize, usize)>>,
}
impl Scanner {
    fn set_pose(&mut self, pose: Pose) {
        self.pose = Some(pose);
        let pos = i2f(pose.0);
        for b in self.beacons.iter_mut() {
            *b = f2i(pose.1 * i2f(*b) + pos);
        }
    }
}

fn find_matchup(
    a: &Scanner,
    b: &Scanner,
    known_beacons: &HashSet<Vec3>,
    all_rotations: &[Mat3],
) -> Option<Pose> {
    let overlap = a
        .dist
        .iter()
        .filter(|(d, _)| b.dist.contains_key(d))
        .map(|(_, v)| v.len())
        .sum::<usize>();
    if overlap < 12 {
        return None;
    }

    let pos_b = b.pose?.0;

    let iter = a
        .dist
        .iter()
        .filter(|(_, v)| v.len() == 1)
        .filter_map(|(d, pa)| b.dist.get(d).map(|pb| (d, pa, pb)))
        .filter(|(_, _, pb)| pb.len() == 1);

    for matchup in iter {
        let (a1, a2) = matchup.1[0];
        let (b1, b2) = matchup.2[0];
        let pa1 = i2f(a.beacons[a1]);
        let pa2 = i2f(a.beacons[a2]);
        let pb1 = i2f(b.beacons[b1]);
        let pb2 = i2f(b.beacons[b2]);
        for (pb1, pb2) in [(pb1, pb2), (pb2, pb1)] {
            'outer: for rot in all_rotations {
                let pa1_trans = rot * pa1;
                let pos = pb1 - pa1_trans;
                let check_pb2 = pos + rot * pa2;
                if check_pb2 != pb2 {
                    continue;
                }
                for p in a.beacons.iter() {
                    let p = f2i(rot * i2f(*p) + pos);
                    if !known_beacons.contains(&p) && moore(p, pos_b) <= 1000 {
                        continue 'outer;
                    }
                }
                pv!(overlap);
                return Some((f2i(pos), *rot));
            }
        }
    }
    None
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/19.txt");

    let mut iter = input.lines();
    let mut scanners = vec![];
    while iter.next().is_some() {
        let beacons = iter
            .by_ref()
            .take_while(|l| !l.is_empty())
            .map(|l| scanf!(l, "{},{},{}", isize, isize, isize).unwrap())
            .map(|(x, y, z)| Vec3::new(x, y, z))
            .to_vec();

        let mut dist = HashMap::new();
        for (i, p) in beacons.iter().enumerate() {
            for (j, q) in beacons.iter().enumerate().skip(i + 1) {
                let d = manhattan(*p, *q);
                dist.entry(d).or_insert_with(Vec::new).push((i, j));
            }
        }

        scanners.push(Scanner {
            pose: None,
            beacons,
            dist,
        });
    }

    scanners[0].set_pose((Vec3::zero(), Mat3::identity()));
    let mut known_beacons = HashSet::new();
    for b in scanners[0].beacons.iter() {
        known_beacons.insert(*b);
    }

    let all_rotations = get_all_rotations();

    let mut found = vec![];
    let mut remaining = scanners.iter_mut().collect::<VecDeque<_>>();
    while let Some(scanner) = remaining.pop_front() {
        if scanner.pose.is_some() {
            found.push(scanner);
            continue;
        }
        if let Some(pose) = found
            .iter()
            .find_map(|other| find_matchup(scanner, other, &known_beacons, &all_rotations))
        {
            scanner.set_pose(pose);
            known_beacons.extend(&scanner.beacons);
            found.push(scanner);
        } else {
            remaining.push_back(scanner);
        }
    }
    let distance = scanners
        .iter()
        .enumerate()
        .flat_map(|(i, s)| {
            let pos = s.pose.unwrap().0;
            scanners[i + 1..]
                .iter()
                .map(move |s2| manhattan(s2.pose.unwrap().0, pos))
        })
        .max()
        .unwrap();
    pv!(distance);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/19.txt");

    let mut iter = input.lines();
    let mut scanners = vec![];
    while iter.next().is_some() {
        let beacons = iter
            .by_ref()
            .take_while(|l| !l.is_empty())
            .map(|l| scanf!(l, "{},{},{}", isize, isize, isize).unwrap())
            .map(|(x, y, z)| Vec3::new(x, y, z))
            .to_vec();

        let mut dist = HashMap::new();
        for (i, p) in beacons.iter().enumerate() {
            for (j, q) in beacons.iter().enumerate().skip(i + 1) {
                let d = manhattan(*p, *q);
                dist.entry(d).or_insert_with(Vec::new).push((i, j));
            }
        }

        scanners.push(Scanner {
            pose: None,
            beacons,
            dist,
        });
    }

    scanners[0].set_pose((Vec3::zero(), Mat3::identity()));
    let mut known_beacons = HashSet::new();
    for b in scanners[0].beacons.iter() {
        known_beacons.insert(*b);
    }

    let all_rotations = get_all_rotations();

    let mut found = vec![];
    let mut remaining = scanners.iter_mut().collect::<VecDeque<_>>();
    while let Some(scanner) = remaining.pop_front() {
        if scanner.pose.is_some() {
            found.push(scanner);
            continue;
        }
        if let Some(pose) = found
            .iter()
            .find_map(|other| find_matchup(scanner, other, &known_beacons, &all_rotations))
        {
            scanner.set_pose(pose);
            known_beacons.extend(&scanner.beacons);
            found.push(scanner);
        } else {
            remaining.push_back(scanner);
        }
    }
    pv!(known_beacons.len());
}
