use std::{
    collections::HashMap,
    ops::{RangeBounds, RangeInclusive},
};

use itertools::Itertools;

fn reboot_reactor_part1(s: &str) -> HashMap<(i32, i32, i32), bool> {
    let range = |r: &str| {
        let (start, end) = r.split_once("..").unwrap();
        (start.parse::<i32>().unwrap(), end.parse::<i32>().unwrap())
    };
    let mut res = HashMap::new();
    for l in s.lines() {
        println!("new range");
        let (on_off, ranges) = l.split_once(' ').unwrap();
        let ranges = ranges
            .split(',')
            .map(|r| r.split_once('=').unwrap().1)
            .map(|r| range(r))
            .filter(|&(s, e)| (-50 < s || -50 < e) && (s <= 50 || e <= 50))
            .map(|(start, end)| start..=end)
            .collect_vec();
        if ranges.len() < 3 {
            continue;
        }
        let iter = ranges
            .into_iter()
            .multi_cartesian_product()
            .map(|v| ((v[0], v[1], v[2]), on_off == "on"))
            .filter(|(pos, _)| part_1_range(*pos));

        res.extend(iter);
    }
    res
}

fn part_1(s: &str) -> usize {
    let reactor = reboot_reactor_part1(s);
    reactor.iter().filter(|(_, on)| **on).count()
}

fn part_1_range((x, y, z): (i32, i32, i32)) -> bool {
    -50 <= x && x <= 50 && -50 <= y && y <= 50 && -50 <= z && z <= 50
}

////////// Part 2

#[derive(Clone)]
struct Cuboid {
    on: bool,
    x_range: RangeInclusive<i64>,
    y_range: RangeInclusive<i64>,
    z_range: RangeInclusive<i64>,
}

impl Cuboid {
    fn parse(s: &str) -> Self {
        let (on_off, ranges) = s.split_once(' ').unwrap();
        let ranges = ranges
            .split(',')
            .map(|r| r.split_once('=').unwrap().1)
            .collect_vec();
        let read_range = |r: &str| {
            let (start, end) = r.split_once("..").unwrap();
            start.parse::<i64>().unwrap()..=end.parse::<i64>().unwrap()
        };
        let x_range = read_range(ranges[0]);
        let y_range = read_range(ranges[1]);
        let z_range = read_range(ranges[2]);
        let on = "on" == on_off;
        Self {
            on,
            x_range,
            y_range,
            z_range,
        }
    }
    fn volume(&self) -> u64 {
        self.x_range.size_hint().0 as u64
            * self.y_range.size_hint().0 as u64
            * self.z_range.size_hint().0 as u64
    }

    fn subtract(&self, subber: &Cuboid) -> Option<Vec<Cuboid>> {
        let overlap_x = overlap_range(self.x_range.clone(), subber.x_range.clone())?;
        let overlap_y = overlap_range(self.y_range.clone(), subber.y_range.clone())?;
        let overlap_z = overlap_range(self.z_range.clone(), subber.z_range.clone())?;
        let x_ranges = [
            *self.x_range.start()..=*overlap_x.start() - 1,
            overlap_x.clone(),
            *overlap_x.end() + 1..=*self.x_range.end(),
        ];
        let y_ranges = [
            *self.y_range.start()..=*overlap_y.start() - 1,
            overlap_y.clone(),
            *overlap_y.end() + 1..=*self.y_range.end(),
        ];
        let z_ranges = [
            *self.z_range.start()..=*overlap_z.start() - 1,
            overlap_z.clone(),
            *overlap_z.end() + 1..=*self.z_range.end(),
        ];
        let mut remainder = vec![];
        [x_ranges, y_ranges, z_ranges]
            .iter()
            .multi_cartesian_product()
            .for_each(|ranges| {
                let is_overlap = ranges[0].start() == overlap_x.start()
                    && ranges[1].start() == overlap_y.start()
                    && ranges[2].start() == overlap_z.start();
                if ranges.iter().all(|r| !r.is_empty()) && !is_overlap {
                    remainder.push(Cuboid {
                        on: true,
                        x_range: ranges[0].clone(),
                        y_range: ranges[1].clone(),
                        z_range: ranges[2].clone(),
                    });
                }
            });
        Some(remainder)
    }
}

fn overlap_range(
    a_range: RangeInclusive<i64>,
    b_range: RangeInclusive<i64>,
) -> Option<RangeInclusive<i64>> {
    let b_inside_a = a_range.contains(b_range.start());
    let a_inside_b = b_range.contains(a_range.start());
    if b_inside_a || a_inside_b {
        let start = a_range.start().max(b_range.start());
        let end = a_range.end().min(b_range.end());
        Some(*start..=*end)
    } else {
        None
    }
}

fn cuboid_subtraction(cuboids: Vec<Cuboid>, subber: &Cuboid) -> Vec<Cuboid> {
    let mut rem = vec![];
    for c in cuboids {
        if let Some(r) = c.subtract(subber) {
            rem.extend(r);
        } else {
            rem.push(c);
        }
    }
    rem
}

fn part_2(input: &str) -> u64 {
    let cuboids = input.lines().map(Cuboid::parse).collect_vec();
    let mut cuboids_on: Vec<Cuboid> = vec![];
    for c in cuboids {
        if c.on {
            cuboids_on.push(c);
        } else {
            cuboids_on = cuboid_subtraction(cuboids_on, &c);
        }
    }

    let mut volume = 0;
    while let Some(next) = cuboids_on.pop() {
        volume += next.volume();
        cuboids_on = cuboid_subtraction(cuboids_on, &next);
    }
    volume
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::part_1;
    use super::part_2;
    #[test]
    fn example22_part1() {
        let input = fs::read_to_string("input/example22").unwrap();
        let ans = part_1(&input);
        assert_eq!(ans, 590784);
    }
    #[test]
    fn day22_part1() {
        let input = fs::read_to_string("input/day22").unwrap();
        let ans = part_1(&input);
        assert_eq!(ans, 589411);
    }

    #[test]
    fn example22_part2() {
        let input = fs::read_to_string("input/example22_part2").unwrap();
        let ans = part_2(&input);
        assert_eq!(ans, 2758514936282235);
    }
    #[test]
    fn day22_part2() {
        let input = fs::read_to_string("input/day22").unwrap();
        let ans = part_2(&input);
        assert_eq!(ans, 1130514303649907);
    }
}
