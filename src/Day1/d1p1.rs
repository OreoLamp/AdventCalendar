pub fn day1pt1(args: Vec<&str>) -> i32 {
    let filepath: &str = args[0];

    // Read the file into a string all at once. Bad for big files, but this is small enough.
    let input: &str = &std::fs::read_to_string(filepath).expect("Unreadable file");

    // Split into lines, and collect to a vector because otherwise it causes issues later
    let lines: Vec<&str> = input.lines().collect::<Vec<&str>>();

    // Get the indecies of the digits, but as options (since they might not be there)
    let digit_options = lines
        .iter()
        .map(|line: &&str| -> (usize, usize) { digit_finder(line) });

    // Get the actual numbers for each line, so unwrap the options and make a number out of them
    let numbers = std::iter::zip(lines.iter(), digit_options)
        .map(|(line, indecies)| -> u8 { number_from_digits(line, &indecies) });

    // Calculate the sum, using fold because sum for some reason can't convert types for me...
    let sum: &i32 = &numbers.fold(0, |acc: i32, x: u8| acc + i32::from(x));

    *sum
}

#[allow(clippy::inline_always)]
#[inline(always)]
fn digit_finder(input: &str) -> (usize, usize) {
    let digit_chars: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let tens: usize = input.find(digit_chars).unwrap();
    let ones: usize = input.rfind(digit_chars).unwrap();
    (tens, ones)
}

#[allow(clippy::inline_always)]
#[inline(always)]
const fn number_from_digits(line: &str, indecies: &(usize, usize)) -> u8 {
    // So, basically, subtracting 48 from the byte converts UTF-8 / ASCII digits to the
    // int values of their respective characters lmao
    // Then just multiply the first one by 10 and add the second one to it
    // Converting via rusts internal things would be slow as balls lol
    (line.as_bytes()[indecies.0] - 48) * 10 + (line.as_bytes()[indecies.1]) - 48
}
