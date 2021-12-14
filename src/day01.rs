use itertools::Itertools;

fn part_1(input: &str) -> usize {
    let depths = input.lines().map(|l| l.parse().unwrap());
    depths
        .tuple_windows()
        .filter(|(a, b): &(u32, u32)| a < b)
        .count()
}

fn part_2(input: &str) -> usize {
    let depths = input.lines().map(|l| l.parse().unwrap());
    depths
        .tuple_windows()
        .map(|(a, b, c): (u32, u32, u32)| a + b + c)
        .fold((0, u32::MAX), |(count, prev_sum), sum| {
            if prev_sum < sum {
                (count + 1, sum)
            } else {
                (count, sum)
            }
        })
        .0
}
#[cfg(test)]
mod tests {
    use crate::day01::{part_1, part_2};
    use std::fs;
    #[test]
    fn example01_part1() {
        let input = fs::read_to_string("input/example01").unwrap();

        assert_eq!(part_1(&input), 7);
    }
    #[test]
    fn day01_part1() {
        let input = fs::read_to_string("input/day01").unwrap();
        assert_eq!(part_1(&input), 1292);
    }
    #[test]
    fn example01_part2() {
        let input = fs::read_to_string("input/example01").unwrap();

        assert_eq!(part_2(&input), 5);
    }
    #[test]
    fn day01_part2() {
        let input = fs::read_to_string("input/day01").unwrap();

        assert_eq!(part_2(&input), 1262);
    }
}
