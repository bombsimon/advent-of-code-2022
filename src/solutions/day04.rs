use crate::input;

pub fn solve() {
    let x = input::file_for_day(4);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: Vec<String>) -> i32 {
    input
        .iter()
        .map(|line| {
            let mut it = line.split(',');
            (it.next().unwrap(), it.next().unwrap())
        })
        .map(|(left, right)| (to_min_max(left), to_min_max(right)))
        .map(|((lmin, lmax), (rmin, rmax))| {
            i32::from((lmin >= rmin && lmax <= rmax) || (rmin >= lmin && rmax <= lmax))
        })
        .sum()
}

fn part_two(input: Vec<String>) -> i32 {
    input
        .iter()
        .map(|line| {
            let mut it = line.split(',');
            (it.next().unwrap(), it.next().unwrap())
        })
        .map(|(left, right)| (to_min_max(left), to_min_max(right)))
        .map(|((lmin, lmax), (rmin, rmax))| i32::from(lmin <= rmax && rmin <= lmax))
        .sum()
}

fn to_min_max(range: &str) -> (i32, i32) {
    let mut parts = range.split('-');
    let start = parts.next().unwrap().parse::<i32>().unwrap();
    let end = parts.next().unwrap().parse::<i32>().unwrap();

    (start, end)
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i32 = 2;
    static SOLUTION_TWO: i32 = 4;
    static TEST_INPUT: &str = r#"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"#;

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
