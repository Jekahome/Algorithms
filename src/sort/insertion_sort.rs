pub use ds_insertion_sort::{insertion_sort,binary_insertion_sort};
/// Insertion sort.
mod ds_insertion_sort{
    pub fn insertion_sort(arr: &mut [i32]) {
        for i in 1..arr.len() {
            let mut j = i;
            while j > 0 && arr[j - 1] > arr[j] {
                arr.swap(j - 1, j);
                j -= 1;
            }
        }
    }

    /// Binary insertion sort.
    /// Двоичная сортировка вставками — это вариант сортировки вставками, в котором используется двоичный 
    /// поиск по сокращению сравнений при обычной сортировке вставками.
    pub fn binary_insertion_sort(arr: &mut [i32]) {
        for i in 1..arr.len() {
            let val = arr[i];
            let mut j = i;
            let pos = arr[..i].binary_search(&val).unwrap_or_else(|pos| pos);
            // Swap all elements until specific position.
            while j > pos {
                arr.swap(j - 1, j);
                j -= 1;
            }
        }
    }
}

/// $ cargo +nightly test --lib algorithms::insertion_sort::test 
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_insertion_sort(){
        let mut items = vec![1, 8, 2, 4, 5];
        insertion_sort(&mut items);
        assert_eq!(vec![1,2,4,5,8],items);
    }
    #[test]
    fn test_binary_insertion_sort(){
        let mut items = vec![1, 8, 2, 4, 5];
        binary_insertion_sort(&mut items);
        assert_eq!(vec![1,2,4,5,8],items);
    }
}