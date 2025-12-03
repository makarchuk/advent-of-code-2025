use clap::Parser;
mod tasks;
use tasks::task::Task;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    day: u32,
    part: u32,
}

fn main() {
    let args = Args::parse();

    let input = std::fs::read_to_string(format!("inputs/day{}", args.day))
        .expect("Failed to read input file");

    match (args.day, args.part) {
        (1, part) => {
            let task = tasks::day1::Task {};
            let result = task.run(&input, part);
            println!("Result: {}", result);
        }
        (2, part) => {
            let task = tasks::day2::Task {};
            let result = task.run(&input, part);
            println!("Result: {}", result);
        }
        (3, part) => {
            let task = tasks::day3::Task {};
            let result = task.run(&input, part);
            println!("Result: {}", result);
        }

        _ => println!("Task not implemented"),
    }
}
