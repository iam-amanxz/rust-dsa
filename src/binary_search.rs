// only works with sorted array

#[allow(dead_code)]
mod binary_search {
    fn search(array: &[i32], target: i32) -> isize {
        let mut left_index = 0 as usize;
        let mut right_index = array.len() - 1;

        while left_index <= right_index {
            let mid_index = (((left_index + right_index) / 2) as f32).floor() as usize;

            if target == array[mid_index] {
                return mid_index as isize;
            }

            if target < array[mid_index] {
                right_index = mid_index - 1;
            }

            if target > array[mid_index] {
                left_index = mid_index + 1;
            }
        }

        -1 as isize
    }

    fn search_recursive_helper(array: &[i32], target: i32, left_index: usize, right_index: usize) -> isize {
        if left_index > right_index {
            return -1 as isize;
        }

        let mid_index = (((left_index + right_index) / 2) as f32).floor() as usize;

        if target == array[mid_index] {
            return mid_index as isize;
        }

        if target < array[mid_index] {
            return search_recursive_helper(array, target, left_index, mid_index - 1);
        }

        search_recursive_helper(array, target, mid_index + 1, right_index)
    }

    fn search_recursive(array: &[i32], target: i32) -> isize {
        search_recursive_helper(&array, target, 0 as usize, array.len() - 1)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn should_return_index() {
            let array = [-5, 2, 4, 6, 10];
            assert_eq!(search(&array, 10), 4);
        }

        #[test]
        fn should_return_minus_one() {
            let array = [-5, 2, 4, 6, 10];
            assert_eq!(search(&array, 20), -1);
        }

        #[test]
        fn recursion_should_return_index() {
            let array = [-5, 2, 4, 6, 10];
            assert_eq!(search_recursive(&array, 10), 4);
        }

        #[test]
        fn recursion_should_return_minus_one() {
            let array = [-5, 2, 4, 6, 10];
            assert_eq!(search_recursive(&array, 20), -1);
        }
    }
}
