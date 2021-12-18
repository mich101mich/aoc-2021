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

    let min_x = (0..x0).find(|x| x * (x + 1) / 2 >= x0).unwrap(); // we have to at least reach x0
    let max_x = x1; // overshooting in the first step is pointless
    let min_y = y0; // undershooting in the first step is pointless
    let max_y = y0.abs(); // if y > 0 then it will always hit exactly 0 at some point.
                          // the next step after that is the starting value + 1, so avoid
                          // overshooting in that step.

    let mut found = 0;
    'x_loop: for x in min_x..=max_x {
        'y_loop: for y in min_y..=max_y {
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
                } else if pos.1 < y0 {
                    // already below target area
                    // either y isn't enough => increase y
                    // or we just missed it => try next y
                    break;
                } else if pos.0 > x1 {
                    // overshot
                    if pos.1 > y1 {
                        // hasn't fallen enough to reach target => y is too big
                        break 'y_loop;
                    } else {
                        continue 'y_loop;
                    }
                }
            }
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

    let min_x = (0..x0).find(|x| x * (x + 1) / 2 >= x0).unwrap(); // we have to at least reach x0
    let max_x = x1; // overshooting in the first step is pointless
    let min_y = 0; // we want to go as high as possible
    let max_y = y0.abs(); // if y > 0 then it will always hit exactly 0 at some point.
                          // the next step after that is the starting value + 1, so avoid
                          // overshooting in that step.

    let mut highest_highest_y = 0;
    'x_loop: for x in min_x..=max_x {
        'y_loop: for y in min_y..=max_y {
            let mut pos = (0, 0);
            let mut vel = (x, y);
            let mut highest_y = highest_highest_y;
            loop {
                pos = (pos.0 + vel.0, pos.1 + vel.1);
                if vel.0 > 0 {
                    vel.0 -= 1;
                }
                vel.1 -= 1;
                if pos.1 > highest_y {
                    highest_y = pos.1;
                }
                if x_range.contains(&pos.0) && y_range.contains(&pos.1) {
                    if highest_y > highest_highest_y {
                        highest_highest_y = highest_y;
                    }
                    break;
                } else if pos.1 < y0 {
                    // already below target area
                    // either y isn't enough => increase y
                    // or we just missed it => try next y
                    break;
                } else if pos.0 > x1 {
                    // overshot
                    if pos.1 > y1 {
                        // hasn't fallen enough to reach target => y is too big
                        break 'y_loop;
                    } else {
                        continue 'y_loop;
                    }
                }
            }
        }
    }
    pv!(highest_highest_y);
}
