use std::fs;
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    let message: String = fs::read_to_string("src/input.txt")?;
    let mut first_col: Vec<i32> = vec![];
    let mut second_col: Vec<i32> = vec![];
    for line in message.lines() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        first_col.push(parts[0].parse::<i32>().unwrap());
        second_col.push(parts[1].parse::<i32>().unwrap())
    }
    // First task
    first_col.sort();
    second_col.sort();
    let mut distance = 0;
    for (first, second) in first_col.iter().zip(second_col.iter()){
        distance += (first - second).abs();
    }
    println!("{}", distance);
    // Second task
    let mut similarity_score = 0;
    for first in first_col.iter(){
        let mut count = 0;
        for second in second_col.iter(){
            if first == second {
                count += 1;
            }
        }
        similarity_score += first * count;
    }
    println!("{}", similarity_score);
    Ok(())
}