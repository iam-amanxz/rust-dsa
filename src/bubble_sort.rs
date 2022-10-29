#[allow(dead_code)]
mod bubble_sort {
    fn bubble_sort<T: Ord>(arr: &mut [T]) -> &[T] {
        for i in 0..arr.len() {
            for j in 0..arr.len() - 1 - i {
                if arr[j] > arr[j + 1] {
                    arr.swap(j, j + 1);
                }
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
                bubble_sort(&mut array),
                [-31, 0, 1, 2, 2, 4, 65, 83, 99, 782]
            );
        }

        #[test]
        fn should_sort_strings_alphabetically() {
            let mut array = ["beach", "hotel", "airplane", "car", "house", "art"];
            assert_eq!(
                bubble_sort(&mut array),
                ["airplane", "art", "beach", "car", "hotel", "house"]
            );
        }
    }
}
