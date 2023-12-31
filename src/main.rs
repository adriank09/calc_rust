use std::{env, process};
use calc::{CalcOp, Calculate};

fn main() {
    let args: Vec<String> = env::args().collect();
    let calc = Calculate::from_args(&args).unwrap_or_else(|err| {
        // Prints output to stderr
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Performing '{}' on '{}' and '{}'.", calc.op, calc.n1, calc.n2);
    let res: i32 = calc.calculate();
    println!("Result: {res}");
}
