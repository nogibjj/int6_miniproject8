use clap::{Parser, ValueEnum};
use int6_miniproject7::{calculate_triangle_area, calculate_triangle_perimeter}; // Import functions from lib.rs

/// A CLI tool that calculates the area or perimeter of a triangle
#[derive(Parser)]
struct Cli {
    /// Choose the operation: area or perimeter
    #[clap(value_enum)]
    mode: Mode,

    /// The lengths of the three sides of the triangle
    #[clap(required = true, num_args(3))]
    sides: Vec<f64>,
}

#[derive(ValueEnum, Clone)]
enum Mode {
    Area,
    Perimeter,
}

use std::process;

fn main() {
    let args = Cli::parse();

    let a = args.sides[0];
    let b = args.sides[1];
    let c = args.sides[2];

    match args.mode {
        Mode::Area => {
            match calculate_triangle_area(a, b, c) {
                Some(area) => println!("Area of the triangle: {:.2}", area),
                None => {
                    eprintln!("Invalid triangle sides provided.");
                    process::exit(1); // Exit with non-zero code on error
                }
            }
        }
        Mode::Perimeter => {
            match calculate_triangle_perimeter(a, b, c) {
                Some(perimeter) => println!("Perimeter of the triangle: {:.2}", perimeter),
                None => {
                    eprintln!("Invalid triangle sides provided.");
                    process::exit(1); // Exit with non-zero code on error
                }
            }
        }
    }
}
