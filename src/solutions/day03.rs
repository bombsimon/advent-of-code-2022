use crate::input;
use std::collections::HashSet;

pub fn solve() {
    let x = input::file_for_day(3);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn value_for_char(chr: &char) -> i32 {
    match chr {
        'a'..='z' => *chr as i32 - 'a' as i32 + 1,
        'A'..='Z' => *chr as i32 - 'A' as i32 + 27,
        _ => 0,
    }
}

fn part_one(input: Vec<String>) -> i32 {
    input
        .iter()
        .map(|line| {
            let (lhs, rhs) = line.split_at(line.len() / 2);
            let left: HashSet<_> = HashSet::from_iter(lhs.chars());
            let right: HashSet<_> = HashSet::from_iter(rhs.chars());

            let unique = left.intersection(&right).collect::<HashSet<_>>();

            unique.iter().fold(0, |acc, &x| acc + value_for_char(x))
        })
        .sum()
}

fn part_two(input: Vec<String>) -> i32 {
    input
        .chunks(3)
        .map(|group| {
            group
                .iter()
                .map(|line| -> HashSet<char> { HashSet::from_iter(line.chars()) })
                .collect::<Vec<_>>()
        })
        .map(|group| {
            let mut it = group.iter();
            let first = it.next().unwrap().to_owned();

            it.fold(first, |acc, i| acc.intersection(i).copied().collect())
        })
        .map(|set| set.iter().fold(0, |acc, x| acc + value_for_char(x)))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i32 = 157;
    static SOLUTION_TWO: i32 = 70;
    static TEST_INPUT: &str = r#"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
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
