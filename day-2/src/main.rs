use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let mut position: (i32, i32) = (0,0); // (horizontal, depth)
    let direction_lookup: HashMap<String, i32> = HashMap::from([
        (String::from("forward"),0),
        (String::from("up"), -1),
        (String::from("down"), 1)
    ]);

    // Part 1:
    let filename = "src/input.txt";
    // 1. Read in input file to vector of tuples basically 
    let file = File::open(filename).expect("File wasn't found.");
    let reader = BufReader::new(file);

    // 2. process 
    let input: Vec<Vec<String>>= reader.lines()
        .map(|line| line.unwrap().split_whitespace().map(|s| String::from(s)).collect())
        .collect();

    // 3. calculate position, this is pretty difficult to read not sure I love it
    // map (forward, up, down) to (0, -1, 1) for easy multiplication trick
    // then for each line do the multiplication to figure out the eventual position
    input.iter()
        .map(|f| (*direction_lookup.get(&f[0]).unwrap(), *(&f[1].parse::<i32>().unwrap())))
        .for_each(|f| {
            position.1 += f.0 * f.1;
            position.0 += (1 - f.0.abs()) * f.1;
        });

    println!("horizontal {}, depth {}",position.0,position.1);
    println!("Multiplied {}", position.0 * position.1);
        
    // Part 2:
    let mut trajectory: (i32, i32, i32) = (0,0,0); // (horizontal, depth, aim)

    // input calculation is the same as above

    // but our processing of it is different
    input.iter()
        .map(|f| (*direction_lookup.get(&f[0]).unwrap(), *(&f[1].parse::<i32>().unwrap())))
        .for_each(|f| {
            trajectory.2 += f.0 * f.1; // aim changes with up/down
            trajectory.1 += (1 - f.0.abs()) * f.1 * trajectory.2;
            trajectory.0 += (1 - f.0.abs()) * f.1;
        });

        println!("horizontal {}, depth {}",trajectory.0,trajectory.1);
        println!("Multiplied {}", trajectory.0 * trajectory.1);
}

