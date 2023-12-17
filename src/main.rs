mod utils;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day_runner;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}