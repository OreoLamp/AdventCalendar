pub fn part_1(path: &str) -> i32 {
    // Read the file into a string all at once. Bad for big files, but this is small enough.
    let input: &str = &std::fs::read_to_string(path).expect("Unreadable file");

    // Assert ascii to make everything a lot easier
    // (and to make the compiler not do utf8 safety checks xd)
    assert!(input.is_ascii());

    // A crap data type for a grid normally, but OK for this case
    // Since performance is completely irrelevant
    let mut schem: Vec<Vec<u8>> = input
        .lines()
        .map(|line: &str| line.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();

    // Get (line, index) tuples (aka coordinates) for all number characters in the schematic
    // This way of getting the indiecies of items in a vector is actually idiotic lol
    let number_indecies = schem.iter().enumerate().flat_map(|(ln, line)| {
        line.iter()
            .enumerate()
            .filter_map(move |(i, ch)| -> Option<(usize, usize)> {
                if ch.wrapping_sub(48) < 10 {
                    Some((ln, i))
                } else {
                    None
                }
            })
    });

    // Filter out all the number coords with a "symbol" next to them
    // Symbol being anything that isn't a digit or a dot
    let meaningless_numbers = number_indecies.filter(|(ln_origin, i_origin)| {
        // Check the line with the number and its neighbors, but only if they exist
        ((ln_origin - 1)..=(ln_origin + 1)).any(|ln: usize| schem.get(ln).is_some_and(|line: &Vec<u8>|
            // Check the row with the number and its neighbors, but only if they exist
            ((i_origin - 1)..=(i_origin + 1)).any(|index: usize| line.get(index).is_some_and(|ch: &u8|
                // Check if the character at this position is an ASCII digit or a dot
                // ASCII digits start from 48, and dot is 46, so this is relatively easy
                (*ch).wrapping_sub(46) < 12 && (*ch).wrapping_sub(46) != 1
            ))
        ))
    });

    0
}
