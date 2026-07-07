`cargo new hello-rust` => Creates Rust project <br>
`cargo check` => compile the project <br>
`cargo run` => runs the project and use the cache from the cargo check command as it is already compiled. <br>
- This is called as **incremental compilation** . So when we did check it did all the parsing , type checking and borrow checking from scratch and then we ran the run command it didn't recompiled because it used the cached version from the cargo check command.


<h1> Day 1 </h1>

- `use std::env` 
- `env::args()` => returns an iterator over CLI args
- `env::args().collect()` => collects the iterator into a vector. 
- `let args: Vec<String> = env::args().collect();` 
- `env::args().nth(1)` => Grabs the 2nd element index1 - skips the program name at index0
- `Option<String>` => This is Rust null-killer. Instead of the value maybe being undefined or null , Rust wraps it in Option: either Some or None . The compiler forces you to handle both cases so you cannot forget to check the missingcase - it won't compile.
- `match` =>  Pattern matching that exhaustively handles every variant . Try deleting the None branch - the compiler will refuse to compile until you handle it. This is fundametally different from JS where we can forget and else and get undefined behaviour at runtime.
- `{n}` => Inside teh string It allows iniline varibale capture in println! format strings, just like JS template literals (`Hello, ${name}!`)
- `eprintln!` => Prints to stderr instead of stdout Error messages go to stderr , normal output goes to stdout. Same convention as every Unix tool.
- `cargo fmt` => To format the code 
- `cargo clippy` => It is an automated senior code reviewer.
- `cargo clippy -- -D warnings` => Production CI command || -D warnings falg turns clippy warnings into hard errors - CI fails if clippy isn't happy.



<h1> Day 2 </h1>

**Project Name** => `Cli-unit-converter`

`Shadowing` => We can use same variable name to store new value but it will be a new variable in memory. 
- `let x = 5;` => x is 5 <br>
- `let x = x + 1;` => x is 6 <br>
- `let x = x * 2;` => x is 12 <br>
- `let x = "Hello";` => x is "Hello" <br>
- `Type safety` => 
- `enums` => Just a fancy way of grouping numbers or strings
- `match` => We pair enums with match to control the flow of the program.
- `Functions and Implicit Returns ` => IN rust the last line of a block or function is automatically returned if you leave off the semicolon(;). This is called implicit return .



<h1> Day 3 </h1>

**Project Name** => `File Duplicate Finder`

- `Ownership` => TO build duplicate finder we need to hold paths to files and the data inside them . But where does that data  live in memory .
- `Concept 1` => `Stack Vs Heap` => **[Sticky Notes vs Library Books]**
- `Stack` => Tiny, superfast but you must know the exact size of the data before program runs.
- `Heap` => Huge , a little slower and used for data that can grow or shrink while the program runs like string where the user types their name. You keep pointer with you which tells you everything where the data is stored. and when you don't need the data anymore (e.g. function ends ) that memory is automatically reclaimed - no Garbage Collection.

- `Concept 2` => `Move Sematics` - If multiple people can change the same data at the same time you get bugs . Rust prevents this entirely through Ownership . IN rust a piece of data can only have ONE owner at a time . 

- `Concept 3` => **Copy vs Clone** - If rust refuses to automatically copy a String how do we make a copy of it when we acually need one .
- `Copy` => Happens automatically for tiny things (numbers, booleans). It's incredibly fast.
- `Clone` => Happens only when we explicitly type .clone(). It forces rust to go to the heap buy a new book, copy all the workds into it andgive you a new library card . It is slower so Rust makes you type it out so you know you're doing something expensive.
- String , Vec and HashMap live on the heap and are huge in size. So rust didn't copy this automatically we have to explicitly type .clone() 

- `Concept 4 ` => **Borrowing (References)** => If we don't want to move a string because we lose it and we don't want to .clone() it because it's slow and waste memory how do we pass a string to to a function so the function can read it . 
- We Borrow it and just let the function look at our library card without taking it ,
- In Rust we borrow something using & symbol.
- If we actually need to string or not . if we awant to just look at it we can use &String.
- This is why Borrowing is so powerful and it lets us pass huge amounts of data to 100 different function without copying the data and without losing it .

- `Concept 5` => **Mutable Borrowing** (&mut T) and the Golden rule.
- We can borrow a string to look at it &String . But what if we want to borrow a string change it and then give it back there we use **Mutable Reference** &mut String. 
- We can have many readers &T or exactly one writer &mut T, but never both at the same time.
- If we have a mutable reference a writer nobody else is allowed to look at the data until we are done . This prevent data races (like two functions trying to edit a file at the exact same millisecond).`
- `&mut` => When we have to change something without destroying it we use it.


<h1>Building Project</h1>

- `std::fs` => To talk with our filesystem we need this 
- `fs::read_dir("/")` => To read the directory we use this 
- `.expect("error")` => If the value is None this will print error and exit the program.
- `HashMap` => We use this to store the hash of the file and the path to the file. `HashMap<String, Vec<String>>` => Key is the hash of the file and value is the vector of paths to the file.
- `{:?}` => We use it instead of {} when we want to print things that are meant for developers{debugging} rather than formatted for end-users
- `.metadata()` => To filter out folders like getting the data like created date and file type.
- `.entry().or_insert()` => It is a hashmap method to insert a value into a hashmap if the key doesn't exist.
- `{:#?}` => Use it instead of `{:?}` when you want to pretty print the output. 


<h1> Day 4 </h1>

- `Custom Data Types` => `Structs` => If we want to use multiple data types together we will need a struct for that 

- `Concept 2` => `Deriving Traits [Printing structs]` => We can't print the whole struct with ```println!("{}", struct_name)``` like this it will throw an error we need to use a macro `#[derive(Debug)]` above the struct to tell Rust to automatically write the debuggin code for our struct.

- `Concept 3` => `impl` - Blocks {methods and constructors} => Eg - Suppose we want is_done a boolean in the struct to always default to false when task is created then we will use implemetation block (impl).
- `Self` => In rust when we are inside an impl block , Self (with a capital S) is just a shortcut alias for the type we are implementing . So anywhere inside impl Task, writing Self is exactly the same as writing Task.

- `Concept 4` => `Methods vs Associcated functions` => (&mut self) -->> The new function we wrote is called Associated function.
- `self` => (the function has access to the instance)
- `&self` => (the function only borrows the instance)
- `&mut self` => (the function borrows the instance mutably)

- `Concept 5` => `Enums inside Structs` 
- `Concept 6` => `Storing Structs in Vec` => In  rust dynamically sized arrays that can grow are called vectors.

- `Concept 7 ` => Using match with enum
- Eg - suppose we need a Cancelled status we add Cancelled to our code but forget to update the if/else chain . Now in js it will be a silent bug or it might return undefined . But in rust match is Exhaustive . It means that compiler forces us to handle every single varianant of ENUM in each method . Supppose if we add cancelled in our TAskStatus our program will refuse to compile unitl didn't add the match statement 

- `Concept 8 ` => Option [The Null Killer] --> Suppose if we try to find an item in array and it doesn't exist it returns null in javascript or undefined but in Rust there is no null instead function return an option.
- An option is just an in built Enum with two variants
- `Some(value)` => I found it here is the data
- `None` => I didn't find it .
- `.find()` => Returns and Option
- `.iter()` => Help us look through the vector

- `Concept 9` => Closures [Rust arrow functions]
- Eg - In javascript when we want to find an item in an array we use arrow function like this let task = tasklist.find(t => t.name = "something");
- Similarly in Rust arrow function is called closure instead of an arrow => Rust puts the variable inside two vertical pipes | | .
- Eg - let task = task_list.iter().find(|t| t.name == "Something")

- `Concept 10 ` => The if let syntax
- So like while using match instead of using if else we will use if let
- For eg if we use match we have to take care of both Some and None otherwise it will not compile but when we don't want all to work like we didnt want None to be described we can use if let 
- Rust give us a special shortand for this called if let . It reads like this - if let task equal Some(found_task) then do this 


<h1>Day 5</h1>

- `Concept 1` => The `Result<T, E>` -> Enum vs Exceptions => In JS if reading a file fails the function throws error we have to wrap it in try/catch block otherwise the whole app crashes at runtime. 
- Rust doesn't have exception intead if a function can fail it returns a Reslt enum. It looks exactly like the Option enum but instead of Some and None it has Ok and Err.
- Eg 
```rust 
    enum Result<T, E> {
        Ok(T), // Contains the success value 
        Err(E), // Contains the Error value
    }
 ```
 - When we try to read file , Rust doesn't give us a string . it gives us a Result<String, std::io::Error>. The compiler forces us to deal with the possibility of failure befor we can access the string inside.
 - So the thing is befoe we are using .expect and if we use that it moslty crashes our app all the times when we didn't have anything but with Result error we can just log it as a danger and avoid crashes everytime.

 - `Concept 2` => `The ? Operator` [Error Propagation] => Like writing out a full match statement every time we want to read a file , write a file or parse JSON gets super repetitive. LIke if a file fails to read we don't want to handle it right on the exact line . we just want to get out of the current function an pass the error back up to whoever called us.
 - `?` It means if it works, unwrap the value inside Ok.If it fails ,  immediately stop this function and return the Err. 
 - Eg - 
```rust
    let content = std::fs::read_to_string("tasks.json")?;
 ```
 - Eg Previously without ? => 
```rust
    let content = match std::fs::read_to_string("tasks.json") {
        Ok(c) => c,
        Err(e) => return Err(e),
    }
  ```
- The catch for `main()` => Because `?` might return an error , you can only use it inside a function that actually returns a `Result` Right now , our `main()` function returns nothing.
- To use `?` in `main()` we have to change its signature so it can return an error to the operating system.
- Note- () is Rust's empty tuple , it basically just means nothing or void

- `Concept 3` => Serialization with serde => If We want our Vec<Task> to survive when the program closes we need to save it to a file. The easiest way is to convert it to JSON .
- IN rust `converting structs to strings (serialization)` and `strings to struct Deserialization` is handled by most famous crate in Rust ecosystem `serde [SERialize/DEserialize]`
- Rust is strictly typed, serde needs to write code for our exact TAsk struct to figure out how to turn it into JSON. It does thsi using MACROS (specifically derive macros). Instead of writing a hundred lines of parsing logic we just add `#[derive(Serialize, Deserialize)]` above our structs and serde writes all the parsing code for us at compile time.
```rust  
//Installation
cargo add serde --features derive
cargo add serde_json
```
- To use  => `use serde::{Serialize, Deserialize};`

- `Concept 4` => Saving to JSON . Once our struct knows how to turn themselves into JSON after adding Serialize and Deserialize to macros .
- For eg - 
```javascript
const jsonString = JSON.stringify(taskList, null, 2);
fs.writeFileSync("tasks.json", jsonString);
```
- In Rust => 
```rust
// 1. Convert Vec<Task> into a formatted JSON  string
// This can fail eg if the struct has a circular reference so we use ?
let json_string = serde_json::to_string_pretty(&task_list)?;
// 2. Write string to that file
// This can also fail so we will use ? here too
std::fs::write("tasks.json", json_string);
```

-  `Concept 5` => Deserialization - Now as we are saving our tasks list at the end of the program , we need to load them at the beginning of the program so they persist across sessions
- For eg -
```javascript
const fileContent = fs.readFileSync("tasks.json", "utf8");
let taskList = JSON.parse(fileContent);
```
- In rust it is similar but with error handling . We read the file and then ask serde_json to parse the string back into Vec<Task>.

- `Concept 6` => The #[must_use] Attribute (No Silent Failures)
- IN javascript if a function return something and we ignore it . javascript don't care but IN rust std::fs::write returns a Result because writing to a disk can fail . Rust tags the Result enum with a special attribute call `#[must_use]`. If we call a function that returns a Result and we don't check it using (match, .unwrap() or ?) the compiler warns us that we are ingnoring failures.


<h1>Day 6</h1>

- `Iterators & Closures` => `.map , .filter , .collect` -> something like .map() and .filter() from javascript
- `Hashmaps` - Store key value pairs natively 

- `Concept 1` => `Hashmaps [Key Value storage]` => It maps a key [the word] to a value[the count] . Just like Vec, it is stored on the heap and can grow  or shrink dynamically
- HashMaps aren't used in every single Rust file unlike Vec or String so they aren't loaded into scope by default. We have to bring them in from the standard library std.

- `use std::collections::HashMap`

- `Concept 2` => `The Entry API`[Counting things idiomaticaly] => If we have thousands of words already inthe hashmap and want to add more we can't keep on checking everytime using match if the key exists , extracts the value and add it and all those things. 
- In Rust  we have Entry API = It is the absolute most idiomatic way to count things in Rust
```rust
let count = words_counts.entry(String::from("apple")).or_insert(0);
```

- `References[&] and Dereferencing(*)` => Suppose you buy a physical house. The house is the actual data in our computers memory like number 1 sitting in our HashMap bucket
- If we want to hire a painter to pain the house , we don't physically pick up our house and give it to painter instead we write down house address on a piece of paper and give it to them .
- The House => The actual data in memory
- The Address => A reference (also known as pointer) . It is just a  piece of paper that says . The house is located at memory bucket #2483788990
- In Rust when we use Entry API , Rust does not give us the actual number 0 back . If it did, we would jsut have a copy of the number 0 in hand and changing it wouldn't affect the hashmap at all.
```rust
let count = word_counts.entry(String::from("apple")).or_insert(0);
```
- Instead Rust gives us the Address &mut i32. It gives us a piece of paper that says `The count for apple is located at bucket #980909090 and you have the permission to change it mutable`.

- `Dereferrecning(*)` => If we have a piece of paper with an address on it we can't just slap paint onto the piece of paper . We have to physically drive to the address to paint the actual house. That is what the dereference * operator does 
```rust
*count += 1;
```
- So we are telling rust that don't try to add 1 to the piece of paper . Follow the address (*) to the actual bucket in memory and add 1 to the actual number inside it.
- `& ` => Gives me the address to the data
- `*`  => Follow this address so I can touch the actual data

- `Concept 3` => `Iterators` [.split_whitespace()] => In javascript when we have a string and we want to turn it into an array of words we do this 
```javascript
let words = "apple apple banana".splite(" ");
```
- In Rust strings are much more complex because they handle UTF-8 encoding[emojis, chinese characters etc]. But rust gives us a super powerful tool for this called an iterator
- So instead of creating a giant new array[Vector] in memory to hold all the words an iterator just points to the string and hands us one word at a time. It is blazing fast and uses almost zero memory. To get an iterator of words we use .split_whitespace()

- `Concept 4` => `String vs &str(String slices)` => In javascript string is a string but in Rust there are two main types of string . 
- `String` => The Owner / The physical house from the example
- `&str` => A String slice / A reference
- let;s understand this with example - So when we do this 
```rust
let text = String::from("rust is fast and rust is safe")
```
- This creates a whole string . It asks computer for RAM, allocated physical space on the heap and stores the letters . it is heavy and expensive . 
- Now what does .split_whitespace() do ? Does it create 7 brand new String object for all 7 words and take up 7x more RAM .
- Nooo The iterator gives us a `&str`.  REmember `&` means reference / pointer
- A `&str` is just a piece of paper that says - Look at the original text String in memory , specifically starting at character 0 and ending at character 4.
- So suppose if we are finding a word rust , it just points to the word rust that already exist inside our original sentence , it doesn't copy memory , it is just fast and uses zero extra RAM. 
Suppose we passed a word (which is &str) into the HashMap , Rust automatically inferred our HashMap type to be HashMap<&str, i32>. Out entire HashMap is storing pointers to the original sentence

- `Concept 5` => Cleaning Data [String Manipulation] => Before we start counting words we need to sanitize the text . Like we do everything .toLowerCase() in javascript , In rust we can do this by chaining methods. 
- `.to_lowercase()` => converts string to lowercase
- `.trim()` => removes whitespace from both ends
- `.chars()` => returns an iterator of characters
- `.contains()` => checks if the string contains a substring
- `.replace()` => replaces a substring with another substring
- `.split()` => splits the string into an iterator of substrings


- `Closures and Sorting` => IN javascript if we want to sort an array of object by specific properly we use arrow functions. In rust arrow function is called closures. Instead of (a,b) => In Rust we do like this - |a,b|.
- `.into_iter()` => Drains the hashmap
- `.collect()` => Gathers it into Vec


- `Concept 6` => Iterators Adapters `.map(), .filter(), .sum(), .count()`
- An iterator adapter is a method that takes an iterator and transforms it into a new iterator or consumes it to calculate a single value. 
- Ex 1 - Counting total words instead of  a for loop we can just ask the iterator how many items it has . 
```rust
let total_words = text.split_whitespace().count();
```
- Ex 2 - Mapping and Summing - If we want the total numbers of characters across all words, we can use .map() {just like in Javascript} to turn the iterator of words into an iterator of lengths, and then sum them up.
```rust
// split into words
// .map() turns every word into a number (its lenght) using a closure |word|
// .sum::<usize()> adds all the numbers together! We have to tell it the type is usize
let total_chars = text.split_whitespace().map(|word| word.len()).sum::<usize>();
```

-`Concept 7` => Sentence Counting `.filter()` => So we have to count how many sentences are in the text. How can we do this ? 
- We know a sentence ends with a period .
- So we can split the entire text by  
- Just like javascript .filter(), it iterates over a list and only keeps item that match a certain condition returning true or false from a closure
```rust
// Loading the raw, uncleaned text
let raw_text = std::fs::read_to_string("book.txt").expect("Failed to read book raw");

// Turn the string into iterator of individual characters (.chars())
// Filter it Keep the character only if it is a period, exclamation or question mark
// count how many passed the filter
let sentence_count = raw_text.chars().filter(|c| *c == '.' || *c == '!' || *c == '?').count();
```
- Because .chars() gives us a reference to the character we use *c to dereference it so we can compare it to actual characters like '.' '!'


<h1>Day 7</h1>

- Installing Clap => 
```bash
cargo add clap --features derive
```
- `Concept 1 = CLI Parsing with clap`
- When we use GIT , we type comands like git status or git commit -m "msg". right now our task tracker runs from top to bottom we want to do this - 
- cargo run --add "Buy Milk" "Go to the store"
- cargo run --list :: Note- The -- tells cargo to pass the rest of the arguments to our program instead of  cargo itslef
- To do all this we use `clap crate`. Clap provides feature called derive , which lets us define our terminal commands purely by writing rust structs and enums. It automatically handles all the parsing, error messages and even generates a --help menu! . 
```rust
use clap::{Parser, Subcommand}

#[derive(Parser, Debug)]
#[command(name = "Task Tracker")]
#[command(about = "A simple CLItask manager", long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
```
- `///` Clap reads those three slash and turns them into the descriptions in the --help menu


- `Concept 2 - Writing CLI commands to logic`

- `Concept 3` - Modules => In javascript we create new file and write module.exports = Task Then in index.js we write const Task  = require("./models").
- In Rust it works slightly different . A new file is automatically considered a module but we have to :
- 1. tell Rust the file exists using the mod keyword in main.rs
- 2. Make the specific things inside the file public using the pub keyword everthing in Rust is private by default
- Create a separate file name models.rs inside src and add enums and structs in there 
- Import that file calling `mod models`  so that rust will look for models.rs file .

- `Concept 4` => `Extractin Logic (Storage)` => Creating a storage module to handle all database / file operations 
- `crate` => We use crate to refer to the root of our project , then models module 
```rust
// Here crate means root of the poject then look for models file and then module which we want to use from the models file
use crate::models::Task;
```


- `Concept 5` => `Unit testing #[test]` -> In Javascript we usually create a separate folder like __tests__ and write test('adds 1 + 2 ', () =. ...) . 
- In rust the culture is completely differnt Unit test live in the exact same file as the code we are testing
- So we don't have to jump between files to see  what function does . Rust achieves this using attributes the `#[...]` syntax
- `#[cfg(test)]` => This tells the compiler "Do not include this code in the final production binary. Only compile this when i run cargo test" - This means our tests takes up zero space in prodcution
- `#[test]` => This tells rust that the function immediately below it is a test case 
- `assert_eq!` => We use it to check if the two values are exactly the same or not 
- `matches!` => We have to use matches! macro because we can't easily compare Enums unless we add `#[derive(PartialEq)]` to them.
```rust
assert!(matches!())
```

- `Concept 6` - `Extraction & pub(crate)`
- There are two types of public in Rust 
- 1. pub - This makes the struct public to our entire project and if we ever published this code to crates.io as library anyone in the world could import and use it .
- 2. pub(crate) - This makes the struct public to every file in our current project only and strictly hides it from outside world


- `Concept 7 - The Newtype Pattern` => Suppose we could just add `pub id:u64` to our `Task` struct. BUt what if we accidentally pass an age or a price to a function that expects a task ID? The compiler wouldn't catch it because they are both just `u64` numbers
- In rust we solve this using Newtype Pattern. We create a tuple struct that wraps the base type turning it into it's own distinct 
```rust
// here is how we define NewType 
// We did pub u64 inside the paranthesis . This allows us to access the inner number by using `.0` [like id.0]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TaskId(pub u64);
```
- We added PartialEq so we can easily compare if two ID's are equal using `==`
- `.iter_mut()` => This lets us modify the items we are iterating over
- `.find()` => It helps us search for the firt item that matches our condition
- `.find()` => It returns an Option [Some if it found it , None if it didn't]

- `Concept 8` => The Builder Pattern - In rust when constructing complex objects we use the Builder Pattern .
- Instead of passing 10 arguments into a single function we create temporary Builder struct that lets us chain methods together one by one .
```rust
let new_task = Task::builder(next_id)
    .name(name)
    .description(description)
    .build();
```

- `Concept 9 - The Delete Command and .retain()` => To Delete a task we could write a for loop find the index of the task and call task_list.remove(index). But in rust we have a powerful iterator methods
- The most idiomatic way to remove items from a Vector based on condition is the .retain() method . 
- `.retain()` loops through the entire vector. if our closure returns true, the item is kept if it returns false the item is deleted


- `Concept 10 ` = `The Stats Command and Iterator Aggregation` => If we want to know how many tasks are Todo, InProgress and Done we could write for loop and increment 3 different counter variales . But again iterators save the day .
- We can use `.iter() , .filter(), .coun()`


<h1>Day 8</h1>

- `Generic Stack and Queue Collection library`
- To create a library instead of an application we use a slightly different cargo command .
```bash
cargo new collections --lib
```
- When we create using `--lib` we didn't get main.rs instead we get lib.rs

- `Concept 1 - Generic Structs <T>`
- When we built our Task Tracker our Vec held a very specific type `Vec<Task>` But Vec can hold anything. That's because under the hood developers of standard library defined it using Generic `Vec<T>`
- The `T` stands for Type . It tells the rust compiler that i don't know exactly what type of data this will hold yet but whatever type the user passes in lock it in and use it everywhere.
- We are going to build a Stack [Last in First out data structure]

