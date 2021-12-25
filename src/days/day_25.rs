use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/25.txt");

    let w = input.lines().next().unwrap().len();
    let h = input.lines().count();

    let mut east = HashSet::new();
    let mut south = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '>' {
                east.insert((x, y));
            } else if c == 'v' {
                south.insert((x, y));
            }
        }
    }

    let mut new_east = HashSet::new();
    let mut new_south = HashSet::new();
    for step in 1.. {
        let mut moved = 0;
        new_east.clear();
        for p in east.iter() {
            let mut new_p = *p + Dir::Right;
            new_p.0 %= w;
            new_p.1 %= h;
            if !east.contains(&new_p) && !south.contains(&new_p) {
                new_east.insert(new_p);
                moved += 1;
            } else {
                new_east.insert(*p);
            }
        }
        std::mem::swap(&mut east, &mut new_east);

        new_south.clear();
        for p in south.iter() {
            let mut new_p = *p + Dir::Down;
            new_p.0 %= w;
            new_p.1 %= h;
            if !south.contains(&new_p) && !east.contains(&new_p) {
                new_south.insert(new_p);
                moved += 1;
            } else {
                new_south.insert(*p);
            }
        }
        std::mem::swap(&mut south, &mut new_south);

        if moved == 0 {
            pv!(step);
            break;
        }
    }
}
