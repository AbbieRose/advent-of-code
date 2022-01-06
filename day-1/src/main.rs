use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    // Part 1: How many measurements are larger than the previous measurement?
    let filename = "src/input.txt";
    // 1. Read in input file
    println!("Reading file {}", filename);
    let file = File::open(filename).expect("File wasn't found.");
    let reader = BufReader::new(file);

    let measurements: Vec<i64> = reader.lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect();

    // 2. Subtract n from n+1 (or reverse)
    let differences: Vec<i64> = measurements.windows(2)
        .map(|d| d[0] - d[1])
        .collect();

    // 3. count the number that are greater than 0 
    let count: usize = differences.iter()
        .filter(|c| c.is_negative())
        .count();

    // I could put these all in one line but for readability sake I don't want to
    println!("Count: {}", count);

    // Part 2: In a sliding window of 3 how many measurements are larger than the previous measurement? 
    // 1. ok so we have measurements already

    // 2. Now we need to generate windows, feel like I could sum them with a function but there's only 3 so I'm just doing it manually
    let windows: Vec<i64> = measurements.windows(3)
        .map(|w| w[0] + w[1] + w[2])
        .collect();

    // 3. The rest is the same as above- Subtract n from n+1 (or reverse)
    let window_differences: Vec<i64> = windows.windows(2)
        .map(|d| d[0] - d[1])
        .collect();

    // 3. count the number that are greater than 0 
    let window_count: usize = window_differences.iter()
        .filter(|c| c.is_negative())
        .count();

        println!("Window Count: {}", window_count);
}
