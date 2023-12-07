#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![feature(pattern)]

#[allow(non_snake_case)]
mod Day1;
#[allow(non_snake_case)]
mod Day2;
#[allow(non_snake_case)]
mod Day3;

fn main() {
    // Input arguments are the path to the file, who knows why it's borrowed
    let binding: Vec<String> = std::env::args().collect::<Vec<String>>();
    let args: Vec<&str> = binding
        .iter()
        .map(|str: &String| &**str)
        .collect::<Vec<&str>>();

    let answer: String = solution_caller(&args);

    println!("The answer is: {answer}");
}

fn solution_caller(args: &[&str]) -> String {
    match args[1] {
        "d1" => {
            assert!(args.len() == 4);
            match args[2] {
                "p1" => Day1::p1(args[3]).to_string(),
                "p2" => Day1::p2(args[3]).to_string(),
                _ => "No such advent calendar solution!".to_string(),
            }
        }
        "d2" => {
            assert!(args.len() == 4);
            match args[2] {
                "p1" => Day2::p1(args[3]).to_string(),
                "p2" => Day2::p2(args[3]).to_string(),
                _ => "No such advent calendar solution!".to_string(),
            }
        }
        "d3" => {
            assert!(args.len()==4);
            match args[2] {
                "p1" => Day3::p1(args[3]).to_string(),
                _ => "No such advent calendar solution!".to_string(),
            }
        }
        _ => "No such advent calendar solution!".to_string(),
    }
}
