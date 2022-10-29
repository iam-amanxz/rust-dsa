#[allow(dead_code)]
mod insertion_sort {
    fn insertion_sort<T: Ord>(arr: &mut [T]) -> &[T] {
        for i in 1..arr.len() {
            let mut j = i;
            while j > 0 && arr[j] < arr[j - 1] {
                arr.swap(j, j - 1);
                j = j - 1;
            }
        }
        arr
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn should_sort_numbers_ascending() {
            let mut array = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
            assert_eq!(
                insertion_sort(&mut array),
                [-31, 0, 1, 2, 2, 4, 65, 83, 99, 782]
            );
        }

        #[test]
        fn should_sort_strings_alphabetically() {
            let mut array = ["beach", "hotel", "airplane", "car", "house", "art"];
            assert_eq!(
                insertion_sort(&mut array),
                ["airplane", "art", "beach", "car", "hotel", "house"]
            );
        }
    }
}
