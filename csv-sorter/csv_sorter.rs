// Rust CSV Sorter
// ITCS4102
//
// Authors: Walker Clem | Sam Edwards | Jordan Smiley | Max Ezzell
//
// /testdata sourced from https://people.sc.fsu.edu/~jburkardt/data/csv/csv.html

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io;
use std::cmp::Ordering;

// implementation of bubble sort algorithm
fn bubble_sort(data: &mut Vec<Vec<String>>, column_index: usize, ascending: bool) {
    let mut n = data.len();
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 1..n {
            let mut order = Ordering::Greater;
            if ascending {
                order = Ordering::Less;
            }
            if data[i][column_index].partial_cmp(&data[i - 1][column_index]).unwrap() == order {
                data.swap(i, i - 1);
                swapped = true;
            }
        }
        n -= 1;
    }
}

// implementation of insertion sort algorithm
fn insertion_sort(data: &mut Vec<Vec<String>>, column_index: usize, ascending: bool) {
    for i in 1..data.len() {
        let mut j = i;
        let mut order = Ordering::Greater;
        if ascending {
            order = Ordering::Less;
        }
        while j > 0 && data[j][column_index].partial_cmp(&data[j - 1][column_index]).unwrap() == order {
            data.swap(j, j - 1);
            j -= 1;
        }
    }
}

// implementation of selection sort algorithm
fn selection_sort(data: &mut Vec<Vec<String>>, column_index: usize, ascending: bool) {
    for i in 0..data.len() {
        let mut min_idx = i;
        for j in (i + 1)..data.len() {
            let mut order = Ordering::Greater;
            if ascending {
                order = Ordering::Less;
            }
            if data[j][column_index].partial_cmp(&data[min_idx][column_index]).unwrap() == order {
                min_idx = j;
            }
        }
        if min_idx != i {
            data.swap(min_idx, i);
        }
    }
}

fn main() {
    println!("Welcome to the Rust CSV Sorter!");

    // save csv path to open, with error checking
    let mut csv_path = String::new();
    println!("Enter path to the CSV you would like to sort:");
    io::stdin()
        .read_line(&mut csv_path)
        .expect("Failed to read csv path.");

    // handle opening file and putting into a reader
    let file = match File::open(csv_path.trim()) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Failed to open csv file: {}", err);
            return;
        }
    };
    let reader = BufReader::new(file);

    // first row vector
    let mut column_names: Vec<String> = Vec::new();
    // 2d vector of the rest of the data
    let mut data: Vec<Vec<String>> = Vec::new();
    for (i, line) in reader.lines().enumerate() {
        let row: Vec<String> = line.unwrap().split(',').map(|s| s.to_string()).collect();
        if i == 0 { // if first row, save column names
            column_names = row.clone();
        } else { // else, add to 2d vector with rest of data
            data.push(row);
        }
    }

    println!("We have found the following column names in the inputted CSV: {:?}", column_names);

    // get column choice
    let mut column_choice = String::new();
    loop {
        println!("Which column would you like to sort by?");
        io::stdin()
            .read_line(&mut column_choice)
            .expect("Failed to read column choice.");
        let trimmed_choice = column_choice.trim();
        println!("choice: {:?}", trimmed_choice);
        if column_names.contains(&trimmed_choice.to_string()) {
            break;
        }
        column_choice.clear();
        println!("That column is not present in the inputted CSV. Try again!");
    }

    // sorting algorithms choices
    let sorting_algorithms = vec![
        ("Bubble sort", bubble_sort as fn(&mut Vec<Vec<String>>, usize, bool)),
        ("Insertion sort", insertion_sort as fn(&mut Vec<Vec<String>>, usize, bool)),
        ("Selection sort", selection_sort as fn(&mut Vec<Vec<String>>, usize, bool)),
    ];

    // get sorting algorithm choice number
    println!("Choose a sorting algorithm:");
    for (i, algorithm) in sorting_algorithms.iter().enumerate() {
        println!("{}) {}", i + 1, algorithm.0);
    }
    let mut algorithm_choice = String::new();
    let mut order_choice = String::new();
    loop {
        io::stdin()
            .read_line(&mut algorithm_choice)
            .expect("Failed to read algorithm choice.");
        let trimmed_algorithm_choice = algorithm_choice.trim();
        match trimmed_algorithm_choice.parse::<usize>() {
            Ok(choice) => {
                if choice > 0 && choice <= sorting_algorithms.len() {
                    loop {
                        // get sorting order
                        println!("Sort in ascending (A) or descending (D) order?");
                        io::stdin()
                            .read_line(&mut order_choice)
                            .expect("Failed to read order choice.");
                        let trimmed_order_choice = order_choice.trim();
                        let column_index = column_names.iter().position(|name| name == &column_choice.trim()).unwrap();
                        if trimmed_order_choice == "A" || trimmed_order_choice == "a" {
                            let algorithm = &sorting_algorithms[choice - 1];
                            algorithm.1(&mut data, column_index, true);
                            break;
                        } else if trimmed_order_choice == "D" || trimmed_order_choice == "d" {
                            let algorithm = &sorting_algorithms[choice - 1];
                            algorithm.1(&mut data, column_index, false);
                            break;
                        }
                        order_choice.clear();
                        println!("Invalid choice. Try again.");
                    }
                    break;
                }
            }
            Err(_) => (),
        }
        algorithm_choice.clear();
        order_choice.clear();
        println!("Invalid choice. Try again.");
    }

    // prints all sorted data!
    println!("Here is the sorted data:");
    for row in data {
        println!("{:?}", row);
    }
}
