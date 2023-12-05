use std::fs;
use day_runner::DayRunner;

mod day_runner;
mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let day4 = day4::Day4{}; 
    run(&day4, 4);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn run(runner: &dyn DayRunner, day_number: u32) {

    let file = fs::read_to_string(format!("B:\\source\\repos\\advent_of_code\\inputs\\day{}.txt", day_number)).unwrap();
    let lines: Vec<String> = file.lines().map(|s| s.to_string()).collect();
    
    runner.run_p2(lines);
}