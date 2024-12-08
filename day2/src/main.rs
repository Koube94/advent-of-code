use std::fs;
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    // Read the file content as a string
    let message = fs::read_to_string("src/input_short.txt")?;

    // Parse the string into a vector of vectors of integers
    let reports: Vec<Vec<i32>> = message
        .lines() // Iterate over lines
        .map(|line| {
            line.split_whitespace() // Split the line into words
                .filter_map(|word| word.parse::<i32>().ok()) // Parse each word into an i32, filter out errors
                .collect::<Vec<i32>>() // Collect parsed integers into a vector
        })
        .collect(); // Collect all lines into a vector of vectors
    let min_diff = 1;
    let max_diff = 3;
    
    // Task 1
    let mut valid_reports = 0;
    for report in reports.iter(){
        let mut increasing = true;
        let mut decreasing = true;
        let mut difference_check = true;
        let mut last_level = -10;
        for level in report.iter(){
            if last_level == -10 {
                last_level = *level;
                continue;
            }
            if level > &last_level{
                decreasing = false;
            }
            if level < &last_level{
                increasing = false;
            }
            let difference = (last_level - level).abs();
            if difference >= min_diff && difference <= max_diff {
            }
            else{
                difference_check = false;
            }
            last_level = *level;
        }
        let level_valid = (increasing || decreasing) && difference_check;
        if level_valid{
            valid_reports += 1;
        }
    }
    println!("{}", valid_reports);
    
    // Task 2

    let mut valid_reports = 0;
    for report in reports.iter(){
        let mut last_valid_index = 0;
        let mut increasing = true;
        let mut decreasing = true;
        let valid_level = true;
        for (index, level) in report.iter().enumerate(){
            if index == 0 {
                continue;
            }
            let difference = report[index] - report[last_valid_index];
            if difference.abs() >= min_diff && difference.abs() <= max_diff{
            }
            else {
                continue;
            }
            if difference < 0 {
                if increasing {
                    increasing = false;
                    continue
                }
            }
            if difference > 0 {
                if decreasing {
                    decreasing = false;
                    continue
                }
            }
            last_valid_index = index;
        }
    }
    println!("{}", valid_reports);
    
    Ok(())
}