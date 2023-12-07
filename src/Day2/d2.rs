use std::collections::HashMap;


pub fn part_1(path: &str) -> i32 {
    // Read the file given in the arguments into a string all at once.
    // Bad for big files, but this is small enough.
    let input: &str = &std::fs::read_to_string(path).expect("Unreadable file");

    // Check which games are doable with 12 red, 13 green, and 14 blue cubes
    let round_ids = game_processor(input).filter(|game: &(i32, HashMap<&str, i32>)| 
        game.1["red"] <= 12 && 
        game.1["green"] <= 13 &&
        game.1["blue"] <= 14
    );

    // Sum up the game IDs, done with fold because I need to access the .0 of each round
    round_ids.fold(0, |acc: i32, id: (i32, HashMap<&str, i32>)| acc + id.0)
}

pub fn part_2(path: &str) -> i32 {
    // Read the file given in the arguments into a string all at once.
    // Bad for big files, but this is small enough.
    let input: &str = &std::fs::read_to_string(path).expect("Unreadable file");

    // Get the power of each set of cubes
    let powers = game_processor(input).map(
       |game: (i32, HashMap<&str, i32>)| game.1.values().product::<i32>());

    powers.sum()
}

#[allow(clippy::inline_always)]
#[inline(always)]
fn game_processor(games: &str) -> impl Iterator<Item = (i32, HashMap<&str, i32>)> {
    // Parse the input to a more sane format
    let games = games
        // Split games from each other, each game is in it's own line
        .lines()
        // Parse the games into a sane format
        .map(game_splitter);

    // Get the highest amount of cubes of each color seen at once for each game
    games.map(|game| (game.0, max_of(game.1.flat_map(max_of))))
}

#[allow(clippy::inline_always)]
#[inline(always)]
fn game_splitter(game: &str) -> (i32, impl Iterator<Item = impl Iterator<Item = (&str, i32)>>) {
    // Split the game ID from the rounds, as they need different kinds of parsing
    let wip: (&str, &str) = game.split_once(':').unwrap();

    // Parse the game ID (literally just convert the number to an actual int)
    let game_id: i32 = wip.0.split_whitespace().last().unwrap().parse::<i32>().unwrap();

    // Constructs a tree-like structure of iterators
    // Each "top-level" iterable is a semicolon-delimited round in the game
    // Each round is an iterator of (color, amount) tuples
    let rounds = wip.1.split(';').map( |round: &str| {

        // Within each round, color entries are delimited by commas
        round.split(',').map( |color: &str| 

            // Color entry format is "number color", so get those to a vector
            // Vector because the size of the collection has to either be knowable at compile time
            // (Which it is, it's always 2, but i don't know how to tell Rust that lol)
            // Or the collection has to be growable, which vec is
             color.split_whitespace().collect::<Vec<&str>>())

            // Convert the vectors to (color, amount) tuples
            .map(|color: Vec<&str>| (color[1], color[0].parse::<i32>().unwrap())
        )
    });
    (game_id, rounds)
}

#[allow(clippy::inline_always)]
#[inline(always)]
fn max_of<'a>(input: impl Iterator<Item = (&'a str, i32)>) -> HashMap<&'a str, i32> {
    // Initialize the hashmap, this is literally the only reason i made this its own function lol
    let mut most_seen: HashMap<&str, i32> = HashMap::new();

    // For each item, insert the value of the second item in the tuple
    // if it's larger than the existing value, otherwise keep existing value (but branchless)
    for item in input {
        most_seen
            .entry(item.0)
            .and_modify(|old: &mut i32| *old = i32::max(*old, item.1))
            .or_insert(item.1);
    }

    most_seen
}
