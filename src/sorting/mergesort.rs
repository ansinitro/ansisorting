pub fn merge_sort<T, F>(arr: &mut [T], compare: &F)
where
    T: PartialOrd + Clone,
    F: Fn(&T, &T) -> bool,
{
    if arr.len() <= 1 {
        return;
    }

    let mid = arr.len() / 2;
    let mut left = arr[..mid].to_vec();
    let mut right = arr[mid..].to_vec();

    merge_sort(&mut left, compare);
    merge_sort(&mut right, compare);

    merge(arr, &left, &right, compare);
}

fn merge<T, F>(arr: &mut [T], left: &[T], right: &[T], compare: &F)
where
    T: PartialOrd + Clone,
    F: Fn(&T, &T) -> bool,
{
    let (mut i, mut j, mut k) = (0, 0, 0);
    while i < left.len() && j < right.len() {
        if compare(&left[i], &right[j]) {
            arr[k] = left[i].clone();
            i += 1;
        } else {
            arr[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i].clone();
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, PartialOrd,Clone)]
    struct Person {
        name: String,
        age: u32,
    }

    #[test]
    fn test_merge_sort_empty() {
        let mut vec: Vec<i32> = vec![];
        merge_sort(&mut vec, &|a, b| a < b);
        assert_eq!(vec, []);
    }

    #[test]
    fn test_merge_sort_single_element() {
        let mut vec = vec![42];
        merge_sort(&mut vec, &|a, b| a < b);
        assert_eq!(vec, [42]);
    }

    #[test]
    fn test_merge_sort_sorted() {
        let mut vec = vec![1, 2, 3, 4, 5];
        merge_sort(&mut vec, &|a, b| a < b);
        assert_eq!(vec, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_merge_sort_reverse_sorted() {
        let mut vec = vec![5, 4, 3, 2, 1];
        merge_sort(&mut vec, &|a, b| a < b);
        assert_eq!(vec, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_merge_sort_random() {
        let mut vec = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        merge_sort(&mut vec, &|a, b| a < b);
        assert_eq!(vec, [1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
    }

    #[test]
    fn test_merge_sort_duplicate_elements() {
        let mut vec = vec![5, 2, 5, 3, 1, 2];
        merge_sort(&mut vec, &|a, b| a < b);
        assert_eq!(vec, [1, 2, 2, 3, 5, 5]);
    }

    #[test]
    fn test_merge_sort_negative_numbers() {
        let mut vec = vec![-5, -2, -4, -1, -3];
        merge_sort(&mut vec, &|a, b| a < b);
        assert_eq!(vec, [-5, -4, -3, -2, -1]);
    }

    #[test]
    fn test_merge_sort_large_input() {
        let mut vec = (1..=1000).rev().collect::<Vec<i32>>();
        merge_sort(&mut vec, &|a, b| a < b);
        assert_eq!(vec, (1..=1000).collect::<Vec<i32>>());
    }

    #[test]
    fn test_merge_sort_sorted_person() {
        let mut vec = vec![
            Person { name: "Alice".to_string(), age: 20 },
            Person { name: "Bob".to_string(), age: 25 },
            Person { name: "Charlie".to_string(), age: 30 },
        ];
        merge_sort(&mut vec, &|a, b| a.age < b.age);
        assert_eq!(vec, vec![
            Person { name: "Alice".to_string(), age: 20 },
            Person { name: "Bob".to_string(), age: 25 },
            Person { name: "Charlie".to_string(), age: 30 },
        ]);
    }

    #[test]
    fn test_merge_sort_random_person() {
        let mut vec = vec![
            Person { name: "Alice".to_string(), age: 30 },
            Person { name: "Bob".to_string(), age: 25 },
            Person { name: "Charlie".to_string(), age: 35 },
            Person { name: "David".to_string(), age: 20 },
        ];
        merge_sort(&mut vec, &|a, b| a.age < b.age);
        assert_eq!(vec, vec![
            Person { name: "David".to_string(), age: 20 },
            Person { name: "Bob".to_string(), age: 25 },
            Person { name: "Alice".to_string(), age: 30 },
            Person { name: "Charlie".to_string(), age: 35 },
        ]);
    }
}

