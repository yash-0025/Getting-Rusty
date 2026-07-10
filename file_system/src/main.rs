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

impl Node {
    // Create a new independent node like a  root folder
    pub fn new(name: &str) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            name: name.to_string(),
            children: vec![],
            parent: None,
        }))
    }

    // Add a child to this node like running mkdir or touch
    pub fn add_child(parent: &Rc<RefCell<Node>>, child_name: &str) -> Rc<RefCell<Node>> {
        // Create new child
        let child = Node::new(child_name);

        // Point the child parent field up to the parent
        // We use Rc::downgrade() to turn the strong Rc into a weak pointer
        child.borrow_mut().parent = Some(Rc::downgrade(parent));

        // Add the child to the parents list of children
        // We use Rc::clone() because the parent strongly owns the child
        parent.borrow_mut().children.push(Rc::clone(&child));

        return child;
    }
}

fn main() {
    // Create root directory
    let root = Node::new("C:");

    // Add some folders inside C:
    let dev = Node::add_child(&root, "Dev");
    let users = Node::add_child(&root, "Users");

    // Add files inside Dev
    let rust = Node::add_child(&dev, "Rust");
    println!("File system built successfully without memory  leaks!");
}



// 1. What it does
// => it creates a struct that can represent any file or folder
// => children use Rc<RefCell<Node>> because the Parent is the absolute owner of it children.
// => parent uses Option<Weak<RefCell<Node>>> becaue a child looks at its parent , but does not own it . We use Option because the very top Root foler does not have a perent

// - If we use Rc for the parent , we would create a permanent memory leak . By using Weak when we delete a parent folder all of its children will automatically be deleted from memory too


// 2. What it does 
// `new` creates a standalone Node with no parent and no children. add_child takes an existing Node the parent creates a new Node and links them together.
// How it works - We use Rc::new(RefCell::new(...)) to create out shared mutable nodes
// When linking the child to parent we use Rc::downgrade(parent) to generate a Weak pointer 
// We then push an Rc::clone(&child) into the prent's children array
// why we did this - This perfectly matches our anlogy. The parent strongly owns the child using Rc but the chiild only weakly references the parent using Weak