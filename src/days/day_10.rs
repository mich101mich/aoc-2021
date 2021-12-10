use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/10.txt");

    let mut parsed = input
        .lines()
        .filter_map(|l| {
            let mut stack = vec![];
            for c in l.chars() {
                if matches!(c, '(' | '[' | '{' | '<') {
                    stack.push(c);
                } else {
                    let open = stack.pop().unwrap_or('_');
                    match c {
                        ')' if open != '(' => return None,
                        ']' if open != '[' => return None,
                        '}' if open != '{' => return None,
                        '>' if open != '<' => return None,
                        _ => {}
                    }
                }
            }
            if stack.is_empty() {
                None
            } else {
                let mut score = 0usize;
                for c in stack.iter().rev() {
                    score *= 5;
                    score += match c {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => unreachable!(),
                    };
                }
                Some(score)
            }
        })
        .to_vec();
    parsed.sort_unstable();
    pv!(parsed[parsed.len() / 2]);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/10.txt");

    let parsed = input
        .lines()
        .map(|l| {
            let mut stack = vec![];
            for c in l.chars() {
                if matches!(c, '(' | '[' | '{' | '<') {
                    stack.push(c);
                } else {
                    let open = stack.pop().unwrap_or('_');
                    match c {
                        ')' if open != '(' => return 3,
                        ']' if open != '[' => return 57,
                        '}' if open != '{' => return 1197,
                        '>' if open != '<' => return 25137,
                        _ => {}
                    }
                }
            }
            0
        })
        .sum::<usize>();
    pv!(parsed);
}
