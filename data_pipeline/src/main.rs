// we import File to open file 
use std::fs::File;
// BuffRead  - Buffered Reader trait which gives us the magical .lines() method to read files memory efficiently.
use std::io::{self, BufRead};
// mpsc to create our channels and thread to spawn background tasks
use std::sync::mpsc;
use std::thread;


// fn generate_logs() {
//     // Attempts to create a folder called data

//     // fs::create_dir returns a Result either Ok or Err. If the folder already esxists it return an Err. 
//     //  We are intentionally ignoring the result . The underscore _ tells the Rust compiler . I know this might fail but I don't care throw the error away and don't warn me about an unused result
//     let _ = fs::create_dir("data");

//     // Creates a new mutable empty string on the Heap, but pre allocates a massive chunk of memory right from the start
//     // Usually if we use String::new() and keep adding data to it , Rust has to constantly pause ask the OS for more memory , copy everything over and delete the old memory . By telling it exactly how much we need upfront 100000 * ~50 characters per line = 5 million bytes , it asks the OS for memory exactly once. this makes the code lightning fast
//     // we are generating 100,000 fake server log lines
//     let mut logs = String::with_capacity(100_000 * 50);
//     // A standard loop from 1 upto 100,000 The = makes it inclusive
//     for i in 1..=100_000 {
//         // Some are INFO some are ERROR
//         // Determines if a log line should be an error
//         // Every 10th loop i % 10 == 0 it binds the string slice ERROR to level. Otherwise it bings INFO .
//         // In Rust if is an expression. It returns a value , We don't need to declare a mutable variable and reassign it we just evaluate the if and bind the result directly to let level
//         let level = if i % 10 == 0 { "ERROR" } else { "INFO" };

//         // constructs the actual log string
//         // format! works exactly like println! but instead of printing to the terminal it returns a brand new String allocated on the heap. We inject our level variable and fake a User i%5 loop from 0 to 4  . \n adds a newline character
//         let log = format!("[2026-07-17T12:00:00Z] {} User_{} logged in\n", level, i % 5);
//         // Appends small log string to our massive logs string buffer
//         // &log : push_str expects a string slice &str so we pass a borrowed reference to our log variable using the & symbol
//         logs.push_str(&log);
//     }

//     // Writes the entire 5MB logs string to the hard drive in single operation
//     // .unwrap() - If the hard drive is full or we don't have permission to write .unwrap() will intentionally crash panic the program. For a simple script this is acceptable
//     fs::write("data/server.log", logs).unwrap();
//     println!("Generated server.log with 100,000 lines!");
// }


fn main() {
    // generate_logs();

    // Create the conveyor belt (channel)
    // Creates our conveyor belt in memory.
    // mpsc::channel() returns a tuple containing two variables:the transmitter tx and the receiver rx . We destructure the tuple instantly using let(tx, rx)
    // We need tx to give to the worker thread and rx to keep in the main thread.
    let (tx, rx) = mpsc::channel();

    // Spawn the Reader Thread (producer)
    // We spawn OS thread. We must use the move keyword so that the thread takes complete ownership of the tx variable from the main thread. If we didn't use move, the thread would only borrow tx , which Rust forbids because the main thread might die before the background thread finishes
    thread::spawn(move || {

        // Opens the file and wraps it in a BuffReader
        // If we use fs::read_to_string() it would load all 5mb of text into RAM at once.
        // BufReader pulls small chunks of data off the hard drive one by one keeping memory usage near zero even if file was 100 gigabytes 
        let file = File::open("data/server.log").unwrap();
        let reader = io::BufReader::new(file);


        // Read the file line by line 
        // reader.lines() is an iterator that yields each line of the file
        // because reading from a hard drive can fail, lines() returns a Result. We unwrap() it to get the actual String line
        // tx.send(line) takes the String variable and moves ownership of it into the channel. The compiler guarantees this thread can no longer touch line. We unwrap the send because it will return an error if the receiver rx has been destroyed
        for line_result in reader.lines() {
            let line = line_result.unwrap();

            // Put the line on the conveyor belt
            tx.send(line).unwrap();

        }

        // The thread finishes here
        // tx goes out of scope and is automatically dropped, closing the channel
    });
    // The main thread reads from the belt
    let mut count = 0;

    // We can loop over the receiver It will block and wait for new data
    // The main thread iterates over the rx receiver
    // This loop will automatically put the main thread to sleep if the belt is empty . As soon as the background thread calls send(). the main thread wakes up pulls the STring off the belt binds it to received_line and runs the loop body .
    // When the background thread finishes reading the file it hits the bottom of the closure . The tx variable is destroyed dropped. The rx receiver detects that all transmitters are dead and it gracefully exits the for loop
    for received_line in rx {
        count += 1;
        if count <= 5 {
            println!("Received: {}", received_line);
        }
    }

    println!("Finished! Total lines received: {}", count);
}
