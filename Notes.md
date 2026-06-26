`cargo new hello-rust` => Creates Rust project <br>
`cargo check` => compile the project <br>
`cargo run` => runs the project and use the cache from the cargo check command as it is already compiled. <br>
- This is called as **incremental compilation** . So when we did check it did all the parsing , type checking and borrow checking from scratch and then we ran the run command it didn't recompiled because it used the cached version from the cargo check command.

```
`use std::env` 
`env::args()` => returns an iterator over CLI args
`env::args().collect()` => collects the iterator into a vector. 
`let args: Vec<String> = env::args().collect();` 

`env::args().nth(1)` => Grabs the 2nd element index1 - skips the program name at index0
`Option<String>` => This is Rust null-killer. Instead of the value maybe being undefined or null , Rust wraps it in Option: either Some or None . The compiler forces you to handle both cases so you cannot forget to check the missingcase - it won't compile.
`match` =>  Pattern matching that exhaustively handles every variant . Try deleting the None branch - the compiler will refuse to compile until you handle it. This is fundametally different from JS where we can forget and else and get undefined behaviour at runtime.
`{n}` => Inside teh string It allows iniline varibale capture in println! format strings, just like JS template literals (`Hello, ${name}!`)
`eprintln!` => Prints to stderr instead of stdout Error messages go to stderr , normal output goes to stdout. Same convention as every Unix tool.
`cargo fmt` => To format the code 
`cargo clippy` => It is an automated senior code reviewer.
`cargo clippy -- -D warnings` => Production CI command || -D warnings falg turns clippy warnings into hard errors - CI fails if clippy isn't happy.
```
