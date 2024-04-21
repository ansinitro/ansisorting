pub fn quick_sort<T, F>(arr: &mut [T], compare: &F)
where
    T: PartialOrd,
    F: Fn(&T, &T) -> bool,
{
    if arr.len() > 1 {
        let pivot_index = partition(arr, compare);
        quick_sort(&mut arr[0..pivot_index], compare);
        quick_sort(&mut arr[pivot_index + 1..], compare);
    }
}

fn partition<T, F>(arr: &mut [T], compare: &F) -> usize
where
    T: PartialOrd,
    F: Fn(&T, &T) -> bool,
{
    let pivot_index = arr.len() / 2;
    arr.swap(pivot_index, arr.len() - 1);
    let mut i = 0;
    for j in 0..arr.len() - 1 {
        if compare(&arr[j], &arr[arr.len() - 1]) {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, arr.len() - 1);
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort_empty() {
        let mut vec: Vec<i32> = vec![];
        quick_sort(&mut vec, &|a, b| a < b);
        assert_eq!(vec, []);
    }

    #[test]
    fn test_quick_sort_single_element() {
        let mut vec = vec![42];
        quick_sort(&mut vec, &|a, b| a < b);
        assert_eq!(vec, [42]);
    }

    #[test]
    fn test_quick_sort_sorted() {
        let mut vec = vec![1, 2, 3, 4, 5];
        quick_sort(&mut vec, &|a, b| a < b);
        assert_eq!(vec, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_quick_sort_reverse_sorted() {
        let mut vec = vec![5, 4, 3, 2, 1];
        quick_sort(&mut vec, &|a, b| a < b);
        assert_eq!(vec, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_quick_sort_random() {
        let mut vec = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        quick_sort(&mut vec, &|a, b| a < b);
        assert_eq!(vec, [1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
    }

    #[test]
    fn test_quick_sort_duplicate_elements() {
        let mut vec = vec![5, 2, 5, 3, 1, 2];
        quick_sort(&mut vec, &|a, b| a < b);
        assert_eq!(vec, [1, 2, 2, 3, 5, 5]);
    }

    #[test]
    fn test_quick_sort_negative_numbers() {
        let mut vec = vec![-5, -2, -4, -1, -3];
        quick_sort(&mut vec, &|a, b| a < b);
        assert_eq!(vec, [-5, -4, -3, -2, -1]);
    }

    #[test]
    fn test_quick_sort_large_input() {
        let mut vec = (1..=1000).rev().collect::<Vec<i32>>();
        quick_sort(&mut vec, &|a, b| a < b);
        assert_eq!(vec, (1..=1000).collect::<Vec<i32>>());
    }

    #[derive(Debug, PartialEq, PartialOrd)]
    struct Person {
        name: String,
        age: u32,
    }

    #[test]
    fn test_quick_sort_sorted_person() {
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
        quick_sort(&mut vec, &|a, b| a.age < b.age);
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
    fn test_quick_sort_random_person() {
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
        quick_sort(&mut vec, &|a, b| a.age < b.age);
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
