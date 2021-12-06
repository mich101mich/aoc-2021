use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/06.txt");

    let parsed = comma_values::<usize>(input);

    let days = 256;
    let mut fishes = vec![0; days + 10];

    for &fish in &parsed {
        fishes[fish] += 1;
    }

    let mut count = parsed.len();
    for day in 0..days {
        let n = fishes[day];
        for d in (day..days).step_by(7) {
            fishes[d + 9] += n;
            count += n;
        }
    }

    pv!(count);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/06.txt");

    let parsed = comma_values::<isize>(input);

    let days = 80;
    let mut new_fish = vec![];
    for &fish in &parsed {
        let created = (days - fish + 6) / 7;
        for f in 0..created {
            new_fish.push(fish + f * 7 + 2);
        }
    }

    let mut count = parsed.len();
    while let Some(fish) = new_fish.pop() {
        count += 1;
        let created = (days - fish + 6) / 7;
        for f in 1..created {
            new_fish.push(fish + f * 7 + 2);
        }
    }
    pv!(count);
}
