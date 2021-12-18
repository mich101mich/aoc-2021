use crate::utils::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Open,
    Close,
    Number(usize),
}
use Token::*;
impl Token {
    fn is_number(&self) -> bool {
        matches!(self, Number(_))
    }
}
impl From<char> for Token {
    fn from(c: char) -> Self {
        match c {
            '[' => Open,
            ']' => Close,
            '0'..='9' => Number(c.to_digit(10).unwrap() as usize),
            _ => panic!("Invalid token: {}", c),
        }
    }
}

fn split(parsed: &mut Vec<Token>, i: usize, n: usize) {
    parsed[i] = Close;
    parsed.insert(i, Number(n - n / 2));
    parsed.insert(i, Number(n / 2));
    parsed.insert(i, Open);
}
#[allow(unused)]
fn print_tokens(message: &str, parsed: &[Token]) {
    print!("{:16}", message);
    for t in parsed.iter() {
        match t {
            Open => print!("["),
            Close => print!("]"),
            Number(n) => print!("{},", n),
        }
    }
    println!();
}

fn reduce(parsed: &mut Vec<Token>) {
    let mut change = true;
    while change {
        change = false;
        let mut level = 0;
        let mut explode = None;
        for (i, w) in parsed.windows(4).enumerate() {
            if let [Open, Number(a), Number(b), Close] = w {
                // pair
                if level >= 4 {
                    explode = Some((i, *a, *b));
                    break;
                } else {
                    level += 1;
                }
            } else if w[0] == Open {
                level += 1;
            } else if w[0] == Close {
                level -= 1;
            }
        }
        if let Some((i, a, b)) = explode {
            parsed[i] = Number(0);
            parsed.remove(i + 1);
            parsed.remove(i + 1);
            parsed.remove(i + 1);
            if let Some(Number(next)) = parsed[i + 1..].iter_mut().find(|t| t.is_number()) {
                *next += b;
            }
            if let Some(Number(prev)) = parsed[..i].iter_mut().rev().find(|t| t.is_number()) {
                *prev += a;
            }
            // print_tokens("after explode:", parsed);
            change = true;
            continue;
        }
        if let Some((i, n)) = parsed.iter().enumerate().find_map(|(i, t)| match t {
            Number(n) if *n >= 10 => Some((i, *n)),
            _ => None,
        }) {
            split(parsed, i, n);
            // print_tokens("after split:", parsed);
            change = true;
        }
    }
}
fn magnitute(iter: &mut impl Iterator<Item = Token>) -> usize {
    match iter.next().unwrap() {
        Open => {
            let a = magnitute(iter);
            let b = magnitute(iter);
            assert_eq!(iter.next(), Some(Close));
            a * 3 + b * 2
        }
        Number(n) => n,
        t => panic!("Invalid starting token: {:?}", t),
    }
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/18.txt");

    let parsed = input
        .lines()
        .map(|l| l.chars().filter(|c| *c != ',').map(Token::from).to_vec())
        .to_vec();

    let max = parsed
        .par_iter()
        .enumerate()
        .map(|(i, a)| {
            let mut total = vec![Open];
            total.extend(a);
            let mut max = 0;

            for (j, b) in parsed.iter().enumerate() {
                if i == j {
                    continue;
                }
                let mut total = total.clone();
                total.extend(b);
                total.push(Close);
                reduce(&mut total);
                let m = magnitute(&mut total.into_iter());
                if m > max {
                    max = m;
                }
            }
            max
        })
        .max()
        .unwrap();
    pv!(max);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/18.txt");

    let mut total = vec![];
    for line in input.lines() {
        let mut parsed = line.chars().filter(|c| *c != ',').map(Token::from).to_vec();
        if total.is_empty() {
            total = parsed;
        } else {
            total.insert(0, Open);
            total.extend(parsed);
            total.push(Close);
        }
        reduce(&mut total);
    }
    let result = magnitute(&mut total.into_iter());
    pv!(result);
}
