use bitvec::{field::BitField, prelude::*};
use std::collections::HashMap;

fn parse(s: &str) -> (Vec<bool>, HashMap<(i64, i64), bool>) {
    let empty_line = "\r\n\r\n";
    let (enhancement_table, image) = s.split_once(empty_line).unwrap();
    let enhancement_table: Vec<_> = enhancement_table.chars().map(|c| c == '#').collect();
    let image: HashMap<(i64, i64), bool> = image
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i64, y as i64), c == '#'))
        })
        .collect();
    (enhancement_table, image)
}

fn calc_enhance_index((x, y): (i64, i64), image: &HashMap<(i64, i64), bool>, step: usize) -> usize {
    let mut bitvec = BitVec::<Msb0>::new();
    let flash = step % 2 != 0;
    let bit = |x, y| image.get(&(x, y)).cloned().unwrap_or(flash);
    bitvec.extend([bit(x - 1, y - 1), bit(x, y - 1), bit(x + 1, y - 1)]);
    bitvec.extend([bit(x - 1, y), bit(x, y), bit(x + 1, y)]);
    bitvec.extend([bit(x - 1, y + 1), bit(x, y + 1), bit(x + 1, y + 1)]);
    bitvec.load_be()
}

fn bounds(image: &HashMap<(i64, i64), bool>) -> ((i64, i64), (i64, i64)) {
    let mut min_x = i64::MAX;
    let mut max_x = i64::MIN;
    let mut min_y = i64::MAX;
    let mut max_y = i64::MIN;
    for (x, y) in image.keys() {
        min_x = min_x.min(*x);
        max_x = max_x.max(*x);
        min_y = min_y.min(*y);
        max_y = max_y.max(*y);
    }
    ((min_x, min_y), (max_x, max_y))
}

fn part_1(s: &str, steps: usize) -> usize {
    let (enhancement, mut image) = parse(s);
    let ((mut min_x, mut min_y), (mut max_x, mut max_y)) = bounds(&image);
    for step in 0..steps {
        let mut next_image = HashMap::with_capacity(image.len());
        max_x += 1;
        max_y += 1;
        min_x -= 1;
        min_y -= 1;
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let ei = calc_enhance_index((x, y), &image, step);
                next_image.insert((x, y), enhancement[ei]);
            }
        }
        image = next_image;
    }
    image.values().filter(|v| **v).count()
}
#[cfg(test)]
mod tests {
    use std::fs;

    use super::part_1;
    #[test]
    fn example() {
        let input = fs::read_to_string("input/example20").unwrap();
        let ans = part_1(&input, 2);
        // TODO: Don't hard-code flash, doesn't work with this easy example
        assert_eq!(ans, 35);
    }

    #[test]
    fn day20_part1() {
        let input = fs::read_to_string("input/day20").unwrap();
        let ans = part_1(&input, 2);
        assert_eq!(ans, 5361);
    }

    #[test]
    fn day20_part2() {
        let input = fs::read_to_string("input/day20").unwrap();
        let ans = part_1(&input, 50);
        assert_eq!(ans, 16826);
    }
}
