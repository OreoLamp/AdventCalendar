#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![feature(pattern)]

extern crate alloc;
use std::env;
use std::fs;
use alloc::str::pattern::Pattern;

fn main() {
    let args: &String = &env::args().collect::<Vec<String>>()[1];
    let input: &str = &fs::read_to_string(args).expect("Unreadable file");
    let digit_chars: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let lines: Vec<&str> = input.lines().collect::<Vec<&str>>();
    // This is actually disgusting, but to be fair, 98% of this is type annotations
    let digit_options = lines.iter().map(|line: &&str| -> (Option<usize>, Option<usize>) 
    {digit_finder(line, digit_chars)})
    .collect::<Vec<(Option<usize>, Option<usize>)>>();
}


#[allow(clippy::inline_always)]
#[inline(always)]
fn digit_finder(input: &str, pats: Pattern) -> (Option<usize>, Option<usize>) {
    let tens: Option<usize> = input.find(pats);
    let ones: Option<usize> = input.rfind(pats);
    (tens, ones)
}


#[allow(clippy::inline_always)]
#[inline(always)]
fn number_constructor(){

}
