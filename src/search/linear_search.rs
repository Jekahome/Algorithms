pub use ds_linear_search::linear_search;
/// Последовательный поиск `O(N)`
mod ds_linear_search{
    pub fn linear_search<T>(target: &T, arr: &[T]) -> Option<usize>
        where
            T: PartialEq,
    {
        for (index, item) in arr.iter().enumerate() {
            if item == target {
                return Some(index);
            }
        }
        None
    }
}

/// $ cargo +nightly test --lib algorithms::linear_search
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_search() {
        let items = vec![1, 2, 3, 4, 5];
        assert_eq!(Some(0), linear_search(&1, &items));
        assert_eq!(Some(1), linear_search(&2, &items));
        assert_eq!(Some(2), linear_search(&3, &items));
        assert_eq!(Some(3), linear_search(&4, &items));
        assert_eq!(Some(4), linear_search(&5, &items));
        assert_eq!(None, linear_search(&0, &items));
        assert_eq!(None, linear_search(&90, &items));
        assert_eq!(None, linear_search(&9000000, &items));

        let items = vec![2, 4, 6, 80, 90, 120, 180, 900, 2000, 4000, 5000, 60000];
        assert_eq!(None, linear_search(&1, &items));

        assert_eq!(None, linear_search(&1, &[]));
    }
}