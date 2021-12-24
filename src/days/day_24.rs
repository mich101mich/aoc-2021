use crate::utils::*;

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
    fn apply(&self, vars: &mut [isize; 4], input: &mut impl Iterator<Item=isize>) {
        match self {
            Inp(a) => vars[*a] = input.next().unwrap(),
            Add(a, b) => vars[*a] += b.get(vars),
            Mul(a, b) => vars[*a] *= b.get(vars),
            Div(a, b) => vars[*a] /= b.get(vars),
            Mod(a, b) => vars[*a] %= b.get(vars),
            Eql(a, b) => vars[*a] = (vars[*a] == b.get(vars)) as isize,
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

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/24.txt");

    let parsed = input
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .to_vec();

    // manually read:
    // p1 p2 a b p3 p4 c d a4 e f a3 a2 a1
    // b = a + 3
    // d = c - 8
    // f = e + 2
    // a4 = p4 + 1
    // a3 = p3 + 8
    // a2 = p2 - 6
    // a1 = p1 - 1

    let mut input = vec![2,7,1,4,1,1,9,1,2,1,3,9,1,1];

    let mut vars = [0; 4];
    let mut iter = input.iter().copied();
    for instr in &parsed {
        instr.apply(&mut vars, &mut iter);
    }

    print_arr!(input);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/24.txt");

    let parsed = input
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .to_vec();

    // manually read:
    // p1 p2 a b p3 p4 c d a4 e f a3 a2 a1
    // b = a + 3
    // d = c - 8
    // f = e + 2
    // a4 = p4 + 1
    // a3 = p3 + 8
    // a2 = p2 - 6
    // a1 = p1 - 1

    let mut input = vec![9,9,6,9,1,8,9,1,9,7,9,9,3,8];

    let mut vars = [0; 4];
    let mut iter = input.iter().copied();
    for instr in &parsed {
        instr.apply(&mut vars, &mut iter);
    }

    print_arr!(input);
}
