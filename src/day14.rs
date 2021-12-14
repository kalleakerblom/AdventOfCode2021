use itertools::Itertools;
use std::collections::HashMap;

type InsertionRules = HashMap<(char, char), char>;
type Count = HashMap<char, u64>;
type Memo = HashMap<(char, char, u32), Count>;

fn parse_rules(s: &str) -> InsertionRules {
    // CH -> B
    let read_rule = |line: &str| {
        let (pair, insert) = line.split_once(" -> ").unwrap();
        let mut pair = pair.chars();
        (
            (pair.next().unwrap(), pair.next().unwrap()),
            insert.chars().next().unwrap(),
        )
    };
    s.lines().map(read_rule).collect()
}

fn count_elements_between(
    a: char,
    b: char,
    rules: &InsertionRules,
    steps: u32,
    memo: &mut Memo,
) -> Count {
    if let Some(memo_count) = memo.get(&(a, b, steps)) {
        return memo_count.clone();
    }
    let mut count = HashMap::new();
    if steps == 0 {
        return count;
    }
    if let Some(&mid) = rules.get(&(a, b)) {
        count = count_elements_between(a, mid, rules, steps - 1, memo);
        let count2 = count_elements_between(mid, b, rules, steps - 1, memo);
        for (el, co) in count2 {
            *count.entry(el).or_default() += co;
        }
        *count.entry(mid).or_default() += 1;
    }
    memo.insert((a, b, steps), count.clone());
    count
}

fn part_1_and_2(input: &str, steps: u32) -> u64 {
    let empty_line = "\r\n\r\n";
    let (poly, rules) = input.split_once(empty_line).unwrap();
    let poly: Vec<_> = poly.chars().collect();
    let rules = parse_rules(rules);
    let mut count = Count::new();
    for c in &poly {
        *count.entry(*c).or_default() += 1;
    }
    let mut memo = Memo::new();
    for (&a, &b) in poly.iter().tuple_windows() {
        let between_count = count_elements_between(a, b, &rules, steps, &mut memo);
        for (el, co) in between_count {
            *count.entry(el).or_default() += co;
        }
    }
    let min = count.iter().min_by_key(|(_, c)| *c).unwrap();
    let max = count.iter().max_by_key(|(_, c)| *c).unwrap();
    max.1 - min.1
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::part_1_and_2;
    #[test]
    fn example14_part1() {
        let input = fs::read_to_string("input/example14").unwrap();
        let ans = part_1_and_2(&input, 10);
        assert_eq!(ans, 1588);
    }
    #[test]
    fn day14_part1() {
        let input = fs::read_to_string("input/day14").unwrap();
        let ans = part_1_and_2(&input, 10);
        assert_eq!(ans, 3048);
    }
    #[test]
    fn example14_part2() {
        let input = fs::read_to_string("input/example14").unwrap();
        let ans = part_1_and_2(&input, 40);
        assert_eq!(ans, 2188189693529);
    }
    #[test]
    fn day14_part2() {
        let input = fs::read_to_string("input/day14").unwrap();
        let ans = part_1_and_2(&input, 40);
        assert_eq!(ans, 3288891573057);
    }
}
