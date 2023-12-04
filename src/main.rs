#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![feature(pattern)]

#[allow(non_snake_case)]
mod Day1;

fn main() {
    // Input arguments are the path to the file, who knows why it's borrowed
    let binding: Vec<String> = std::env::args().collect::<Vec<String>>();
    let args = binding.iter().map(|str: &String| &**str).collect::<Vec<&str>>();

    let answer: String = solution_caller(&args);

    println!("The answer is: {answer}");
}

fn solution_caller(args: &[&str]) -> String {
    match args[1] {
        "d1p1" => {
            Day1::d1p1::day1pt1(args[2]).to_string()
        }
        "d1p2" => {
            Day1::d1p2::day1pt2(args[2]).to_string()
        }
        _ => {
            "No such advent calendar solution!".to_string()
        }
    }
}