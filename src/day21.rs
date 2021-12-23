use std::collections::HashMap;

use itertools::Itertools;

#[derive(Clone)]
struct Player {
    score: u16,
    goal: u16,
    pos: u16,
}

impl Player {
    fn new(start_score: u16, goal: u16) -> Self {
        Self {
            score: 0,
            pos: start_score - 1,
            goal,
        }
    }
    fn step(&mut self, steps: u16) -> bool {
        self.pos = (self.pos + steps) % 10;
        self.score += self.pos + 1;
        self.score >= self.goal
    }
}

struct DetDice(u16);

impl DetDice {
    fn new() -> Self {
        Self(0)
    }
    fn roll(&mut self) -> u16 {
        let res = self.0;
        self.0 = (self.0 + 1) % 100;
        res + 1
    }
}

fn part_1(start_1: u16, start_2: u16) -> u64 {
    let mut players = [Player::new(start_1, 1000), Player::new(start_2, 1000)];
    let mut die = DetDice::new();
    let mut rolls = 0;
    for i in 0.. {
        let p: &mut Player = &mut players[i % 2];
        rolls += 3;
        let winner = p.step(die.roll() + die.roll() + die.roll());
        if winner {
            return rolls * players[(i + 1) % 2].score as u64;
        }
    }
    panic!()
}

enum GameResult {
    P1(u64),
    P2(u64),
    Undetermined(u64),
}

const DICE_SUM_AND_MULTIPLIER: [(u16, u64); 7] =
    [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

fn play_subgame(
    mut p1: Player,
    mut p2: Player,
    p1_turn: bool,
    steps: u16,
    universe_count: u64,
    p1_wins: &mut u64,
    p2_wins: &mut u64,
) {
    if p1_turn {
        if p1.step(steps) {
            *p1_wins += universe_count;
            return;
        }
    } else if p2.step(steps) {
        *p2_wins += universe_count;
        return;
    }
    // No winner yet
    for out in DICE_SUM_AND_MULTIPLIER {
        play_subgame(
            p1.clone(),
            p2.clone(),
            !p1_turn,
            out.0,
            universe_count * out.1,
            p1_wins,
            p2_wins,
        )
    }
}
fn count_wins(p1: Player, p2: Player) -> (u64, u64) {
    let mut p1_wins = 0;
    let mut p2_wins = 0;
    for out in DICE_SUM_AND_MULTIPLIER {
        play_subgame(
            p1.clone(),
            p2.clone(),
            true,
            out.0,
            out.1,
            &mut p1_wins,
            &mut p2_wins,
        )
    }
    (p1_wins, p2_wins)
}

fn part_2(start_1: u16, start_2: u16) -> u64 {
    let (p1_wins, p2_wins) = count_wins(Player::new(start_1, 21), Player::new(start_2, 21));
    p1_wins.max(p2_wins)
}

#[cfg(test)]
mod tests {

    use super::part_1;
    use super::part_2;
    #[test]
    fn example21_part1() {
        let ans = part_1(4, 8);
        assert_eq!(ans, 739785);
    }
    #[test]
    fn day21_part1() {
        let ans = part_1(4, 5);
        assert_eq!(ans, 864900);
    }

    #[test]
    fn example21_part2() {
        let ans = part_2(4, 8);
        assert_eq!(ans, 444356092776315);
    }
    #[test]
    fn day21_part2() {
        let ans = part_2(4, 5);
        assert_eq!(ans, 575111835924670);
        // 726 ms
    }
}
