pub fn selection_sort<T, F>(arr: &mut [T], compare: &F)
where
    T: PartialOrd,
    F: Fn(&T, &T) -> bool,
{
    let len = arr.len();
    for i in 0..len {
        let mut min_idx = i;
        for j in i + 1..len {
            if compare(&arr[j], &arr[min_idx]) {
                min_idx = j;
            }
        }
        arr.swap(min_idx, i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_sort_empty() {
        let mut vec: Vec<i32> = vec![];
        selection_sort(&mut vec, &|a, b| a < b);
        assert_eq!(vec, []);
    }

    #[test]
    fn test_selection_sort_single_element() {
        let mut vec = vec![42];
        selection_sort(&mut vec, &|a, b| a < b);
        assert_eq!(vec, [42]);
    }

    #[test]
    fn test_selection_sort_sorted() {
        let mut vec = vec![1, 2, 3, 4, 5];
        selection_sort(&mut vec, &|a, b| a < b);
        assert_eq!(vec, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_selection_sort_reverse_sorted() {
        let mut vec = vec![5, 4, 3, 2, 1];
        selection_sort(&mut vec, &|a, b| a < b);
        assert_eq!(vec, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_selection_sort_random() {
        let mut vec = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        selection_sort(&mut vec, &|a, b| a < b);
        assert_eq!(vec, [1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
    }

    #[test]
    fn test_selection_sort_duplicate_elements() {
        let mut vec = vec![5, 2, 5, 3, 1, 2];
        selection_sort(&mut vec, &|a, b| a < b);
        assert_eq!(vec, [1, 2, 2, 3, 5, 5]);
    }

    #[test]
    fn test_selection_sort_negative_numbers() {
        let mut vec = vec![-5, -2, -4, -1, -3];
        selection_sort(&mut vec, &|a, b| a < b);
        assert_eq!(vec, [-5, -4, -3, -2, -1]);
    }

    #[test]
    fn test_selection_sort_large_input() {
        let mut vec = (1..=1000).rev().collect::<Vec<i32>>();
        selection_sort(&mut vec, &|a, b| a < b);
        assert_eq!(vec, (1..=1000).collect::<Vec<i32>>());
    }

    #[derive(Debug, PartialEq, PartialOrd)]
    struct Person {
        name: String,
        age: u32,
    }

    #[test]
    fn test_selection_sort_sorted_person() {
        let mut vec = vec![
            Person {
                name: "Alice".to_string(),
                age: 20,
            },
            Person {
                name: "Bob".to_string(),
                age: 25,
            },
            Person {
                name: "Charlie".to_string(),
                age: 30,
            },
        ];
        selection_sort(&mut vec, &|a, b| a.age < b.age);
        assert_eq!(
            vec,
            vec![
                Person {
                    name: "Alice".to_string(),
                    age: 20
                },
                Person {
                    name: "Bob".to_string(),
                    age: 25
                },
                Person {
                    name: "Charlie".to_string(),
                    age: 30
                },
            ]
        );
    }

    #[test]
    fn test_selection_sort_random_person() {
        let mut vec = vec![
            Person {
                name: "Alice".to_string(),
                age: 30,
            },
            Person {
                name: "Bob".to_string(),
                age: 25,
            },
            Person {
                name: "Charlie".to_string(),
                age: 35,
            },
            Person {
                name: "David".to_string(),
                age: 20,
            },
        ];
        selection_sort(&mut vec, &|a, b| a.age < b.age);
        assert_eq!(
            vec,
            vec![
                Person {
                    name: "David".to_string(),
                    age: 20
                },
                Person {
                    name: "Bob".to_string(),
                    age: 25
                },
                Person {
                    name: "Alice".to_string(),
                    age: 30
                },
                Person {
                    name: "Charlie".to_string(),
                    age: 35
                },
            ]
        );
    }
}
