use crate::input;

pub fn solve() {
    let x = input::raw_file_for_day(5);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: String) -> String {
    let predicate = |columns: &mut Vec<Vec<char>>, count: usize, from: usize, to: usize| {
        for _ in 0..count {
            let val = columns[from].remove(0);
            columns[to].insert(0, val);
        }
    };

    parse_with_predicate(input, predicate)
}

fn part_two(input: String) -> String {
    let predicate = |columns: &mut Vec<Vec<char>>, count: usize, from: usize, to: usize| {
        for i in (0..count).rev() {
            let val = columns[from].remove(i);
            columns[to].insert(0, val);
        }
    };

    parse_with_predicate(input, predicate)
}

fn parse_with_predicate<F>(input: String, predicate: F) -> String
where
    F: Fn(&mut Vec<Vec<char>>, usize, usize, usize),
{
    let (crates, instructions) = input.split_once("\n\n").unwrap();
    let mut columns = Vec::new();

    crates.lines().for_each(|line| {
        if line.starts_with(" 1") {
            return;
        }

        let ch = line.chars().collect::<Vec<_>>();

        for x in (1..line.len()).step_by(4) {
            let col = x / 4;
            let val = ch[x];

            if columns.len() <= col {
                columns.push(vec![]);
            }

            if val != ' ' {
                columns[col].push(val);
            }
        }
    });

    instructions.lines().for_each(|line| {
        let parts = line.split(' ').collect::<Vec<_>>();
        let (count, from, to) = (
            parts[1].parse::<usize>().unwrap(),
            parts[3].parse::<usize>().unwrap() - 1,
            parts[5].parse::<usize>().unwrap() - 1,
        );

        predicate(&mut columns, count, from, to);
    });

    let first = columns.iter().map(|i| i[0].to_string()).collect::<Vec<_>>();

    first.join("")
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: &str = "CMZ";
    static SOLUTION_TWO: &str = "MCD";
    static TEST_INPUT: &str = r#"
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

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
