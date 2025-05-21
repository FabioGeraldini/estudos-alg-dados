pub struct Stack<T> {   
    elements: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {elements: Vec::new() }
    }

    pub fn push(&mut self, value: T) {
        self.elements.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.elements.last()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_operations() {
        let mut stack = Stack::new();
        assert!(stack.is_empty());

        stack.push(10);
        stack.push(20);

        assert_eq!(stack.peek(), Some(&20));

        assert_eq!(stack.pop(), Some(20));
        assert_eq!(stack.pop(), Some(10));

        assert!(stack.is_empty());
    }
}
