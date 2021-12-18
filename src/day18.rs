use itertools::Itertools;
use std::{cmp, iter::Peekable};

#[derive(Clone)]
enum Element {
    Number(u64),
    Pair(Box<Element>, Box<Element>),
}

fn parse(s: &str) -> Box<Element> {
    parse_rec(&mut s.chars().peekable())
}

fn parse_rec<I: Iterator<Item = char>>(chars: &mut Peekable<I>) -> Box<Element> {
    if matches!(chars.peek(), Some('[')) {
        chars.next().unwrap();
        let left = parse_rec(chars);
        chars.next().unwrap(); // removing ,
        let right = parse_rec(chars);
        chars.next().unwrap(); // removing ]
        Box::new(Element::Pair(left, right))
    } else {
        let mut num = String::new();
        loop {
            if matches!(chars.peek(), Some(c) if c.is_ascii_digit()) {
                num.push(chars.next().unwrap());
            } else {
                break;
            }
        }
        Box::new(Element::Number(num.parse().unwrap()))
    }
}

fn add(el_a: Box<Element>, el_b: Box<Element>) -> Box<Element> {
    Box::new(Element::Pair(el_a, el_b))
}

fn explode(el: &mut Element) -> bool {
    let mut flag = false;
    explode_rec(0, el, &mut flag);
    flag
}
fn explode_rec(depth: u32, el: &mut Element, exp_flag: &mut bool) -> (Option<u64>, Option<u64>) {
    let mut exploded = false;
    let mut result = (None, None);
    match el {
        Element::Number(_) => (),
        Element::Pair(a, b) if depth == 4 => {
            if let (&Element::Number(a), &Element::Number(b)) = (a.as_ref(), b.as_ref()) {
                *exp_flag = true;
                exploded = true;
                result = (Some(a), Some(b));
            } else {
                panic!()
            }
        }
        Element::Pair(a, b) => {
            let left_exp = explode_rec(depth + 1, a, exp_flag);
            if !*exp_flag {
                let right_exp = explode_rec(depth + 1, b, exp_flag);
                if let Some(to_add) = right_exp.0 {
                    if !add_to_rightmost(a, to_add) {
                        result = (Some(to_add), result.1);
                    }
                }
                if let Some(to_add) = right_exp.1 {
                    result = (result.0, Some(to_add));
                }
            } else {
                if let Some(to_add) = left_exp.0 {
                    result = (Some(to_add), result.1);
                }
                if let Some(to_add) = left_exp.1 {
                    if !add_to_leftmost(b, to_add) {
                        result = (result.0, Some(to_add));
                    }
                }
            }
        }
    }
    if exploded {
        *el = Element::Number(0);
        return result;
    }
    result
}
fn add_to_rightmost(el: &mut Element, val: u64) -> bool {
    match el {
        Element::Number(n) => {
            *n += val;
            true
        }
        Element::Pair(a, b) => {
            if add_to_rightmost(b, val) {
                true
            } else {
                add_to_rightmost(a, val)
            }
        }
    }
}

fn add_to_leftmost(el: &mut Element, val: u64) -> bool {
    match el {
        Element::Number(n) => {
            *n += val;
            true
        }
        Element::Pair(a, b) => {
            if add_to_leftmost(a, val) {
                true
            } else {
                add_to_leftmost(b, val)
            }
        }
    }
}

fn split(el: &mut Element) -> bool {
    let mut split_num = None;
    match el {
        Element::Number(n) if *n > 9 => {
            split_num = Some(*n);
        }
        Element::Pair(a, b) => {
            if split(a) {
                return true;
            } else {
                return split(b);
            }
        }
        _ => (),
    }
    if let Some(n) = split_num {
        *el = Element::Pair(
            Box::new(Element::Number(n / 2)),
            Box::new(Element::Number(n - n / 2)),
        );
        true
    } else {
        false
    }
}

fn magnitude(el: &Element) -> u64 {
    match el {
        Element::Number(n) => *n,
        Element::Pair(a, b) => 3 * magnitude(a) + 2 * magnitude(b),
    }
}

fn reduce(el: &mut Element) {
    loop {
        if explode(el) {
            continue;
        }
        if split(el) {
            continue;
        }
        break;
    }
}

fn part_1(s: &str) -> u64 {
    let mut lines = s.lines();
    let first = lines.next().unwrap();
    let first_el = parse(first);
    let final_element = lines.fold(first_el, |acc, l| {
        let el = parse(l);
        let mut result = add(acc, el);
        reduce(&mut result);
        result
    });
    magnitude(&final_element)
}

fn part_2(s: &str) -> u64 {
    let elements = s.lines().map(parse).collect_vec();
    elements
        .iter()
        .combinations(2)
        .map(|combo| {
            let a = combo[0];
            let b = combo[1];
            let mut result_1 = add(a.clone(), b.clone());
            let mut result_2 = add(b.clone(), a.clone());
            reduce(&mut result_1);
            reduce(&mut result_2);
            cmp::max(magnitude(&result_1), magnitude(&result_2))
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::part_1;
    use super::part_2;
    #[test]
    fn example() {
        let input = fs::read_to_string("input/example18").unwrap();

        let ans = part_1(&input);
        assert_eq!(ans, 4140);
    }
    #[test]
    fn day18_part1() {
        let input = fs::read_to_string("input/day18").unwrap();

        let ans = part_1(&input);
        assert_eq!(ans, 4433);
    }
    #[test]
    fn example18_part2() {
        let input = fs::read_to_string("input/example18").unwrap();
        let ans = part_2(&input);
        assert_eq!(ans, 3993);
    }
    #[test]
    fn day18_part2() {
        let input = fs::read_to_string("input/day18").unwrap();
        let ans = part_2(&input);
        assert_eq!(ans, 4559);
    }
}
