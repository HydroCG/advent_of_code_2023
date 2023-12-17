use regex::Regex;

#[allow(dead_code)]
fn run_p1(_: Vec<String>) -> u32{
    println!("Run");
    
    53194
}

#[allow(dead_code)]
fn run_p2(lines: Vec<String>) -> u32 {

    let mut sum = 0;
    let regex = Regex::new(r"1|2|3|4|5|6|7|8|9|one|two|three|four|five|six|seven|eight|nine").unwrap();

    for line in lines {
        sum += parse_line(&line, &regex);
    }

    sum
}

fn parse_line(line: &str, regex: &Regex) -> u32 {
    // let numbers: Vec<char> = line.chars().into_iter().filter(|c| c.is_numeric()).collect();
    
    let mut finds: Vec<&str> = vec![];
    let mut start = 0;

    loop {
        match regex.find_at(line, start) {
            Some(result) => {
                finds.push(result.as_str());
                start = start + 1;
            },
            None => {
                break
            }
        }
    }
 
    let first = parse_number(finds.first().unwrap());
    let last = parse_number(finds.last().unwrap());

    let result = format!("{}{}", first, last).parse::<u32>().unwrap();
    
    result
}

fn parse_number(num: &str) -> u32 {
    match num.parse::<u32>() {
        Ok(result) => {
            return result
        },
        Err(_)  => {

            return match num {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                _ => panic!("Unknown number")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_p1() {
        let result = run_p1(crate::utils::read_input(1));
        assert_eq!(result, 53194);
    }

    #[test]
    fn test_run_p2() {
        let result = run_p2(crate::utils::read_input(1));
        assert_eq!(result, 54249);
    }
}