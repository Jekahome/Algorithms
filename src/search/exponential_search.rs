pub use algo_exponential_search::exponential_search;
mod algo_exponential_search {
    // Найдите диапазон, в котором присутствует элемент
    // Выполните двоичный поиск в найденном выше диапазоне.
    pub fn exponential_search<T>(x: T, arr: &[T]) -> Option<usize>
    where
        T: PartialOrd,
    {
        let n = arr.len();
        if n == 0 {
            return None;
        }
        if arr[0] == x {
            return Some(0);
        }
        // Найти диапазон для бинарного поиска по повторное удвоение
        let mut i = 1usize;
        while i < n && arr[i] <= x {
            i = i * 2;
        }

        // Вызов бинарного поиска для найденного диапазона.
        fn binary_search<T>(arr: &[T], left: usize, right: usize, x: T, n: usize) -> Option<usize>
        where
            T: PartialOrd,
        {
            if right >= left {
                let mid = left + (right - left) / 2;
                if mid < n && arr[mid] == x {
                    return Some(mid);
                }
                if mid > 0 && mid < n && arr[mid] > x {
                    return binary_search(arr, left, mid - 1, x, n);
                } else {
                    return binary_search(arr, mid + 1, right, x, n);
                }
            }
            return None;
        }
        let to_index = if i < n { i } else { n };
        binary_search(arr, i / 2, to_index, x, n)
    }
}

/// $ cargo test exponential_search
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_exponential_search() {
        let items = vec![1, 2, 3, 4, 5];
        assert_eq!(Some(0), exponential_search(1, &items));
        assert_eq!(Some(1), exponential_search(2, &items));
        assert_eq!(Some(2), exponential_search(3, &items));
        assert_eq!(Some(3), exponential_search(4, &items));
        assert_eq!(Some(4), exponential_search(5, &items));
        assert_eq!(None, exponential_search(0, &items));
        assert_eq!(None, exponential_search(90, &items));
        assert_eq!(None, exponential_search(9000000, &items));

        let items = vec![2, 4, 6, 80, 90, 120, 180, 900, 2000, 4000, 5000, 60000];
        assert_eq!(None, exponential_search(1, &items));

        assert_eq!(None, exponential_search(1, &[]));
    }
}
