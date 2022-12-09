use crate::input;
use std::collections::HashSet;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn step(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

pub fn solve() {
    let x = input::file_for_day(8);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: Vec<String>) -> i32 {
    let grid = input
        .iter()
        .map(|row| {
            row.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let height = grid.len() - 1;
    let width = grid[0].len() - 1;
    let edges_count = (((width + 1) * 2 + (height + 1) * 2) - 4) as i32;

    let mut visible: HashSet<(i32, i32)> = HashSet::new();

    for (x, row) in grid.iter().enumerate() {
        for (y, me) in row.iter().enumerate() {
            let direction_to_check = match (x, y) {
                // Ignore corners.
                (0, 0) => None,
                (row, col) if row == 0 && col == width => None,
                (row, col) if row == height && col == 0 => None,
                (row, col) if row == height && col == width => None,

                (row, _) if row == 0 => Some(Direction::Down),
                (row, _) if row == height => Some(Direction::Up),
                (_, col) if col == 0 => Some(Direction::Right),
                (_, col) if col == width => Some(Direction::Left),

                (_, _) => None,
            };

            if let Some(direction) = direction_to_check {
                check_direction(*me, (x as i32, y as i32), direction, &grid, &mut visible);
            }
        }
    }

    edges_count + visible.len() as i32
}

fn part_two(input: Vec<String>) -> i32 {
    let grid = input
        .iter()
        .map(|row| {
            row.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut sums = Vec::new();
    for (x, row) in grid.iter().enumerate() {
        for (y, me) in row.iter().enumerate() {
            let mut distances = Vec::new();
            for direction in [
                Direction::Up,
                Direction::Down,
                Direction::Right,
                Direction::Left,
            ] {
                distances.push(check_distance(*me, (x as i32, y as i32), direction, &grid));
            }

            sums.push(distances.iter().product());
        }
    }

    *sums.iter().max().unwrap()
}

fn get_neighbor(
    x: i32,
    y: i32,
    direction: &Direction,
    grid: &[Vec<u32>],
) -> Option<(i32, i32, u32)> {
    let (add_x, add_y) = direction.step();
    let (new_x, new_y) = (x + add_x, y + add_y);
    let (new_x_u, new_y_u) = (new_x as usize, new_y as usize);

    if new_x_u >= grid.len() - 1 || new_y_u >= grid[0].len() - 1 || new_x < 1 || new_y < 1 {
        None
    } else {
        Some((new_x, new_y, grid[new_x_u][new_y_u]))
    }
}

fn check_direction(
    max_seen: u32,
    (me_x, me_y): (i32, i32),
    direction: Direction,
    grid: &[Vec<u32>],
    visible: &mut HashSet<(i32, i32)>,
) {
    if let Some((new_x, new_y, neighbor)) = get_neighbor(me_x, me_y, &direction, grid) {
        if neighbor > max_seen {
            visible.insert((new_x, new_y));
        }

        let new_max = std::cmp::max(max_seen, neighbor);
        check_direction(new_max as u32, (new_x, new_y), direction, grid, visible)
    };
}

fn check_distance(
    start_value: u32,
    (me_x, me_y): (i32, i32),
    direction: Direction,
    grid: &[Vec<u32>],
) -> i32 {
    if let Some((new_x, new_y, neighbor)) = get_neighbor(me_x, me_y, &direction, grid) {
        if neighbor < start_value {
            return 1 + check_distance(start_value as u32, (new_x, new_y), direction, grid);
        }
    }

    1
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i32 = 21;
    static SOLUTION_TWO: i32 = 8;
    static TEST_INPUT: &str = r#"
30373
25512
65332
33549
35390"#;

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
