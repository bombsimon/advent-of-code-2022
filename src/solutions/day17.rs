use crate::input;
use std::collections::HashSet;

pub fn solve() {
    let x = input::raw_file_for_day(17);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: String) -> i32 {
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

fn part_two(_input: String) -> i32 {
    0
}

/// A shape represents a block that can have one of four different shapes:
///
/// Horizontal    Cross   Angle   Vertical    Square
/// -----------+--------+-------+-----------+-------
///    ####        #        #        #        ##
///               ###       #        #        ##
///                #      ###        #
///                                  #
#[derive(Debug)]
enum Shape {
    HorizontalBar(i32, i32),
    Cross(i32, i32),
    Angle(i32, i32),
    VerticalBar(i32, i32),
    Square(i32, i32),
}

impl Shape {
    fn coords(&self) -> HashSet<(i32, i32)> {
        let mut coords: HashSet<(i32, i32)> = HashSet::new();

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
    arena: HashSet<(i32, i32)>,
    highest_block: i32,
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

        for y in (0..=self.highest_block + 7).rev() {
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
    static SOLUTION_ONE: i32 = 3068;
    static SOLUTION_TWO: i32 = 0;
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
