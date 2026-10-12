pub struct MinStack {
    min_stack: Vec<i32>,
    stack : Vec<i32>,
}

impl MinStack {
    pub fn new() -> Self {
        Self {
            min_stack : Vec::new(), 
            stack : Vec::new()
        }
    }

    pub fn push(&mut self, val: i32) {
        if self.stack.is_empty() {
            self.min_stack.push(val);
        } else {
            self.min_stack.push(*self.min_stack.last().unwrap().min(&val));
        }
        self.stack.push(val);
    }

    pub fn pop(&mut self) {
        self.stack.pop();
        self.min_stack.pop();
    }

    pub fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    pub fn get_min(&self) -> i32 {
        *self.min_stack.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::MinStack;

    #[test]
    fn basic_case() {
        let mut stack = MinStack::new();
        stack.push(-2);
        stack.push(0);
        stack.push(-3);

        assert_eq!(stack.get_min(), -3);

        stack.pop();

        assert_eq!(stack.top(), 0);
        assert_eq!(stack.get_min(), -2);
    }

    #[test]
    fn duplicate_minimums() {
        let mut stack = MinStack::new();
        stack.push(2);
        stack.push(1);
        stack.push(1);

        assert_eq!(stack.get_min(), 1);

        stack.pop();
        assert_eq!(stack.get_min(), 1);

        stack.pop();
        assert_eq!(stack.get_min(), 2);
    }

    #[test]
    fn minimum_changes_after_pop() {
        let mut stack = MinStack::new();
        stack.push(5);
        stack.push(3);
        stack.push(4);

        assert_eq!(stack.get_min(), 3);

        stack.pop();
        assert_eq!(stack.get_min(), 3);

        stack.pop();
        assert_eq!(stack.get_min(), 5);
    }

    #[test]
    fn negative_values() {
        let mut stack = MinStack::new();
        stack.push(-10);
        stack.push(-20);
        stack.push(-5);

        assert_eq!(stack.top(), -5);
        assert_eq!(stack.get_min(), -20);
    }
}
