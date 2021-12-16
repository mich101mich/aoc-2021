use crate::utils::*;

fn to_number(bits: &[bool]) -> usize {
    bits.iter().fold(0, |acc, b| (acc << 1) | (*b as usize))
}
fn take_number(bits: &mut &[bool], n: usize) -> usize {
    let ret = to_number(&bits[..n]);
    *bits = &bits[n..];
    ret
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/16.txt");

    use nom::{
        branch::alt,
        bytes::complete::{tag, take},
        multi::*,
        sequence::preceded,
        IResult,
    };

    fn to_number(bits: &[u8]) -> usize {
        bits.iter().fold(0, |acc, b| (acc << 1) | (*b as usize))
    }
    fn bit_number(n: usize) -> impl Fn(&[u8]) -> IResult<&[u8], usize> {
        move |i| take(n)(i).map(|(i, bits)| (i, to_number(bits)))
    }

    fn packet(i: &[u8]) -> IResult<&[u8], usize> {
        let (i, version) = bit_number(3)(i)?;
        let (i, type_id) = bit_number(3)(i)?;

        if type_id == 4 {
            let (i, n) = fold_many0(
                preceded(tag(&[1]), bit_number(4)),
                || 0,
                |acc, n| (acc << 4) | n,
            )(i)?;
            let (i, last) = preceded(tag(&[0]), bit_number(4))(i)?;
            Ok((i, (n << 4) | last))
        } else {
            let (i, sub_packets) = alt((
                preceded(tag(&[0]), length_value(bit_number(15), many0(packet))),
                preceded(tag(&[1]), length_count(bit_number(11), packet)),
            ))(i)?;

            let number = match type_id {
                0 => sub_packets.iter().sum(),
                1 => sub_packets.iter().product(),
                2 => *sub_packets.iter().min().unwrap(),
                3 => *sub_packets.iter().max().unwrap(),
                5 => (sub_packets[0] > sub_packets[1]) as usize,
                6 => (sub_packets[0] < sub_packets[1]) as usize,
                7 => (sub_packets[0] == sub_packets[1]) as usize,
                n => panic!("Unknown type_id: {}", n),
            };
            Ok((i, number))
        }
    }

    let bits = input
        .chars()
        .map(|c| c.to_digit(16).unwrap() as u8)
        .flat_map(|n| [(n >> 3) & 1, (n >> 2) & 1, (n >> 1) & 1, n & 1])
        .to_vec();
    let number = packet(&bits).unwrap().1;
    pv!(number);

    // Initial Solution:
    // Manually moving slices around. Same approach as part 1.

    // fn parse_packet(bits: &mut &[bool]) -> usize {
    //     let version = take_number(bits, 3);
    //     let type_id = take_number(bits, 3);

    //     if type_id == 4 {
    //         let mut number = vec![];
    //         while bits.get(0) == Some(&true) {
    //             number.extend(&bits[1..5]);
    //             *bits = &bits[5..];
    //         }
    //         number.extend(&bits[1..5]);
    //         *bits = &bits[5..];
    //         to_number(&number)
    //     } else {
    //         let mut sub_packets = vec![];
    //         let length_type_id = take_number(bits, 1);
    //         if length_type_id == 0 {
    //             let length = take_number(bits, 15);
    //             let mut sub_bits = &bits[..length];
    //             while !sub_bits.is_empty() {
    //                 sub_packets.push(parse_packet(&mut sub_bits));
    //             }
    //             *bits = &bits[length..];
    //         } else {
    //             let num_packets = take_number(bits, 11);
    //             for _ in 0..num_packets {
    //                 sub_packets.push(parse_packet(bits));
    //             }
    //         }
    //         match type_id {
    //             0 => sub_packets.iter().sum(),
    //             1 => sub_packets.iter().product(),
    //             2 => *sub_packets.iter().min().unwrap(),
    //             3 => *sub_packets.iter().max().unwrap(),
    //             5 => (sub_packets[0] > sub_packets[1]) as usize,
    //             6 => (sub_packets[0] < sub_packets[1]) as usize,
    //             7 => (sub_packets[0] == sub_packets[1]) as usize,
    //             n => panic!("Unknown type_id: {}", n),
    //         }
    //     }
    // }
    // let bits = input
    //     .chars()
    //     .map(|c| c.to_digit(16).unwrap() as u8)
    //     .flat_map(|n| {
    //         [
    //             (n >> 3) & 1 == 1,
    //             (n >> 2) & 1 == 1,
    //             (n >> 1) & 1 == 1,
    //             n & 1 == 1,
    //         ]
    //     })
    //     .to_vec();
    // let number = parse_packet(&mut &bits[..]);
    // pv!(number);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/16.txt");

    let parsed = input.chars().map(|c| c.to_digit(16).unwrap()).to_vec();
    let mut bits = vec![];
    for n in parsed.iter() {
        bits.push((n >> 3) & 1 == 1);
        bits.push((n >> 2) & 1 == 1);
        bits.push((n >> 1) & 1 == 1);
        bits.push(n & 1 == 1);
    }

    fn parse_packet(bits: &mut &[bool]) -> usize {
        let version = take_number(bits, 3);
        let type_id = take_number(bits, 3);

        if type_id == 4 {
            let mut number = vec![];
            while bits.get(0) == Some(&true) {
                number.extend(&bits[1..5]);
                *bits = &bits[5..];
            }
            number.extend(&bits[1..5]);
            *bits = &bits[5..];
            let number = to_number(&number);
            version
        } else {
            let mut sum = version;
            let length_type_id = take_number(bits, 1);
            if length_type_id == 0 {
                let length = take_number(bits, 15);
                let mut sub_bits = &bits[..length];
                while !sub_bits.is_empty() {
                    sum += parse_packet(&mut sub_bits);
                }
                *bits = &bits[length..];
            } else {
                let num_packets = take_number(bits, 11);
                for _ in 0..num_packets {
                    sum += parse_packet(bits);
                }
            }
            sum
        }
    }
    let sum = parse_packet(&mut &bits[..]);
    pv!(sum);
}
