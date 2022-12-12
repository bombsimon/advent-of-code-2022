use crate::input;
use pathfinding::prelude::dijkstra;

pub fn solve() {
    let x = input::file_for_day(12);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: Vec<String>) -> i64 {
    let predicate = |c: char| c == 'S';
    lowest_distance(&input, predicate)
}

fn part_two(input: Vec<String>) -> i64 {
    let predicate = |c: char| c == 'S' || c == 'a';
    lowest_distance(&input, predicate)
}

fn lowest_distance<P>(input: &[String], predicate: P) -> i64
where
    P: Fn(char) -> bool,
{
    let height = input.len() as i32;
    let width = input[0].len() as i32;

    let mut inner: Vec<char> = vec!['0'; (width * height) as usize];
    let mut end = Pos(0, 0);

    let mut starting_candidates = vec![];

    input.iter().enumerate().for_each(|(i, line)| {
        for (j, ch) in line.chars().enumerate() {
            let actual = match ch {
                'E' => {
                    end = Pos(i as i32, j as i32);
                    'z'
                }
                c if predicate(c) => {
                    starting_candidates.push(Pos(i as i32, j as i32));
                    if c == 'S' {
                        'a'
                    } else {
                        c
                    }
                }
                _ => ch,
            };

            inner[i * width as usize + j] = actual
        }
    });

    let grid = Grid {
        inner,
        width,
        height,
    };

    starting_candidates
        .iter_mut()
        .filter_map(|candidate| dijkstra(candidate, |p| p.successors(&grid), |p| *p == end))
        .map(|result| result.0.len() as i64 - 1)
        .min()
        .unwrap()
}

#[derive(Debug)]
struct Grid {
    inner: Vec<char>,
    width: i32,
    height: i32,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
    fn successors(&self, grid: &Grid) -> Vec<(Pos, usize)> {
        let &Pos(x, y) = self;
        let me = get(grid, x, y).unwrap();

        vec![Pos(x + 1, y), Pos(x - 1, y), Pos(x, y + 1), Pos(x, y - 1)]
            .into_iter()
            .filter_map(|p| match get(grid, p.0, p.1) {
                None => None,
                Some(ch) if ch as i32 - me as i32 > 1 => None,
                Some(ch) if ch == me => Some((p, 1)),
                Some(_) => Some((p, 1)),
            })
            .collect()
    }
}

fn get(grid: &Grid, x: i32, y: i32) -> Option<char> {
    if x >= grid.height || y >= grid.width {
        return None;
    }

    match x * grid.width + y {
        _ if x < 0 => None,
        _ if y < 0 => None,
        pos if pos > grid.inner.len() as i32 - 1 => None,
        pos => Some(grid.inner[pos as usize]),
    }
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i64 = 31;
    static SOLUTION_TWO: i64 = 29;
    static TEST_INPUT: &str = r#"
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;

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
