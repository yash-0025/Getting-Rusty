use std::env;

// fn main() {

//     let args: Vec<String> = env::args().collect();

//     if args.len()< 2 {
//         println!("Usage:  hello-rust <name>");
//         println!("Example: hello-rust Yash");
//         return;
//     }

//     let name = &args[1];
//     println!("Hello, {}! Welcome to Rust.", name);
// }

fn main() {
    // Instead of collection everything into a Vec, we use .nth(1) to grab the second element directly from the iterator. This returns Option<String> - either Some("Yash") or None.
    let name = env::args().nth(1);

    match name {
        Some(n) => println!("Hello, {n}! Welcome to Rust."),
        None => {
            eprintln!("Usage: hello-rust <name>");

            std::process::exit(1);
        }
    }
}
