use day00::read_text_file;
use day00::solve_day;

fn main() {
    println!("Hello, Advent of Code!");
    let parsed_lines = read_text_file();

    for parsed_line in parsed_lines {
        println!("{}", parsed_line); // This will use the Display implementation
    }

    println!("{}", solve_day());
}
