use crate::input;

pub fn solve() {
    let x = input::file_for_day(2);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

#[derive(Clone, Debug, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissor,
}

impl From<&str> for Shape {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" => Shape::Scissor,
            _ => unreachable!(),
        }
    }
}

impl Shape {
    fn value(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissor => 3,
        }
    }

    fn score(&self, other: &Self) -> i32 {
        match (other, self) {
            (Shape::Rock, Shape::Scissor)
            | (Shape::Paper, Shape::Rock)
            | (Shape::Scissor, Shape::Paper) => 0,
            (a, b) if a == b => 3,
            (Shape::Rock, Shape::Paper)
            | (Shape::Paper, Shape::Scissor)
            | (Shape::Scissor, Shape::Rock) => 6,
            _ => unreachable!(),
        }
    }

    fn wins(&self) -> Self {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissor,
            Shape::Scissor => Shape::Rock,
        }
    }

    fn loses(&self) -> Self {
        match self {
            Shape::Rock => Shape::Scissor,
            Shape::Paper => Shape::Rock,
            Shape::Scissor => Shape::Paper,
        }
    }
}

fn part_one(input: Vec<String>) -> i32 {
    input
        .iter()
        .map(|row| {
            let mut x = row.split(' ');
            (
                Shape::from(x.next().unwrap()),
                Shape::from(x.next().unwrap()),
            )
        })
        .map(|(elf, me)| me.score(&elf) + me.value())
        .sum()
}

fn part_two(input: Vec<String>) -> i32 {
    input
        .iter()
        .map(|row| {
            let mut x = row.split(' ');
            (
                Shape::from(x.next().unwrap()),
                Shape::from(x.next().unwrap()),
            )
        })
        .map(|(elf, me)| {
            let my_choice = match me {
                Shape::Rock => elf.loses(),
                Shape::Paper => elf.clone(),
                Shape::Scissor => elf.wins(),
            };

            my_choice.score(&elf) + my_choice.value()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i32 = 15;
    static SOLUTION_TWO: i32 = 12;
    static TEST_INPUT: &str = r#"
A Y
B X
C Z
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
