use aho_corasick::AhoCorasick;

pub fn part_2(path: &str) -> i32 {
    // Digit pattern list for use in the finder
    const DIGITS: [&str; 20] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1",
        "2", "3", "4", "5", "6", "7", "8", "9",
    ];

    // Read the file into a string all at once. Bad for big files, but this is small enough.
    let input: &str = &std::fs::read_to_string(path).expect("Unreadable file");

    // Split into lines, and collect to a vector because otherwise it causes issues later
    let lines: Vec<&str> = input.lines().collect::<Vec<&str>>();

    // Construct an aho-corasick finder for pattern matching
    // Done here because the construction takes ages.
    let finder: AhoCorasick = AhoCorasick::builder()
        .ascii_case_insensitive(false)
        .build(DIGITS)
        .unwrap();

    // Gets the digits as numbers and flattens the result
    let numbers = lines
        .iter()
        .flat_map(|line: &&str| -> [i32; 2] { digit_finder(line, &finder) });

    numbers.sum()
}

#[allow(clippy::inline_always)]
#[inline(always)]
fn digit_finder(input: &str, finder: &AhoCorasick) -> [i32; 2] {
    // .pattern returns the pattern ID, which is the ordinal in which the pattern was added
    // So in my case because the list of patterns is what it is, i should be able to just
    // divide by 10 and take the remainder (not modulo because ??? rust ???)
    // and it should be the correct number
    // Other than that this just constructs a vector of all the possible matches, overlaps included
    let all_matches: Vec<i32> = finder
        .find_overlapping_iter(input)
        .map(|mat: aho_corasick::Match| -> i32 { mat.pattern().as_i32() % 10 })
        .collect::<Vec<i32>>();

    // The fact that this is a list of length 2 is fucking stupid
    // But for some reason unpack doesnt work on tuples...
    return [
        *all_matches.first().unwrap() * 10,
        *all_matches.last().unwrap(),
    ];
}
