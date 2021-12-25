use crate::utils::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Value {
    Variable(usize),
    Value(isize),
}
impl Value {
    fn get(&self, vars: &[isize; 4]) -> isize {
        match self {
            Value::Variable(c) => vars[*c],
            Value::Value(v) => *v,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Instruction {
    Inp(usize),
    Add(usize, Value),
    Mul(usize, Value),
    Div(usize, Value),
    Mod(usize, Value),
    Eql(usize, Value),
}
use Instruction::*;
impl Instruction {
    #[allow(unused)]
    fn apply(&self, vars: &mut [isize; 4], input: &mut impl Iterator<Item = isize>) {
        match *self {
            Inp(a) => vars[a] = input.next().unwrap(),
            Add(a, b) => vars[a] += b.get(vars),
            Mul(a, b) => vars[a] *= b.get(vars),
            Div(a, b) => vars[a] /= b.get(vars),
            Mod(a, b) => vars[a] %= b.get(vars),
            Eql(a, b) => vars[a] = (vars[a] == b.get(vars)) as isize,
        }
    }
}
impl std::str::FromStr for Instruction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        let op = iter.next().ok_or("no op")?;
        let a = iter.next().ok_or("no a")?.chars().next().ok_or("wrong a")?;
        let a = (a as u8 - b'w') as usize;
        let b = iter.next().map(|s| match s.parse::<isize>() {
            Ok(n) => Value::Value(n),
            Err(_) => Value::Variable((s.chars().next().unwrap() as u8 - b'w') as usize),
        });
        let instr = match op {
            "inp" => Inp(a),
            "add" => Add(a, b.unwrap()),
            "mul" => Mul(a, b.unwrap()),
            "div" => Div(a, b.unwrap()),
            "mod" => Mod(a, b.unwrap()),
            "eql" => Eql(a, b.unwrap()),
            _ => return Err(format!("unknown op {}", op)),
        };
        Ok(instr)
    }
}

fn is_value(set: &HashSet<isize>, v: isize) -> bool {
    set.len() == 1 && set.contains(&v)
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/24.txt");

    let parsed = input
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .to_vec();

        let mut possible_values = vec![[None, None, None, None]; parsed.len() + 1];
        possible_values[0].fill_with(|| Some(std::iter::once(0isize).to_set()));
    
        assert!(forward_propagate(&parsed, &mut possible_values));
    
        let z = possible_values.last_mut().unwrap()[3].as_mut().unwrap();
        assert!(is_value(z, 0));
    
        for (i, instr) in parsed.iter().enumerate() {
            if matches!(instr, Inp(_)) {
                let value = possible_values[i + 1][0]
                    .as_ref()
                    .unwrap()
                    .iter()
                    .min()
                    .unwrap();
                print!("{}", value);
            }
        }
        println!();
    }

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/24.txt");

    let parsed = input
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .to_vec();

    let mut possible_values = vec![[None, None, None, None]; parsed.len() + 1];
    possible_values[0].fill_with(|| Some(std::iter::once(0isize).to_set()));

    assert!(forward_propagate(&parsed, &mut possible_values));

    let z = possible_values.last_mut().unwrap()[3].as_mut().unwrap();
    assert!(is_value(z, 0));

    for (i, instr) in parsed.iter().enumerate() {
        if matches!(instr, Inp(_)) {
            let value = possible_values[i + 1][0]
                .as_ref()
                .unwrap()
                .iter()
                .max()
                .unwrap();
            print!("{}", value);
        }
    }
    println!();
}

fn forward_propagate(
    instructions: &[Instruction],
    possible_values: &mut [[Option<HashSet<isize>>; 4]],
) -> bool {
    for (i, instr) in instructions.iter().enumerate() {
        let mut values = possible_values[i]
            .iter()
            .map(|o| o.clone().unwrap())
            .to_vec();
        let mut next_possible = [None, None, None, None];

        match *instr {
            Inp(a) => next_possible[a] = Some((1..=9).collect()),
            Add(a, b) => match b {
                Value::Variable(b) => {
                    next_possible[a] = Some(if is_value(&values[b], 0) {
                        values[a].clone()
                    } else {
                        values[a]
                            .iter()
                            .flat_map(|va| values[b].iter().map(move |vb| va + vb))
                            .collect()
                    });
                }
                Value::Value(b) => {
                    next_possible[a] = Some(if b == 0 {
                        values[a].clone()
                    } else {
                        values[a].iter().map(|va| va + b).collect()
                    });
                }
            },
            Mul(a, b) => match b {
                Value::Variable(b) => {
                    next_possible[a] = Some(if is_value(&values[b], 0) {
                        std::iter::once(0).collect()
                    } else if is_value(&values[b], 1) {
                        values[a].clone()
                    } else {
                        values[a]
                            .iter()
                            .flat_map(|va| values[b].iter().map(move |vb| va * vb))
                            .collect()
                    });
                }
                Value::Value(b) => {
                    next_possible[a] = Some(if b == 0 {
                        std::iter::once(0).collect()
                    } else if b == 1 {
                        values[a].clone()
                    } else {
                        values[a].iter().map(|va| va * b).collect()
                    });
                }
            },
            Div(a, b) => match b {
                Value::Variable(_) => unimplemented!(),
                Value::Value(b) => {
                    next_possible[a] = Some(if b == 1 {
                        values[a].clone()
                    } else {
                        values[a].iter().map(|va| va / b).collect()
                    });
                }
            },
            Mod(a, b) => match b {
                Value::Variable(_) => unimplemented!(),
                Value::Value(b) => {
                    next_possible[a] = Some(values[a].iter().map(|v| v % b).collect());
                }
            },
            Eql(a, b) => match b {
                Value::Variable(b) => {
                    next_possible[a] = Some(if values[a].len() == 1 && values[a] == values[b] {
                        std::iter::once(1).collect()
                    } else {
                        let intersection = &values[a] & &values[b];
                        if !intersection.is_empty() {
                            if possible_values[i][a].as_ref() != Some(&intersection)
                                || possible_values[i][b].as_ref() != Some(&intersection)
                            {
                                // enforce equality
                                possible_values[i][a] = Some(intersection.clone());
                                possible_values[i][b] = Some(intersection);
                                if !back_propagate(&instructions[..=i], possible_values) {
                                    return false;
                                }
                                values = possible_values[i]
                                    .iter()
                                    .map(|o| o.clone().unwrap())
                                    .to_vec();
                            }
                            std::iter::once(1).collect()
                        } else {
                            std::iter::once(0).collect()
                        }
                    });
                }
                Value::Value(b) => {
                    next_possible[a] = Some(if is_value(&values[a], b) {
                        std::iter::once(1).collect()
                    } else if values[a].contains(&b) {
                        [0, 1].iter().copied().collect()
                    } else {
                        std::iter::once(0).collect()
                    });
                }
            },
        }
        for ((old_estimate, new_estimate), backup) in possible_values[i + 1]
            .iter_mut()
            .zip(next_possible)
            .zip(values)
        {
            let mut new_estimate = new_estimate.unwrap_or(backup);
            if let Some(old_estimate) = old_estimate {
                new_estimate.retain(|v| old_estimate.contains(v));
                if new_estimate.is_empty() {
                    return false;
                }
                if new_estimate.len() < old_estimate.len() {
                    *old_estimate = new_estimate;
                }
            } else {
                *old_estimate = Some(new_estimate);
            }
        }
    }

    true
}

fn back_propagate(
    instructions: &[Instruction],
    possible_values: &mut [[Option<HashSet<isize>>; 4]],
) -> bool {
    for (i, instr) in instructions[..instructions.len() - 1]
        .iter()
        .enumerate()
        .rev()
    {
        let result = possible_values[i + 1]
            .iter()
            .map(|o| o.clone().unwrap())
            .to_vec();
        let mut prev_possible = [None, None, None, None];

        match *instr {
            Inp(a) => prev_possible[a] = Some(possible_values[i][a].clone().unwrap()),
            Add(a, b) => match b {
                Value::Variable(b) => {
                    if is_value(&result[b], 0) {
                        prev_possible[a] = Some(result[a].clone());
                    } else {
                        let old_a = possible_values[i][a].as_ref().unwrap();
                        let old_b = possible_values[i][b].as_ref().unwrap();
                        if is_value(old_a, 0) {
                            prev_possible[a] = Some(old_a.clone());
                            prev_possible[b] = Some(&result[a] & &result[b]);
                        } else {
                            let new_a = result[a]
                                .iter()
                                .flat_map(|va| result[b].iter().map(move |vb| va - vb))
                                .filter(|x| old_a.contains(x))
                                .to_set();
                            let new_b = old_b
                                .iter()
                                .filter(|&vb| new_a.iter().any(|va| result[a].contains(&(va + vb))))
                                .copied()
                                .collect();
                            prev_possible[a] = Some(new_a);
                            prev_possible[b] = Some(new_b);
                        }
                    }
                }
                Value::Value(b) => {
                    prev_possible[a] = Some(if b == 0 {
                        result[a].clone()
                    } else {
                        result[a].iter().map(|va| va - b).collect()
                    });
                }
            },
            Mul(a, b) => match b {
                Value::Variable(b) => {
                    if is_value(&result[b], 0) {
                        prev_possible[a] = Some(possible_values[i][a].clone().unwrap());
                    } else if is_value(&result[b], 1) {
                        prev_possible[a] = Some(result[a].clone());
                    } else {
                        let old_a = possible_values[i][a].as_ref().unwrap();
                        let old_b = possible_values[i][b].as_ref().unwrap();
                        let new_a = result[a]
                            .iter()
                            .flat_map(|va| {
                                result[b]
                                    .iter()
                                    .filter(move |vb| va % *vb == 0)
                                    .map(move |vb| va / vb)
                            })
                            .filter(|x| old_a.contains(x))
                            .to_set();
                        let new_b = old_b
                            .iter()
                            .filter(|&vb| new_a.iter().any(|va| result[a].contains(&(va * vb))))
                            .copied()
                            .collect();
                        prev_possible[a] = Some(new_a);
                        prev_possible[b] = Some(new_b);
                    }
                }
                Value::Value(b) => {
                    prev_possible[a] = Some(if b == 0 {
                        possible_values[i][a].clone().unwrap()
                    } else if b == 1 {
                        result[a].clone()
                    } else {
                        result[a]
                            .iter()
                            .filter(|va| *va % b == 0)
                            .map(|va| va / b)
                            .collect()
                    });
                }
            },
            Div(a, b) => match b {
                Value::Variable(_) => unimplemented!(),
                Value::Value(b) => {
                    prev_possible[a] = Some(if b == 1 {
                        result[a].clone()
                    } else {
                        let old = possible_values[i][a].as_ref().unwrap();
                        result[a]
                            .iter()
                            .flat_map(|va| (0..b).map(move |offset| va * b + offset))
                            .filter(|x| old.contains(x))
                            .collect()
                    });
                }
            },
            Mod(a, b) => match b {
                Value::Variable(_) => unimplemented!(),
                Value::Value(b) => {
                    prev_possible[a] = Some(
                        possible_values[i][a]
                            .as_ref()
                            .unwrap()
                            .iter()
                            .filter(|va| result[a].contains(&(*va % b)))
                            .copied()
                            .collect(),
                    );
                }
            },
            Eql(a, b) => match b {
                Value::Variable(b) => {
                    let old_a = possible_values[i][a].as_ref().unwrap();
                    if is_value(&result[a], 0) {
                        // have to be different
                        let new_a = old_a - &result[b];
                        prev_possible[b] = Some(&result[b] - &new_a);
                        prev_possible[a] = Some(new_a);
                    } else if is_value(&result[a], 1) {
                        let intersection = old_a & &result[b];
                        prev_possible[a] = Some(intersection.clone());
                        prev_possible[b] = Some(intersection);
                    } else {
                        prev_possible[a] = Some(possible_values[i][a].clone().unwrap());
                        prev_possible[b] = Some(possible_values[i][b].clone().unwrap());
                    }
                }
                Value::Value(b) => {
                    let old = possible_values[i][a].as_ref().unwrap();
                    prev_possible[a] = Some(if is_value(&result[a], 0) {
                        // have to be different
                        let mut old = old.clone();
                        old.remove(&b);
                        old
                    } else if is_value(&result[a], 1) {
                        std::iter::once(b).collect()
                    } else {
                        old.clone()
                    });
                }
            },
        }

        let mut change = false;
        for ((old_estimate, new_estimate), backup) in
            possible_values[i].iter_mut().zip(prev_possible).zip(result)
        {
            let mut new_estimate = new_estimate.unwrap_or(backup);
            if let Some(old_estimate) = old_estimate {
                new_estimate.retain(|v| old_estimate.contains(v));
                if new_estimate.is_empty() {
                    return false;
                }
                if new_estimate.len() < old_estimate.len() {
                    *old_estimate = new_estimate;
                    change = true;
                }
            } else {
                *old_estimate = Some(new_estimate);
                change = true;
            }
        }

        if !change {
            return forward_propagate(&instructions[i..], &mut possible_values[i..]);
        }
    }
    forward_propagate(instructions, possible_values)
}
