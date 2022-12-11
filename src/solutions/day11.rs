use crate::input;

pub fn solve() {
    let x = input::raw_file_for_day(11);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: String) -> i64 {
    monkey_business(&input, 3, 20)
}

fn part_two(input: String) -> i64 {
    monkey_business(&input, 1, 10000)
}

#[derive(Debug, Clone, PartialEq)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Debug, Clone)]
enum OperatorValue {
    Old,
    N(i64),
}

#[derive(Debug, Clone)]
struct Instruction {
    monkey: usize,
    items: Vec<i64>,
    operation: Operator,
    operation_val: OperatorValue,
    test_divisible_by: i64,
    throw_if_true: usize,
    throw_if_false: usize,
}

impl Instruction {
    fn default() -> Self {
        Self {
            monkey: 0,
            items: vec![],
            operation: Operator::Add,
            operation_val: OperatorValue::Old,
            test_divisible_by: 0,
            throw_if_true: 0,
            throw_if_false: 0,
        }
    }

    fn new(input: &str) -> Self {
        let mut i = Self::default();

        input.lines().for_each(|line| match line.trim() {
            x if x.starts_with("Monkey") => {
                i.monkey = line
                    .split(' ')
                    .nth(1)
                    .unwrap()
                    .trim_end_matches(':')
                    .parse()
                    .unwrap()
            }
            x if x.starts_with("Starting items:") => {
                let (_, items) = line.split_once(": ").unwrap();
                i.items = items
                    .split(", ")
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect()
            }
            x if x.starts_with("Operation:") => {
                let (_, expression) = line.split_once("old ").unwrap();
                let (o, n) = expression.split_once(' ').unwrap();

                match o {
                    "+" => i.operation = Operator::Add,
                    "*" => i.operation = Operator::Multiply,
                    _ => unreachable!(),
                }

                match n {
                    "old" => i.operation_val = OperatorValue::Old,
                    n => i.operation_val = OperatorValue::N(n.parse().unwrap()),
                }
            }
            x if x.starts_with("Test:") => {
                i.test_divisible_by = x.split(' ').nth(3).unwrap().parse().unwrap()
            }
            x if x.starts_with("If true:") => {
                i.throw_if_true = x.split(' ').nth(5).unwrap().parse().unwrap()
            }
            x if x.starts_with("If false:") => {
                i.throw_if_false = x.split(' ').nth(5).unwrap().parse().unwrap()
            }
            _ => unreachable!(),
        });

        i
    }
}

fn monkey_business(input: &str, worry_level_reducer: i64, steps: usize) -> i64 {
    let mut monkeys = input
        .split("\n\n")
        .map(Instruction::new)
        .collect::<Vec<_>>();
    let mut monkey_inspection: Vec<i64> = vec![0; monkeys.len()];
    let div = monkeys.iter().map(|m| m.test_divisible_by).product::<i64>();

    for _ in 0..steps {
        for current_monkey in 0..monkeys.len() {
            let monkey = monkeys[current_monkey].clone();
            monkey_inspection[current_monkey] += monkey.items.len() as i64;

            for item in monkey.items {
                let worry_level = match (&monkey.operation, &monkey.operation_val) {
                    (Operator::Add, OperatorValue::N(n)) => item + n,
                    (Operator::Add, OperatorValue::Old) => item + item,
                    (Operator::Multiply, OperatorValue::N(n)) => item * n,
                    (Operator::Multiply, OperatorValue::Old) => item * item,
                } / worry_level_reducer;

                let monkey_idx = if worry_level % monkey.test_divisible_by == 0 {
                    monkey.throw_if_true
                } else {
                    monkey.throw_if_false
                };

                monkeys[monkey_idx].items.push(if worry_level_reducer > 1 {
                    worry_level
                } else {
                    worry_level % div
                });
            }

            monkeys[monkey.monkey].items.clear();
        }
    }

    monkey_inspection.sort_by(|a, b| b.cmp(a));
    monkey_inspection.iter().take(2).product()
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i64 = 10605;
    static SOLUTION_TWO: i64 = 2713310158;
    static TEST_INPUT: &str = r#"
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"#;

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
