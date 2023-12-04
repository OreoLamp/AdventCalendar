#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![feature(pattern)]

#[allow(non_snake_case)]
mod Day1;
#[allow(non_snake_case)]
mod Day2;

fn main() {
    // Input arguments are the path to the file, who knows why it's borrowed
    let binding: Vec<String> = std::env::args().collect::<Vec<String>>();
    let args: Vec<&str> = binding.iter().map(|str: &String| &**str).collect::<Vec<&str>>();

    let answer: String = solution_caller(args);

    println!("The answer is: {answer}");
}

fn solution_caller(args: Vec<&str>) -> String {
    match args[1] {
        "d1p1" => {
            Day1::d1p1::day1pt1(args[2..args.len() - 1].to_vec()).to_string()
        }
        "d1p2" => {
            Day1::d1p2::day1pt2(args[2..args.len() - 1].to_vec()).to_string()
        }
        "d2p1" => {
            Day2::d2p1::day2pt1(args[2..args.len() - 1].to_vec()).to_string()
        }
        _ => {
            "No such advent calendar solution!".to_string()
        }
    }
}