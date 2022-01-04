mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub fn get_solution_for_day(day: u32) -> Option<fn(String)> {
    match day {
        1 => Some(day1::solve),
        2 => Some(day2::solve),
        3 => Some(day3::solve),
        4 => Some(day4::solve),
        5 => Some(day5::solve),
        6 => Some(day6::solve),
        7 => Some(day7::solve),
        8 => Some(day8::solve),
        9 => Some(day9::solve),
        10 => Some(day10::solve),
        11 => Some(day11::solve),
        12 => Some(day12::solve),
        13 => Some(day13::solve),
        14 => Some(day14::solve),
        15 => Some(day15::solve),
        16 => Some(day16::solve),
        17 => Some(day17::solve),
        18 => Some(day18::solve),
        19 => Some(day19::solve),
        20 => Some(day20::solve),
        21 => Some(day21::solve),
        22 => Some(day22::solve),
        23 => Some(day23::solve),
        24 => Some(day24::solve),
        25 => Some(day25::solve),
        _ => return None,
    }
}
