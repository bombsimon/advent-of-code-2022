use crate::input;

pub fn solve() {
    let x = input::raw_file_for_day(6);

    let start = std::time::Instant::now();
    let part1 = part_one(x.clone());
    let duration = start.elapsed();
    println!("Solution part 1: {} ({:?})", part1, duration);

    let start = std::time::Instant::now();
    let part2 = part_two(x);
    let duration = start.elapsed();
    println!("Solution part 2: {} ({:?})", part2, duration);
}

fn part_one(input: String) -> i32 {
    find_unique_consecutives(input, 4)
}

fn part_two(input: String) -> i32 {
    find_unique_consecutives(input, 14)
}

fn find_unique_consecutives(input: String, count: usize) -> i32 {
    let mut acc: std::collections::VecDeque<char> =
        std::collections::VecDeque::with_capacity(count);

    for (i, val) in input.chars().enumerate() {
        if acc.contains(&val) {
            while acc[0] != val {
                acc.pop_front();
            }

            acc.pop_front();
        }

        acc.push_back(val);

        if acc.len() == count {
            return i as i32 + 1;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one() {
        let cases = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];

        for (case, expect) in cases {
            assert_eq!(super::part_one(case.to_string()), expect);
        }
    }

    #[test]
    fn part_two() {
        let cases = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];

        for (case, expect) in cases {
            assert_eq!(super::part_two(case.to_string()), expect);
        }
    }
}
