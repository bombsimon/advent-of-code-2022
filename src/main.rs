use std::env;

mod input;
mod solutions;
mod time;

fn main() {
    let day = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("1"))
        .parse()
        .unwrap_or(1);

    solutions::solution_for(day)
}
