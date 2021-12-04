use bitvec::prelude::*;
use bitvec::vec::BitVec;

fn string_to_bitvec(s: &str) -> BitVec {
    s.chars().map(|c| c == '1').collect()
}

fn most_common_bits(input_bits: &[BitVec]) -> BitVec {
    let capacity = input_bits[0].len();
    let mut result = BitVec::with_capacity(capacity);
    for i in 0..capacity {
        let oxygen_count: i32 = input_bits.iter().map(|sc| if sc[i] { 1 } else { -1 }).sum();
        result.push(oxygen_count >= 0);
    }
    result
}

fn bitvec_to_u32(mut bits: BitVec) -> u32 {
    bits.reverse(); // TODO: Why must we reverse?
    bits.load()
}

fn find_scrubber_and_oxygen_ratings(bitvecs: Vec<BitVec>) -> (BitVec, BitVec) {
    let mut scrubber_candidates = bitvecs.clone();
    let mut oxygen_candidates = bitvecs;
    let bit_count_at_i =
        |candidates: &[BitVec], i| candidates.iter().map(|c| if c[i] { 1 } else { -1 }).sum();
    // Search for oxygen rating
    for i in 0..oxygen_candidates[0].len() {
        let oxygen_count: i32 = bit_count_at_i(&oxygen_candidates, i);
        oxygen_candidates.retain(|candidate| candidate[i] == (oxygen_count >= 0));
        if oxygen_candidates.len() == 1 {
            break;
        }
    }
    // Search for scrubber rating
    for i in 0..scrubber_candidates[0].len() {
        let scrubber_count: i32 = bit_count_at_i(&scrubber_candidates, i);
        scrubber_candidates.retain(|candidate| candidate[i] != (scrubber_count >= 0));
        if scrubber_candidates.len() == 1 {
            break;
        }
    }
    (scrubber_candidates[0].clone(), oxygen_candidates[0].clone())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::day03::{bitvec_to_u32, find_scrubber_and_oxygen_ratings};

    use super::{most_common_bits, string_to_bitvec, BitVec};
    #[test]
    fn example03_part1() {
        let input = fs::read_to_string("input/example03").unwrap();
        let input: Vec<BitVec> = input.lines().map(string_to_bitvec).collect();
        let common = most_common_bits(&input);
        let gamma = bitvec_to_u32(common.clone());
        let epsilon = bitvec_to_u32(!common);
        assert_eq!(gamma * epsilon, 198);
    }
    #[test]
    fn day03_part1() {
        let input = fs::read_to_string("input/day03").unwrap();
        let input: Vec<BitVec> = input.lines().map(string_to_bitvec).collect();
        let common = most_common_bits(&input);
        let gamma = bitvec_to_u32(common.clone());
        let epsilon = bitvec_to_u32(!common);
        assert_eq!(gamma * epsilon, 845186);
    }
    #[test]
    fn example03_part2() {
        let input = fs::read_to_string("input/example03").unwrap();
        let input: Vec<BitVec> = input.lines().map(string_to_bitvec).collect();
        let (scrub, oxygen) = find_scrubber_and_oxygen_ratings(input);
        let ans = bitvec_to_u32(scrub) * bitvec_to_u32(oxygen);
        assert_eq!(ans, 230);
    }
    #[test]
    fn day03_part2() {
        let input = fs::read_to_string("input/day03").unwrap();
        let input: Vec<BitVec> = input.lines().map(string_to_bitvec).collect();
        let (scrub, oxygen) = find_scrubber_and_oxygen_ratings(input);
        let ans = bitvec_to_u32(scrub) * bitvec_to_u32(oxygen);
        assert_eq!(ans, 4636702);
    }
}
