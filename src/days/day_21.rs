use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/21.txt");

    let parsed = input
        .lines()
        .map(|l| scanf!(l, "Player {} starting position: {}", usize, usize).unwrap())
        .to_vec();

    let mut rolls = vec![0; 10];
    for a in 1..=3 {
        for b in 1..=3 {
            for c in 1..=3 {
                rolls[a + b + c] += 1;
            }
        }
    }
    let rolls = rolls
        .into_iter()
        .enumerate()
        .filter(|(_, v)| *v > 0)
        .to_vec();
    assert_eq!(rolls.iter().map(|(_, n)| *n).sum::<usize>(), 27);

    let mut states = vec![HashMap::new(); 21];
    states[0].insert((parsed[0].1, parsed[1].1, 0), 1usize);

    let mut first_wins = 0;
    let mut second_wins = 0;

    for score_a in 0..21 {
        for ((a, b, score_b), n) in std::mem::take(&mut states[score_a]) {
            for (roll_a, roll_a_n) in rolls.iter() {
                let a = (a + roll_a - 1) % 10 + 1;
                let score_a = score_a + a;
                let n = n * roll_a_n;
                if score_a >= 21 {
                    first_wins += n;
                    continue;
                }
                for (roll_b, roll_b_n) in rolls.iter() {
                    let b = (b + roll_b - 1) % 10 + 1;
                    let score_b = score_b + b;
                    let n = n * roll_b_n;
                    if score_b >= 21 {
                        second_wins += n;
                        continue;
                    }
                    *states[score_a].entry((a, b, score_b)).or_insert(0) += n;
                }
            }
        }
    }
    pv!(first_wins.max(second_wins));
    // 462067600041672 low
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/21.txt");

    let parsed = input
        .lines()
        .map(|l| scanf!(l, "Player {} starting position: {}", usize, usize).unwrap())
        .to_vec();
    let mut a = parsed[0].1;
    let mut b = parsed[1].1;

    let mut score_a = 0;
    let mut score_b = 0;

    let mut dice = 1;
    let mut rolled = 0;
    let mut roll = || {
        let ret = dice;
        dice = dice % 100 + 1;
        rolled += 1;
        ret
    };
    while score_a < 1000 && score_b < 1000 {
        a += roll() + roll() + roll();
        a = (a - 1) % 10 + 1;
        score_a += a;
        if score_a >= 1000 {
            break;
        }

        b += roll() + roll() + roll();
        b = (b - 1) % 10 + 1;
        score_b += b;
    }
    let final_score = if score_a >= 1000 {
        score_b * rolled
    } else {
        score_a * rolled
    };
    pv!(final_score);
}
