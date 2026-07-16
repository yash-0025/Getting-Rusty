// use std::thread;
// use std::time::Duration;


// fn main() {
//     // Give the main thread some data
//     let my_data = String::from("Secret Recipe");

//     // Spawn a new OS thread The line cook
//     // thread::spawn This function asks the Operating system to create a brand new physical hardware thread

//     // move || {...}  - This is the closure (the recipe ) we give the thread. We must use the move keyword because the closure is capturing my_data from the main function. If we didn't use move the thread would only borrow my_data and rust would panic because the main thread might finish and destroy my_data before the new thread finishes

//     // let handle = spawn returns a JoinHandle. This is like a pager we get at a restaurant . It allows us to check on the thread or wait for it.

//     let handle = thread::spawn(move || {
//         println!("Line Cook : Started working on {}", my_data);
//         thread::sleep(Duration::from_secs(2));
//         println!("Line cook: Finished");

//         // Closure returns a value back to the main
//         return 42;
//     });
//     // Main thread keeps workign simultaneuously
//     println!("Main Chef : I am boiling water while the cook works!");
//     thread::sleep(Duration::from_secs(1));
//     println!("Main Chef: Water is boiling");

//     // Wait for the line cook to finish
//     // handle.join() - this blocks the main thread . the main thread will literally pause execution on this exact line until the spawned thread finishes and returns it  value (42)

//     // .unwrap() - If the spawned thread crashes (panics), .join() will return an Err. We use .unwrap() to say if the line cook crashed I want to crash too.
//     let result = handle.join().unwrap();
//     println!("Main chef: The cook returned the number: {}", result);
// }



use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn generate_dummy_files() {
    // Craate a data directory ignore the error if it already exists
    // let _ = fs::create_dir(data) - the underscore _ tells the ocmpiler I know this returns a result but i don't care if it failes like if the folder already exists just ignore it
    let _ = fs::create_dir("data");

    let text = "rust is fast and rust is safe and rust is fun";

    // Loop 5 times to create 5 files
    for i in 1..=5 {
        let filename = format!("data/file{}.txt", i);
        // .repeat(100_000) takes our small string and copies it 100,000 times
        let content = text.repeat(100_000);
        // Write the massive string to the file
        let _ = fs::write(filename, content);
    }

    println!("Generate 5 massive dummy files!");
}

fn main() {
    // Generate the files
    generate_dummy_files();
// Instant::now() - We grab the exact time on the CPU clock right before the counting starts
    // Start the stopwatch
    let start = Instant::now();

    // Create our single threaded HashMap
    let mut word_counts: HashMap<String, u32> = HashMap::new();

    // Single threaded loop
    for i in 1..=5 {
        let filename = format!("data/file{}.txt", i);

        // Read the entire 4.5 mb file into a string
        let content = fs::read_to_string(filename).unwrap();

        // Loop over every single word in that string
        // .split_whitespace() - An iterator that splits a string into chunks separated by spaces or newlines
    
        for word in content.split_whitespace() {
            // Find the word in the Hashmap. If it's not there insert a 0
            // word.to_string() - word is jus a borrowd &str. We must convert it to an owned string so the HashMap can take permanet ownership of it 
            let count = word_counts.entry(word.to_string()).or_insert(0);
            // Dreference the pointer * and add 1 to the count
            *count += 1;

        }
    }
// Stop the stopwatch and print the results
// start.elapsed() - Check how much time has passed since we called Instant::now()
    let duration = start.elapsed();
    println!("Single Threaded time : {:?}", duration);

    // Check how many times the word rust appeared
    let rust_count = word_counts.get("rust").unwrap_or(&0);
    println!("The word 'rust' appears {} times.", rust_count);

}