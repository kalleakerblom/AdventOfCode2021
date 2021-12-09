use std::collections::HashMap;

fn simulate_fish_part1(mut fish: Vec<u8>, days: usize) -> usize {
    let mut next_fish = Vec::with_capacity(fish.len());
    for _ in 0..days {
        for &f in &fish {
            if f > 0 {
                next_fish.push(f - 1);
            } else {
                next_fish.push(6);
                next_fish.push(8);
            }
        }
        std::mem::swap(&mut fish, &mut next_fish);
        next_fish.clear();
    }
    fish.len()
}
fn recursive_fish_count(fish: usize, mut days: usize, memo: &mut HashMap<usize, usize>) -> usize {
    if fish >= days {
        return 1;
    }
    days -= fish + 1;
    if let Some(count) = memo.get(&days) {
        return *count;
    }
    let start_days = days;
    let mut count = 1;
    count += recursive_fish_count(8, days, memo);
    while days >= 7 {
        days -= 7;
        count += recursive_fish_count(8, days, memo);
    }
    memo.insert(start_days, count);
    count
}
fn simulate_fish_fast_part2(fish: &[u8], days: usize) -> usize {
    let mut result = 0;
    let mut memo = HashMap::new();
    for f in fish {
        result += recursive_fish_count(*f as usize, days, &mut memo);
    }
    result
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::day06::simulate_fish_fast_part2;

    use super::simulate_fish_part1;
    const EXAMPLE_06: &str = "3,4,3,1,2";
    #[test]
    fn example06_part1() {
        let fish = EXAMPLE_06.split(',').map(|n| n.parse().unwrap()).collect();
        let ans = simulate_fish_part1(fish, 80);
        assert_eq!(ans, 5934);
    }
    #[test]
    fn day06_part1() {
        let input = fs::read_to_string("input/day06").unwrap();
        let fish: Vec<u8> = input
            .split(',')
            .map(|n| n.trim().parse().unwrap())
            .collect();
        let ans = simulate_fish_part1(fish, 80);
        assert_eq!(ans, 358214);
    }

    #[test]
    fn example06_part2() {
        let fish: Vec<u8> = EXAMPLE_06.split(',').map(|n| n.parse().unwrap()).collect();
        let ans = simulate_fish_fast_part2(&fish, 256);
        assert_eq!(ans, 26984457539);
    }
    #[test]
    fn day06_part2() {
        let input = fs::read_to_string("input/day06").unwrap();
        let fish: Vec<u8> = input
            .split(',')
            .map(|n| n.trim().parse().unwrap())
            .collect();
        let ans = simulate_fish_fast_part2(&fish, 256);
        assert_eq!(ans, 1622533344325);
    }
}
