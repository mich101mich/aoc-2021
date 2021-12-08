use crate::utils::*;

#[allow(clippy::many_single_char_names)]
#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/08.txt");

    let parsed = input
        .lines()
        .map(|l| {
            l.split(" | ")
                .map(|s| s.split_whitespace().to_vec())
                .to_vec()
        })
        .to_vec();

    let digit_map = [
        "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
    ]
    .iter()
    .enumerate()
    .map(|(i, s)| (s.chars().to_vec(), i))
    .to_map();

    let mut sum = 0;
    for v in parsed.iter() {
        let (a, b, c, d, e, f, g);
        let mut checks = v[0].iter().map(|s| s.chars().to_set()).to_vec();
        checks.sort_by_key(|s| 7 - s.len());
        let one = checks.pop().unwrap();
        let seven = checks.pop().unwrap();
        let four = checks.pop().unwrap();
        let _eight = checks.remove(0);
        a = (&seven - &one).into_iter().next().unwrap();
        let mut b_or_d = &four - &one;
        let mut c_or_f = one.clone();
        let three = checks
            .iter()
            .filter(|ch| (*ch & &seven).len() == seven.len())
            .find(|ch| (*ch - &seven).len() == 2)
            .unwrap()
            .clone();
        checks.retain(|ch| *ch != three);
        let mut d_or_g = &three - &seven;
        d = (&d_or_g & &b_or_d).into_iter().next().unwrap();
        d_or_g.remove(&d);
        g = d_or_g.into_iter().next().unwrap();
        b_or_d.remove(&d);
        b = b_or_d.into_iter().next().unwrap();

        let mut cef = checks
            .into_iter()
            .filter_map(|mut ch| {
                for x in [a, b, d, g].iter() {
                    if !ch.remove(x) {
                        return None;
                    }
                }
                Some(ch)
            })
            .to_vec();
        cef.sort_by_key(|s| 3 - s.len());
        f = cef.pop().unwrap().into_iter().next().unwrap();
        c_or_f.remove(&f);
        c = c_or_f.into_iter().next().unwrap();
        let mut e_or_f = cef.into_iter().find(|ch| !ch.contains(&c)).unwrap();
        e_or_f.remove(&f);
        e = e_or_f.into_iter().next().unwrap();
        #[rustfmt::skip]
        let map = [(a, 'a'), (b, 'b'), (c, 'c'), (d, 'd'), (e, 'e'), (f, 'f'), (g, 'g')]
            .iter().copied().to_map();

        let mut output_value = 0;
        for s in v[1].iter() {
            let mut key = vec![];
            for ch in s.chars() {
                key.push(map[&ch]);
            }
            key.sort_unstable();
            let digit = digit_map[&key];
            output_value = output_value * 10 + digit;
        }
        pv!(output_value);
        sum += output_value;
    }
    pv!(sum);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/08.txt");

    let parsed = input
        .lines()
        .map(|l| {
            l.split(" | ")
                .map(|s| s.split_whitespace().to_vec())
                .to_vec()
        })
        .to_vec();

    let special = parsed
        .iter()
        .flat_map(|v| v[1].iter())
        .filter(|s| s.len() != 5 && s.len() != 6)
        .count();
    pv!(special);
}
