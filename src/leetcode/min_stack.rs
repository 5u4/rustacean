//! Design a stack that supports push, pop, top, and retrieving the minimum element in constant time.
//!
//! Implement the MinStack class:
//!
//! MinStack() initializes the stack object.
//! void push(int val) pushes the element val onto the stack.
//! void pop() removes the element on the top of the stack.
//! int top() gets the top element of the stack.
//! int getMin() retrieves the minimum element in the stack.
//!
//! Constraints:
//!
//! -2^31 <= val <= 2^31 - 1
//! Methods pop, top and getMin operations will always be called on non-empty stacks.
//! At most 3 * 104 calls will be made to push, pop, top, and getMin.
#[allow(dead_code)]
#[derive(Default)]
struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl MinStack {
    fn new() -> Self {
        Self::default()
    }

    fn push(&mut self, val: i32) {
        match self.min_stack.last() {
            Some(&min) if min < val => {}
            _ => {
                self.min_stack.push(val);
            }
        }
        self.stack.push(val);
    }

    fn pop(&mut self) {
        let val = self.stack.pop().unwrap();
        match self.min_stack.last() {
            Some(&min) if min < val => {}
            _ => {
                self.min_stack.pop();
            }
        }
    }

    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.min_stack.last().unwrap()
    }
}

// Your MinStack object will be instantiated and called as such:
// let obj = MinStack::new();
// obj.push(val);
// obj.pop();
// let ret_3: i32 = obj.top();
// let ret_4: i32 = obj.get_min();

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
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
