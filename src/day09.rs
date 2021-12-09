use std::collections::HashMap;

type Pos = (usize, usize);
fn is_low_point(map: &[Vec<u32>], (x, y): Pos) -> bool {
    let center = map[y][x];
    if y != 0 && map[y - 1][x] <= center {
        return false;
    }
    if x != 0 && map[y][x - 1] <= center {
        return false;
    }
    if y != map.len() - 1 && map[y + 1][x] <= center {
        return false;
    }
    if x != map[0].len() - 1 && map[y][x + 1] <= center {
        return false;
    }
    true
}

fn sum_risk_levels(height_map: &[Vec<u32>]) -> u32 {
    get_low_points(height_map)
        .iter()
        .map(|&(x, y)| 1 + height_map[y][x])
        .sum()
}

fn get_low_points(map: &[Vec<u32>]) -> Vec<Pos> {
    let mut result = vec![];
    let width = map[0].len();
    for y in 0..map.len() {
        for x in 0..width {
            if is_low_point(map, (x, y)) {
                result.push((x, y));
            }
        }
    }
    result
}

//// Part 2

fn get_basin_map(map: &[Vec<u32>]) -> Vec<Vec<usize>> {
    let mut basin = vec![vec![0; map[0].len()]; map.len()];
    // Fill in the low points
    for (id, lp) in get_low_points(map).iter().enumerate() {
        basin[lp.1][lp.0] = id + 1; // first id 1
    }
    // Fill in all points
    for y in 0..basin.len() {
        for x in 0..basin[0].len() {
            if map[y][x] == 9 {
                continue; // Not part of any basin
            }
            if basin[y][x] != 0 {
                continue; // Filled in already
            }
            let mut stack = vec![(x, y)];
            let mut lower = find_lower_point(map, (x, y));
            while basin[lower.1][lower.0] == 0 {
                stack.push(lower);
                lower = find_lower_point(map, lower);
            }
            let basin_id = basin[lower.1][lower.0];
            for p in stack {
                basin[p.1][p.0] = basin_id;
            }
        }
    }
    basin
}

fn find_lower_point(map: &[Vec<u32>], (x, y): Pos) -> Pos {
    let center = map[y][x];
    if y != 0 && map[y - 1][x] < center {
        return (x, y - 1);
    }
    if x != 0 && map[y][x - 1] < center {
        return (x - 1, y);
    }
    if y != map.len() - 1 && map[y + 1][x] < center {
        return (x, y + 1);
    }
    if x != map[0].len() - 1 && map[y][x + 1] < center {
        return (x + 1, y);
    }
    (x, y)
}

fn multiply_three_largest_basin_areas(basin_map: &[Vec<usize>]) -> u32 {
    let mut basin_areas: HashMap<usize, u32> = HashMap::new();
    for id in basin_map.iter().flatten().filter(|&&b_id| b_id != 0) {
        *basin_areas.entry(*id).or_default() += 1;
    }
    let mut basin_areas: Vec<u32> = basin_areas.values().cloned().collect();
    // TODO: Only sort top 3
    basin_areas.sort_unstable();
    basin_areas.iter().rev().take(3).product()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::day09::{get_basin_map, multiply_three_largest_basin_areas};

    use super::sum_risk_levels;
    #[test]
    fn example09_part1() {
        let input = fs::read_to_string("input/example09").unwrap();
        let height_map: Vec<Vec<u32>> = input
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();
        let ans = sum_risk_levels(&height_map);
        assert_eq!(ans, 15);
    }
    #[test]
    fn day09_part1() {
        let input = fs::read_to_string("input/day09").unwrap();
        let height_map: Vec<Vec<u32>> = input
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();
        let ans = sum_risk_levels(&height_map);
        assert_eq!(ans, 566);
    }
    #[test]
    fn example09_part2() {
        let input = fs::read_to_string("input/example09").unwrap();
        let height_map: Vec<Vec<u32>> = input
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();
        let basin_map = get_basin_map(&height_map);
        let ans = multiply_three_largest_basin_areas(&basin_map);
        assert_eq!(ans, 1134);
    }
    #[test]
    fn day09_part2() {
        let input = fs::read_to_string("input/day09").unwrap();
        let height_map: Vec<Vec<u32>> = input
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();
        let basin_map = get_basin_map(&height_map);
        let ans = multiply_three_largest_basin_areas(&basin_map);
        assert_eq!(ans, 891684);
    }
}
