use crate::input;

pub fn solve() {
    let x = input::raw_file_for_day(13);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: String) -> i64 {
    let pairs = input
        .split("\n\n")
        .map(|group| {
            let mut line = group.lines();
            let left_chars = line.next().unwrap().chars().collect::<Vec<_>>();
            let right_chars = line.next().unwrap().chars().collect::<Vec<_>>();
            let (_, left) = parse(&left_chars, 0);
            let (_, right) = parse(&right_chars, 0);

            (left[0].subpackets.clone(), right[0].subpackets.clone())
        })
        .collect::<Vec<_>>();

    let mut indicies: Vec<usize> = vec![];
    for (i, (lhs, rhs)) in pairs.iter().enumerate() {
        let idx = i + 1;
        let decider = compare_order(lhs, rhs);

        if decider != Order::Incorrect {
            indicies.push(idx);
        }
    }

    indicies.iter().sum::<usize>() as i64
}

fn part_two(input: String) -> i64 {
    let full_input = format!("{}\n[[2]]\n[[6]]", input);
    let mut pairs = full_input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let ch = line.chars().collect::<Vec<_>>();
            let (_, mut pkt) = parse(&ch, 0);

            pkt[0].tag = match line {
                "[[2]]" => Some(2),
                "[[6]]" => Some(6),
                _ => None,
            };

            pkt[0].clone()
        })
        .collect::<Vec<_>>();

    for i in 0..pairs.len() {
        for j in 0..pairs.len() - i - 1 {
            if compare_order(&[pairs[j].clone()], &[pairs[j + 1].clone()]) == Order::Incorrect {
                pairs.swap(j, j + 1);
            }
        }
    }

    let (mut a, mut b) = (0, 0);
    for (i, pkt) in pairs.iter().enumerate() {
        match pkt.tag {
            Some(2) => a = i + 1,
            Some(6) => b = i + 1,
            _ => (),
        }
    }

    (a * b) as i64
}

#[derive(Debug, Clone)]
struct Packet {
    is_value: bool,
    value: i32,
    subpackets: Vec<Packet>,
    tag: Option<i32>,
}

impl Packet {
    fn new(v: i32) -> Self {
        Self {
            is_value: true,
            value: v,
            subpackets: vec![],
            tag: None,
        }
    }

    fn new_subpacket(sub: Vec<Self>) -> Self {
        Self {
            is_value: false,
            value: 0,
            subpackets: sub,
            tag: None,
        }
    }

    fn is_none(&self) -> bool {
        !self.is_value && self.subpackets.is_empty()
    }

    fn has_sub(&self) -> bool {
        !self.subpackets.is_empty()
    }
}

#[derive(Debug, PartialEq)]
enum Order {
    Correct,
    Incorrect,
    Unknown,
}

fn compare_order(l: &[Packet], r: &[Packet]) -> Order {
    for (i, lhs) in l.iter().enumerate() {
        if r.len() < i + 1 {
            break;
        }

        let rhs = &r[i];
        if rhs.is_none() && !lhs.is_none() {
            return Order::Incorrect;
        }

        if lhs.is_none() && !rhs.is_none() {
            return Order::Correct;
        }

        let order = match (lhs, rhs) {
            (l, r) if lhs.is_value && rhs.is_value => {
                match (l.value, r.value) {
                    (l, r) if l == r => continue,
                    (l, r) if l < r => return Order::Correct,
                    (l, r) if l > r => return Order::Incorrect,
                    (_, _) => unreachable!(),
                };
            }
            (l, r) if lhs.has_sub() && rhs.has_sub() => compare_order(&l.subpackets, &r.subpackets),
            (l, r) if lhs.has_sub() => compare_order(&l.subpackets, &[r.clone()]),
            (l, r) if rhs.has_sub() => compare_order(&[l.clone()], &r.subpackets),
            (_, _) => continue,
        };

        if order != Order::Unknown {
            return order;
        }
    }

    match l.len().cmp(&r.len()) {
        std::cmp::Ordering::Less => Order::Correct,
        std::cmp::Ordering::Greater => Order::Incorrect,
        std::cmp::Ordering::Equal => Order::Unknown,
    }
}

fn parse(input: &[char], start_at: usize) -> (usize, Vec<Packet>) {
    let mut pkts = Vec::new();
    let mut i = start_at;

    while i < input.len() {
        let item = input[i];

        match item {
            ',' => (),
            ']' => return (i, pkts),
            '[' => {
                let (read_until, subpackets) = parse(input, i + 1);
                i = read_until;

                pkts.push(Packet::new_subpacket(subpackets));
            }
            c if c.is_ascii_digit() => {
                // Yeah let's assume 10 is the largest number...
                let value = if input[i + 1].is_ascii_digit() {
                    i += 1;
                    10
                } else {
                    c.to_digit(10).unwrap() as i32
                };

                pkts.push(Packet::new(value));
            }
            _ => unreachable!(),
        }

        i += 1;
    }

    (0, pkts)
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i64 = 13;
    static SOLUTION_TWO: i64 = 140;
    static TEST_INPUT: &str = r#"
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;

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
