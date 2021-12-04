use std::collections::{HashMap, HashSet};

struct BingoBoard {
    numbers: HashMap<u32, (usize, usize)>,
    marked: HashSet<(usize, usize)>,
    width: usize,
    height: usize,
}

impl BingoBoard {
    fn mark_number(&mut self, n: u32) -> bool {
        if !self.numbers.contains_key(&n) {
            return false;
        }
        let pos = self.numbers.get(&n).unwrap();
        self.marked.insert(*pos);
        self.check_winner(*pos)
    }
    fn check_winner(&self, pos: (usize, usize)) -> bool {
        // check column for win
        (0..self.height)
            .map(|y| (pos.0, y))
            .all(|p| self.marked.contains(&p)) ||
        // check row for win
        (0..self.width)
            .map(|x| (x, pos.1))
            .all(|p| self.marked.contains(&p))
    }
    fn parse(s: &str) -> Self {
        let mut numbers = HashMap::new();
        for (y, line) in s.lines().enumerate() {
            for (x, n) in line.split_whitespace().enumerate() {
                let n: u32 = n.parse().expect("Failed to read board number");
                numbers.insert(n, (x, y));
            }
        }
        // TODO: Find width & height from s (or numbers?).
        Self {
            numbers,
            width: 5,
            height: 5,
            marked: HashSet::new(),
        }
    }
    fn sum_unmarked(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|(_, pos)| !self.marked.contains(pos))
            .map(|(n, _)| n)
            .sum()
    }
}

fn parse_bingo(input: &str) -> (Vec<u32>, Vec<BingoBoard>) {
    let (to_draw, board_strs) = input.split_once("\r\n\r\n").unwrap();
    let to_draw: Vec<u32> = to_draw.split(',').map(|n| n.parse().unwrap()).collect();
    let mut boards = Vec::new();
    for board_str in board_strs.split("\r\n\r\n") {
        boards.push(BingoBoard::parse(board_str));
    }
    (to_draw, boards)
}

fn play_bingo_part1(to_draw: &[u32], boards: &mut [BingoBoard]) -> u32 {
    for draw in to_draw {
        for board in boards.iter_mut() {
            if board.mark_number(*draw) {
                //winner!
                return draw * board.sum_unmarked();
            }
        }
    }
    panic!("No winner");
}

fn play_bingo_part2(to_draw: &[u32], boards: &mut [BingoBoard]) -> u32 {
    let mut last_score;
    let mut playing_count = boards.len();
    let mut playing = vec![true; boards.len()];
    for drawn in to_draw {
        for (i, board) in boards.iter_mut().enumerate() {
            if !playing[i] {
                continue;
            }
            if board.mark_number(*drawn) {
                //winner
                last_score = drawn * board.sum_unmarked();
                if playing_count == 1 {
                    return last_score;
                }
                playing_count -= 1;
                playing[i] = false;
            }
        }
    }
    panic!("No winner");
}

#[cfg(test)]
mod tests {

    use super::{parse_bingo, play_bingo_part1, play_bingo_part2};
    use std::fs;
    #[test]
    fn example04_part1() {
        let input = fs::read_to_string("input/example04").unwrap();
        let (draw, mut boards) = parse_bingo(&input);
        let ans = play_bingo_part1(&draw, &mut boards);
        dbg!(ans);
    }
    #[test]
    fn day04_part1() {
        let input = fs::read_to_string("input/day04").unwrap();
        let (draw, mut boards) = parse_bingo(&input);
        let ans = play_bingo_part1(&draw, &mut boards);
        dbg!(ans);
    }
    #[test]
    fn example04_part2() {
        let input = fs::read_to_string("input/example04").unwrap();
        let (draw, mut boards) = parse_bingo(&input);
        let ans = play_bingo_part2(&draw, &mut boards);
        dbg!(ans);
    }
    #[test]
    fn day04_part2() {
        let input = fs::read_to_string("input/day04").unwrap();
        let (draw, mut boards) = parse_bingo(&input);
        let ans = play_bingo_part2(&draw, &mut boards);
        assert_eq!(ans, 12635);
    }
}
