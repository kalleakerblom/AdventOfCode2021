fn count_increasing(input: &[u32]) -> usize {
    input.windows(2).filter(|w| w[0] < w[1]).count()
}

fn count_increasing_window_sums(input: &[u32], window_size: usize) -> usize {
    input
        .windows(window_size)
        .map(|w| w.iter().sum())
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
    use crate::day01::{count_increasing, count_increasing_window_sums};
    use std::fs;
    #[test]
    fn example01_part1() {
        let input = fs::read_to_string("input/example01").unwrap();
        let depths: Vec<u32> = input.lines().map(|l| l.parse().unwrap()).collect();
        assert_eq!(count_increasing(&depths), 7);
    }
    #[test]
    fn day01_part1() {
        let input = fs::read_to_string("input/day01").unwrap();
        let depths: Vec<u32> = input.lines().map(|l| l.parse().unwrap()).collect();
        assert_eq!(count_increasing(&depths), 1292);
    }
    #[test]
    fn example01_part2() {
        let input = fs::read_to_string("input/example01").unwrap();
        let depths: Vec<u32> = input.lines().map(|l| l.parse().unwrap()).collect();
        assert_eq!(count_increasing_window_sums(&depths, 3), 5);
    }
    #[test]
    fn day01_part2() {
        let input = fs::read_to_string("input/day01").unwrap();
        let depths: Vec<u32> = input.lines().map(|l| l.parse().unwrap()).collect();
        assert_eq!(count_increasing_window_sums(&depths, 3), 1262);
    }
}
