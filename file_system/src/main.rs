use std::rc::{Rc, Weak};
use std::cell::RefCell;

// A node can be file or directory
pub struct Node {
    pub name: String,
    // if it's a file it might not have a children if it's a dir, it has a list of children
    pub children: Vec<Rc<RefCell<Node>>>,
    // The child points back to the parent using a Weak pointer
    pub parent: Option<Weak<RefCell<Node>>>,
}

fn main() {
    println!("Hello, world!");
}



// 1. What it does
// => it creates a struct that can represent any file or folder
// => children use Rc<RefCell<Node>> because the Parent is the absolute owner of it children.
// => parent uses Option<Weak<RefCell<Node>>> becaue a child looks at its parent , but does not own it . We use Option because the very top Root foler does not have a perent

// - If we use Rc for the parent , we would create a permanent memory leak . By using Weak when we delete a parent folder all of its children will automatically be deleted from memory too
