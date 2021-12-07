fn find_min_cost(crabs: &[i32]) -> i32 {
    let mut pos = *crabs.iter().min().unwrap();
    let mut min_cost = crabs.iter().map(|c| (c - pos).abs()).sum();
    loop {
        pos += 1;
        let new_cost: i32 = crabs.iter().map(|c| (c - pos).abs()).sum();
        if new_cost < min_cost {
            min_cost = new_cost;
        } else {
            break min_cost;
        }
    }
}

fn find_min_cost_part2(crabs: &[i32]) -> i32 {
    let min_pos = *crabs.iter().min().unwrap();
    let max_pos = *crabs.iter().max().unwrap();
    let mut min_cost = i32::MAX;
    for pos in min_pos..=max_pos {
        let new_cost: i32 = crabs
            .iter()
            .map(|c| (c - pos).abs())
            .map(|steps| (1..=steps).sum::<i32>())
            .sum();
        if new_cost < min_cost {
            min_cost = new_cost;
        }
    }
    min_cost
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::day07::find_min_cost_part2;

    use super::find_min_cost;
    const EXAMPLE_07: &str = "16,1,2,0,4,2,7,1,2,14";
    #[test]
    fn example07_part1() {
        let crabs: Vec<i32> = EXAMPLE_07
            .split(',')
            .map(|n| n.trim().parse().unwrap())
            .collect();
        let ans = find_min_cost(&crabs);
        assert_eq!(ans, 37);
    }
    #[test]
    fn day07_part1() {
        let input = fs::read_to_string("input/day07").unwrap();
        let crabs: Vec<i32> = input
            .split(',')
            .map(|n| n.trim().parse().unwrap())
            .collect();
        let ans = find_min_cost(&crabs);
        assert_eq!(ans, 352997);
    }
    #[test]
    fn example07_part2() {
        let crabs: Vec<i32> = EXAMPLE_07
            .split(',')
            .map(|n| n.trim().parse().unwrap())
            .collect();
        let ans = find_min_cost_part2(&crabs);
        assert_eq!(ans, 168);
    }
    #[test]
    fn day07_part2() {
        let input = fs::read_to_string("input/day07").unwrap();
        let crabs: Vec<i32> = input
            .split(',')
            .map(|n| n.trim().parse().unwrap())
            .collect();
        let ans = find_min_cost_part2(&crabs);
        assert_eq!(ans, 101571302);
    }
}
