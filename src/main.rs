use std::{env, process};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use chrono::Local;
use calc::calculator::{CalcOp, Calculator};

/// Main program.
fn main() {
    let args: Vec<String> = env::args().collect();
    let calc = Calculator::from_args(&args).unwrap_or_else(|err| {
        // Prints output to stderr
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    match calc.op {
        // Perform calculations if it is add/sub/mul/div
        CalcOp::Add | CalcOp::Sub | CalcOp::Mul | CalcOp::Div => {
            let res: i32 = calc.calculate();

            // Write to file
            log_calculation_to_file(&calc, res);
            println!("Result: {res}");
        },
        // Show calculation history
        CalcOp::History => {
            if let Ok(file_exists) = Path::new(calc::calculator::HISTORY_FILE).try_exists() {
                if !file_exists {
                    println!("Calculation history does not exist.");
                }
                else {
                    if let Ok(mut file) = File::open(calc::calculator::HISTORY_FILE) {
                        let mut contents = String::new();
                        file.read_to_string(&mut contents).expect("Able to read history file");

                        println!("[History]\n{}", contents);
                    }
                    else {
                        eprintln!("Failed to open history file.");
                    }
                }
            }
            else {
                eprintln!("Failed to check if history file exists.");
            }
        }
    }
}

/// Logs the calculation result to history file.
fn log_calculation_to_file(calc: &Calculator, res: i32) {
    // Get date/time now
    let dt = Local::now();

    // Prepare the line of text to be written
    let line = format!("{}\t{} {} {} (Result={})\n",
                       dt.format("%Y%m%d %H:%M:%S"),
                       calc.op, calc.n1, calc.n2,
                       res);

    // Check if file exists: if no, create one
    if let Ok(file_exists) = Path::new(calc::calculator::HISTORY_FILE).try_exists() {
        if !file_exists {
            File::create(calc::calculator::HISTORY_FILE).unwrap();
        }
    }

    // Opens the file and prepare it for writing
    let file = OpenOptions::new()
        .append(true)
        .open(calc::calculator::HISTORY_FILE);

    // Write to file
    if let Ok(mut file) = file {
        file.write(&line.as_bytes()).expect("Able to write into file");
    }
    else {
        eprintln!("Failed to write to file.");
    }
}