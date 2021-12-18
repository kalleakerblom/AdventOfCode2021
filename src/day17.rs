use std::cmp;

fn max_height_if_inside(
    win_x: (i32, i32),
    win_y: (i32, i32),
    mut vx: i32,
    mut vy: i32,
) -> Option<i32> {
    let mut x = 0;
    let mut y = 0;
    let mut max_y = 0;
    let mut inside = false;
    loop {
        if win_x.0 <= x && x <= win_x.1 && win_y.0 <= y && y <= win_y.1 {
            inside = true;
        }
        if (vx < 0 && x < win_x.0) || (vy < 0 && y < win_y.0) {
            break;
        }
        max_y = cmp::max(max_y, y);
        x += vx;
        y += vy;
        vx -= vx.signum();
        vy -= 1;
    }
    if inside {
        Some(max_y)
    } else {
        None
    }
}

fn part_1(win_x: (i32, i32), win_y: (i32, i32), range: i32) -> i32 {
    let mut max_y = 0;
    for vy in 0..range {
        for vx in 0..range {
            if let Some(my) = max_height_if_inside(win_x, win_y, vx, vy) {
                max_y = cmp::max(max_y, my);
            }
        }
    }
    max_y
}

fn part_2(win_x: (i32, i32), win_y: (i32, i32), range: i32) -> i32 {
    let mut count = 0;
    for vy in -range..range {
        for vx in 0..range {
            if max_height_if_inside(win_x, win_y, vx, vy).is_some() {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {

    use super::part_1;
    use super::part_2;
    #[test]
    fn example17_part1() {
        let ans = part_1((20, 30), (-10, -5), 100);
        assert_eq!(ans, 45);
    }
    #[test]
    fn day17_part1() {
        let ans = part_1((150, 193), (-136, -86), 400);
        assert_eq!(ans, 9180);
    }
    #[test]
    fn example17_part2() {
        let ans = part_2((20, 30), (-10, -5), 100);
        assert_eq!(ans, 112);
    }
    #[test]
    fn day17_part2() {
        let ans = part_2((150, 193), (-136, -86), 400);
        assert_eq!(ans, 3767);
    }
}
