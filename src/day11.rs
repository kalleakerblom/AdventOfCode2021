use itertools::Itertools;
use std::collections::HashSet;
fn simulate_flashing_octos(input: &str, steps: u32) -> (u64, Option<u32>) {
    let mut octo_map: Vec<Vec<u8>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();
    // Results
    let mut flash_count = 0;
    let mut steps_to_first_all_flash = None;
    // Run the sim
    for turn in 0..steps {
        octo_map
            .iter_mut()
            .flatten()
            .for_each(|octopus| *octopus += 1);
        let mut will_flash: Vec<(usize, usize)> = octo_map
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, &octo)| octo > 9)
                    .map(move |(x, _)| (x, y))
            })
            .collect();
        let mut has_flashed = HashSet::<(usize, usize)>::new();
        while let Some(pos) = will_flash.pop() {
            if has_flashed.contains(&pos) {
                continue;
            }
            has_flashed.insert(pos);
            flash_count += 1;
            for neighbor in get_neighbors(pos, 9, 9) {
                if has_flashed.contains(&neighbor) {
                    continue;
                }
                let neighbor_octo: &mut u8 = &mut octo_map[neighbor.1][neighbor.0];
                *neighbor_octo += 1;
                if *neighbor_octo > 9 {
                    will_flash.push(neighbor);
                }
            }
        }
        has_flashed.iter().for_each(|&(x, y)| octo_map[y][x] = 0);
        if has_flashed.len() == 100 && steps_to_first_all_flash.is_none() {
            steps_to_first_all_flash = Some(turn + 1);
        }
    }
    (flash_count, steps_to_first_all_flash)
}

fn get_neighbors((x, y): (usize, usize), max_x: usize, max_y: usize) -> Vec<(usize, usize)> {
    // TODO: Faster with i8?
    let x_vals = [x as i64 - 1, x as i64, x as i64 + 1];
    let y_vals = [y as i64 - 1, y as i64, y as i64 + 1];
    x_vals
        .iter()
        .filter(|&&x_val| 0 <= x_val && x_val <= max_x as i64)
        .cartesian_product(
            y_vals
                .iter()
                .filter(|&&y_val| 0 <= y_val && y_val <= max_y as i64),
        )
        .map(|(x, y)| (*x as usize, *y as usize))
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::simulate_flashing_octos;
    #[test]
    fn example11_part1() {
        let input = fs::read_to_string("input/example11").unwrap();
        let ans = simulate_flashing_octos(&input, 100);
        assert_eq!(ans.0, 1656);
    }
    #[test]
    fn day11_part1() {
        let input = fs::read_to_string("input/day11").unwrap();
        let ans = simulate_flashing_octos(&input, 100);
        assert_eq!(ans.0, 1640);
    }
    #[test]
    fn example11_part2() {
        let input = fs::read_to_string("input/example11").unwrap();
        let ans = simulate_flashing_octos(&input, 200);
        assert_eq!(ans.1, Some(195));
    }
    #[test]
    fn day11_part2() {
        let input = fs::read_to_string("input/day11").unwrap();
        let ans = simulate_flashing_octos(&input, 500);
        assert_eq!(ans.1, Some(312));
    }
}
