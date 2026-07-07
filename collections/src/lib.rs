use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Stack<T> {
    items: Vec<T>,
}


#[derive(Debug, Clone)]
pub struct Queue<T> {
    items: VecDeque<T>,
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

        assert_eq!(my_stack.pop(), Some(20));
        assert_eq!(my_stack.pop(), Some(10));
        assert_eq!(my_stack.pop(), None)
    }

    #[test]
    fn test_queue() {
        let mut my_queue: Queue<i32> = Queue::new();

        my_queue.enqueue(10);
        my_queue.enqueue(20);

        println!("My Queue state: {:?}", my_queue);

        assert_eq!(my_queue.dequeue(), Some(10));
        assert_eq!(my_queue.dequeue(), Some(20));
        assert_eq!(my_queue.dequeue(), None);


    }
}
