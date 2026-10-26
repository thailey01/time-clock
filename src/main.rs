use std::env;

struct Session {
    id: u32,
    start: u64,
    end: Option<u64>,
    project_id: Option<u32>,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!("Invalid number of arguments");
    }

    let first: u64 = args[1].parse::<u64>().expect("Not a valid number");
    let second: u64 = args[2].parse::<u64>().expect("Not a valid number");

    let result1 = first - second;
    let result2 = second - first;

    println!("{result1}");
    println!("{result2}");
}
