use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/04.txt");

    let mut iter = input.lines();

    let sequence = comma_values::<isize>(iter.next().unwrap());

    let mut cards = vec![];

    while iter.next().is_some() {
        let card = number_grid_whitespace(iter.by_ref().take(5));
        cards.push(card);
    }

    let mut last_score = 0;
    for n in sequence {
        let mut remove = vec![];
        for (i, card) in cards.iter_mut().enumerate() {
            let mut found = None;
            for (p, v) in card.grid_iter_mut_index() {
                if *v == n {
                    *v = -1;
                    found = Some(p);
                    break;
                }
            }
            if let Some((x, y)) = found {
                if card.row(y).all(|&v| v == -1) || card.col(x).all(|&v| v == -1) {
                    let mut score = card.grid_iter().filter(|v| **v != -1).sum::<isize>();
                    score *= n;
                    last_score = score;

                    remove.push(i);
                }
            }
        }
        for i in remove.into_iter().rev() {
            cards.remove(i);
        }
        if cards.is_empty() {
            break;
        }
    }
    pv!(last_score);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/04.txt");

    let mut iter = input.lines();

    let sequence = comma_values::<isize>(iter.next().unwrap());

    let mut cards = vec![];

    while iter.next().is_some() {
        let card = number_grid_whitespace(iter.by_ref().take(5));
        cards.push(card);
    }

    for n in sequence {
        for card in &mut cards {
            let mut found = None;
            for (p, v) in card.grid_iter_mut_index() {
                if *v == n {
                    *v = -1;
                    found = Some(p);
                    break;
                }
            }
            if let Some((x, y)) = found {
                if card.row(y).all(|&v| v == -1) || card.col(x).all(|&v| v == -1) {
                    let mut score = card.grid_iter().filter(|v| **v != -1).sum::<isize>();
                    score *= n;
                    pv!(score);
                    return;
                }
            }
        }
    }
}
