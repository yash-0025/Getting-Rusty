
// Telling Rust main can fall and return an IO error
fn main() -> Result<(), std::io::Error> {
    
    // let file_result = std::fs::read_to_string("LOGS.md");
    let content = std::fs::read_to_string("LOGS.md")?;

    println!("File Contents: {}", content);


    // match file_result {
    //     Ok(content) => println!("File contents: {}", content),
    //     Err(error) => println!("Oops, failed to read file: {}", error),
    // }

    // Return Ok at the very end to signal the program finished succesfully
    Ok(())
    
    
}
