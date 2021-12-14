use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/14.txt");

    let mut iter = input.lines();
    let parsed = iter.next().unwrap().chars().to_vec();
    iter.next().unwrap();

    let formulas = iter
        .map(|l| scanf!(l, "{}{} -> {}", char, char, char).unwrap())
        .map(|(a, b, c)| ([a, b], c))
        .to_map();

    let mut all_chars = formulas.values().copied().to_set();

    let mut creates = formulas.keys().map(|k| (*k, HashMap::new())).to_map();

    for _ in 0..40 {
        creates = formulas
            .iter()
            .map(|(&[a, b], &c)| {
                let new_created = all_chars
                    .iter()
                    .map(|&target_c| {
                        let mut count = 0usize;
                        if c == target_c {
                            count += 1;
                        }
                        count += creates[&[a, c]].get(&target_c).unwrap_or(&0);
                        count += creates[&[c, b]].get(&target_c).unwrap_or(&0);
                        (target_c, count)
                    })
                    .to_map();
                ([a, b], new_created)
            })
            .to_map();
    }

    let mut counts = all_chars.iter().map(|c| (*c, 0usize)).to_map();
    let mut polymer = parsed.clone();
    for c in polymer.iter() {
        *counts.entry(*c).or_insert(0) += 1;
    }
    for w in polymer.windows(2) {
        for &target_c in all_chars.iter() {
            let count = counts.get_mut(&target_c).unwrap();
            *count += creates[w][&target_c];
        }
    }
    let max = counts.values().max().unwrap();
    let min = counts.values().min().unwrap();
    pv!(max - min);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/14.txt");

    let mut iter = input.lines();
    let mut polymer = iter.next().unwrap().chars().to_vec();
    iter.next().unwrap();

    let formulas = iter
        .map(|l| scanf!(l, "{}{} -> {}", char, char, char).unwrap())
        .map(|(a, b, c)| ([a, b], c))
        .to_map();

    let mut new_polymer = vec![];
    for _ in 0..10 {
        for w in polymer.windows(2) {
            new_polymer.push(w[0]);
            new_polymer.push(formulas[w]);
        }
        new_polymer.push(*polymer.last().unwrap());
        std::mem::swap(&mut polymer, &mut new_polymer);
        new_polymer.clear();
    }
    print_arr!(polymer);

    let mut count = HashMap::new();
    for c in polymer.iter() {
        *count.entry(c).or_insert(0) += 1;
    }
    let max = count.values().max().unwrap();
    let min = count.values().min().unwrap();
    pv!(max - min);
}
