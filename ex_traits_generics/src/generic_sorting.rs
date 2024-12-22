#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut numbers = vec![64, 34, 25, 12, 22, 11, 90];
        bubble_sort(&mut numbers);
        assert_eq!(numbers, vec![11, 12, 22, 25, 34, 64, 90]);

        let mut words = vec!["apple", "mark", "xavier", "bob"];
        bubble_sort(&mut words);
        assert_eq!(words, vec!["apple", "bob", "mark", "xavier"]);
    }
}

fn bubble_sort<T: Ord>(arr: &mut Vec<T>) {
    let len = arr.len();
    
    for i in 0..len {
        for j in 0..len - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

pub fn selection_sort<T: Ord>(arr: &mut Vec<T>) {

}
