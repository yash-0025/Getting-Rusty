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

    // let text = String::from("rust is fast and rust is safe");
    let text = std::fs::read_to_string("book.txt").expect("Failed to read book.txt").to_lowercase().replace(",", "").replace(".", "");

    for word in text.split_whitespace() {
        // println!("I found a  word: {}", word);
        let count = word_counts.entry(word).or_insert(0);
        *count += 1;
    }

    // .1 is for accessig the second item in the tuple for first item we use .0
    let mut count_vec: Vec<(&str, i32)> = word_counts.into_iter().collect();
    count_vec.sort_by(|a,b| b.1.cmp(&a.1));

    // println!("{:#?}", word_counts);
    // Printing the top 5 words only
    println!("----TOP 5 WORDS---");
    for i in 0..5 {
        println!("{}: {} times", count_vec[i].0, count_vec[i].1);
    }

}
