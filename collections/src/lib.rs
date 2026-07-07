use std::collections::VecDeque;

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
}
