mod day01;

pub fn solution_for(day: i32) {
    println!("Showing solution for day {}\n", day);

    match day {
        1 => day01::solve(),
        d => panic!("Day {} not implemented", d),
    }
}
