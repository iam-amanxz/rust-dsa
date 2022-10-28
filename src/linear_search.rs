// Linear Search is defined as a sequential search algorithm that starts at one end and goes through
// each element of a list until the desired element is found, otherwise the search continues
// till the end of the data set. It is the easiest searching algorithm

// Input: arr[] = {10, 20, 80, 30, 60, 50,110, 100, 130, 170}, x = 110;
// Output: 6
// Explanation: Element x is present at index 6

// Input: arr[] = {10, 20, 80, 30, 60, 50,110, 100, 130, 170}, x = 175;
// Output: -1
// Explanation: Element x is not present in arr[].

#[allow(dead_code)]
mod linear_search {
    fn search(array: &[i32], item: i32) -> isize {
        if array.is_empty() {
            return -1 as isize;
        }

        for (index, el) in array.iter().enumerate() {
            if *el == item {
                return index as isize;
            }
        }

        -1 as isize
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn should_return_index() {
            let array = [10, 20, 80, 30, 60, 50, 110, 100, 130, 170];
            assert_eq!(search(&array, 110), 6);
        }

        #[test]
        fn should_return_minus_one() {
            let array = [10, 20, 80, 30, 60, 50, 110, 100, 130, 170];
            assert_eq!(search(&array, -20), -1);
        }
    }
}
