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

    // Print tree recursively
    pub fn print_tree(&self, depth: usize) {
        // Create an indent string 2 spaces per depth level
        let indent = "  ".repeat(depth);
        println!("{}| - {}", indent, self.name);

        // Recursively print all children
        for child in &self.children {
            // We use .borrow() to get read-access to the child node
            child.borrow().print_tree(depth + 1);
        }
    }

    // A helper funcction to securely read the parents name
    pub fn get_parent_name(&self) -> String {
        // WE look at our Option<Weak> parent
        match &self.parent {
            Some(weak_parent) => {
                // We must try to upgrade it to a strong Rc to use it
                if let Some(strong_parent) = weak_parent.upgrade() {
                    strong_parent.borrow().name.clone()
                } else {
                    "Parent was deleted!".to_string()
                }
            }
            None => "No parent (I am root)".to_string(),
        }
    }

    pub fn ls(&self) {
        for child in &self.children {
            println!("{}", child.borrow().name);
        }
    }

    pub fn rm(&mut self, target_name: &str) {
        // .retain() keeps elements whre the closure return true
        // it drops elements where the closure returns false
        self.children.retain(|child| child.borrow().name != target_name);
    }


}

// WE are tellin rust - Hey right before you destroy a Node print this message
impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping node: {}", self.name);
    }
}


fn main() {
    // // 1. Create the root directory
    // let root = Node::new("C:");

    // // 2. Add some folders inside C:
    // let dev = Node::add_child(&root, "Dev");
    // let _users = Node::add_child(&root, "Users");

    // // 3. Add files inside Dev
    // let rust = Node::add_child(&dev, "Rust");
    // println!("File system built successfully without memory  leaks!");

    // // Print the entire tree starting from root
    // root.borrow().print_tree(0);

    let rust_weak;

    {
        println!("--- Building the File System ---");
        let root = Node::new("c:");
        let dev = Node::add_child(&root, "Dev");
        let _users = Node::add_child(&root, "Users");


        let rust = Node::add_child(&dev, "Rust");
        rust_weak = Rc::downgrade(&rust);

        println!("--- File system Built ---");

        println!("\n> ls c:/Dev/");
        dev.borrow().ls();


        println!("\n> rm Rust");
        dev.borrow_mut().rm("Rust"); // this will trigger the drop trait immediately

        println!("\n> ls c:/Dev/ (after rm)");
        dev.borrow().ls();

        println!("\n--- Leaving Scope ---");

    }

    println!("--- Outside Scope ---");
    if rust_weak.upgrade().is_none() {
        println!("Memory was freed successfully! No leaks !");
    } else {
        println!("Memory leak!! The node still exists");
    }
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


// What it does? => print_tree visually outputs the file system hierarchy. get_parent_name looks up the tree to find the name of the folder that owns it
// How it works? => print_tree uses recursion . It prints itself, then loops through self.children and tells them to print themselves with a larger depth
// get_parent_name uses pattern mathcing on the Option . If a weak pointer exists , it uses if let Some(..) = weak_parent.upgrade() to safely attempt to read the paren'ts memory
// Why - Forcing us to .upgrade() a weak pointer in rust way of guaranteeing memory safety. In languages like C++ looking at a deleted parent causes a catastrophic crash. In rust it just safely returns None


// what it does ? => It builds the tree inside a constrained {...} block. When the block ends the variables go out of scope, triggering the Drop trait
// How it works? => When root goes out of scope , its Rc count drops to 0. Rust destroys root. But root owns a children array containing dev and _users . So rust destroys the children array containing dev and _users. So Rust destroys the children array which drops the Rc count for dev to 0. 
// So Rust destroys dev . The cascading effect perfectly cleans upt the entire tree
// Why we did this way?? => If we had use an Rc for the parent pointer, dev would have kept the root alive, and root would keep dev alive. The Drop trait would never trigger. Our  test proves that Weak breaks the cycle



// What it does ? => ls loops over the children array and prints their name . rm searches the children array for a specific name and delted it from the array
// How it workss? => rm uses the buil in .retain() method on Vectors, .retain() loops through the array and if the condition returns false meaning the child's name matches the target name it permanently deletes that element from the array
// Why we did this ? => Deleting the element from the children array causes its Rc count to drop. Because there are no cycles thanks to Weak the Drop trait will immediately trigger and free the memory