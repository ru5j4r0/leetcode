/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */

#[allow(dead_code)]
struct MyQueue {
    push_stack: Vec<i32>,
    pop_stack: Vec<i32>,
}

impl MyQueue {
    #[allow(dead_code)]
    fn new() -> Self {
        MyQueue {
            push_stack: Vec::new(),
            pop_stack: Vec::new(),
        }
    }

    #[allow(dead_code)]
    fn push(&mut self, x: i32) {
        self.push_mode();
        self.push_stack.push(x);
    }

    #[allow(dead_code)]
    fn pop(&mut self) -> i32 {
        self.pop_mode();
        self.pop_stack.pop().unwrap()
    }

    #[allow(dead_code)]
    fn peek(&mut self) -> i32 {
        self.pop_mode();
        *self.pop_stack.last().unwrap()
    }

    #[allow(dead_code)]
    fn empty(&self) -> bool {
        self.push_stack.is_empty() && self.pop_stack.is_empty()
    }

    #[allow(dead_code)]
    fn switch_mode(from: &mut Vec<i32>, to: &mut Vec<i32>) {
        while let Some(e) = from.pop() {
            to.push(e);
        }
    }

    #[allow(dead_code)]
    fn push_mode(&mut self) {
        Self::switch_mode(&mut self.pop_stack, &mut self.push_stack);
    }

    #[allow(dead_code)]
    fn pop_mode(&mut self) {
        Self::switch_mode(&mut self.push_stack, &mut self.pop_stack);
    }
}
