pub fn day2pt1(args: Vec<&str>) -> i32 {
    let filepath: &str = args[0];

    // Read the file into a string all at once. Bad for big files, but this is small enough.
    let input: &str = &std::fs::read_to_string(filepath).expect("Unreadable file");
}
