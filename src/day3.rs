// Note: CRIMES WERE COMMITTED ON THIS DAY. THIS AIN'T RIGHT MAN...

use std::{vec, collections::HashMap};
use crate::day_runner::DayRunner;

pub struct Day3;

struct Part {
    part_number: u32,
    touching: Vec<(char, u32, u32)>
}

impl DayRunner for Day3 {
    fn run_p1(&self, lines: Vec<String>) {
        
        let parts = extract_parts(lines);
        let mut sum = 0;

        for part in &parts {
            sum += part.part_number;
        }

        println!("Result: {}", sum);
    }

    fn run_p2(&self, lines: Vec<String>) {

        let parts = extract_parts(lines);
        let mut gears: HashMap<(u32, u32), Vec<u32>> = HashMap::new();
        
        for part in &parts {
            
            for gear in (&part).touching.iter().filter(|gear|gear.0 == '*') {

                let key = (gear.1, gear.2);

                match gears.get_mut(&key) {
                    Some(partsOnGear) => {
                        partsOnGear.push(part.part_number);
                    }
                    None => {
                        gears.insert(key, vec![part.part_number]);
                    }
                }
            }

        }

        let mut sum = 0;

        for adjacents in gears.values().into_iter() {
            if adjacents.len() == 2 {
                sum += adjacents.iter().product::<u32>();
            }
        }

        println!("Result {}", sum);
    }

}

fn extract_parts(lines: Vec<String>) -> Vec<Part> { 

    // Pad this so we can avoid doing really annoying bounds checks later... Fell over for ages so came to this hack :(.
    let mut char_lines: Vec<Vec<char>> = vec![];
    char_lines.push(".".repeat(lines[1].chars().count() + 2).chars().collect());

    for line in &lines {
        char_lines.push(format!(".{}.", line.to_string()).chars().collect());
    }

    char_lines.push(".".repeat(lines[1].chars().count() + 2).chars().collect());

    let mut line_number: i32 = -1;
    let mut parts: Vec<Part> = vec![];

    for line in &char_lines {
        line_number += 1;

        let mut started = false;
        let mut start_index = 0;
        let mut char_index = -1;

        for character in line {
            char_index += 1;
            
            if character.is_alphanumeric() {

                if started {
                    continue;
                } else {
                    started = true;
                    start_index = char_index;
                }
            } else {
                if started {
                    started = false;
                    let str: String = line[(start_index as usize)..(char_index as usize)].iter().collect();
                    
                    let touching_symbols = get_touching_symbols(line_number, start_index, char_index - 1, &char_lines);

                    if touching_symbols.len() > 0 {
                        parts.push(Part {
                            part_number: str.parse::<u32>().unwrap(),
                            touching: touching_symbols
                        });
                    }

                    started = false;
                    start_index = 0;
                }
            }
        }
    }

    return parts;
}

fn get_touching_symbols(line_number: i32, start: i32, end: i32, lines: &Vec<Vec<char>>) -> Vec<(char, u32,u32)> {

    let mut results: Vec<(char, u32,u32)> = vec![];

    for x in start-1..end + 2 { // end is exclusive.....
        for y in line_number-1..line_number+2 {
            // println!("{}, {}", x, y);
            let char = lines[y as usize][x as usize];
            
            if !char.is_alphanumeric() && char != '.' {
                println!("Matched on symbol {} LN {}", char, line_number+1);

                results.push((char, y as u32, x as u32));
            }
        }
    }

    return results;
}