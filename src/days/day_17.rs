use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/17.txt");

    let (x0, x1, y0, y1) = scanf!(
        input,
        "target area: x={}..{}, y={}..{}",
        isize,
        isize,
        isize,
        isize
    )
    .unwrap();

    let x_range = x0..=x1;
    let y_range = y0..=y1;

    let min_x = (0..x0)
        .take_while(|x| x * (x + 1) / 2 <= x0)
        .last()
        .unwrap();

    let mut found = 0;
    'x_loop: for x in min_x..=x1 {
        let mut final_y = 100_000;
        let mut y = y0;
        'y_loop: while y <= final_y {
            let mut pos = (0, 0);
            let mut vel = (x, y);
            loop {
                pos = (pos.0 + vel.0, pos.1 + vel.1);
                if vel.0 > 0 {
                    vel.0 -= 1;
                }
                vel.1 -= 1;
                if x_range.contains(&pos.0) && y_range.contains(&pos.1) {
                    found += 1;
                    break;
                } else if vel.0 == 0 && pos.0 < x0 {
                    // x isn't enough
                    continue 'x_loop;
                } else if pos.1 < y0 {
                    if vel.0 == 0 && final_y == 100_000 {
                        final_y = y * y;
                    }
                    break;
                } else if pos.0 > x1 {
                    // overshot
                    break 'y_loop;
                }
            }
            y += 1;
        }
    }
    pv!(found);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/17.txt");

    let (x0, x1, y0, y1) = scanf!(
        input,
        "target area: x={}..{}, y={}..{}",
        isize,
        isize,
        isize,
        isize
    )
    .unwrap();

    let x_range = x0..=x1;
    let y_range = y0..=y1;

    let min_x = (0..x0)
        .take_while(|x| x * (x + 1) / 2 <= x0)
        .last()
        .unwrap();

    let mut max_max_y = 0;
    'x_loop: for x in min_x..=x1 {
        let mut final_y = 100_000;
        let mut y = 0;
        'y_loop: while y <= final_y {
            let mut pos = (0, 0);
            let mut vel = (x, y);
            let mut max_y = max_max_y;
            loop {
                pos = (pos.0 + vel.0, pos.1 + vel.1);
                if vel.0 > 0 {
                    vel.0 -= 1;
                }
                vel.1 -= 1;
                if pos.1 > max_y {
                    max_y = pos.1;
                }
                if x_range.contains(&pos.0) && y_range.contains(&pos.1) {
                    if max_y > max_max_y {
                        max_max_y = max_y;
                    }
                    break;
                } else if vel.0 == 0 && pos.0 < x0 {
                    // x isn't enough
                    continue 'x_loop;
                } else if pos.1 < y0 {
                    if vel.0 == 0 && final_y == 100_000 {
                        final_y = y * y;
                    }
                    break;
                } else if pos.0 > x1 {
                    // overshot
                    break 'y_loop;
                }
            }
            y += 1;
        }
    }
    pv!(max_max_y);
}
