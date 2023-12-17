use regex::Regex;

use crate::day_runner::DayRunner;

pub struct DayX {}

impl DayRunner for DayX {
    
    fn run_p1(&self, _lines: Vec<String>) {
        println!("Run");
    }

    fn run_p2(&self, _lines: Vec<String>) {
        println!("Run");     
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_p1() {
        let result = run_p1(crate::utils::read_input(8));
        assert_eq!(result, 250120186);
    }

    #[test]
    fn test_run_p2() {
        let result = run_p2(crate::utils::read_input(8));
        assert_eq!(result, 250665248);
    }
}
