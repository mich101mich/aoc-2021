use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/25.txt");

    let w = input.lines().next().unwrap().len();
    let h = input.lines().count();

    let mut cucumbers = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .map(move |(x, c)| ((x, y), Dir::from(c)))
        })
        .to_map();

    for step in 1.. {
        let mut moved = 0;
        let mut new_cumbers = HashMap::new();
        for (p, d) in cucumbers.iter() {
            if *d == Dir::Right {
                let mut new_p = *p + *d;
                new_p.0 %= w;
                new_p.1 %= h;
                if !cucumbers.contains_key(&new_p) {
                    new_cumbers.insert(new_p, *d);
                    moved += 1;
                } else {
                    new_cumbers.insert(*p, *d);
                }
            } else {
                new_cumbers.insert(*p, *d);
            }
        }
        cucumbers = std::mem::take(&mut new_cumbers);
        for (p, d) in cucumbers.iter() {
            if *d == Dir::Down {
                let mut new_p = *p + *d;
                new_p.0 %= w;
                new_p.1 %= h;
                if !cucumbers.contains_key(&new_p) {
                    new_cumbers.insert(new_p, *d);
                    moved += 1;
                } else {
                    new_cumbers.insert(*p, *d);
                }
            } else {
                new_cumbers.insert(*p, *d);
            }
        }
        cucumbers = new_cumbers;

        if moved == 0 {
            pv!(step);
            break;
        }
    }
}
