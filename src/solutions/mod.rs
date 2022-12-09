mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

pub fn solution_for(day: i32) {
    println!("Showing solution for day {}\n", day);

    match day {
        1 => day01::solve(),
        2 => day02::solve(),
        3 => day03::solve(),
        4 => day04::solve(),
        5 => day05::solve(),
        6 => day06::solve(),
        7 => day07::solve(),
        8 => day08::solve(),
        9 => todo!(),
        10 => todo!(),
        11 => todo!(),
        12 => todo!(),
        13 => todo!(),
        14 => todo!(),
        15 => todo!(),
        16 => todo!(),
        17 => todo!(),
        18 => todo!(),
        19 => todo!(),
        20 => todo!(),
        21 => todo!(),
        22 => todo!(),
        23 => todo!(),
        24 => todo!(),
        25 => todo!(),
        _ => panic!("Invalid day"),
    }
}
