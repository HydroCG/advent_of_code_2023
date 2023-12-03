use std::fs;
use day_runner::DayRunner;

mod day_runner;
mod day1;
mod day2;
mod day3;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let day3 = day3::Day3{}; 
    run(&day3, 3);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn run(runner: &dyn DayRunner, day_number: u32) {

    let file = fs::read_to_string(format!("B:\\source\\repos\\advent_of_code\\inputs\\day{}.txt", day_number)).unwrap();
    let mut lines: Vec<String> = file.lines().map(|s| s.to_string()).collect();
    
    runner.run_p2(lines);
}