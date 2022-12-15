use crate::input;
use std::collections::HashMap;

const GOAL: i32 = 2000000;

pub fn solve() {
    let x = input::file_for_day(15);

    println!("Solution part 1: {}", part_one(x.clone(), GOAL));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: Vec<String>, goal: i32) -> i64 {
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
            let x1 = it.next().unwrap().parse::<i32>().unwrap();
            let y1 = it.next().unwrap().parse::<i32>().unwrap();
            let x2 = it.next().unwrap().parse::<i32>().unwrap();
            let y2 = it.next().unwrap().parse::<i32>().unwrap();

            ((x1, y1), (x2, y2))
        })
        .collect::<Vec<_>>();

    let mut grid: HashMap<(i32, i32), Source> = HashMap::new();

    for (sensor, beacon) in x {
        let (x1, y1) = sensor;
        let (x2, y2) = beacon;
        let manhattan_distance = (x1 - x2).abs() + (y1 - y2).abs();

        check_adjecent(&mut grid, sensor, manhattan_distance, goal);

        // Just to print example output.
        grid.insert(sensor, Source::Sensor);
        grid.insert(beacon, Source::Beacon);
    }

    grid.iter()
        .filter(|((_, y), source)| y == &goal && source != &&Source::Beacon)
        .count() as i64
}

fn part_two(_input: Vec<String>) -> i64 {
    0
}

#[derive(Debug, PartialEq)]
enum Source {
    Sensor,
    Beacon,
    Invalid,
}

#[allow(dead_code)]
fn print_grid(grid: &HashMap<(i32, i32), Source>) {
    let (start, stop) = (-10, 35);
    print!("{:<3}", "");
    for i in start..=stop {
        print!("{:<3}", i);
    }
    println!();
    for y in start..=stop {
        print!("{:<3}", y);

        for x in start..=stop {
            let c = match grid.get(&(x, y)) {
                Some(Source::Sensor) => 'S',
                Some(Source::Beacon) => 'B',
                Some(Source::Invalid) => '#',
                _ => '.',
            };

            print!("{:<3}", c)
        }
        println!()
    }
}

fn check_adjecent(
    grid: &mut HashMap<(i32, i32), Source>,
    (x, y): (i32, i32),
    distance: i32,
    goal: i32,
) {
    for i in -distance..=distance {
        let y1 = y + i;
        if y1 != goal {
            continue;
        }

        let sidesteps = (i.abs() - distance).abs();
        for j in -sidesteps..=sidesteps {
            let pos = (x + j, y1);
            if grid.get(&pos).is_none() {
                grid.insert(pos, Source::Invalid);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::input;

    const TEST_GOAL: i32 = 10;
    static SOLUTION_ONE: i64 = 26;
    static SOLUTION_TWO: i64 = 0;
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
