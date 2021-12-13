use std::collections::HashSet;
#[derive(Clone, Copy)]
enum FoldInstruction {
    X(i64),
    Y(i64),
}
impl FoldInstruction {
    fn parse(s: &str) -> Self {
        let s = s.strip_prefix("fold along ").unwrap();
        let (dir, pos) = s.split_once('=').unwrap();
        match dir {
            "x" => Self::X(pos.parse().unwrap()),
            "y" => Self::Y(pos.parse().unwrap()),
            _ => panic!(),
        }
    }
}
fn parse(input: &str) -> (Vec<(i64, i64)>, Vec<FoldInstruction>) {
    let empty_line = "\r\n\r\n";
    let (dots, instructions) = input.split_once(empty_line).unwrap();
    let dots: Vec<(i64, i64)> = dots
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();
    let instructions = instructions.lines().map(FoldInstruction::parse).collect();
    (dots, instructions)
}

fn fold_dots(dots: &mut [(i64, i64)], fold: FoldInstruction) {
    match fold {
        FoldInstruction::X(axis) => dots
            .iter_mut()
            .for_each(|dot| *dot = (axis - (axis - dot.0).abs(), dot.1)),
        FoldInstruction::Y(axis) => dots
            .iter_mut()
            .for_each(|dot| *dot = (dot.0, axis - (axis - dot.1).abs())),
    }
}

fn draw_dots(mut dots: Vec<(i64, i64)>) {
    dots.sort_unstable_by_key(|(x, _)| *x);
    dots.sort_by_key(|(_, y)| *y);
    let mut prev_x = 0;
    let mut prev_y = 0;
    let mut line = String::new();
    for dot in dots {
        if prev_y != dot.1 {
            // new line
            println!("{}", line);
            line.clear();
            for _ in (prev_y + 1)..dot.1 {
                println!();
            }
            prev_y = dot.1;
        }

        for _ in (prev_x + 1)..dot.0 {
            line.push(' ');
        }
        line.push('#');
        prev_x = dot.0;
    }
    println!("{}", line);
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::day13::FoldInstruction;

    use super::{draw_dots, fold_dots, parse};
    #[test]
    fn example13_part1() {
        let input = fs::read_to_string("input/example13").unwrap();
        let (mut dots, _) = parse(&input);
        fold_dots(&mut dots, FoldInstruction::Y(7));
        dots.sort_unstable();
        dots.dedup();
        assert_eq!(dots.len(), 17);
    }
    #[test]
    fn day13_part1() {
        let input = fs::read_to_string("input/day13").unwrap();
        let (mut dots, _) = parse(&input);
        fold_dots(&mut dots, FoldInstruction::X(655));
        dots.sort_unstable();
        dots.dedup();
        assert_eq!(dots.len(), 664);
    }
    #[test]
    fn example13_part2() {
        let input = fs::read_to_string("input/example13").unwrap();
        let (mut dots, instructions) = parse(&input);
        instructions
            .iter()
            .for_each(|ins| fold_dots(&mut dots, *ins));
        dots.sort_unstable();
        dots.dedup();
        draw_dots(dots);
        // TODO: Dump drawing to file "expected_example13", then assert_eq in test
    }
    #[test]
    fn day13_part2() {
        let input = fs::read_to_string("input/day13").unwrap();
        let (mut dots, instructions) = parse(&input);
        instructions
            .iter()
            .for_each(|ins| fold_dots(&mut dots, *ins));
        dots.sort_unstable();
        dots.dedup();
        draw_dots(dots);
        // TODO: Dump drawing to file "expected_day13", then assert_eq in test
    }
}
