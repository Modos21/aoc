pub mod task;

use crate::task::Part;

fn main() {
    let sum = task::run("day03/input/input.txt", Part::Two).expect("Failed to read file");
    println!("Sum of biggest possible joltages: {sum}")
}