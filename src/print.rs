pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    // Basic Formatting
    println!("Number: {}", 1);
    println!("{} is from {}", "Ayush ", "Indore");

    // Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Ayush", "Indore", "code"
    );

    // Named Arguments
    println!(
        "{name} likes to play {activity}",
        name = "Ayush",
        activity = "Badminton"
    );
}
