use std::collections::VecDeque;
use std::ops::Add;


/// A generic last-in-first-out (LIFO) stack data structure
/// 
/// #Examples
/// 
/// ```
/// use collections::Stack;
///
/// let mut stack = Stack::new();
/// stack.push(100);
/// stack.push(200);
///
/// assert_eq!(stack.pop(), Some(200));
/// assert_eq!(stack.pop(), Some(100));
/// ```

// What it does - it documents the Stack struct and provides a Markdown code block showing how to use it
// How it works - When we run cargo test. Cargo scans our files for /// comments. If it finds a Markdown code block the triple backticks ``` it extracts that code , compiles it and runs the assert_eq! just like a normal test
// Why we did this - Writing examples in our docs makes our library easier to use. Having the compiler verify those examples gguarantees our docs never lie

#[derive(Debug, Clone)]
pub struct Stack<T> {
    items: Vec<T>,
}


#[derive(Debug, Clone)]
pub struct Queue<T> {
    items: VecDeque<T>,
}

pub trait Collection {
    // Required method : Whoever implements this trait must write this logic
    fn len(&self) -> usize;

    // Default method : Whoever implements this trait gets this for Free
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}


// We must put <T> after impl to tell Rust that T is a generic type and then <T> after stack to say we are implementing this specific generic struct
impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            items: Vec::new(),
        }
    }
// item must be exactly the type T
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            items: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, item: T) {
        self.items.push_back(item);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.items.pop_front()
    }
}


impl<T> Collection for Stack<T> {
    fn len(&self) -> usize {
        self.items.len()
    }

    // We do not need to implement is_empty() here we get it for free
}

impl<T> Collection for Queue<T> {
    fn len(&self) -> usize {
        self.items.len()
    }
}

impl<T> Iterator for Stack<T> {
    // We lock in the associated type
    // we are telling rust the items this iterator yield will be of type T
    type Item = T;

    // We must return an Option of our Associated type
    fn next(&mut self) -> Option<Self::Item> {
        // Our existing pop() method already does exactly what we need
        self.pop()
    }
}


impl<T> Add for Stack<T> {
    // Add also uses an Associated type to declare what the Result of the addition is
    // When we add two stacks together , we should get a new stack out
    type Output = Stack<T>;

    // the add method takes ownership of self the left side of the +
    // and rhs right hand side of the +
    fn add(mut self, mut rhs: Stack<T>) -> Self::Output {
        // We append the right side items into the left side items
        self.items.append(&mut rhs.items);
        // And returns the newly combined stack
        self
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }

    #[test]
    fn test_stack() {
        let mut my_stack: Stack<i32> = Stack::new();

        my_stack.push(10);
        my_stack.push(20);

        // assert_eq!(my_stack.pop(), Some(20));
        // assert_eq!(my_stack.pop(), Some(10));
        // assert_eq!(my_stack.pop(), None)

        // Because Stack implements iterator, we can use it direclty in a for loop
        // Note:: This will drain the stack because our next calls pop
        for item in my_stack {
            println!("Popped: {}", item);
        }
    }

    #[test]
    fn test_queue() {
        let mut my_queue: Queue<i32> = Queue::new();

        my_queue.enqueue(10);
        my_queue.enqueue(20);

        println!("My Queue state: {:?}", my_queue);

        assert_eq!(my_queue.dequeue(), Some(10));
        assert_eq!(my_queue.dequeue(), Some(20));

        assert_eq!(my_queue.is_empty(), true);
        assert_eq!(my_queue.dequeue(), None);

    }


    #[test]
    fn test_add_stacks() {
        let mut stack1: Stack<i32> = Stack::new();
        stack1.push(1);

        let mut stack2: Stack<i32> = Stack::new();
        stack2.push(2);

        let mut combined_stack = stack1 + stack2;

        // The last item pushed was 2 so it should pop first
        assert_eq!(combined_stack.pop(), Some(2));
        assert_eq!(combined_stack.pop(), Some(1));

    }

    #[test]
    fn test_stack_push_and_pop() {
        let mut stack = Stack::new();
        stack.push(10);
        stack.push(20);

        assert_eq!(stack.pop(), Some(20));
        assert_eq!(stack.pop(), Some(10));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_stack_peek() {
        let mut stack = Stack::new();

        stack.push(99);

        assert_eq!(stack.peek(), Some(&99));
        assert_eq!(stack.pop(), Some(99));
    }
}


// What it does - It creates a hidden tests module that contains two test function for our Stack
// How it works -The #[cfg(test)] attribute acts like a gatekeeper, the code inside doesn't even exist unless we tell Cargo taht we are running test . Inside  the test functions assert_eq!(A,B) checks if A==B . If it's false the test panics and fails
// Why we did this way - Putting unit tests in the same file as the source code allows the tests to access private helper functions if needed . The #[cfg(test)]  attribute ensures this test code never bloats our final production binary

