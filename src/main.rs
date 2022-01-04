use std::env;
use std::fs;

mod solutions;

fn main() {
    let day = match env::args().nth(1) {
        Some(day) => match day.parse::<u32>() {
            Ok(day) => day,
            Err(_) => {
                println!("This program's day argument must be a valid number.");
                return;
            }
        },
        None => {
            println!("This program requires 1 argument specifying the day number.");
            return;
        }
    };

    let input = format!("input/day{}.txt", day);

    let solver = solutions::get_solution_for_day(day);
    match solver {
        Some(solver) => {
            match fs::read_to_string(input) {
                Ok(input) => solver(input),
                Err(_) => {
                    println!("Unable to read input file, make sure the file exists and has a name of the form \"input/day<number>.txt\"");
                    return;
                }
            };
        }
        None => {
            println!("No solution implemented for day {}.", day);
            return;
        }
    }
}
