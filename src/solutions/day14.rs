use crate::input;
use std::collections::HashMap;

pub fn solve() {
    let x = input::file_for_day(14);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: Vec<String>) -> i64 {
    drop_with_or_without_floor(&input, false)
}

fn part_two(input: Vec<String>) -> i64 {
    drop_with_or_without_floor(&input, true)
}

fn drop_with_or_without_floor(input: &[String], with_floor: bool) -> i64 {
    let x = input
        .iter()
        .map(|l| l.split(" -> ").collect::<Vec<_>>())
        .map(|items| {
            items
                .iter()
                .map(|&item| {
                    let (x, y) = item.split_once(',').unwrap();
                    (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut grid = Cave::new((480, 0), 25, with_floor);

    for ex in x {
        for i in 0..ex.len() - 1 {
            let (start, stop) = (ex[i], ex[i + 1]);
            grid.draw_rock(start, stop);
        }
    }

    while grid.drop_sand() {}

    grid.inner.iter().filter(|&(_, v)| v == &Item::Sand).count() as i64
}

#[derive(Debug)]
struct Cave {
    start: (i32, i32),
    size: i32,
    lowest_floor: i32,
    with_floor: bool,
    inner: HashMap<(i32, i32), Item>,
}

impl std::fmt::Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (x1, y1) = self.start;

        let mut s = "".to_string();

        for y in y1..y1 + self.size {
            s.push_str(format!("{:<4} ", y).as_str());

            for x in x1..x1 + self.size {
                let item = self.get((x, y));
                s.push_str(format!("{}", item).as_str());
            }

            s.push('\n');
        }

        write!(f, "{}", s)
    }
}

impl Cave {
    fn new(start: (i32, i32), size: i32, with_floor: bool) -> Self {
        let mut inner = HashMap::new();
        inner.insert((500, 0), Item::Source);

        Self {
            start,
            size,
            inner,
            with_floor,
            lowest_floor: 0,
        }
    }

    fn draw_rock(&mut self, mut start: (i32, i32), stop: (i32, i32)) {
        let step = match (start, stop) {
            ((x1, _), (x2, _)) if x1 > x2 => (-1, 0),
            ((x1, _), (x2, _)) if x1 < x2 => (1, 0),
            ((_, y1), (_, y2)) if y1 < y2 => (0, 1),
            ((_, y1), (_, y2)) if y1 > y2 => (0, -1),
            (_, _) => unreachable!(),
        };

        if stop.1 > self.lowest_floor {
            if self.with_floor {
                self.lowest_floor = stop.1 + 2
            } else {
                self.lowest_floor = stop.1
            }
        }

        while start != stop {
            self.inner.insert(start, Item::Rock);
            start = (start.0 + step.0, start.1 + step.1);
        }

        self.inner.insert(start, Item::Rock);
    }

    #[allow(dead_code)]
    fn draw_with_sand(&mut self, sand: (i32, i32)) {
        self.inner.insert(sand, Item::Sand);
        input::print_and_wait(format!("{}", self));

        let restore = if sand == (500, 0) {
            Item::Source
        } else {
            Item::Air
        };

        self.inner.insert(sand, restore);
    }

    fn get(&self, pos: (i32, i32)) -> &Item {
        if self.with_floor && pos.1 == self.lowest_floor {
            &Item::Rock
        } else {
            self.inner.get(&pos).unwrap_or(&Item::Air)
        }
    }

    fn drop_sand(&mut self) -> bool {
        let mut sand = (500, 0);

        loop {
            let down = (sand.0, sand.1 + 1);
            let down_left = (sand.0 - 1, sand.1 + 1);
            let down_right = (sand.0 + 1, sand.1 + 1);

            if down.1 > self.lowest_floor {
                return false;
            }

            // self.draw_with_sand(sand);

            if self.get(down) == &Item::Air {
                sand = down;
            } else if self.get(down_left) == &Item::Air {
                sand = down_left;
            } else if self.get(down_right) == &Item::Air {
                sand = down_right;
            } else {
                self.inner.insert(sand, Item::Sand);
                return sand.1 != 0;
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum Item {
    Source,
    Rock,
    Air,
    Sand,
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            Item::Source => "+",
            Item::Air => " ",
            Item::Rock => "#",
            Item::Sand => "o",
        };

        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i64 = 24;
    static SOLUTION_TWO: i64 = 93;
    static TEST_INPUT: &str = r#"
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;

    #[test]
    fn part_one() {
        let x = input::test_vec(TEST_INPUT);
        assert_eq!(super::part_one(x), SOLUTION_ONE);
    }

    #[test]
    fn part_two() {
        let x = input::test_vec(TEST_INPUT);
        assert_eq!(super::part_two(x), SOLUTION_TWO);
    }
}
