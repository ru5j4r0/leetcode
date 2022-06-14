/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */

struct MyQueue {
    push_stack: Vec<i32>,
    pop_stack: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        MyQueue {
            push_stack: Vec::new(),
            pop_stack: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.push_mode();
        self.push_stack.push(x);
    }

    fn pop(&mut self) -> i32 {
        self.pop_mode();
        self.pop_stack.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        self.pop_mode();
        *self.pop_stack.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.push_stack.is_empty() && self.pop_stack.is_empty()
    }

    fn switch_mode(from: &mut Vec<i32>, to: &mut Vec<i32>) {
        while let Some(e) = from.pop() {
            to.push(e);
        }
    }

    fn push_mode(&mut self) {
        Self::switch_mode(&mut self.pop_stack, &mut self.push_stack);
    }

    fn pop_mode(&mut self) {
        Self::switch_mode(&mut self.push_stack, &mut self.pop_stack);
    }
}
