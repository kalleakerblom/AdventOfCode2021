use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

type RiskMap = Vec<Vec<u32>>;
type Pos = (usize, usize);

fn get_neighbors((x, y): Pos, max_x: usize, max_y: usize) -> impl Iterator<Item = Pos> {
    let (x, y, max_x, max_y) = (x as i64, y as i64, max_x as i64, max_y as i64);
    let neighbors = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
    neighbors
        .into_iter()
        .filter(move |&(nx, ny)| 0 <= nx && nx <= max_x && 0 <= ny && ny <= max_y)
        .map(|(nx, ny)| (nx as usize, ny as usize))
}

fn min_risk_path(risk_map: &RiskMap) -> u32 {
    let end = (risk_map[0].len() - 1, risk_map.len() - 1);
    let mut frontier = BinaryHeap::<Reverse<(u32, Pos)>>::new(); // Need Reverse to get min-heap
    frontier.push(Reverse((0, (0, 0))));
    let mut min_path_risk_map = HashMap::new();
    while let Some(Reverse((curr_risk, curr_pos))) = frontier.pop() {
        if curr_pos == end {
            return curr_risk;
        }
        for neighbor_pos in get_neighbors(curr_pos, end.0, end.1) {
            let (nx, ny) = neighbor_pos;
            let new_risk = curr_risk + risk_map[ny][nx];
            let min_path_risk = min_path_risk_map.entry(neighbor_pos).or_insert(u32::MAX);
            if new_risk < *min_path_risk {
                *min_path_risk = new_risk;
                frontier.push(Reverse((new_risk, neighbor_pos)));
            }
        }
    }
    panic!("Goal not found.");
}

fn build_full_map(map: &RiskMap) -> RiskMap {
    let small_width = map[0].len();
    let small_height = map.len();
    let calc_risk = |x: usize, y: usize| -> u32 {
        let bonus_risk = x / small_width + y / small_height;
        let small_x = x % small_width;
        let small_y = y % small_height;
        let mut cell_risk = bonus_risk as u32 + map[small_y][small_x];
        while cell_risk > 9 {
            cell_risk -= 9;
        }
        cell_risk
    };
    (0..small_height * 5)
        .map(|y| (0..small_width * 5).map(|x| calc_risk(x, y)).collect())
        .collect()
}

fn part_1(input: &str) -> u32 {
    let risk_map = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    min_risk_path(&risk_map)
}

fn part_2(input: &str) -> u32 {
    let risk_map = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let full_map = build_full_map(&risk_map);
    min_risk_path(&full_map)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::part_1;
    use super::part_2;
    #[test]
    fn example15_part1() {
        let input = fs::read_to_string("input/example15").unwrap();
        let ans = part_1(&input);
        assert_eq!(ans, 40);
    }
    #[test]
    fn day15_part1() {
        let input = fs::read_to_string("input/day15").unwrap();
        let ans = part_1(&input);
        assert_eq!(ans, 373);
    }
    #[test]
    fn example15_part2() {
        let input = fs::read_to_string("input/example15").unwrap();
        let ans = part_2(&input);
        assert_eq!(ans, 315);
    }
    #[test]
    fn day15_part2() {
        let input = fs::read_to_string("input/day15").unwrap();
        let ans = part_2(&input);
        assert_eq!(ans, 2868);
    }
}
