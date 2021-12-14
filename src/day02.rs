use std::{error::Error, str::FromStr};

#[derive(Debug, Clone, Copy)]
enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl FromStr for Command {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().split_once(' ') {
            Some(("forward", n)) => Ok(Command::Forward(n.parse()?)),
            Some(("down", n)) => Ok(Command::Down(n.parse()?)),
            Some(("up", n)) => Ok(Command::Up(n.parse()?)),
            _ => Err("Invalid command string.".into()),
        }
    }
}

fn drive_sub_part1(cmds: impl Iterator<Item = Command>) -> (i32, i32) {
    let mut x = 0;
    let mut depth = 0;
    for cmd in cmds {
        match cmd {
            Command::Forward(f) => x += f,
            Command::Down(d) => depth += d,
            Command::Up(u) => depth -= u,
        }
    }
    (x, depth)
}

fn drive_sub_part2(cmds: impl Iterator<Item = Command>) -> (i32, i32) {
    let mut x = 0;
    let mut depth = 0;
    let mut aim = 0;
    for cmd in cmds {
        match cmd {
            Command::Down(d) => aim += d,
            Command::Up(u) => aim -= u,
            Command::Forward(f) => {
                x += f;
                depth += aim * f;
            }
        }
    }
    (x, depth)
}

fn part_1(input: &str) -> (i32, i32) {
    let cmds_iter = input.lines().map(|l| l.parse().unwrap());
    drive_sub_part1(cmds_iter)
}

fn part_2(input: &str) -> (i32, i32) {
    let cmds_iter = input.lines().map(|l| l.parse().unwrap());
    drive_sub_part2(cmds_iter)
}
#[cfg(test)]
mod tests {
    use crate::day02::*;
    use std::fs;
    #[test]
    fn example_day02_part1() {
        let input = fs::read_to_string("input/example02").unwrap();
        assert_eq!(part_1(&input), (15, 10));
    }
    #[test]
    fn day02_part1() {
        let input = fs::read_to_string("input/day02").unwrap();
        let pos = part_1(&input);
        assert_eq!(pos.0 * pos.1, 1561344);
    }

    #[test]
    fn example_day02_part2() {
        let input = fs::read_to_string("input/example02").unwrap();
        let pos = part_2(&input);
        assert_eq!(pos, (15, 60));
    }
    #[test]
    fn day02_part2() {
        let input = fs::read_to_string("input/day02").unwrap();
        let pos = part_2(&input);
        assert_eq!(pos.0 * pos.1, 1848454425);
    }
}
