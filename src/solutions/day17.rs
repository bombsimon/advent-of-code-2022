use crate::input;
use std::collections::HashSet;

pub fn solve() {
    let x = input::raw_file_for_day(17);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: String) -> i64 {
    let mut b = Board {
        current_shape: Shape::HorizontalBar(2, 4),
        arena: HashSet::new(),
        highest_block: 0,
    };

    let mut i = 0;
    for step in input.trim().chars().cycle() {
        b.push(step);
        if !b.push('v') {
            b.lock_shape();

            i += 1;
            if i == 2022 {
                // b.visualize();
                break;
            }
        }
    }

    b.highest_block
}

fn part_two(input: String) -> i64 {
    let mut b = Board {
        current_shape: Shape::HorizontalBar(2, 4),
        arena: HashSet::new(),
        highest_block: 0,
    };

    let mut blocks_at_rest = 0i64;
    let mut last_blocks_at_rest = 0i64;
    let mut last_highest_block = 0i64;
    let mut total_iterations = 0;
    let mut instruction_idx = 0;
    let mut round_of_instructions = 0;
    let mut total_1721 = 0;

    let mut visualize = false;
    // let mut seen_combination = HashSet::new();

    for step in input.trim().chars().cycle() {
        // for (ii, s) in input.trim().chars().enumerate() {
        //     if j == ii {
        //         print!("[{}]", s);
        //     } else {
        //         print!("{}", s);
        //     }
        // }
        // println!();

        // if k > input.len()
        //     && seen_combination
        //         .get(&(j, b.current_shape.clone()))
        //         .is_some()
        //     && b.current_shape
        //         .coords()
        //         .iter()
        //         .map(|(_x, y)| *y)
        //         .min()
        //         .unwrap_or(0)
        //         <= b.highest_block
        visualize = if total_iterations > 0 && instruction_idx == 0 {
            total_1721 = if blocks_at_rest - last_blocks_at_rest == 1721 {
                total_1721 + 1
            } else {
                total_1721
            };

            println!(
                "round={:<4} highest_block={:<8} diff_since_last={:<6} blocks_at_rest={:<8} diff_since_last_round={:<6} starting_with={:?}",
                round_of_instructions,
                b.highest_block,
                b.highest_block - last_highest_block,
                blocks_at_rest,
                blocks_at_rest - last_blocks_at_rest,
                //total_1721,
                b.current_shape,
            );

            last_blocks_at_rest = blocks_at_rest;
            last_highest_block = b.highest_block;
            round_of_instructions += 1;

            blocks_at_rest % 100000 == 0
        } else {
            false
        };

        b.push(step);
        if !b.push('v') {
            b.lock_shape();

            blocks_at_rest += 1;
        }

        if visualize {
            b.visualize();
            input::print_and_wait(format!(
                "i={} j={} k={} l={}",
                blocks_at_rest, instruction_idx, total_iterations, round_of_instructions
            ));
        }

        instruction_idx = if instruction_idx == input.len() - 1 {
            0
        } else {
            instruction_idx + 1
        };
        total_iterations += 1;
    }

    b.highest_block
}

/// A shape represents a block that can have one of four different shapes:
///
/// Horizontal    Cross   Angle   Vertical    Square
/// -----------+--------+-------+-----------+-------
///    ####        #        #        #        ##
///               ###       #        #        ##
///                #      ###        #
///                                  #
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Shape {
    HorizontalBar(i64, i64),
    Cross(i64, i64),
    Angle(i64, i64),
    VerticalBar(i64, i64),
    Square(i64, i64),
}

impl Shape {
    fn coords(&self) -> HashSet<(i64, i64)> {
        let mut coords: HashSet<(i64, i64)> = HashSet::new();

        match self {
            Self::HorizontalBar(x, y) => {
                coords.insert((*x, *y));
                coords.insert((x + 1, *y));
                coords.insert((x + 2, *y));
                coords.insert((x + 3, *y));
            }
            Self::Cross(x, y) => {
                coords.insert((x + 1, *y));
                coords.insert((*x, y - 1));
                coords.insert((x + 1, y - 1));
                coords.insert((x + 2, y - 1));
                coords.insert((x + 1, y - 2));
            }
            Self::Angle(x, y) => {
                coords.insert((x + 2, *y));
                coords.insert((x + 2, y - 1));
                coords.insert((*x, y - 2));
                coords.insert((x + 1, y - 2));
                coords.insert((x + 2, y - 2));
            }
            Self::VerticalBar(x, y) => {
                coords.insert((*x, *y));
                coords.insert((*x, y - 1));
                coords.insert((*x, y - 2));
                coords.insert((*x, y - 3));
            }
            Self::Square(x, y) => {
                coords.insert((*x, *y));
                coords.insert((x + 1, *y));
                coords.insert((*x, y - 1));
                coords.insert((x + 1, y - 1));
            }
        }

        coords
    }
}

#[derive(Debug)]
struct Board {
    current_shape: Shape,
    arena: HashSet<(i64, i64)>,
    highest_block: i64,
}

impl Board {
    fn next_shape(&mut self) {
        let gap = self.highest_block + 3;

        self.current_shape = match self.current_shape {
            Shape::HorizontalBar(_, _) => Shape::Cross(2, gap + 3),
            Shape::Cross(_, _) => Shape::Angle(2, gap + 3),
            Shape::Angle(_, _) => Shape::VerticalBar(2, gap + 4),
            Shape::VerticalBar(_, _) => Shape::Square(2, gap + 2),
            Shape::Square(_, _) => Shape::HorizontalBar(2, gap + 1),
        }
    }

    fn push(&mut self, force: char) -> bool {
        let (fx, fy) = match force {
            '>' => (1, 0),
            '<' => (-1, 0),
            'v' => (0, -1),
            _ => unreachable!(),
        };

        let new_position = match self.current_shape {
            Shape::HorizontalBar(x, y) => Shape::HorizontalBar(x + fx, y + fy),
            Shape::Cross(x, y) => Shape::Cross(x + fx, y + fy),
            Shape::Angle(x, y) => Shape::Angle(x + fx, y + fy),
            Shape::VerticalBar(x, y) => Shape::VerticalBar(x + fx, y + fy),
            Shape::Square(x, y) => Shape::Square(x + fx, y + fy),
        };

        for (x, y) in new_position.coords() {
            if y == 0 {
                return false;
            }

            if !(0..=6).contains(&x) {
                return false;
            }

            if self.arena.get(&(x, y)).is_some() {
                return false;
            }
        }

        self.current_shape = new_position;

        true
    }

    fn lock_shape(&mut self) {
        match self.current_shape {
            Shape::HorizontalBar(x, y) => {
                self.arena.insert((x, y));
                self.arena.insert((x + 1, y));
                self.arena.insert((x + 2, y));
                self.arena.insert((x + 3, y));
            }
            Shape::Cross(x, y) => {
                self.arena.insert((x + 1, y));
                self.arena.insert((x, y - 1));
                self.arena.insert((x + 1, y - 1));
                self.arena.insert((x + 2, y - 1));
                self.arena.insert((x + 1, y - 2));
            }
            Shape::Angle(x, y) => {
                self.arena.insert((x + 2, y));
                self.arena.insert((x + 2, y - 1));
                self.arena.insert((x, y - 2));
                self.arena.insert((x + 1, y - 2));
                self.arena.insert((x + 2, y - 2));
            }
            Shape::VerticalBar(x, y) => {
                self.arena.insert((x, y));
                self.arena.insert((x, y - 1));
                self.arena.insert((x, y - 2));
                self.arena.insert((x, y - 3));
            }
            Shape::Square(x, y) => {
                self.arena.insert((x, y));
                self.arena.insert((x + 1, y));
                self.arena.insert((x, y - 1));
                self.arena.insert((x + 1, y - 1));
            }
        }

        self.highest_block = self
            .current_shape
            .coords()
            .iter()
            .map(|(_, y)| *y)
            .max()
            .unwrap()
            .max(self.highest_block);

        self.next_shape()
    }

    #[allow(dead_code)]
    fn visualize(&self) {
        let shape = self.current_shape.coords();

        for y in (self.highest_block - 40..=self.highest_block + 7).rev() {
            print!("{:<5}", y);

            for x in -1..=7 {
                let to_print = match x {
                    -1 | 7 => '|',
                    _ if x == -1 && y == 0 => '+',
                    _ if x == 7 && y == 0 => '+',
                    _ if y == 0 => '-',
                    _ if shape.get(&(x, y)).is_some() => '#',
                    _ if self.arena.get(&(x, y)).is_some() => '#',
                    _ => '.',
                };

                print!("{}", to_print);
            }

            println!()
        }

        println!("high={}", self.highest_block);
    }
}

#[cfg(test)]
mod tests {
    static SOLUTION_ONE: i64 = 3068;
    static SOLUTION_TWO: i64 = 1514285714288;
    static TEST_INPUT: &str = r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"#;

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(TEST_INPUT.to_owned()), SOLUTION_ONE);
    }

    #[test]
    fn part_two() {
        assert_eq!(super::part_two(TEST_INPUT.to_owned()), SOLUTION_TWO);
    }
}
