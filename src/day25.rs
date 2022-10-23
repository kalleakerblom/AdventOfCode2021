enum Tile {
    Empty,
    Right,
    Down,
}

impl Tile {
    fn parse(c: char) -> Self {
        match c {
            '>' => Tile::Right,
            'v' => Tile::Down,
            _ => Tile::Empty,
        }
    }
}

type Tiles = Vec<Vec<Tile>>;

fn step(tiles: &mut Tiles) -> bool {
    let mut moved;
    // Move the east-facing cucumbers
    {
        let width = tiles[0].len();
        let next_east = |x, y| ((x + 1) % width, y);
        let mut to_move = vec![];
        for y in 0..tiles.len() {
            for x in 0..tiles[0].len() {
                if let Tile::Right = tiles[y][x] {
                    let (nx, ny) = next_east(x, y);
                    if let Tile::Empty = tiles[ny][nx] {
                        to_move.push((x, y));
                    }
                }
            }
        }
        moved = !to_move.is_empty();
        for (x, y) in to_move {
            let (nx, ny) = next_east(x, y);
            tiles[y][x] = Tile::Empty;
            tiles[ny][nx] = Tile::Right;
        }
    }
    // Move the south-facing cucumbers
    {
        let height = tiles.len();
        let next_south = |x, y| (x, (y + 1) % height);
        let mut to_move = vec![];
        for y in 0..tiles.len() {
            for x in 0..tiles[0].len() {
                if let Tile::Down = tiles[y][x] {
                    let (nx, ny) = next_south(x, y);
                    if let Tile::Empty = tiles[ny][nx] {
                        to_move.push((x, y));
                    }
                }
            }
        }
        moved = moved || !to_move.is_empty();
        for (x, y) in to_move {
            let (nx, ny) = next_south(x, y);
            tiles[y][x] = Tile::Empty;
            tiles[ny][nx] = Tile::Down;
        }
    }
    moved
}

fn parse_tiles(s: &str) -> Tiles {
    let mut result = Tiles::new();
    for l in s.lines() {
        let mut row = vec![];
        for c in l.chars() {
            row.push(Tile::parse(c));
        }
        result.push(row);
    }
    result
}

fn part_1(input: &str) -> u32 {
    let mut tiles = parse_tiles(input);
    for i in 1.. {
        if !step(&mut tiles) {
            return i;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::part_1;
    use std::fs;
    #[test]
    fn example25_part1() {
        let input = fs::read_to_string("input/example25").unwrap();
        assert_eq!(part_1(&input), 58);
    }
    #[test]
    fn day25_part1() {
        let input = fs::read_to_string("input/day25").unwrap();
        assert_eq!(part_1(&input), 598);
    }
}
