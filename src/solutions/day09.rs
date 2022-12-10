use crate::input;
use std::collections::HashSet;

pub fn solve() {
    let x = input::file_for_day(9);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

#[derive(Debug)]
enum Direction {
    Up,
    UpRight,
    UpLeft,
    Down,
    DownRight,
    DownLeft,
    Left,
    Right,
}

impl Direction {
    fn step(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::UpRight => (-1, 1),
            Direction::UpLeft => (-1, -1),
            Direction::Down => (1, 0),
            Direction::DownRight => (1, 1),
            Direction::DownLeft => (1, -1),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    fn walk(&self, (from_x, from_y): (i32, i32)) -> (i32, i32) {
        let (xd, yd) = self.step();
        (from_x + xd, from_y + yd)
    }
}

fn approach((from_x, from_y): (i32, i32), (target_x, target_y): (i32, i32)) -> (i32, i32) {
    if (target_x - from_x).abs() <= 1 && (target_y - from_y).abs() <= 1 {
        return (from_x, from_y);
    }

    match (target_x, target_y) {
        (x, y) if y == from_y && x < from_x => Direction::Up.walk((from_x, from_y)),
        (x, y) if y == from_y && x > from_x => Direction::Down.walk((from_x, from_y)),
        (x, y) if x == from_x && y < from_y => Direction::Left.walk((from_x, from_y)),
        (x, y) if x == from_x && y > from_y => Direction::Right.walk((from_x, from_y)),
        (x, y) if x < from_x && y > from_y => Direction::UpRight.walk((from_x, from_y)),
        (x, y) if x < from_x && y < from_y => Direction::UpLeft.walk((from_x, from_y)),
        (x, y) if x > from_x && y > from_y => Direction::DownRight.walk((from_x, from_y)),
        (x, y) if x > from_x && y < from_y => Direction::DownLeft.walk((from_x, from_y)),

        _ => unreachable!(),
    }
}

fn part_one(input: Vec<String>) -> i32 {
    move_rope(&input, 2)
}

fn part_two(input: Vec<String>) -> i32 {
    move_rope(&input, 10)
}

fn move_rope(input: &[String], length: usize) -> i32 {
    let mut rope = vec![(0, 0); length];
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));

    input
        .iter()
        .map(|line| {
            let (dir, ns) = line.split_once(' ').unwrap();
            let n = ns.parse::<i32>().unwrap();

            match dir {
                "U" => (Direction::Up, n),
                "D" => (Direction::Down, n),
                "L" => (Direction::Left, n),
                "R" => (Direction::Right, n),
                _ => unreachable!(),
            }
        })
        .for_each(|(dir, steps)| {
            for _ in 0..steps {
                rope[0] = dir.walk(rope[0]);

                for knot in 1..rope.len() {
                    let this = rope[knot];
                    let next = rope[knot - 1];
                    let new_pos = approach(this, next);
                    rope[knot] = new_pos;
                }

                let _ = visited.insert(rope[rope.len() - 1]);
            }
        });

    visited.len() as i32
}

#[allow(dead_code)]
fn show(size: i32, visited: &HashSet<(i32, i32)>, rope: &[(i32, i32)]) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    for x in -size..=size {
        for y in -size..=size {
            let seen = visited.get(&(x, y));

            let c = match (x, y) {
                (a, b) if a == 0 && b == 0 => "⭐️",
                (a, b) if rope[0] == (a, b) => "🤖",
                (a, b) if rope.contains(&(a, b)) => "💢",
                (_, _) if seen.is_some() => "⬛️",
                (_, _) => "  ",
            };

            print!("{}", c);
        }

        println!();
    }
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i32 = 13;
    static SOLUTION_TWO: i32 = 36;
    static TEST_INPUT_ONE: &str = r#"
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;
    static TEST_INPUT_TWO: &str = r#"
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;

    #[test]
    fn part_one() {
        let x = input::test_vec(TEST_INPUT_ONE);
        assert_eq!(super::part_one(x), SOLUTION_ONE);
    }

    #[test]
    fn part_two() {
        let x = input::test_vec(TEST_INPUT_TWO);
        assert_eq!(super::part_two(x), SOLUTION_TWO);
    }
}
