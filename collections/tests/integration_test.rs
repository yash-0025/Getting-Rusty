// Because this is an external test, we have to import our library by its crate name
use collections::Stack;


#[test]
fn test_stack_from_outside() {
    let mut my_stack = Stack::new();
    my_stack.push("Apple");
    my_stack.push("Banana");

    // this proves that our stack can hold strings and that pop is perfectly public
    assert_eq!(my_stack.pop(), Some("Banana"));
}


// What it does - It creates a brand new testing environment that pretends to be a completely differnt project dowloading our collection crate 
// How it workds - Notice that there is NO #[cfg(tests)] here , Because this entire tests/ folder is exclusively for testing , Cargo automatically knows to only compile this folder whe we run cargo test. Notice we also have to explicitly use collections::Stack to import it
// Why we dit it this way - If we accidentally forgot to put pub in front of our STack struct or pop() method. this test would immediately fail, saving us from publishinig a broken library to crates.io

