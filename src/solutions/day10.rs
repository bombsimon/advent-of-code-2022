use crate::input;
use std::collections::HashMap;

pub fn solve() {
    let x = input::file_for_day(10);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: Vec<String>) -> i32 {
    let mut register = 1;
    let mut cycle = 0;
    let mut cycle_value_map: HashMap<i32, i32> = HashMap::new();

    input
        .iter()
        .map(|l| {
            if l == "noop" {
                None
            } else {
                let (_, val) = l.split_once(' ').unwrap();
                Some(val.parse::<i32>().unwrap())
            }
        })
        .for_each(|instruction| {
            let cycles = if instruction.is_none() { 1 } else { 2 };

            for _ in 0..cycles {
                cycle += 1;

                if cycles == 20 || (cycle % 40) - 20 == 0 {
                    let signal_strenght = register * cycle;
                    cycle_value_map.insert(cycle, signal_strenght);
                }
            }

            if let Some(val) = instruction {
                register += val;
            }
        });

    cycle_value_map.values().sum()
}

fn part_two(input: Vec<String>) -> i32 {
    let mut register = 1;
    let mut cycle = 0;
    let mut crt: Vec<&str> = Vec::new();

    input
        .iter()
        .map(|l| {
            if l == "noop" {
                None
            } else {
                let (_, val) = l.split_once(' ').unwrap();
                Some(val.parse::<i32>().unwrap())
            }
        })
        .for_each(|instruction| {
            let cycles = if instruction.is_none() { 1 } else { 2 };

            for _ in 0..cycles {
                let row_pos = if cycles == 39 { 39 } else { cycle % 40 };
                let (sprite_start, sprite_end) = (register - 1, register + 1);
                let pixel = if row_pos >= sprite_start && row_pos <= sprite_end {
                    "█"
                } else {
                    " "
                };

                crt.push(pixel);
                cycle += 1;
            }

            if let Some(val) = instruction {
                register += val;
            }
        });

    for (i, pixel) in crt.iter().enumerate() {
        print!("{}", pixel);
        if (i + 1) % 40 == 0 {
            println!();
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i32 = 13140;
    static SOLUTION_TWO: i32 = 0;
    static TEST_INPUT: &str = r#"
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;

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
