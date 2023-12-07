pub fn part_1(path: &str) -> i32 {
    // Read the file into a string all at once. Bad for big files, but this is small enough.
    let input: &str = &std::fs::read_to_string(path).expect("Unreadable file");

    // Assert ascii to make everything a lot easier lol
    assert!(input.is_ascii());

    // Get the line length of the schematic for indexing purposes later
    let line_len: usize = input.find('\n').unwrap();

    // A single iterator because I'll need to go through the entire thing anyway
    // Splitting it up to more would just make the whole thing way more annoying
    let schematic: &str = input.trim_matches(|char: char| char == '\n');

    let seed_indecies = schematic.find();

    0
}
