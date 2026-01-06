const STACK_SIZE: usize = 16;

pub struct Stack {
    stack: [u16; STACK_SIZE],
    pointer: usize, // points *one past top of stack*
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            stack: [0u16; STACK_SIZE],
            pointer: 0,
        }
    }

    pub fn push(&mut self, value: u16) {
        if self.pointer >= STACK_SIZE {
            panic!("Stack overflow");
        }
        self.stack[self.pointer] = value;
        self.pointer += 1;
    }

    pub fn pop(&mut self) -> u16 {
        if self.pointer == 0 {
            panic!("Stack underflow");
        }
        self.pointer -= 1;
        let value = self.stack[self.pointer];
        self.stack[self.pointer] = 0; // optional: clear
        value
    }

    pub fn peek(&self) -> u16 {
        if self.pointer == 0 {
            panic!("Stack empty");
        }
        self.stack[self.pointer - 1]
    }

    pub fn is_empty(&self) -> bool {
        self.pointer == 0
    }

    pub fn is_full(&self) -> bool {
        self.pointer == STACK_SIZE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop_basic() {
        let mut stack = Stack::new();

        // Push one value
        stack.push(0x123);
        assert_eq!(stack.peek(), 0x123);

        // Pop it
        let value = stack.pop();
        assert_eq!(value, 0x123);
        assert!(stack.is_empty());
    }

    #[test]
    fn test_lifo_behavior() {
        let mut stack = Stack::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.pop(), 3);
        assert_eq!(stack.pop(), 2);
        assert_eq!(stack.pop(), 1);
        assert!(stack.is_empty());
    }

    #[test]
    #[should_panic(expected = "Stack underflow")]
    fn test_pop_underflow() {
        let mut stack = Stack::new();
        stack.pop(); // Should panic
    }

    #[test]
    #[should_panic(expected = "Stack overflow")]
    fn test_push_overflow() {
        let mut stack = Stack::new();
        for i in 0..STACK_SIZE {
            stack.push(i as u16);
        }
        // Next push should panic
        stack.push(0xABCD);
    }

    #[test]
    fn test_is_full_and_is_empty() {
        let mut stack = Stack::new();
        assert!(stack.is_empty());
        assert!(!stack.is_full());

        for i in 0..STACK_SIZE {
            stack.push(i as u16);
        }

        assert!(stack.is_full());
        assert!(!stack.is_empty());
    }

    #[test]
    fn test_peek_does_not_pop() {
        let mut stack = Stack::new();
        stack.push(0x55);

        let top = stack.peek();
        assert_eq!(top, 0x55);
        assert_eq!(stack.pop(), 0x55);
        assert!(stack.is_empty());
    }
}
