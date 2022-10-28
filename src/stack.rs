#[allow(dead_code)]
mod stack {
    use std::fmt::Debug;

    struct Stack<T: PartialOrd + Copy + Debug> {
        top: i32,
        stack: Vec<T>,
        capacity: i32,
    }

    impl<T: PartialOrd + Copy + Debug> Stack<T> {
        fn new(capacity: i32) -> Self {
            return Stack {
                top: -1,
                stack: Vec::new(),
                capacity,
            };
        }

        fn get_stack(&self) -> Vec<T> {
            self.stack.clone()
        }

        fn is_empty(&self) -> bool {
            self.top == -1
        }

        fn is_full(&self) -> bool {
            self.top == self.capacity - 1
        }

        fn peek(&self) -> T {
            *self.stack.last().unwrap()
        }

        fn push(&mut self, item: T) -> bool {
            if self.is_full() {
                println!("Maximum capacity reached");
                return false;
            }
            self.stack.push(item);
            self.top += 1;
            true
        }

        fn pop(&mut self) -> bool {
            if self.is_empty() {
                println!("Stack is already empty");
                return false;
            }
            self.stack.pop();
            self.top -= 1;
            true
        }

        fn print(&self) {
            for (index, item) in self.stack.iter().enumerate() {
                println!("Item at index: {} -> {:?}", index, item)
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn should_be_empty_when_created() {
            let stack: Stack<i32> = Stack::new(10);
            assert_eq!(stack.is_empty(), true);
        }

        #[test]
        fn should_be_full_when_max_capacity_is_reached() {
            let mut stack = Stack::new(2);
            stack.push(4);
            stack.push(123);
            assert_eq!(stack.is_full(), true);
        }

        #[test]
        fn should_add_items_to_the_stack() {
            let mut stack = Stack::new(2);
            stack.push(4);
            stack.push(123);
            assert_eq!(stack.get_stack(), vec![4, 123]);
        }

        #[test]
        fn should_remove_items_from_the_stack() {
            let mut stack = Stack::new(5);
            stack.push(1);
            stack.push(2);
            stack.push(3);
            stack.push(4);
            stack.push(5);
            stack.pop();
            stack.pop();
            assert_eq!(stack.get_stack(), vec![1, 2, 3]);
        }

        #[test]
        fn should_not_allow_adding_more_item_than_the_capacity() {
            let mut stack = Stack::new(1);
            stack.push(1);
            let status = stack.push(2);

            assert_eq!(status, false);
        }

        #[test]
        fn should_not_allow_removing_item_when_the_stack_is_empty() {
            let mut stack: Stack<i32> = Stack::new(1);
            let status = stack.pop();

            assert_eq!(status, false);
        }

        #[test]
        fn should_return_the_top_element() {
            let mut stack = Stack::new(3);
            stack.push(1);
            stack.push(2);
            stack.push(3);
            let element = stack.peek();

            assert_eq!(element, 3);
        }
    }
}
