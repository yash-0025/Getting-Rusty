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


- `Concept 2 - Queue<T> and VecDeque` => To build a queue we need to push items to the back and pop them from the front
- we could just use a Vec<T> again call self.items.remove(0) . But in vector if we remove the 0th item Rust has to shift every single other item one spot to the left . 
- If we have 1 million items that is going to incredible slow O(N) time complexity
- Rust standard library provides a solution VecDeque ( Vector Double-Ended Queue)
- A VecDeque is essentially a ring buffer under the hood , meaning pushing and popping from the front OR the back is blazingly fast O(1) time complexity
- To use we have to use 
```rust
use std::collections::VecDeque;
```

- `Concept 3 - Deriving Traits and Generic Types` => We can automatically implement common traits like Debug[For printing] and Clone[for duplicating memory] using the `#[derive(...)]` macro.
```rust
#[derive(Debug,Clone)]
```
- The golden rule of Generic - Because Queue<T> is generic, Rust only allow us to print Queue<T> if the inner type T also implements Debug
- For eg -  Queue<i32> can be printed because integers implement Debug
- Queue<String> can be printed because strings implement Debug
- But if we made a custom struct `struct MyData {}` and did not add `#[derive(Debug)]` to it, and still tries to print Queue<MyData> the compiler will block us
- **Note** - When running tests , cargo hides printing statements by default. To force it to show them we have to run this 
```bash
cargo test -- --nocapture
```

- `Concept 4 - Defining Custom Traits and Default Methods`
- In Typescript or Java we use interface to define a contract of shared behaviour , In Rust we use Traits.
- Let's imagine we want to guarantee that any collection in our library can report its length and whether it is empty. We can create a Collection trait
- We can provide a default method implementation if a type implements len() we can figure out it is empty without them writing any extra code


- `Concept 5 - Associated Types vs Generic Type Parameters`
- if we look at standard library, the `iterator` trait is defined like this 
```rust
pub trait Iterator {
    type: Item; // What is this ?
    fn next(&mut self) -> Option<Self::Item>;
}
```
- Why did Rust use type Item instead of generic type like `pub trait Iterator<T>` => This is called an Associated Type
- We use Generic Parameters(Trait<T>) when a struct could implement the trait multiple times for different types
- We use Associated types (type Item) when a stuct should only ever implement the trait exactly ONE way.
- A Stack<i32> can only iterate over i32 . It makes no sense to let someone implement Iterator<String> on a Stack<i32>. By using an associated type Rust enforces that a collection only has ONE way to iterate

- `Concept 6 - Operator Overloading (std::ops)`
- Waht if we wanted to take stack1 and stack2 and just add them together using + operator?
- In many languages we cant redefine what + does , In rust we can . Rust maps operators to standard triats in the std::ops module
- `+` operator is mapped to the `std::ops::Add` trait
- `*` operator is mapped to the `std::ops::Mul` trait
- `==` operator is mapped to the `std::cmp::PartialEq` trait which is why #[derive(PartialEq)] works


<h1>Day 9</h1>

- `Concept 1 - Static Dispatch (Monomorphization)` -> If we create Shape trait and implement it for a circle and a rectangle we can write a function that takes any shape using Generics<T: Shape>
- When we compile this code rust performs monomorphization mono=one morp=shape. The compiler looks at everywhere we called that function and literally copy paste a specialized hardcoded version of the function for every unique type we used
- Pros - Zero runtime cost. It is as fast as if we hand wrote separate functions for Circle and Rectangle
- Cons - Slightly larger binary size since the compiler generates multiple copies of the function

- `Concept 2 - Dynamic Dispatch (dyn Trait) and Trait Objects` => Instead of putting the raw shpaes into the Vector we wrap them in a Box . A Box puts the data on the Heap and leaves a Pointer on the Stack
- Since all the pointer are the exact same size in memory (8 bytes on a 64 bit system) the Vec is happy
- We call this a Trait Object Box<dyn Shape>. The dyn keyword stands for dynamic . At runtime Rust doesn't know exactly what shape is behind the pointer so it looks at a hidden table the vtable to figure out where to call the circle's area() or the Rectangle's area()

- What is Box and why do we need it ?
- When we create a struct like Circle { radius: 10.0}. Rust by default stores it on the Stack. The Stack is very fast but everything on it must have a known fixed size at compile time
- A circle is 8 butes one f64
- A rectangle is 16 bytes two f64
- A vec also required that every single itme inside it be the exact same size. So Vec<Shape> is illegal because Rust says I don't know how big a shpae is Some are 8 bytes some are 16 bytes 
- This is where Box comes in Box::new(...) is a way to tell Rust
- "Take this data move it off the stack and put it on the heap the dynamic memory " Then leave a pointer behind on the stack that tells me where to find it .
- A pointer is essentially just a memory address. No matter how big the data on the Heap is , a pointer is alwyas exactly 8 bytes on a 64bit computer
- So when we write Vec<Box<dyn Shape>> we are telling the Vector - You are going to hold a bunch of pointers. Every pointer is exactly 8 bytes so you are perfectly safe. Those pointers will point to data on the heap that implements the Shape trait

- What is `vec![]`?
- In past to create Vector and put things inside it we use 
```rust
let mut my_list = Vec::new()
my_list.push(1);
my_list.push(2);
```
- this takes 3 line of code and required the variable to be mut . 
- `vec![]` is a macro like `println!()` . It is just a shortcut provided by Rust that lets us create a Vector and fill it with data in one single step.
```rust
let my_list = vec![1,2];
```
- `{:.2}` in the print statement => When we use {} in println! . Rust will print out the full number . If teh area of the circle was 314.15789797.... it would print the whole thing which looks messy
- `:` - This says I want to apply some special foramtting
- `.2` - This says Round this floating point numer to exactly 2 decimal places 

- `Concept 3 - impl Trait Syntax [Syntactic Sugar for Generics]`
- When we wrote our statically dispatched function, it looked like 
```rust
pub fn print_area_static<T: Shape>(shape: &T) {...}
```
- Using <T: Trait> and then using &T can get verbose and hard to read, especially if a function takes multiple arguments of different traits. 
- Rust provides a shortcut or syntactic sugar for this exact pattern called impl Trait
```rust
pub fn print_area_static(shape: &impl Shape) {
    println!("The area of the {} is {:.2}", shape.name(), shape.area());
}
```
- We can read this like - This function takes a reference to some type that implements the Shape trait


- `Concept 4 - Object Safety rules`
- We cannot magically turns every trait into a dyn Trait. A trait must be Object Safe to be used dynamically
- When is a trait NOT Object Safe?
- 1. When it returns Self - Think back to the builder pattern. The methods return self [The struct itself]. If we try to use dyn Builder the compiler panics because the vtable doesn't know how many bytes Self is at runtime since it could be any struct
- 2. When it has generic methods - If our trait has a method like `fn do_things<U>(&self, arg:U)` the vtable would need to hold an infinite number of pointers for every possible type`U` we might pass in . the compiler won't allow this
- If trait breaks this rules we must use Static Dispatch Generics to use it 

- `Concept 5 The Enum vs Trait Object TradeOff [Senior Design Decision]`
- If we are building a system that needs to handle multiple types of things like a list of Circle and REctangle we have two choices in Rust
- 1. Wrap them in an Enum(enum Shape {Circle(Circle), Rectangle(Rectangle)})
- 2. Use Trait Objects (Box<dyn Shape>)
- How Do we choose ?
- > Use an ENUM when our system is closed . If we know every single shape that will ever exist in our app its a closed set we can use enum
- > Use Trait Objects (Box<dyn Trait>) when our system is open . If we are writing a library and want other develolpers to be able to create their own custom shapes like a Hexagon and pass them into our library without modifying our source code we must use dyn Trait

<h1>Day 10</h1>

- `Concept 1 - What is Lifetime` => When we pass a reference like &str into a function and reutrn a reference , the Rust compiler is terrified of a Dangling Pointer (returning a reference to memory that has been deleted)
- A lifetime (`'a`, read as tick a) does not control how long memory lives. It is a descriptive label we give to compiler to prove that the returned reference is tied to the input reference.
- If we write a function that takes two strings and returns one of them the compiler will panic, because it doesn't know which string you are returning and therefore doesn't know if its safe.

- We have written functions that take references before like fn name(&self) -> &str. Why didn't we have to use lifetimes there? because of lifetime ellision rules. The compiler is smart enough to guress the lifetimes in 90% of cases. The main rule is :If there is exactly one input reference , Rust assumes the output reference is tied to it 

```rust
// What we write
fn get_first_word(s: &str) -> &str {...}
// What Rust silently turns it into [Lifetime Elision]
fn get_first_word(s: &'a str) -> &'a str {...}
```
- But when there are two input reference the compiler cannot guess , We have to setp in and apply labels manually

- `Explicit Lifetimes in Functions` => To use a lifetime label , we first have to declare it on the function just like we have to declare generic types <T> and then we tag the referecnes with it.


- `Concept 2 - Lifetimes in Struct & Zero-Copy Architecture`
- In python or Nodejs parsing a configuration file means reading the file and then creating a brand new String in memory for every single key and value we parse. There is a lot of slow heap allocations
- In rust we can build a Zero Copy Parser intstead of making new Strings our config struct will just hold references to the original string that we read from the file.
- If a Struct holds a reference &str , it must declare a lifetime tag. This allows the compiler to guarantee that the Config struct will be destroyed befor the original string gets deleted
- `.lines()` gives us an iterator that goes line by line
- `.split_once()` cuts the line into two pieces at the '='

- `The static Lifetime` 
- In rust we can name our lifetime anything we want. We could use <'a> , <'b> or even <'cool_lifetime> It is just a community standard to use 'a 'b to keep things short . However there is one special lifetime name that is reserved by teh compiler `'static`
- If we see a reference tagged with `'static` like `&'static str` it means the data will never ever be destroyed . It will live for the entire duration of the program
- Every single time we write a hardcoded string surrounded by quotes in our code we are creating a 'static reference
- Becayse when we compile our Rust code into an executable .exe or binary the text "Yash" is literally baked directly into the hard drive file of our binary . When we run the program it loads that binary into a permanent, read only section of RAM . It physically cannot be deleted until the program shuts down. Therefore the compiler assings it `&'static str`lifetime meaning it's guranteed to live forever

- `Concept - 3 Lifetime Bounds on Generics (T: 'a)`
- We know that we can use <T> to make a struct that can hold anything but what if we put a reference inside that generic struct. How the compiler know how long it will live
- Suppose we have a physical Backpack a generic struct Because it is a generic backpack <T> it can hold absolutely anything we can put a heavy iron dumbbell String in it or we can put a sandwich &str in it
- Dumbell will lasts forever but sandwich has an expiry if our backpack exists for 5 days but we put a 2 day old sandwich in it and by day 3 we will reach into our backpack and grab rotten garbage a Dangling pointer
- to prevernt this `T:'a` we put a strict rule on the Backpack like - We don't care what we put in this backpack `T` but whatever it is , its expiration date must be longer than the 5 days this backpack exists `'a`

- `Concept 4 - Ergonomic Conversions (From and Into traits)`
- In rust standard library From<T> and Into<T> are twin traits used for value to value conversions . The magic of rust is taht we can never implement into trait ourself'
- If we implement From<A> for B meaning define how to create to create type B out of type A . Rust compiler uses a feature called a Blanket Implementation to automatically write Into<B> for A on our behalf.
- It is heavily used in function arguments to make API's ergonomic .Instead of forcing ther user to pass a very specific type we accept impl Into<T>


<h1>Day 11</h1>

- `Box<T>` - Storing data on the heap to create recursive structures
- `Rc<t>` - Reference Counting (letting multiple things own the exact same data without cloning it)
- `RefCell<T>` - Interior mutability changing data even when it is supposedly immutable

- `Concept 1- Recursive Enums and Box<t>` -> To evaluate `5 + (3 * 2)` we need to break it down into and AST Abstract Syntax Tree. An AST is an recursive tree structure where an expression can contain other expressions
- In rust all Structs and enums must have a known size at a compile time so they can be places on the Stack. If we define an enum w here a variant contains the enum itself it is a Recursive type. 
- its size is theoretically infinite. To break this cycle we need to wrap the inner types in Smart Pointer called a Box. 
- A Box allocates the actual data on the heap and leaves behind a simple pointer on the Stack. Because a pointer is always exactly 8 bytes on a 64-bit system. The compiler now knows the exact size of the Enum  and the code successfully compiles

- `Concept 2 - Reference Counting with Rc<t> `
- Remember the single library book ownership if we own the book we can take it home when we leave but what if 3 people in a house need to share a TV remote we can't give strict ownership to person A, because if person A leaves the house they will take the remote with them and the others can't watch TV
- We attach a digital sign out sheet a reference counter to the remote . Every time someone grabs the reomte they add a tally +1 . When they leave the room they erase their tally -1
- When the tally hits 0 means the very last person has left the room that person is repsonsible for thowing the remote in the trash [freeing the memory]
- `Rc<T>` stands for reference Counted like a box it allocates data on the Heap but instead of strict single ownership it places a tiny integer counter next to the data 
- When we call .clone() on an RC, it does not copy the heavy data. it simple increments the integer counter . Because cloning and Rc is just adding 1 to an integer it is incredibly fast (O(1)). When an Rc goes out of scope the Drop trait automatically decrements the counter . When the counter reaches 0 the Heap memory is finally freed.
```rust 
use std::rc:Rc
```

- The problem with Rc is that it is strictly immutable what if we need multiple owners but we also need to change the data then we use this .

- `Concept 3 - Interior Mutability with RefCell<T>`
- Rust strict rules sya if we are sharing a notebook with multiple peope an immutable reference No one is allowed to write in it . It is like placing the notebook in a locked glass case
- Suppose we hire a Security Guard (Refcell) and place them next to the glass case . The rust compiler says okay i trust the security guard. It will compiler our code
- When the program is actually running we walk up to the guard and ask to borrow the notebook to write in it (.borrow_mut) . The guard physically looks around if no one else is currently reading or writing in it , they unlock the case and hand it to us
- If someone else is already reading or writing in it and security guard panics sounds the alarm and immediately crashes the entire program

- `RefCell<T>` provides what is called Interior Mutability . Normally Rust enforces its borrowing rules at compile time.
- `RefCell<T>` enforces those exact same rules at runtime. This allows us to mutate data even when we only have an immutable reference &self to the RefCell.
- Because the checks happen at runtime, it costs a tiny bit of performance. If we accidentally break the rules at runtime eg. calling .borrow_mut() twive in a row before the first one finishes, our program will literally panic! and crash

```rust
use std::cell::RefCell
```
- Rc<RefCell<T>> which is the holy grail patter for Shared Mutable state in single threaded rust. It is used constantly when building GUI applications or complex data structures like trees and graphs 

- `Concept 4 - Deref Coercion`
```rust
Expr::Add(left, right) => left.eval() + right.eval()
```
- left and right are both Box<Expr> . They are Pointers they are not actual Expr so how in the world were we able to call .eval() on them
- Imagine we have a locked safe a Box containing a calculator the data
- To use the calculator we would normally have to unlock the safe pull out the calculator *box press the buttons .eval() and then put it back
- Deref Coercion is like having an invisible buttler. Instead of doing the work ourself we just shout add 5 and 3 . The invisible buttler will automatically opens the safe hits the buttons on the inner calculator and hands us the result
- Deref Coercion happens when a Smart Pointer implements the std::ops::Deref trait. Both Box<T> and Rc<T> implement this trait
- When we have a Box<Expr> and we write left.eval() the compiler notices the eval() does not exist on Box itself. Instead of throwing and error the compiler uses the Dered trait to automatically insert the dereference opereator * for us.
- It turns left.eval() into (*left).eval() behdind the scenes at compile time.
- This is why smart pointers in rust feel so incredible ergonomic to use the compiler automatically looks through the pointer so we can treat the pointer exactly like the data it contains


<h1>Day 12</h1>

- `Concept 1 - Reference cycles and Weak<T>` 
- This is a tree Data structure, In a tree, a parent folder needs to point on its own child files, but a  child file also needs to point back to its parent folder eg - when we type cd.. in terminal
- Imagaine Alice [Parent] and Bob [child] are holding hands in a room. the rule of the room is as long as someone is holding your hand you cann't leave
- alice is holding bobs hand Rc and Bob is holding Alice hand Rc . Since neither will let go first the room things they both are permanently busy . they are trapped in the room forever. This is called as Reference cycel
- Solution - Weak<T> Allice holdsd Bob's hand firmly Rc but Bob only looks at Alice without physically holding her hand Weak . When Alice decides she is done and leaves she drops bobs hand because Bob wasn't physically holding onto Alice she is free to go. The cycle is broken

- When we learned taht Rc<T> has a strong_count counter. Memory is only freed when the counter hits 0 . if Node A points to Node B and Node B points to Node A both of their counters will permanently be stuck at 1 . The Drop trait will never trigger and the Heap memory will leak forever . 
- rust gives us Weak<T> to fixt his companion to Rc. It allows us to hold a reference to data without incrementing the strong_count.
```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;
```

- `Concept 2 - Rc::downgrade` => How do we actually create a Weak pointer? We don't create it directly. Instead we take an existing strong Rc pointer and we downgrade it
- Rc::downgrade(&strong_pointer) creates a Weak pointer. It's like Alice handing Bob a picture of herself instead of letting him hold her hand. 

- `Concept 3 - Weak::upgrade()` => Because a Weak pointer doesn't keep data alive , the data it points to might have been deleted. Therefore we can't just read data directly from a Weak pointer. Instead we must upgrade it to strong Rc pointer first by calling .upgrade() . Because the parent might be gone, .upgrade() returns an Option<Rc<T>>. if it returns Some , the parent is still alive. if it returns None, the parent was deleted

- Proving memory safety - We claimed that usign Weak prevenets a memory leak. How can we prove that ? we need to prove that when a folder is deleted everything inside it gets deleted too. and that nothing is left behind permanently holding memory.

- `Concept 4 - The Drop trait` - IN rust we don't manually delete memory like C/C++ and there is no garbage collector randomly pausing our program like in Java/C# . 
- Memory is managed deterministically via the Drop trait . Whever a variable goes out of sope like hitting the } at the end of the function. 
- Rust automatically calls it drop method to clean up the memory. we can look into this to see exactly  when our memory is freed

- `Concept 5 - Rc<RefCell> vs Arc<Mutex>` 
- The danger of RefCell - When we call .borrow_mut() Rust checks at runtime if someone else is currently borrowing it. If they are our program will crash instantly. It bypasses compile time safety . We must be extremely careful to never call
- Single Threaded only - Rc and RefCell are not Thread Safe . IF we try to share an Rc pointer between two CPU threads Rust will throw a massive compiler  error
- If we ever build a multi threaded app like a web server , we must replace Rc with Arc [Atomic Reference count] and we must replace RefCell with Mutex
- Arc<Mutex<T>> does the exact same thing as Rc<RefCell<T>> but it safely locks data so multiple CPU threads can read / write without crashing

<h1>Day 13</h1>


- `Concept 1 - Why do we test in Rust?`
- If the Rust compiler is so strict that it catches every memory leak and type error why do we even need to write tests . The compiler proves that our code is safe but it cannot prove that our code is correct. If we build a calculator and write 2+2=5 the rust compiler will compile it perfectly because 5 is a valid integer . Tests exists to prove business logic words
- IN rust testing isn't third party library we have to install like Jest in JS or PyTest in python. It is built directly into language and the cargo tool

- There are three types of tests in Rust
- 1. Unit Test - Tiny tests written in the exact same file as our code. They test individual private functions
- 2. Integration tests - Tests written in separate tests/ folder. They test our project exactly like an outside user would accessing public methods
- 3. Doc Tests - Code example we write in our /// comments. Rust actually compiles and run our comments to ensure our documentation never lies
```rust
use super::*
#[test]
```
- super::* imports everything from the parent module into our test module
- #[test] tells the test runner that this specific function is a test
- `assert_eq!()`- checks if the two values are exactly equal. if they aren't the test instantly panics and fails
- `Testing Edge cases #[should_panic]` -> Testing happy paths like 10 + 20 is easy but what happens when a user types something completely invalid. Does the program crash gracefully or does it do something unpredictable
- What happends if we type bad syntax like 5.0 + * 3.0. Our parse_factor method actually panics with expected number or '('
- WE can write test to guarantee that actually crashes when given bad input . We do this using the `should_panic` attribute

- `Concept 2 - Integration Tests` 
- A unit test lives inside the exact same file as the souce code . Because it lives inside the file it has God Mode access. It can test private functions and private variables. But when we publish a library like collections we want to make sure that other developers can actually use it from outside .
- An integration test is placed in a completely separate folder . Rust compiles it as if it were a totally different 3rd party project downloading our code from teh internet. It can only access our pub structs and pub methods . This proves our public API actually works


- `Concept 3 - Doc Tests & cargo doc`
- We have used a third party library in Javascript or Python copied the code example from their documentation and it completely crashed because the docs were out of date .
- Rust solves this brilliantly. Code examples in your documentation are automatically compiled and run as tests.
- This means it is literally impossible to publish a rust library with broken code example.

- `Concept 4 - Module system deep dive and closures vs function pointers`
- We have actually been using the module system (pub, mod, super::*) throughout the week But there is one final rule to learn about modules in rust.
- When we create a file like storage.rs , it automatically becomes a module names storage. But what if storage gets huge and we want to split it into a folder
- The Modern style - If we want a module named database, we can create a folder called database and inside it put a file called mod.rs
- database/mod.rs acts as the entry point of the database module.
- Inside mod.rs we can declare other sub modules like pub mod mysql whihch points to database/mysql.rs

- Function Pointers vs Closures(fn vs Fn) - We have been using closures like (|X| x + 1) . Closure are anonymous functions that can captures variables from the environment
- Fn, FnMut and FnOnce are traits that representss closures.
- fn (lowercase) is a Function Pointer. It points to a regular function defined with the fn keyword. Function pointer cannot capture environment variables meaning they use slightly less memory and have no overhead but are less flexible than closures

- `Modern Module system (mod.rs)` - Imagine we are building a backend and our database.rs file becomes 2000 lines long . We want to split it into a folder
- ```rust
    src/
        database.rs
```
- ```rust
    src/
        database/
            mod.rs
    Node!
        mysql.rs
        postgres.rs
```
- In rust a folder named database needs a file named mod.rs inside it . That mod.rs acts as the entry point for the entire folder inside database/mod.rs we would write
```rust
pub mod mysql;
pub mod postgres;
```
- Now the rest of our app can just `use database::mysql;` exactly as if it was all still in one giant file

- `Closures (Fn) vs Function Pointers(fn)`
- A Function Pointer is just a variable that points to a normal function. A Closure is an anonymous function like and arrow function in js . The biggest difference is Capturing the Environment
```rust
// This is a normal function
fn add_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    // 1. Function Pointer (lowercase fn)
    // It points to add_one . It takes very little memory
    let my_pointer: fn(i32) -> i32 = add_one;
    println!("{}", my_pointer(5)); // Print 6

    // 2. Closure (uppercase Fn trait)
    let external_var = 10;

    // A closure can capture variables from outside its scope
    // A function pointer cannot do this
    let my_closure = |x| { x + external_var };
    println!("{}", my_closure(5)); // Print 15
}
```
- Why it matters - If we build a system that accepts callback(like .map(|x| x + 1)), we almost always want to use the Fn trait , not the fn pointer, because users will almost always want their callback to read local variable  from the evnrionment.

<h1>Day 14 </h1>

- `Concept 1 - std::time Concepts ` - we need to understand how Rust handles time. Since this is a cache with a TTL (Time to live) we need to know when an item expires
- Rust provide two very important structs in the standard library std::time
- `Duration` - Represents a length of time eg 5 seconds . It is just a number
- `Instant` - Represents a specific point in time on the machines clock eg- Right now. It is monotonically increasing , meaning it never goes backwards even if the user changes their system clock or Daylight Saving Time hits
- If we want an item in our cache to expire in 5 seconds we don't just store 5 seconds we don't just store 5 seconds.
- We calculate expiration_time = Instant::now() + Duration::from_secs(5);
- Whenever someone asks for the item, we check if Instant::now() >= expiration_time { // it's expand }

- `Concept 2 - Core Structure`
- 1. `CacheItem<V>` - This is a tiny wrapper that holds the data `value: V` and tells us exactly when it expires (epires_at: Option<Instant>). We use Option because because maybe the user wants an item to stay in the cache forever (no expiration)
- 2. `Cache<K, V>` - This is the main structure . It contains a `HashMap` that maps the keys K to our new CachceItem<V>

- `Concept 3 - Trait Bounds and Generics`
- When we use a generic like K for a HashMap key, we can't just use any type . For a HashMap to work, it has to be able to hash the key (turn it into a number ) and compare two keys for equality.
- So when we define our Cache we can't just say Cache<K, V>. We have to enforce rules on K using traits Cache<K, V>. We have to enforce rules on K using traits: Cache<K: std::hash::Hash + std::cmp::Eq, V>

- `Concept 4 - Implementing Methods on Trait Bounded Generics`
- Since our Cache struct has trait bounds on K (K: std::hash::Hash + std::cmp::Eq) whenever we write the impl block for Cache , we must repeat those exact same trait bounds, if we don't Rust won't let us use the HashMap inside it
- new() - A simple function that just returns a Cache with an empty HashMap
- set() - A function that takes a key, a value and an optional ttl (Time to live as a Duration)

- How set() workss logically
- If the user passes Some(duration) we calculate the exact expiration time on the clock Instant::now() + duration. 
- If the user passes None, the item just never expires
- WE then take the Calculated time and the value wrap them in our CacheItem, and insert them into the HashMap


<h1>Day 14 again</h1>

- `Concept 1` => Project Setup and Time Concepts
- How Rust handles time? Since our cache has a TTL, we need to know exactly when an item expires
- => Rust provides two very important structs `std::time`:`Duration` and `Instant`. 
- `Duration` => The Movie length - This is just a measurement of time like saying - This movie is exactly 2 hours long. It has no start or end point
- `Instant` => The Stopwatch :Imagine we have a physical stopwatch that started running the moment our computer was turned on and it can never be paused, stopped or reversed. Calling `Instant::now()` is like looking down at the stopwatch and recording the exact millisecond we see on the screen
- Combining them - If we want to know exactly when a 2 hour movie will finish we look down on our stopwatch right now Instant and add the length of the movie Duration and write down that exact future stopwatch time
- If we want a cache item to expire in 5 seconds we don't just save 5 seconds . We calculate expiration_time = Instant::now() + Duration::from_secs(5);
- Whenever someone asks for the item, we check if our current stopwatch time has passed the expiration time . IF Instant::now() >= expiration_time { //It's expired }


- `Concept 2 ` => Core Structures 
- 1. `CacheItem<V>` 
- What it does ? => This is a small wrapper struct that holds the actual data(value) and tracks exactly when that specific piece of data expires(expires_at)
- How it works? => It is generic over type <V> (the Value). We use an Option<Instant> for the expiration time. If the user passes Some(time). It expires at that time. If the user passes None, it never expires. We also add a helper method is_expired() that checks if Instant::now() is greater than or equal to the expires_at time.
- Why we wrote this way ? - If we didn't have this wrapper our HashMap would just map a key to a value. The HashMap wouldn't know anything about time.. BY wrapping the Value in CacheItem , We permanently glue the expiration stopwatch to the data itself.

- 2. `Cache<K, V>` 
- What it does => This is the main struct users will interact with it contains a standard HashMap that maps keys to our new CacheItem wrapper
- How it works => We learned about Generics and traits. Now we combine them. We can't just say Cache<K,V> because a HashMap is strict: it has to be able to hash its keys into memory buckets and it has to check keys for equality. So we enforce a Trait Bound: Cache<K: std::hash::Hash + std::cmp::Eq, V>. 
- This tells the compiler Only allow types for K that have signed the Hash and Eq contracts. 
- Why we wrote it this way - If we didn't add the trait bounds the Rust compiler would instantly crash, screaming. I don't know how to put a generic type K into a HashMap because I don't know if K is hashable

- `Concept 3` Implementing Methods
- What it does ? - `new()` - simply creates a fresh, empty Cache ready to be used.
- `set()` - Allows the user to insert a piece of data. It takes three arguments a key a value and an optional time to live ttl . It calculates the exact expiration time on the clock, packages the data into our CacheItem wrapper and saves it in the HashMap.
- How it works? - Inside a set() we use a match statement to look at the ttl which is an Option<Duration>:
- If the user provided Some(duration) eg-5seconds we add that duration to Instant::now(). This calculates the exact future stopwatch time. WE wrap the future time in Some()
- If the user passed None , we just leave it as None it lives forever
- Finally we packagae the value and the calculated expires_at time into a Cacheitem struct and call self.store.insert(key, item) to save it in the HashMap
- Why we did this - We calculated the exact Instant inside the set method to the user doesn't have to do the math themselves. This make our API extremely friendly to use. A user just says keep this for 5 seconds Duration::from_secs(5) and our cache handles calculation the exact clock time behind the scenes


- `Concept 4` Lazy Expiration (The Refrigerator vs The Sniff Test)
- `Active Expiration (The refrigerator)` - Imaging hiring a buttler whose only job is to stand in front of the fridger 24/7 , constantly checking the expiration date on every single item. If something expires they throw it out immediately. This is highly accurate but it wastes a massive amount of the buttlers time (CPU resources).
- `Lazy Expiration (The Sniff Test)` - We don't actively check the fridge. Instead we only check an items expiration date at the exact moment we want to eat it . If we grab the milk and see it's expired we throw it away right then and there. It saves tons of time CPU because we only check when necessary
- `Cache::get()` 
- What it does ? - The get method allows the user to ask for an item by its key. If the item exist and it has not expired it returns the value Some(&V) .  If the item doesn't exist or if it has expired it returns None.
- How it works? - We use the .get() method on our internal HashMap
- If we get None(it doesn't exist in the HashMap) we just return None.
- If we get Some(item) we immediatelly check our item.is_expired() helper method
- If it is expired we actually delete it from the HashMap right then and there using .remove() and return None.
- if it is not expired we return Some(&itme.value)
- Why we di this - As explained in the Sniff Test analogy deleting epired items only when a user asks for them costs zero background CPU resources. We don't need a background thread looping endlessly to clean up the cache


- `delete()` - this allows user to manually delete a specific key from the cache even if it hasn't expired yet
- `cleanup_expired()` - While lazy expiration `get()` is great, what if a user puts 10,000 items in the cache and never calls get() on them? They will sit in memory forever . This method loops through the entire HashMap and deleted everything that is expired

- `Concept 5` - `PhantomData<T> and Default Type Parameters`
- Imagine we are hosting two identical parties in two identical rooms. One is VIP party and one is general party.
- The rooms are exactly the same size and shape like our HashMap but we want the bouncer the rust compiler to prevent general guests from accidentally walking into the VIP room
- The Wristband(PhantomData<T>) - We give everyone a wristband (a marker type <T>) . However a wristband is just a piece of paper it doesn't physically take up a chair in the room . In rust if we declare a generic <T> on our struct but don't physically store it in fields the compiler throws a fatabl error saying. We have a wristband rule but nobody is wearing one in the Room
- `std::marker::PhantomDatat<T>` is a zero sized type . It takes up 0 bytes of RAM. it is how we tell the compiler. Pretend I am storing this wristband for the rule checking purpose even though it physically takes up zero bytes.
- `General Admission [Default Parameters]` - Most people don't care about wristband. If they don't ask for a VIP band, we have to just assume they are General Admission. In rust we can say <T = ()>. The () is the empty typle. This means if the user doesn't specify the type of wristband just default it to the empty tuple General Admission

- Why do we want this our Cache ? The why => Right now our Cache is just Cache<K, V>. But what if a user wants to run a Production cache and a Test CAche. They might accidentally pass the Test cache into a function that expects the Production cache. By adding a generic Context  type the wristband, we allow the user to label their caches. The Rust compiler will guarantee they never mix them up.

- `Concept 6 - Const Generics for fixed capacity`
- As of now Cache can grow infinitely . If a user puts 10 million items in their computer it might run out of RAM and crash we need to set maximum capacity
- we could use  `max_capacity: usize` field to the struct but we are going to learn a much more powerful advanced Rust feature: Cost Generics
- `Standard Generics <T>` - This is the Bouncer looking at our wristband type eg ONly VIPS allowed . it checks what kind of thing we are .
- `Const Generics <const N: usize>` - This isn't a bouncer check. this is the architect drawing the exact fire code limit directly into the physical blueprint of the room. By baking the number into the blueprint Cache<String, i32, 100>, the Rust compiler knows the exact maximum size of the cache before the program ever runs.


- `Concept 7 - Storing Closures for Eviction Callbacks`
- Sometimes when an item expires in our cache we don't just want it to quietly vanish. We might want to let the user run a custom function a closure to do something like loggin the deletion or updating a database . We call this Eviction Callback. To do this we have to store a user's function inside our Cache struct . This is notoriously difficult in Rust.

- `The mystery box with a walkie-talkie`
- The problem - In rust , a struct is like a perfectly measured shippping container. The compiler needs to know exactly how many cubic inches bytes every field takes up . 
- A closure a custom function can be tiny or huge depending on how many variables it captures . We cannot put an unknown sized blob into a perfectly measured shipping container. The compiler will panic
- The solution `Box<dyn Fn>` - Instead of putting the blob in the shipping container , we put the blob out in the Heap (the massive warehouse of memory). Then we put a Box inside our shipping container. A Box is just a tiny fixed size treasure map a pointer that tells you exactly where in the warehourse the blob is located. The shipping container stays perfectly measured
- `dyn Fn (Dynamic Dispatch)` - This stands for Dynamic Function . It is the walkie talkie . It means 'I don't know the exact size or name of the function sitting in the warehouse but i promise if you talk into this walkie talkie it will act like a funciont that takes X arguments and returns Y'


<h1>Day 15 </h1>

- `Concept 1 - The main Kitches nad the Line of Cooks`
- `Single threaded` - we are the only chef in the kitchen. we chop the onions, boil the water . ONe line of code executes at a time
- `Multi threaded` - We hire a line cook (a new OS thread). We hand them a recipe (a closure) and say 'Go do this' . Now we can boil the water while they chop the onions
- `move` Closures - Imagine we want the line cook to chop our onions. If we just let them look at our onions &onions, what happens if we shift ends and we take the onions home? The line cook will chop empty air (a dangling pointer). Rust prevents this . We must physically hand them over (move). The Line cook now owns the onions. We can never touch them again
- `.join()` Joining - We can;t serve the meal until the line cook is done. Calling .join() means we stand by the pass and wait for the line cook to finish their recipe and hand us the result

- `Concept 2 - Single threaded baseline word counter`
- `Concept 3 - Shared Mutable state`
- The problem with concurrency - In our basefile one thread loop reads file 1 then file 2 etc and tallys words into a HashMap. if we spawn 5 threads to read all 5 files at the same time, they all need to tally their words into the exact same HashMap
- If thread 1 and thread 2 try to add +1 to the word rust at the exact same millisecond the CPU overwrite one of the operations the count is corrupted and the program crashes . This  is called a Data Race
- `The shared Whiteboard and the Bathroom key`
- The problem - We have 5 line cooks [threads] and only one whiteboard [HashMap] where they all tally words. If cook 1 and cook 2 write on the same spot at the exact same time, their markers collide and the whiteboard is ruined
- `Arc`- `[Atomic Reference Counting] - The Invincible Whiteboard` 
- In week 2 we used Rc to share data, But Rc is fragile; if two threads clone an Rc simultaneously the counter breaks,
- Arc is an Rc wrapped in titanium. It uses CPU level atomic hardware instructions so multiple threads can share it safely. However Arc only lets the 5 cooks look at the whiteboard. it does not let them write
- `Mutex - [Mutual Exclusion]` - The Bathroom Key 
- To stop cooks from writing at the same time, we put a physical lock on the whiteboard. To write a cook must hold the key (.lock()). If cook 1 has the key , cook 2 must wait in the line. When cook 1 is done they drop the key and cook 2 can take it . This guarantees only one cook is writing at a time

- What it is - `Arc<Mutex<T>>` - This is the standard Rust pattern for sharing data across threads that needs to be mutated.
- How it workds - We wrap our `HashMap` in a `Mutex` and then wrap that `Mutex` in an `Arc`. 
```rust
let word_counts = Arc::new(Mutex::new(HashMap::new()));
```
- why we use it  - `Arc` allows us to .clone() the pointer so we can hand a copy to all 5 threads [spawn(move || ...)]. Inside the thread when we actually want to update the hashmap, we call .lock().unwrap(). This forces the thread to pause if another thread is currently writing . Once it has the lock it can update the HashMap safely.

`Concept 4 - Multi threaded with shared state`
- We are going to spawn 5 threads Each thread will read one of the 5 files we generated. Because they are all going to tally their words into the exact same HashMap , We will wrap that Hashmap in an Arc<Mutex<T>>

- `Concept 5 - Fearless Concurrency (Send and Sync)`
- Rust compiler has two special Marker traits built in
- 1. `Send` - Tells the compiler it is safe to pack this type in a box and mail it to the another thread
- 2. `Sync` - Tells the compiler it is safe to let multiple threads look at this type at the exact same time.
- Rc the Smart Pointer from last week is explicitly marked !Send (Not Send) by the compiler because its internal counter is fragile . If we try to pass an Rc into thread::spawn, the Rust compiler literally stops us in C++  , this would compile fine and cause a production crash. In Rust data races are a compile time error/

- `Concept 6 - The Map/Reduce Pattern`
- To fix our lock contention we are going to use Map/Reduce. Instead of giving the 5 Cooks one whiteboard we are going to give them each their personal notepad. 
- They will tally their own file (Map), and then hand their notepad to the Head Chef when they are done. The Head Chef will add all 5 notepads together `Reduce`. No Mutex locks required.


<h1>Day 16</h1>

- We learned the two ways threads can share data
- 1. `Shared Mutable state ` [Arc + Mutex] - Multiple threads fighting over the same whiteboard (lock contention)
- 2. `Map/Reduce` - Threads working completely isolated and returning their values at the very end.

- But what if we are building something like a streaming data processor, where data is flowing continuously? We can't wait until the very end to return a value , and locking a Mutex every milisecond is too slow. This is third way :: `Message Passing`
- do not communicate by sharing memory, instead share memory by communication - go proverb also heavily used in Rust
- We are going  to build a pipeline : Reader Stage (Read logs lines) -> Parser Stage (Extract data) -> Aggregator Stage (Calculates stats). Instead of sharing a Mutex they will pass data to each other on a conveyour belt a channel

- `Concept 1 - Message Passing and mpsc channels`
- Imagine  a restaurant kitches . In our mutex ecample 5 cooks were fighting over 1 whiteboard to write down orders. With channels we build a conveyor belt. The line cook [thread 1] chops vegetabls and puts them on the conveyor belt.
- The head chef [thread 2] stands at the end of the belt and takes the vegetables off to cook them. The cook and the chef never have to talk to each other or fight over a whiteboard. 
- The conveyour belt safely moves the food from one peron to the other .
- In rust , a channel is called mpsc which stands for Multi Producer , Single Consumer.
- `Producer`- `tx for Transmitter` - The end of the channel that sends data . We can clone the transmitter, allowing multiple threads to send data into the same channel
- `Consumer` - `rx for Receiver` - The end of the channel that receives data. There can only be one receiver. When we send data into a channel, we move ownership of that data into the channel.
- The Rust compiler guarantees that the sending thread can no longer touch it, completely preventing Data Races without needing a slow Mutex

- `Concept 2 - Mutexes vs Channels [When to use each]`
- Analogy - Google Docs vs Email attachments
- `Communicate by sharing memory (Mutex)` - This is like a shared Google Doc. Multiple people are editing the exact same document. It is great when everyone needs to see the exact current sate at all times, but to prevent chaos , people have to take turns typing locking
- `Share memory by conmmunicating [Channels]` - This is like an email chain with an attachment. We finish our work on a file , attach it to an email, and send it to the next person on the team . They now completely own the file. No one has to wait in line to type , making it perfect for step by step assembly lines
- Use `Arc<Mutex<T>>` when we have global state that many threads need to read and update randomly (eg- an in memory cache)
- Use `mpsc` Channels when we have a direction flow of data eg- adata pipeline or log processing . Channels move ownership of the data across thread boundaries , completely bypassing the need for expensive lock acquisition

- Right now the mpsc::channel() we are using is unbounded. That means it has inifinite size . Reading a file from a hard drive is much faster than processing and aggregating data.
- If our log file was 50 Gigabytes instead of 5 Megabytes , the Reader thread would instantly dump 50 GB of strings onto the conveyor belt in RAM. The Outcomes - Our computer would run out of memory OOM and the program would crash
- We are going to change our code to use Bounded Channels. We will put strict limit on the conveyor belt eg maximum 100 items. If the belt gets full the Reader thread will automatically be forced to pause and wait. This guarantees our program will use almost zero RAM , even if the file is 100 Terabytes


- `Concept 3 - Bounded channels and Backpressure`
- Imagine a factory conveyor belt. If the guy putting boxes on the belt works 10x faster than the guy taking them off the boxes will pile up and fall all over the floor Out of Memory Crash.
- To fix this we tell the fast guy - If there are 100 boxes on the belt, stop working until the slow guy catches up. This forced pausing is called `Backpressure`
- In Rust an unbounded channel is created with `mpsc::channel()`. A bounded channel with a fixed memory buffer. If a producer thread calls tx.send() when the buffer is full , the producer thread will block (go to sleep) until the consumer calls rx.recv() to free up space. 
- This ensures predictable 0(1) memory usage regardless of how large the input data stream is.


<h1>DAy 17</h1>

- What we are building ?
=> A new CLI tool called health_checker. We will give it a list of 100 website URLs and it will check if they are online HTTP(200 Ok) or offline
- Outcome 
=> A blazing fast program that pings all 100 websites concurrently , printing a nice formatted table to our terminal showing the URL . The HTTP status code and how many milliseconds it took to respond 
- Why we are building
- Up until today we have been using OS threads `std::thread::spawn`. OS threads are heavy . If we try to spawn 10,000 OS threads to check 10,000 websited our computer will literally crash because the operatingsystem cannot handle that much context switching. 
- Because wiating for website to respond is I/O Bound we are just sitting around waiting for the internet not using the CPU . Rust has a completely different system called `async/await` and `Tokio`
- It allows us to check 10,000 websites concurrently using only a single OS thread. This is how modern high performance web servers like Discord and AWS are built in Rust


- Adding dependencies - In `Cargo.toml` file Add the following 
```toml
[dependencies]
tokio = { version = "1.37.0" , features = ["full"] }
reqwest = { version = "0.12.4" }
```
- `tokio` - This is the standard Async Runtime in Rust . By default Tokio is  very lightweight so we have to manually opt-in to features
- `features = ["full"]` - This tells cargo to download every single piece of the Tokio library [ timers, networking, file I/O etc]. For a learning project "full" is easiest so we don't get missing feature errors
- `reqwest` - This is the standard HTTP client in Rust (the equivalent of fetch or axios in Js). we just specify the version "0.12.4"

- `Concept 1 - Why Async ?  OS Threads vs Green Threads`
- The waiter at a Restaurant
- `OS threads` [Synchronous] - A waiter takes our order, walks to the kitchen and literally stands there doing absolutely nothing, staring at the chef for 20 minutes until the food is ready. If we have 100 table we must hire 100 waiters . This is incredibly expensive because hiring waiters costs more money [RAM]
- `Green threads` [Async] - A single waiter takes our order, hands it to the kitchen and while the food is cooking , they immediately walk to the next table to take their order. One waiter can easily handle 100 tables because an order[CPU] is fast but waiting for the food to cook [I/O, like a Network Request] is slow. The watier is never blocked

- `std::thread::spawn` creates a real OS thread managed by the kernel. Each thread allocates roughly 2MB of memory for its stack. If we spawn 10,000 OS threads to make 10,000 HTTP requests, we consume 20GB of RAM just for idle threads waiting on the network
- Tokio [the async runtime] uses Green threads (tasks) via `tokio::spawn`. TAsks run on tiny pool of OS threads [usually one per CPU core]. When a Task makes an I/O request like fetching a website, Tokio parks that task and instantly switches to another Task on the exact same OS thread.
- The context switch happens in user space(nanoseconds) rather than kernel-space (microseconds). this allows us to handle tens of thousands of concurrent I/O operations with virtually zero memory overhead.


- The Goal : We need to undersatnd the fundamenatl difference between how Javascript handles async [Promises] vs how Rus handles async [Futures] and then we will write the code to start our Tokio runtime. 
- The Outcome : Our src/main.rs file will be converted into a Tokio-powered async entrypoint

- `Concept 2 - Futures and The Tokio Runtime [Lazy State Machines]`
- Javascript Promises [eager] - In JS, a promise is like ordering a pizza. The second you call fetch() the delivery guy starts driving to your house . It executes immediatedly even before we write .then()
- Rust Futures [Lazy] - In Rust an `async fn` returns a `Future`. A Future is just a pizza recipe. WE can write it down, hand it to a friend or put it in drawer . Absolutely no cooking happens until we explicitly hand it to a chef and say - Cook this now (by calling `.await`)

- Because Rust has no built in runtime (unlike Node.js or the browser) calling an async fn does nothing on its own, it just compiles into a state machine describing the work.
- TO actually execute the future , it must be polled by an executor. We use the `tokio` runtime for this. When we decorate our main function wit the `[#tokio::main]` macro it secretly rewrites our main function into a synchronomous function that builds the tokio runtime, blocks the main thread and execute our async code inside it. when we call .await inside that runtime we are yielding control back to Tokio saying I can't make progress until this I/O finishes go run another task while i wait

- The Goal : We are going to use the   `reqwest` library to make a single HTTP GET request to test website (https://httpbin.org/status/200). 
- The Outcome : When we run the code , our program will reach out to the internet , pause execution while it waits for a response and then print status: 200 OK to our terminal to prove it worked

- The Goal : Checking one website is nice but we need to check 100 concurrently . WE are going to create a list of URLs and launch an async network request for every single one of them at the exact same time.
- The Outcome : When we run the code, it will fetch 5 different website concurrently. Instead of taking 5 seconds (1 second per site sequentially) it will finish all of them in ~1 second total

- `Concept 3 - Implement concurrent checking for multiple URLs`
- Mailing 100 letters 
- 1. Synchronous - When we write one letter, walk to the post office, drop it off walk home and start the next letter. This takes weeks.
- 2. Async(Tokio Tasks) - We write all 100 letters, put them in a big pile on our desk and call FedEx to pick them all up at once. They are all delivered at the exact same time. Calling `tokio::spawn` is like handing one letter to the Fedx guy.

- When we call `tokio::spawn(async { ... } )` we are giving a Future to the Tokio runtime and saying start running this in background immediately. It returns a `JoinHandle` .
- A `JoinHandle` is just a ticket that we can .await later to get the final result. If we spawn 100 tasks in loop they all start executing concurrently across Tokios thread pool. 
- WE can collect all 100 tickets into a Vec<JoinHandle> and loop through them to .await their results

- What if we had 10,000 URLs instead of 5?
- The Goal - If we run 10000 concurrent network request using tokio::spawn, we might crash our home router , get our IP banned by the target server or exhaust our OS file descriptors.
- We need to add Rate Limiting to our loop. At the same time we will format our output into a clean, professional CLI table
- The Outcome - We will use a Semaphore to limit concurrency to exactly 2 active connections at a time. The program will fetch the URLs in small batches of 2 and print the results cleanly

- `Concept 4 - Rate Limiting with `**Semaphore**` and Table Formating`
- The NightClub Bouncer - Imagine a night club with a strict bouncer. The club only has a capacity of 100 people . If we want to go in, the bouncer gives us a VIP wristband. When we leave , we give the wristband back. If 1000 people show up at once the first 100 get wristband and go inside immediately.
- The 901st must wait in line outside until someone leaves and hands back a wristband. A `Semaphore` is the bouncer. The wristband is a permit

- `tokio::sync::Semaphore` is a concurrency primitive. We create it with a fixed number of permits eg - 2 . Before a spawned task is allowed to make its HTTP request , it must call `semaphore.acquire().await` . If all permits are taken, the task gracefully suspends (goes to sleep) without blocking the OS thread . When the request finishes the permit is dropped, waking up the next task in line

- The Goal - We need to wrap our network request in a tokio Timeout so it gives up ifa webstie takes too long . WE also need to measure exactly how long the request took using `std::thread::Instant`.
- The Outcome - The table will have a new column showing "Latency(ms)" and any request that takes longer than 2 seconds will print a `Timeout` error instead of hanging


<h1>Day 18</h1>

- **Rate Limited Web Scraper**
- What we are building : A new CLI tool called web_scraper. We are going to take the async networking foundation we just learned and weaponize them to scrape data from website. It will handle retries [if a site drops a connection], respect rate limits and output(the scraped data into structured JSON or CSV)
- The Final Outdome : A program that gracefully crawls multiple websites pulls specific HTML data out of them, recovers from network failures automatically using  retry logic and dumps the results into a file.
- Why we are building it (The Architectural Shift) : Right now , our network requests are fire and forget . If `reqwest::get()` fails, we just print `NETWORK_ERROR` and give up. IN a production environment, network requetfail constantly for random reasons. On Day 18 we learn to use `tokio::select!` and `tokio::time::sleep` to built Resilient Systems that automatically retry failed operations and we will learn how to parse HTML in Rust

- The Goal : We are going to build an async function that tries to fetch a website. IF the website is donw instead fo crashing it will wait exactly 3 seconds , and then try agian. 
- It will do this upto 3 times before finally giving up
- The OUtCOME : We will test this by intentionally providing a broken URL. We will see the program fail wait 3 seconds fail again wait 3 second fail a final time and then gracefully exit

- `Concept 1 - The Retry Loop and The sleep anti-pattern`
- WAiting for the Oven - WE want to bake a cake but we need to wait 30 minutes for the oven to preheat
- `Anit-pattern [std::thread::sleep]` - We literally stand perfectly still in fron of the oven for 30 minutes . We block anyone else in the kitches from using the sink or the fridge
- `Pattern [tokio::time::sleep]` - We set a kitchen timer for 30 minutes , leave the kitchen and go fold laundry. When the timer goes off we come back.

- When scraping websites , we often need to implement a retry loop (if the connection fails wait 3 seconds and try again ) . If we use the standard `std::thread::sleep()` inside an `async fn` , we are committing a cardinal sin:Starving the Runtime. 
- Because Tokio multiplexes hundreds of tasks onto a single OS thread , blocking that OS thread with a synchronous sleep means none of the other tasks on that thread can make progress for 3 seconds . We must always use `tokio::time::sleep(...).await`. This tells Tokio to park the current task for 3 seconds and immediately run other tasks on that thread in the meantime

- Our network request are now virtually bulletproof. They will keep trying until the website comes back online , but they won't block the Rest of our app from doing other things
- The Goal :Right now when a request succeeds we just print Downloaded 54321 bytes. That's useless data. We want to actually read the HTML, find the `<title>` tag and extract the text inside of it .
- The Outcome : We will write a function that takes the raw HMTL string, builds, DOM tree and uses a CSS selector to find the `<title>` tag. When we run it, it will print exactly Title: Rust Programming Language instead of the whole page


- `Concept 2 - Parsing HTML in Rust`
- `The Index and the Librarian` : Imagine we have a 1000 page encyclopedia RAW HTML and we only want to read about "Lions". Parsing is like looking at the index at the back of the book to find exactly which page "Lions" is on. A CSS Selector is like handling a librarian a sticky note that says "Give me all the bold text on page 42". The librarian (the `scrapper`)crate does all the hard work of reading the pages and handing us back exactly the sentences we asked for 
- When we download HTML via reqwest, it is just a giant String, Rust doesn't know what a `<div> or a <title>` is. The scrapper crate takes that String and builds a Document Tree DOM in memory using `Html::parse_document(&html)`. WE then compile a CSS Selector like h1 or .title and ask the Document Tree to hand us an iterator of all the HTML elements that match that selector.

- The Goal : WE are going to bring back the concept of tokio::spawn, Semaphore and tokio::time::timeout and combine them with our Retry loop and HTMLparser. Finally instead of printing to the terminal we will append our results to a CSV file. 

- The Outcome : We will provide a list of URL's , The program will scrape them concurrently (rate limited to 3 at a time) . When finished we will have to results.csv file our hard drive with the data


- `Concept 3 - The Final Production Scraper [Structured Output]`
- FedEx Drivers with Clipboards - We have hired 3 FedEx drivers [Semaphore] . Each one is given a timer [TImeout] and told that if a house doesn't answer they should wait and knock again [Retry loop]
- When they finally get a package (HTML title), they don't just shout it into the void(Terminal) . They write it down on a clipboard in a nice grid [CSV file]
- In a production web scrapper we never just `println!` our data . WE write to a database or file . We will use `std::fs::File` and `std::io::Write` to append lines to a .csv file. We will use `tokio::spawn` to run our `fetch_with_retry()` function concurrently, guarded by an `Arc<Semaphore>`to prevent rate limits and wrapped in a timeout to prevent hanging requests


- The Goal : We need to understand how Tokio is actually able to time out a network request. In python or Node.js cancelling a rnning network request is notoriously difficult. In Rust it is hilariously easy.
- The Outcome : WE are going to replace our massive scraper code with a tiny 15 line Pizza-Race simulation to see `tokio::select!` instantly cancel a running task

- `Concept 4 - Racing Futures and Cancellation`
- Racing Pizza Delivery - We order pizza from dominos and Papa Johns exactly the same time. We wait at the door . Whichever delivery driver arrives fist we pay them and take the pizza. We immediately call the other driver and tell them to throw their pizza away [Cancel]
- `tokio::select!` - Lets us `.await` multiple futures at once on a single thread . Whichever Future finishes first wins the race and its code block is executed. The magic of Rust is what happends to loser. it is immediately dropper. Because Futures in Rust are lazy state machines dropping them instatnly cancels any further work they were going to do .
- WE don't need complex cancellation tokens or signals. This is exactly how `tokio::time::timeout` is built under the hood it races our network request against a `tokio::time::sleep()` timer . whichever finishes first cancels the other


<h1>Day 19 </h1>

- Imagine building an Amazon clone. When a user clicks Checkout our server must charge their credit card. . To do this we write a PaymentProcessor struct that uses the Stripe API to move real money. However we also want to write automated tests for our checkout systems. If our test runs the PaymentProcessor it will actualy charge a real credit card every time we run cargo test . That is a massive bug. WE need a way to swap out the real Stripe backedn for a fake Mock backend during testing .

- The Solution - We will use a design pattern called Dependency injection. Instead of our PaymentProcessor being  hardcoded to use Stripe, we will tell it to use any struct that promises to act like a payment backend. we will enforce this promise using a Rust `trait`

- The Flow - We will define a trait called PaymentBackend with a Single method: charge_card()
- We will create two completely different structs: Stripe and MockBackend
- We will implement the PaymentBackend trait for both structs
- We will build our core PaymentProcessor struct. Instead of giving it a specific type, we will give it a pointer to dynamic memory (Box<dyn PaymnetBackend>)
- At runtime we can inject whichever backend we want into the processor and it wont care It just knows it can call `.charge_card()` on it 
- This pattern is the backbone of all major enterprise software . By separating the behavior (the trait) from the data (the struct ) we decouple our system

- `Concept 1 - Traits and Interfaces [Behavior, not Data]`
- The Goal - We need to define the Contract the Trait that any payment backend must follow. Then we need to create two dummy structs (Stripe and Mockbackend) and force them to sign that contract. 
- The Outcome - We will have two different structs that both implement the exact same charge_card method meaning they can eventually be swapped interchangeably

- The Cashier - The trait is a piece of paper taped to the wall that says JOB Description Cashier. Must be able to take an amount of money
- The structs are the applicants . Stripe is a real human (Alice) with a real API key. MockBackend is a crash-test dummy used for fire drills
- The Implementation - impl is the act of handling the cashier nametag to both Alice and Dummy. They sign a contract promising they know how to do the job

- Technical - Up until now , we used traits to add methods to a specific struct. But the true power of trait is acting as an Interface. A trait defines a contract of behavior . By implementing PaymentBackend for both Stripe and MockBackend, we decouple our system. Our future PaymentProcessor won't ask for a Stripe struct(data) it will ask for anything that implements PaymentBackend behavior 



- `Concept 2 - Dependency injection and Dynamic Dispatch`
- The Goal: We need to build the acutal PaymentProcessor struct, this struct needs to hold some backend either Stripe or Mockbackend inside of it so it can call .charge_card() on it.
- The OUtcome : We will create a PaymentProcessor struct that doesn't know what backend it is holding . We will then update our main() function to inject a Stripe backend into it and run it 

- The Universal USB PORT - Imagine we build a giant stereo system. We don't handwire an iphone directly into the motherboard of the stereo. If we did , we could never plug in an Android phone.
- Instead we build a Universal USB Port on the front of the stereo. We tell the stereo - I don't care what device is plugged in , as along as it can send an audio signal. When we plug a phone in , we are injecting the dependency.


- When a struct needs to hold a trait, the Rust compiler panics. Rust must know exactly how many bytes of memory a struct takes up at compile time. But Stripe might be 24 bytes and MockBackend might be 1 byte. To fix this we put the backend inside a Box. A Box stores the actual struct on the heap and keeps a fixed size pointer 8-bytes on the stack. 
- The dyn keyword means Dynamic dispatch - we are telling the compiler we will figure out exactly which struct this is at runtime.

- The Goal: We need to prove that our PaymentProcessor is truly decoupled. We will instantiate three different processor: one with Stripe , one with a successful MockBackend and one with a failing MockBackend

- The Outcome: When we run cargo run, we will see all three transactions process through the exact same process() method. The PaymentProcessor code does not change at all, yet it executes three completely different behaviors depending on what was injected into the Box


- Code overview at runtime : When prod_processor.process(99.99) runs, Rust looks at the pointer inside backend and calls Stripe::charge_card.
- When test_processor_1.process(49.50) runs, Rust looks at the pointer inside backend and calls MockBackend::charge_card (which enters the if self.should_succeed branch and returns Ok(()) ).
- When test_processor_2.process(12.00) runs Rust looks at the pointer inside backend and calls MockBackend::charge_card (which enters the else branch and returns Err("Card declined by mock"))


<h1>Day 20</h1>

- `REST API with DATABASE (Axum + sqlx + SQLite)` - Production grade async web microservices

- The PRoblem - When building Web API's in Node.js Express/Fastify or python FastAPI , request routing and database calls are easy to write but they suffer from hidden runtime bugs. A typo in an SQL qery string or a missing field in a JSON payload won't be caught until a user hits our API endpoint in production
- IN Rust we want compile-time type safety for our entire web server and database layer.

- If a client sends invalid JSON , the web framework(Axum) automatically rejects it before our handler function even runs .
- If we write an invalid SQL Query, the database driver (sqlx) will fail the build (cargo build) at compile time by checking our SQL queries against our database schema

- The Architecture - We are going to build a full CRUD (Create, Read, Update , Delete) REST API for a Bookmark Manager Service(bookmark_api). 
- Users will be able to 
> POST /bookmarks - Save a new URL with a title and tags
> GET /bookmarks - List all saved bookmarks
> GET /bookmarks/search?q=rust - Search bookmarks by keyword or tag
> DELETE /bookmarks/:id - Delete a bookmark by ID

- Our architecture consists of 4 decoupled layers
1. **The Web Framework(Axum)** - Powered by `tokio`, `hyper` and `tower`. Handles HTTP routing TCP sockets and middleware.
2. **Extractors & Serds** - Automatically parses incoming HTTP JSON payloads (`Json<CreateBookmarkReq>`) and query parameters (`Query<SearchParams>`) into strongly typed Rust structs
3. **Shared Application State (Arc<AppState>)** - Shares our database connection pool thread safely across all incoming async HTTP request tasks.
4. **Compile-Time Persistence Layer(sqlx + SQLite)** - Manages connection pooling and executes type-checked SQL queries against an embedded SQLite Database

```
[ Incoming HTTP Request ] 
          │
          ▼
    [ Axum Router ] ──(Path / Method matching)
          │
          ▼
   [ Serde Extractor ] ──(Parses JSON body into Struct; 400 Bad Request if invalid)
          │
          ▼
 [ Async Handler Fn ] ──(Accesses shared Arc<AppState>)
          │
          ▼
 [ sqlx Pool Query ] ──(Executes type-checked SQL against SQLite DB)
          │
          ▼
 [ HTTP JSON Response ] ◄──(Converts return struct to JSON; 200 OK or 500 Error)

```

- `Concept 1 - Axum Routing & Extractors [Path, Query, Json, State]`
- The Goal : Build a production grade async web microservice bookmark_api capable of performing full CRUD operations on bookmarks stored in an embedded SQLite database
- The Outcome : A web server running on localhost:3000 where all HTTP requests , JSON payloads, and SQL queries are verified for type safety at compile time.

- The Problem - When building Web APIs is Node.js (Express/Fastify) or Python (FastAPI) request routing and database queries are easy to write but they suffer from hidden runtime bugs
- A typo in an SQL query string (`SELECT * FRM bookmarks`) won't be caught until a user hits our endpoint in production.
- A missing field or wrong data type in JSON payload requires manual validation logic inside every route handler.

- In Rust we want compile time type safety for our entire web server and database layer
1. If a client sends invalid JSON, the web framework(Axum) automatically rejects it with a 400 Bad Request before our handler function even runs
2. If we write an invalid SQL query, the database driver(sqlx) will fail the build (cargo buil) at compile time by checking our SQL queries against our database schema.

- The Airport Security Checkpoint : Imagine an international airport. Before we board a plane passengers line up at different gates (Router). At each gate , security guards (Extractors) inspect incoming passengers:
- `Path` Extractor - Checks our passport to extract our Destination Gate ID (eg: /bookmarks/:id)
- `Query` Extractor - Checks our baggage ticket for special requests (eg: ?.search=rust&limit=10)
- `Json` Extractor - Scans our luggaage contents and unpacks it into a standard luggage bin(CreateBookmarkReq struct).
- `State` Extractor - Hands us an airport badge to access shared facilities(database connection pool)

- If our passport is invalid or our luggage contents are corrupt the Extractor security guard turn us around at the gate immediately with a 400 Bad Request before we ever reach the flight attendent (our handler function)

- `The Technical Explanation - Type Based Extraction (FromRequest)`
- In traditional web frameworks, we manually parse request bodies and parameters . In Axum a handler is just an `async fn`. Axum uses **Type-Based Extractors**.
- An extractor is any Rust type that impement Axum's `FromRequest` or `FromRequestParts` trait.
- When an HTTP request hits an Axum route, Axum inspects the type signature of our functoin arguments. It automatically invokes the `FromRequest` trait implementation for each argument in order , deserializing JSON, parsing URL parameters and injecting shared state

```rust
async fn update_bookmark(
- Extracts the database connection pool shared across the entire web server
- Pattern matches State(pool) from the argument type State<SqlitePool>
- Allows multi threaded async request tasks to access the SQLite connection pool thread safely
    
    State(pool): State<SqlitePool>,

- Extract route variables like :id from URL routes eg: /bookmarks/42
- Pattern matches Path(id) from the argument type Path<i64> Parses the URL path segment into a 64 bit signed integer i64
- If a client request /bookmarks/abc passing text instead of a number , Axum's Path extractor automatically fails an returns 400 Bad Request without executing our function
    
    Path(id): Path<i64>,

- Extracts URL query string parameters eg- q=rust&sort=desc
- Pattern matches Query(params) from the argument type Query<SearchParams>
- Deserializes key-value query parameters directly into our strongly typed SearchParams struct using serde

    Query(params): Query<SearchParams>,

- Extracts and deserializes the HTTP request body application/jsob
- Pattern matches Json(payload) from the argument type Json<UpdateBookmarkReq>
- Enforces that the incoming JSON body matches our struct fields and data types exactly. If fields are missing or mistyped , Axum returns 400 Bad Request

    Json(payload): Json<UpdateBookmarkReq>,

) -> Result<Json<Bookmark>, StatusCode> {

- Defines the HTTP response type returned by the function
- Returns Result. On Success (ok) returns Json(Bookmark) which Axum serializes to JSON with HTTP 2000 Ok . ON failure Err returns an HTTP StatusCode enum (eg StatusCode::NOT_FOUND or StatusCode::INTERNAL_SERVER_ERROR)
- Axum uses the IntoResponse trait to convert any return type into a valid HTTP response sent back over TCP.
}
```
