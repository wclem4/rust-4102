// Rust CSV Sorter
// ITCS4102
//
// Authors: Walker Clem | Sam Edwards | Jordan Smiley | Max Ezzell

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io;

fn main() {
    println!("Welcome to the Rust CSV Sorter!");

    // save csv path to open, with error checking
    let mut csv_path = String::new();
    println!("Enter path to the CSV you would like to sort:");
    io::stdin()
        .read_line(&mut csv_path)
        .expect("Failed to read csv path.");

    // handle opening file and putting into a reader
    let file = File::open(csv_path.trim()).unwrap();
    let reader = BufReader::new(file);

    // first row vector
    let mut column_names: Vec<String> = Vec::new();
    // 2d vector of the rest of the data
    let mut data: Vec<Vec<String>> = Vec::new();
    for (i, line) in reader.lines().enumerate() {
        // splits up the line by comma, and maps into vector
        let row: Vec<String> = line.unwrap().split(',').map(|s| s.to_string()).collect();
        if i == 0 { // if first row, save column names
            column_names = row.clone();
        } else { // else, add to 2d vector with rest of data
            data.push(row);
        }
    }

    println!("We have found the following column names in the inputted CSV: {:?}", column_names);

    /* prints all data
        for row in data {
            println!("{:?}", row);
        }
    */
}
