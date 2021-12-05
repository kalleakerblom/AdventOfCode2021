use std::collections::HashMap;

type Pos = (u32, u32);
struct Line {
    start: Pos,
    end: Pos,
}

impl Line {
    fn parse(s: &str) -> Self {
        let (start, end) = s.split_once(" -> ").unwrap();
        let (start_x, start_y) = start.split_once(',').unwrap();
        let start = (start_x.parse().unwrap(), start_y.parse().unwrap());

        let (end_x, end_y) = end.split_once(',').unwrap();
        let end = (end_x.parse().unwrap(), end_y.parse().unwrap());
        Self { start, end }
    }
    fn get_points<'a>(&'a self, diagonals: bool) -> Box<dyn Iterator<Item = Pos> + 'a> {
        use std::cmp::Ordering::{Equal, Greater, Less};
        match (self.start.0.cmp(&self.end.0), self.start.1.cmp(&self.end.1)) {
            (Less, Equal) => Box::new((self.start.0..=self.end.0).map(|x| (x, self.start.1))),
            (Greater, Equal) => Box::new((self.end.0..=self.start.0).map(|x| (x, self.start.1))),
            (Equal, Less) => Box::new((self.start.1..=self.end.1).map(|y| (self.start.0, y))),
            (Equal, Greater) => Box::new((self.end.1..=self.start.1).map(|y| (self.start.0, y))),
            (Equal, Equal) => Box::new(std::iter::once(self.start)),
            // part 2
            (Less, Less) if diagonals => {
                Box::new((self.start.0..=self.end.0).zip(self.start.1..=self.end.1))
            }
            (Less, Greater) if diagonals => {
                Box::new((self.start.0..=self.end.0).zip((self.end.1..=self.start.1).rev()))
            }
            (Greater, Less) if diagonals => Box::new(
                (self.end.0..=self.start.0)
                    .rev()
                    .zip(self.start.1..=self.end.1),
            ),
            (Greater, Greater) if diagonals => Box::new(
                (self.end.0..=self.start.0)
                    .rev()
                    .zip((self.end.1..=self.start.1).rev()),
            ),
            _ => Box::new(std::iter::empty()),
        }
    }
}

fn fill_map(lines: &[Line], diagonal_lines: bool) -> HashMap<Pos, u32> {
    let mut result = HashMap::new();
    for l in lines {
        for p in l.get_points(diagonal_lines) {
            *result.entry(p).or_default() += 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{fill_map, Line};
    #[test]
    fn example05_part1() {
        let input = fs::read_to_string("input/example05").unwrap();
        let lines: Vec<Line> = input.lines().map(Line::parse).collect();
        let map = fill_map(&lines, false);
        let ans = map.iter().filter(|(_, v)| **v > 1).count();
        assert_eq!(ans, 5);
    }
    #[test]
    fn day05_part1() {
        let input = fs::read_to_string("input/day05").unwrap();
        let lines: Vec<Line> = input.lines().map(Line::parse).collect();
        let map = fill_map(&lines, false);
        let ans = map.iter().filter(|(_, v)| **v > 1).count();
        assert_eq!(ans, 8111);
    }
    #[test]
    fn example05_part2() {
        let input = fs::read_to_string("input/example05").unwrap();
        let lines: Vec<Line> = input.lines().map(Line::parse).collect();
        let map = fill_map(&lines, true);
        let ans = map.iter().filter(|(_, v)| **v > 1).count();
        assert_eq!(ans, 12);
    }
    #[test]
    fn day05_part2() {
        let input = fs::read_to_string("input/day05").unwrap();
        let lines: Vec<Line> = input.lines().map(Line::parse).collect();
        let map = fill_map(&lines, true);
        let ans = map.iter().filter(|(_, v)| **v > 1).count();
        assert_eq!(ans, 22088);
    }
}
