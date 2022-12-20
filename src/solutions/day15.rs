use crate::input;
use std::collections::HashSet;

const GOAL: i64 = 2000000;

pub fn solve() {
    let x = input::file_for_day(15);

    println!("Solution part 1: {}", part_one(x.clone(), GOAL));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: Vec<String>, goal: i64) -> i64 {
    let x = input
        .iter()
        .map(|l| {
            l.replace("Sensor at ", "")
                .replace(": closest beacon is at", ",")
                .replace("x=", "")
                .replace("y=", "")
        })
        .map(|s| {
            let mut it = s.split_terminator(", ");
            let x1 = it.next().unwrap().parse::<i64>().unwrap();
            let y1 = it.next().unwrap().parse::<i64>().unwrap();
            let x2 = it.next().unwrap().parse::<i64>().unwrap();
            let y2 = it.next().unwrap().parse::<i64>().unwrap();

            ((x1, y1), (x2, y2))
        })
        .collect::<Vec<_>>();

    let mut grid: HashSet<(i64, i64)> = HashSet::new();

    for (sensor, beacon) in x {
        let (x1, y1) = sensor;
        let (x2, y2) = beacon;
        let manhattan_distance = (x1 - x2).abs() + (y1 - y2).abs();

        // TODO: This is stupid, we don't need to check every coordinate, we could check ranges
        // instead.
        check_adjecent(&mut grid, sensor, manhattan_distance, goal);
    }

    grid.iter().filter(|(_, y)| y == &goal).count() as i64 - 1
}

fn part_two(input: Vec<String>) -> i128 {
    let parsed = input
        .iter()
        .map(|l| {
            l.replace("Sensor at ", "")
                .replace(": closest beacon is at", ",")
                .replace("x=", "")
                .replace("y=", "")
        })
        .map(|s| {
            let mut it = s.split_terminator(", ");
            let x1 = it.next().unwrap().parse::<i128>().unwrap();
            let y1 = it.next().unwrap().parse::<i128>().unwrap();
            let x2 = it.next().unwrap().parse::<i128>().unwrap();
            let y2 = it.next().unwrap().parse::<i128>().unwrap();

            ((x1, y1), (x2, y2))
        })
        .collect::<Vec<_>>();

    let max_coords = 4000000;
    let xym = |sensor: (i128, i128), beacon: (i128, i128)| -> ((i128, i128), i128) {
        let (x1, y1) = sensor;
        let (x2, y2) = beacon;
        let manhattan_distance = (x1 - x2).abs() + (y1 - y2).abs();

        ((x1, y1), manhattan_distance)
    };
    let mut candidates: HashSet<(i128, i128)> = HashSet::new();

    for (sensor, beacon) in parsed.iter() {
        let ((x, y), m) = xym(*sensor, *beacon);

        let ml = get_lines_for_box(x, y, m);
        for (other_sensor, other_beacon) in parsed.iter() {
            let ((ox, oy), om) = xym(*other_sensor, *other_beacon);
            if ox == x && oy == y {
                continue; // This is ourselves
            }

            let ol = get_lines_for_box(ox, oy, om);
            let perms = vec![
                (ml.up_right, ol.up_left),
                (ml.up_right, ol.down_right),
                (ml.down_left, ol.up_left),
                (ml.down_left, ol.down_right),
                (ml.down_right, ol.down_left),
                (ml.down_right, ol.up_right),
                (ml.up_left, ol.down_left),
                (ml.up_left, ol.up_right),
            ];

            for (me, other) in perms {
                let me_start_x = me.0 .0;
                let me_end_x = me.1 .0;
                let other_start_x = other.0 .0;
                let other_end_x = other.1 .0;

                if other_start_x >= me_end_x || other_end_x <= me_start_x {
                    continue;
                }

                let me_start_y = me.0 .1;
                let me_end_y = me.1 .1;
                let other_start_y = other.0 .1;
                let other_end_y = other.1 .1;

                if other_start_y >= me_end_y || other_end_y <= me_start_y {
                    // continue;
                }

                if let Some((px, py)) = line_intersection(me, other) {
                    if px < 0 || py < 0 || px > max_coords || py > max_coords {
                        continue;
                    }

                    if (px >= max_coords || py >= max_coords)
                        || (py < oy - om || py > oy + om)
                        || (py < y - m || py > y + m)
                        || (px < ox - om || px > ox + om)
                        || (px < x - m || px > x + m)
                    {
                        continue;
                    }

                    candidates.insert((px, py));
                }
            }
        }
    }

    let mut int = (0, 0);
    for c in &candidates {
        let (cx, cy) = c;

        let two_right = (cx + 2, *cy);
        let up_right = (cx + 1, cy - 1);
        let down_right = (cx + 1, cy + 1);
        let right = (cx + 1, *cy);

        if candidates.contains(&up_right)
            && candidates.contains(&two_right)
            && candidates.contains(&down_right)
            && !candidates.contains(&right)
        {
            int = right;
            break;
        }
    }

    int.0 * max_coords + int.1
}

struct Square {
    up_right: ((i128, i128), (i128, i128)),
    down_right: ((i128, i128), (i128, i128)),
    up_left: ((i128, i128), (i128, i128)),
    down_left: ((i128, i128), (i128, i128)),
}

fn get_lines_for_box(x: i128, y: i128, m: i128) -> Square {
    Square {
        up_right: ((x, y - m), (x + m, y)),   // Up right
        down_right: ((x, y + m), (x + m, y)), // Down right
        up_left: ((x - m, y), (x, y - m)),    // Up left
        down_left: ((x - m, y), (x, y + m)),  // Down left
    }
}

fn line_intersection(
    ((x1, y1), (x2, y2)): ((i128, i128), (i128, i128)),
    ((x3, y3), (x4, y4)): ((i128, i128), (i128, i128)),
) -> Option<(i128, i128)> {
    let xdiff = (x1 - x2, x3 - x4);
    let ydiff = (y1 - y2, y3 - y4);

    let det = |(a1, b1): (i128, i128), (a2, b2): (i128, i128)| -> i128 { a1 * b2 - a2 * b1 };
    let div = det(xdiff, ydiff);
    if div == 0 {
        return None;
    }

    let d = (det((x1, y1), (x2, y2)), det((x3, y3), (x4, y4)));
    let x = det(d, xdiff) / div;
    let y = det(d, ydiff) / div;

    Some((x, y))
}

fn check_adjecent(grid: &mut HashSet<(i64, i64)>, (x, y): (i64, i64), distance: i64, goal: i64) {
    for i in -distance..=distance {
        let y1 = y + i;
        if y1 != goal {
            continue;
        }

        let sidesteps = (i.abs() - distance).abs();
        for j in -sidesteps..=sidesteps {
            let pos = (x + j, y1);
            if grid.get(&pos).is_none() {
                grid.insert(pos);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::input;

    const TEST_GOAL: i64 = 10;
    static SOLUTION_ONE: i64 = 26;
    static SOLUTION_TWO: i128 = 56000011;
    static TEST_INPUT: &str = r#"
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#;

    #[test]
    fn part_one() {
        let x = input::test_vec(TEST_INPUT);
        assert_eq!(super::part_one(x, TEST_GOAL), SOLUTION_ONE);
    }

    #[test]
    fn part_two() {
        let x = input::test_vec(TEST_INPUT);
        assert_eq!(super::part_two(x), SOLUTION_TWO);
    }
}
