use std::collections::HashSet;

fn part_1(s: &str) -> usize {
    let (_, output) = s.split_once('|').unwrap();
    output
        .split_whitespace()
        .filter(|d| matches!(d.len(), 2 | 3 | 4 | 7))
        .count()
}

fn part_2(s: &str) -> usize {
    let (all_digits, output) = s.split_once('|').unwrap();
    let mut signals_1: Option<HashSet<char>> = None;
    let mut signals_4: Option<HashSet<char>> = None;

    for signals in all_digits.split_whitespace() {
        match signals.len() {
            2 => {
                //digit 1: c,f
                signals_1 = Some(signals.chars().collect());
            }
            4 => {
                // digit 4: b,c,d,f
                signals_4 = Some(signals.chars().collect());
            }
            _ => (),
        }
    }

    let mut e_sig = None;
    let mut c_sig = None;
    let whole_set: HashSet<_> = ['a', 'b', 'c', 'd', 'e', 'f', 'g'].into();
    for signals in all_digits.split_whitespace().filter(|s| s.len() == 6) {
        let set: HashSet<_> = signals.chars().collect();
        if set.is_superset(signals_1.as_ref().unwrap()) {
            if set.is_superset(signals_4.as_ref().unwrap()) {
                // Found 9; gives signal for segment e.
                e_sig = whole_set.difference(&set).next().cloned();
            }
            // Else found 0; gives segment d but not needed.
        } else {
            // Found 6; gives signal for segment c.
            c_sig = whole_set.difference(&set).next().cloned();
        }
    }
    let e_sig = e_sig.unwrap();
    let c_sig = c_sig.unwrap();
    let mut result = String::new();
    for signals in output.split_whitespace() {
        match signals.len() {
            2 => result.push('1'),
            3 => result.push('7'),
            4 => result.push('4'),
            5 if signals.contains(e_sig) => result.push('2'),
            5 if signals.contains(c_sig) => result.push('3'),
            5 => result.push('5'),
            6 if !signals.contains(e_sig) => result.push('9'),
            6 if !signals.contains(c_sig) => result.push('6'),
            6 => result.push('0'),
            7 => result.push('8'),
            _ => panic!(),
        }
    }
    result.parse().unwrap()
}
#[cfg(test)]
mod tests {
    use std::fs;

    use crate::day08::{part_1, part_2};
    #[test]
    fn day08_part1() {
        let input = fs::read_to_string("input/day08").unwrap();
        let ans: usize = input.lines().map(part_1).filter(|&n| n != 0).sum();
        dbg!(ans);
    }
    #[test]
    fn example08_part2() {
        let input = fs::read_to_string("input/example08").unwrap();
        let ans: usize = input.lines().map(part_2).sum();
        assert_eq!(ans, 61229);
    }
    #[test]
    fn day08_part2() {
        let input = fs::read_to_string("input/day08").unwrap();
        let ans: usize = input.lines().map(part_2).sum();
        assert_eq!(ans, 1011823);
    }
}
