pub struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl MinStack {
    pub fn new() -> Self {
        MinStack {
            stack: vec![],
            min_stack: vec![],
        }
    }

    pub fn push(&mut self, val: i32) {
        self.stack.push(val);
        match self.min_stack.last() {
            Some(x) => {
                if val <= *x {
                    self.min_stack.push(val)
                }
            }
            None => self.min_stack.push(val),
        }
    }

    pub fn pop(&mut self) {
        if let Some(stack_top) = self.stack.pop() {
            if let Some(&min_stack_top) = self.min_stack.last() {
                if stack_top == min_stack_top {
                    self.min_stack.pop();
                }
            }
        }
    }

    pub fn top(&self) -> i32 {
        self.stack.last().copied().unwrap_or(0)
    }

    pub fn get_min(&self) -> i32 {
        self.min_stack.last().copied().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut ms = MinStack::new();
        ms.push(-2);
        ms.push(0);
        ms.push(-3);
        assert_eq!(ms.get_min(), -3);
        ms.pop();
        assert_eq!(ms.top(), 0);
        assert_eq!(ms.get_min(), -2);
    }
}
