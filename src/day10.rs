use std::collections::HashMap;

enum SyntaxResult {
    Valid,
    Incomplete(Vec<char>),
    Error(char),
}

fn check_syntax(line: &str) -> SyntaxResult {
    let mut stack = vec![];
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => match (stack.pop(), c) {
                (Some('('), ')') | (Some('['), ']') | (Some('{'), '}') | (Some('<'), '>') => (),
                _ => return SyntaxResult::Error(c),
            },
            _ => panic!(),
        }
    }
    if !stack.is_empty() {
        return SyntaxResult::Incomplete(stack);
    }
    SyntaxResult::Valid
}

fn part_1(input: &str) -> u32 {
    let error_score: HashMap<char, u32> = [(')', 3), (']', 57), ('}', 1197), ('>', 25137)].into();
    input
        .lines()
        .map(check_syntax)
        .filter_map(|res| {
            if let SyntaxResult::Error(c) = res {
                Some(c)
            } else {
                None
            }
        })
        .map(|c| error_score[&c])
        .sum()
}

fn calculate_completion_score(to_close: &[char]) -> u64 {
    let scoring: HashMap<_, _> = [('(', 1), ('[', 2), ('{', 3), ('<', 4)].into();
    to_close.iter().rev().fold(0, |mut score, c| {
        score *= 5;
        score += scoring[c];
        score
    })
}

fn part_2(input: &str) -> u64 {
    let mut completion_scores: Vec<u64> = input
        .lines()
        .map(check_syntax)
        .filter_map(|res| {
            if let SyntaxResult::Incomplete(v) = res {
                Some(v)
            } else {
                None
            }
        })
        .map(|v| calculate_completion_score(&v))
        .collect();
    let middle = completion_scores.len() / 2;
    *completion_scores.select_nth_unstable(middle).1
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{part_1, part_2};
    #[test]
    fn example10_part1() {
        let input = fs::read_to_string("input/example10").unwrap();
        assert_eq!(part_1(&input), 26397);
    }
    #[test]
    fn day10_part1() {
        let input = fs::read_to_string("input/day10").unwrap();
        assert_eq!(part_1(&input), 392367);
    }
    #[test]
    fn example10_part2() {
        let input = fs::read_to_string("input/example10").unwrap();
        assert_eq!(part_2(&input), 288957);
    }
    #[test]
    fn day10_part2() {
        let input = fs::read_to_string("input/day10").unwrap();
        assert_eq!(part_2(&input), 2192104158);
    }
}
