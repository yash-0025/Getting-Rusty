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


