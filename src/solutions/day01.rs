use crate::input;

pub fn solve() {
    let x = input::raw_file_for_day(1);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: String) -> i32 {
    input
        .lines()
        .fold((0, 0), |(max, acc), x| {
            if x.is_empty() {
                (std::cmp::max(max, acc), 0)
            } else {
                let val = x.parse::<i32>().unwrap();
                (max, acc + val)
            }
        })
        .0
}

fn part_two(input: String) -> i32 {
    let mut groups = Vec::new();
    let mut current_sum = 0;

    for x in input.lines() {
        if x.is_empty() {
            groups.push(current_sum);
            current_sum = 0;
        } else {
            let val = x.parse::<i32>().unwrap();
            current_sum += val;
        }
    }

    // Push last group
    groups.push(current_sum);
    groups.sort_by(|a, b| b.cmp(a));

    groups.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i32 = 24000;
    static SOLUTION_TWO: i32 = 45000;
    static TEST_INPUT: &str = r#"
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"#;

    #[test]
    fn part_one() {
        let x = input::test_vec_raw(TEST_INPUT);
        assert_eq!(super::part_one(x), SOLUTION_ONE);
    }

    #[test]
    fn part_two() {
        let x = input::test_vec_raw(TEST_INPUT);
        assert_eq!(super::part_two(x), SOLUTION_TWO);
    }
}
