use std::collections::{HashMap, HashSet};

type CaveMap = HashMap<String, Vec<String>>;
fn read_cave_map(input: &str) -> CaveMap {
    let mut result: HashMap<String, Vec<String>> = HashMap::new();
    for l in input.lines() {
        let (cave_a, cave_b) = l.split_once('-').unwrap();
        result
            .entry(cave_a.to_string())
            .or_default()
            .push(cave_b.to_string());
        result
            .entry(cave_b.to_string())
            .or_default()
            .push(cave_a.to_string());
    }
    result
}
// TODO: Use Cow for visited small caves?
fn path_search(
    cave: &str,
    mut visited_small_caves: HashSet<String>,
    map: &CaveMap,
    path_count: &mut u32,
    mut extra_small_visit: bool,
) {
    if cave == "end" {
        *path_count += 1;
        return;
    }

    if cave.chars().all(|c| c.is_ascii_lowercase()) {
        // small cave
        if visited_small_caves.contains(cave) {
            if extra_small_visit {
                extra_small_visit = false;
            } else {
                return;
            }
        }
        visited_small_caves.insert(cave.to_string());
    }
    for next in map[cave].iter().filter(|&neighbor| neighbor != "start") {
        path_search(
            next,
            visited_small_caves.clone(),
            map,
            path_count,
            extra_small_visit,
        );
    }
}

fn part_1(input: &str) -> u32 {
    let map = read_cave_map(input);
    let mut path_count = 0;
    let extra_small_visit = false;
    path_search(
        "start",
        HashSet::new(),
        &map,
        &mut path_count,
        extra_small_visit,
    );
    path_count
}

fn part_2(input: &str) -> u32 {
    let map = read_cave_map(input);
    let mut path_count = 0;
    let extra_small_visit = true;
    path_search(
        "start",
        HashSet::new(),
        &map,
        &mut path_count,
        extra_small_visit,
    );
    path_count
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{part_1, part_2};
    #[test]
    fn example12_part1() {
        let input = fs::read_to_string("input/example12").unwrap();
        let ans = part_1(&input);
        assert_eq!(ans, 10);
    }
    #[test]
    fn day12_part1() {
        let input = fs::read_to_string("input/day12").unwrap();
        let ans = part_1(&input);

        assert_eq!(ans, 3495);
    }
    #[test]
    fn day12_part2() {
        let input = fs::read_to_string("input/day12").unwrap();
        let ans = part_2(&input);
        assert_eq!(ans, 94849);
    }
}
