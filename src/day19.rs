use std::collections::HashSet;

use cgmath::*;
use itertools::Itertools;

type V3d = Vector3<f64>;
type V3dI = Vector3<i64>;
type M3d = Matrix3<f64>;

const IDENTITY: M3d = M3d::new(1., 0., 0., 0., 1., 0., 0., 0., 1.);

fn get_transforms() -> Vec<M3d> {
    let cos = [1., 0., -1., 0.];
    let sin = [0., 1., 0., -1.];
    let mut res = Vec::new();
    for i in 0..4 {
        // rotation around positive/negative x
        let pos_x = Matrix3::new(1., 0., 0., 0., cos[i], sin[i], 0., -sin[i], cos[i]);
        let neg_x = Matrix3::new(-1., 0., 0., 0., -cos[i], -sin[i], 0., -sin[i], cos[i]);
        res.push(pos_x);
        res.push(neg_x);
        // rotation around positive/negative y
        let pos_y = Matrix3::new(0., 1., 0., -cos[i], 0., sin[i], sin[i], 0., cos[i]);
        let neg_y = Matrix3::new(0., -1., 0., cos[i], 0., -sin[i], sin[i], 0., cos[i]);
        res.push(pos_y);
        res.push(neg_y);
        // rotation around positive/negative z
        let pos_z = Matrix3::new(0., 0., 1., sin[i], cos[0], 0., -cos[i], sin[i], 0.);
        let neg_z = Matrix3::new(0., 0., -1., -sin[i], -cos[i], 0., -cos[i], sin[i], 0.);
        res.push(pos_z);
        res.push(neg_z);
    }

    res
}

fn parse_scanners(s: &str) -> Vec<Vec<V3d>> {
    let line_to_vector = |l: &str| {
        let mut nums = l.split(',').map(|i| i.parse::<f64>().unwrap());
        Vector3::new(
            nums.next().unwrap(),
            nums.next().unwrap(),
            nums.next().unwrap(),
        )
    };
    let parse_scanner = |block: &str| {
        block
            .lines()
            .skip(1)
            .map(|l| line_to_vector(l))
            .collect_vec()
    };
    let empty_line = "\r\n\r\n";
    s.split(empty_line).map(|b| parse_scanner(b)).collect_vec()
}

fn find_relative_position(scan_a: &[V3d], scan_b: &[V3d]) -> Option<(V3d, M3d)> {
    let transforms = get_transforms();
    for t in transforms {
        for a in scan_a {
            for j in 0..scan_b.len() {
                let translation = a - (t * scan_b[j]);
                let mut match_count = 0;
                for (_i, b) in scan_b.iter().enumerate() {
                    let tb = t * b + translation;
                    if scan_a.contains(&tb) {
                        match_count += 1;
                    }
                    // TODO: break early using i, scan_b.len() & match_count when match 12 is impossible
                }
                if match_count >= 12 {
                    return Some((translation, t));
                }
            }
        }
    }
    None
}

fn map_relations_between_scanners(scanners: &[Vec<V3d>]) -> Vec<Vec<(usize, V3d, M3d)>> {
    let mut relations = vec![vec![]; scanners.len()];
    for (i, scanner) in scanners.iter().enumerate() {
        for (j, other) in scanners.iter().enumerate() {
            if j == i {
                continue;
            }
            if let Some((vec, trans)) = find_relative_position(other, scanner) {
                relations[i].push((j, vec, trans));
            }
        }
    }
    relations
}

fn determine_beacon_set(
    scanners: &[Vec<V3d>],
    relations: &[Vec<(usize, V3d, M3d)>],
) -> HashSet<V3dI> {
    let mut to_scanner0 = vec![vec![]; scanners.len()];
    to_scanner0[0].push((V3d::new(0., 0., 0.), IDENTITY));
    loop {
        for i in 0..scanners.len() {
            if !to_scanner0[i].is_empty() {
                continue;
            }
            for r in &relations[i] {
                if !to_scanner0[r.0].is_empty() {
                    to_scanner0[i] = to_scanner0[r.0].clone();
                    to_scanner0[i].push((r.1, r.2));
                    break;
                }
            }
        }
        if to_scanner0.iter().all(|v| !v.is_empty()) {
            break;
        }
    }
    let chain_t_n_t = |v: &mut V3d, tnts: &Vec<(V3d, M3d)>| {
        tnts.iter().for_each(|(translate, transform)| {
            *v = transform * *v + translate;
        })
    };
    let mut set = HashSet::new();
    for (i, scan) in scanners.iter().enumerate() {
        let mut t_scan = scan.clone();
        t_scan
            .iter_mut()
            .rev()
            .for_each(|v| chain_t_n_t(v, &to_scanner0[i]));
        set.extend(
            t_scan
                .iter()
                .map(|v| V3dI::new(v.x as i64, v.y as i64, v.z as i64)),
        );
    }
    set
}

fn part_1(s: &str) -> usize {
    let scanners = parse_scanners(s);
    let relations = map_relations_between_scanners(&scanners);
    let set = determine_beacon_set(&scanners, &relations);
    set.len()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::part_1;

    #[test]
    fn example() {
        let input = fs::read_to_string("input/example19").unwrap();
        let ans = part_1(&input);
        assert_eq!(ans, 79);
    }
}
