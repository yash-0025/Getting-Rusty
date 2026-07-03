use std::collections::HashMap;



fn main() {
    // Creating a new , empty , mutable HashMap
    let mut word_counts = HashMap::new();

    // Insert some keys and values
    // Rust infers the type as HashMap<String, i32> automatically
    // word_counts.insert(String::from("hello"),1);
    // word_counts.insert(String::from("world"),5);

    // // Overwriting a value is the exact same syntax
    // word_counts.insert(String::from("hello"),2);

    // let c1 = word_counts.entry(String::from("apple")).or_insert(0);
    // *c1 += 1;

    // let c2 = word_counts.entry(String::from("apple")).or_insert(0);
    // *c2 += 1;

    // let c3 = word_counts.entry(String::from("banana")).or_insert(0);
    // *c3 += 1;

    let text = String::from("rust is fast and rust is safe");

    for word in text.split_whitespace() {
        println!("I found a  word: {}", word);
        let count = word_counts.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", word_counts);
}
