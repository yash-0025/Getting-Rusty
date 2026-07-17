use std::fs;

fn generate_logs() {
    // Attempts to create a folder called data

    // fs::create_dir returns a Result either Ok or Err. If the folder already esxists it return an Err. 
    //  We are intentionally ignoring the result . The underscore _ tells the Rust compiler . I know this might fail but I don't care throw the error away and don't warn me about an unused result
    let _ = fs::create_dir("data");

    // Creates a new mutable empty string on the Heap, but pre allocates a massive chunk of memory right from the start
    // Usually if we use String::new() and keep adding data to it , Rust has to constantly pause ask the OS for more memory , copy everything over and delete the old memory . By telling it exactly how much we need upfront 100000 * ~50 characters per line = 5 million bytes , it asks the OS for memory exactly once. this makes the code lightning fast
    // we are generating 100,000 fake server log lines
    let mut logs = String::with_capacity(100_000 * 50);
    // A standard loop from 1 upto 100,000 The = makes it inclusive
    for i in 1..=100_000 {
        // Some are INFO some are ERROR
        // Determines if a log line should be an error
        // Every 10th loop i % 10 == 0 it binds the string slice ERROR to level. Otherwise it bings INFO .
        // In Rust if is an expression. It returns a value , We don't need to declare a mutable variable and reassign it we just evaluate the if and bind the result directly to let level
        let level = if i % 10 == 0 { "ERROR" } else { "INFO" };

        // constructs the actual log string
        // format! works exactly like println! but instead of printing to the terminal it returns a brand new String allocated on the heap. We inject our level variable and fake a User i%5 loop from 0 to 4  . \n adds a newline character
        let log = format!("[2026-07-17T12:00:00Z] {} User_{} logged in\n", level, i % 5);
        // Appends small log string to our massive logs string buffer
        // &log : push_str expects a string slice &str so we pass a borrowed reference to our log variable using the & symbol
        logs.push_str(&log);
    }

    // Writes the entire 5MB logs string to the hard drive in single operation
    // .unwrap() - If the hard drive is full or we don't have permission to write .unwrap() will intentionally crash panic the program. For a simple script this is acceptable
    fs::write("data/server.log", logs).unwrap();
    println!("Generated server.log with 100,000 lines!");
}

fn main() {
    generate_logs();
}
