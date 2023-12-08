use core::str::from_utf8;

pub fn part_1(_path: &str) -> i32 {
    // Read the file into a string all at once. Bad for big files, but this is small enough.
    // let input: &str = &std::fs::read_to_string(path).expect("Unreadable file");
    let input = 
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    // Assert ascii to make everything a lot easier
    // (and to make the compiler not do utf8 safety checks xd)
    assert!(input.is_ascii());

    // A crap data type for a grid normally, but OK for this case
    // Since performance is completely irrelevant
    let schem: Vec<Vec<u8>> = input
        .lines()
        .map(|line: &str| line.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();

    // Get the coordinates of every number in the schematic, as well as the number of digits in them
    // This way of getting the indiecies of items in a vector is actually idiotic but I don't know anything better
    let number_indecies = schem.iter().enumerate().flat_map(|(line_number, line)| {
        // Constructs a sliding-window iterator of length 4 for filter_map
        line.windows(4).enumerate().filter_map(move |(index, slice)|
        // Detects starts of numbers, only return something when one is found
        (!slice[0].is_ascii_digit() && slice[1].is_ascii_digit()).then_some(
            // Construct the (ln, i, len) tuple. First element of slice is the char before the first digit,
            // so the number starts at i+1, len is calculated by checking at which index do the 
            (line_number, index + 1, slice[1..].iter().position(|ch| !ch.is_ascii_digit()).unwrap_or(3))
        ))
    });

    // for i in &number_indecies {
    //     println!("{}, {}, {}", i.0, i.1, i.2);
    // }

    // Filter out coords with a "symbol" next to them
    // Symbol being anything that isn't a digit or a dot
    let part_number_positions = number_indecies.filter(|(line_number, index, length)| 
        // Check the line that the number is on as well as its immediate neighbors
        // Everything is done with get and is_some_and because get handles nonexisting indexes but returns options
        (line_number.saturating_sub(1)..=line_number + 1).any(|ln| schem.get(ln).is_some_and(
            // Check all the rows any digit of the number occupies as well as their immediate neighbors
            |line| (index.saturating_sub(1)..=index + length).any(|i| line.get(i).is_some_and(|ch|
                // If any of them contain a character that isn't an ascii digit or a dot (46 in ascii), yeet them
                !ch.is_ascii_digit() && *ch != 46)
            )
        )
    ));

    // for pos in &part_number_positions {
    //     println!("{}, {}, {}", pos.0, pos.1, pos.2);
    // }

    // Calculate the sum of all relevant parts
    let part_numbers = part_number_positions.map(|(ln, i, len)| 
        // from_utf8 makes an Option(&str) that can then be unwrapped and converted
        from_utf8(&schem[ln][i..i+len]).unwrap().parse::<i32>().unwrap()
    );

    // for part in &part_numbers {
    //     println!("{part}");
    // }

    part_numbers.sum()
}
