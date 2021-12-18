use bitvec::prelude::*;
type BVec = bitvec::prelude::BitVec<Msb0>;
type BSlice = bitvec::prelude::BitSlice<Msb0>;
type TypeId = u8;
enum Packet {
    Literal(u64),
    Operator(Vec<Packet>, TypeId),
}

fn hex_to_bitvec(s: &str) -> BVec {
    let mut bitvec = BVec::new();
    for c in s.trim().chars() {
        match c {
            '0' => bitvec.extend(bits![0, 0, 0, 0]),
            '1' => bitvec.extend(bits![0, 0, 0, 1]),
            '2' => bitvec.extend(bits![0, 0, 1, 0]),
            '3' => bitvec.extend(bits![0, 0, 1, 1]),
            '4' => bitvec.extend(bits![0, 1, 0, 0]),
            '5' => bitvec.extend(bits![0, 1, 0, 1]),
            '6' => bitvec.extend(bits![0, 1, 1, 0]),
            '7' => bitvec.extend(bits![0, 1, 1, 1]),
            '8' => bitvec.extend(bits![1, 0, 0, 0]),
            '9' => bitvec.extend(bits![1, 0, 0, 1]),
            'A' => bitvec.extend(bits![1, 0, 1, 0]),
            'B' => bitvec.extend(bits![1, 0, 1, 1]),
            'C' => bitvec.extend(bits![1, 1, 0, 0]),
            'D' => bitvec.extend(bits![1, 1, 0, 1]),
            'E' => bitvec.extend(bits![1, 1, 1, 0]),
            'F' => bitvec.extend(bits![1, 1, 1, 1]),
            c => panic!("unknown char:{:?}", c),
        }
    }
    bitvec
}

fn read_header(bits: &mut &BSlice) -> (u8, TypeId) {
    let version = bits[0..3].load_be();
    let type_id = bits[3..6].load_be();
    *bits = &bits[6..];
    (version, type_id)
}

fn read_literal(bits: &mut &BSlice) -> Packet {
    let mut literal = BVec::new();
    loop {
        literal.extend(bits[1..5].iter().by_ref());
        let more = bits[0];
        *bits = &bits[5..];
        if !more {
            break;
        }
    }

    Packet::Literal(literal.load_be())
}

fn read_operator(bits: &mut &BSlice, type_id: TypeId, v_sum: &mut u64) -> Packet {
    let length_id = bits[0];
    if length_id {
        let num_subpackets: usize = bits[1..12].load_be();
        *bits = &bits[12..];
        let mut sub_packets = Vec::<Packet>::with_capacity(num_subpackets);
        for _ in 0..num_subpackets {
            let pack = read_packet(bits, v_sum);
            sub_packets.push(pack);
        }
        Packet::Operator(sub_packets, type_id)
    } else {
        let bit_len_subpackets: usize = bits[1..16].load_be();
        *bits = &bits[16..];
        let before = bits.len();
        let mut sub_packets = Vec::new();

        while (before - bits.len()) != bit_len_subpackets {
            let pack = read_packet(bits, v_sum);
            sub_packets.push(pack);
        }
        Packet::Operator(sub_packets, type_id)
    }
}

fn read_packet(bits: &mut &BSlice, v_sum: &mut u64) -> Packet {
    let (version, type_id) = read_header(bits);
    *v_sum += version as u64;
    if type_id == 4 {
        read_literal(bits)
    } else {
        read_operator(bits, type_id, v_sum)
    }
}

fn part_1(s: &str) -> u64 {
    let bitvec = hex_to_bitvec(s);
    let mut slice = &bitvec[..];
    let mut v_sum = 0;
    read_packet(&mut slice, &mut v_sum);
    v_sum
}

fn eval(pack: &Packet) -> u64 {
    match pack {
        Packet::Literal(n) => *n,
        Packet::Operator(subpacks, type_id) => match type_id {
            0 => subpacks.iter().map(eval).sum(),
            1 => subpacks.iter().map(eval).product(),
            2 => subpacks.iter().map(eval).min().unwrap(),
            3 => subpacks.iter().map(eval).max().unwrap(),
            5 => {
                if eval(&subpacks[0]) > eval(&subpacks[1]) {
                    1
                } else {
                    0
                }
            }
            6 => {
                if eval(&subpacks[0]) < eval(&subpacks[1]) {
                    1
                } else {
                    0
                }
            }
            7 => {
                if eval(&subpacks[0]) == eval(&subpacks[1]) {
                    1
                } else {
                    0
                }
            }
            _ => panic!(),
        },
    }
}

fn part_2(s: &str) -> u64 {
    let bitvec = hex_to_bitvec(s);
    let mut slice = &bitvec[..];
    let pack = read_packet(&mut slice, &mut 0);
    eval(&pack)
}

#[cfg(test)]
mod tests {
    use super::part_1;
    use super::part_2;
    use std::fs;
    #[test]
    fn example16_part1() {
        let s = "620080001611562C8802118E34";
        let ans = part_1(s);
        assert_eq!(ans, 12);
    }
    #[test]
    fn day16_part1() {
        let input = fs::read_to_string("input/day16").unwrap();
        let ans = part_1(&input);
        assert_eq!(ans, 963);
    }
    #[test]
    fn example16_part2() {
        assert_eq!(part_2("C200B40A82"), 3);
        assert_eq!(part_2("04005AC33890"), 54);
        assert_eq!(part_2("880086C3E88112"), 7);
        assert_eq!(part_2("CE00C43D881120"), 9);
        assert_eq!(part_2("D8005AC2A8F0"), 1);
        assert_eq!(part_2("F600BC2D8F"), 0);
        assert_eq!(part_2("9C005AC2F8F0"), 0);
        assert_eq!(part_2("9C0141080250320F1802104A08"), 1);
    }
    #[test]
    fn day16_part2() {
        let input = fs::read_to_string("input/day16").unwrap();
        let ans = part_2(&input);
        assert_eq!(ans, 1549026292886);
    }
}
