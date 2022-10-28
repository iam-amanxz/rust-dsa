#[allow(dead_code)]
mod queue {
    use std::fmt::Debug;

    struct Queue<T: PartialOrd + Copy + Debug> {
        capacity: i32,
        queue: Vec<T>,
    }

    impl<T: PartialOrd + Copy + Debug> Queue<T> {
        fn new(capacity: i32) -> Self {
            return Queue {
                capacity,
                queue: Vec::new(),
            };
        }

        fn get_queue(&self) -> Vec<T> {
            self.queue.clone()
        }

        fn get_size(&self) -> usize {
            self.queue.len()
        }

        fn is_empty(&self) -> bool {
            self.queue.is_empty()
        }

        fn is_full(&self) -> bool {
            self.queue.len() == self.capacity as usize
        }

        fn peek(&self) -> T {
            *self.queue.first().unwrap()
        }

        fn enqueue(&mut self, item: T) -> bool {
            if self.is_full() {
                println!("Maximum capacity reached");
                return false;
            }
            self.queue.push(item);
            true
        }

        fn dequeue(&mut self) -> bool {
            if self.is_empty() {
                println!("Queue is already empty");
                return false;
            }
            self.queue.remove(0);
            true
        }

        fn print(&self) {
            for (index, item) in self.queue.iter().enumerate() {
                println!("Item at index: {} -> {:?}", index, item)
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn should_be_empty_when_created() {
            let queue: Queue<i32> = Queue::new(10);
            assert_eq!(queue.is_empty(), true);
        }

        #[test]
        fn should_be_full_when_max_capacity_is_reached() {
            let mut queue = Queue::new(2);
            queue.enqueue(1);
            queue.enqueue(2);
            assert_eq!(queue.is_full(), true);
        }

        #[test]
        fn should_add_items_to_the_queue() {
            let mut queue = Queue::new(2);
            queue.enqueue(4);
            queue.enqueue(123);
            assert_eq!(queue.get_queue(), vec![4, 123]);
        }

        #[test]
        fn should_remove_items_from_the_queue() {
            let mut queue = Queue::new(5);
            queue.enqueue(1);
            queue.enqueue(2);
            queue.enqueue(3);
            queue.enqueue(4);
            queue.enqueue(5);
            queue.dequeue();
            queue.dequeue();            queue.print();
            assert_eq!(queue.get_queue(), vec![3, 4, 5]);
        }

        #[test]
        fn should_not_allow_adding_more_item_than_the_capacity() {
            let mut queue = Queue::new(1);
            queue.enqueue(1);
            let status = queue.enqueue(2);

            assert_eq!(status, false);
        }

        #[test]
        fn should_not_allow_removing_item_when_the_queue_is_empty() {
            let mut queue: Queue<i32> = Queue::new(1);
            let status = queue.dequeue();

            assert_eq!(status, false);
        }

        #[test]
        fn should_return_the_top_element() {
            let mut queue = Queue::new(3);
            queue.enqueue(1);
            queue.enqueue(2);
            queue.enqueue(3);
            let element = queue.peek();

            assert_eq!(element, 1);
        }
    }
}
