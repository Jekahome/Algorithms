
///
/// Бинарный поиск — это эффективный алгоритм поиска элемента в отсортированном списке. 
/// Он работает путем многократного деления пополам искомой части массива, пока не будет найдено искомое значение.
/// 

pub use alg_binary_search::{binary_search,binary_search2};
mod alg_binary_search{
    use std::cmp::Ordering;

    ///
    /// Условие работы алгоритма - массив должен быть отсортирован по возрастанию
    /// Временная сложность алгоритма - логарифмическая O(log N)
    /// каждая итерация сокращает вдвое количество элементов/значение
    ///
    pub fn binary_search(k: i32, items: &[i32]) -> Option<usize> {
        let mut low: usize = 0;
        let mut high: usize = items.len();

        while low < high {
            let middle = (high + low) / 2;
            match items[middle].cmp(&k) {
                Ordering::Equal => return Some(middle),
                Ordering::Greater => high = middle,
                Ordering::Less => low = middle + 1,
            }
        }
        None
    }

    pub fn binary_search2<T>(target: &T, arr: &[T]) -> Result<usize, usize>
        where
            T: PartialOrd,
    {
        let mut size = arr.len();
        if size == 0 {
            return Err(0);
        }
        let mut base = 0_usize;

        while size > 1 {
            // mid: [base..size)
            let half = size / 2;
            let mid = base + half;
            if arr[mid] <= *target {
                base = mid
            }
            size -= half;
        }

        if arr[base] == *target {
            Ok(base)
        } else {
            // Return the expected position in the array.
            Err(base + (arr[base] < *target) as usize)
        }
    }
}

/// $ cargo  +nightly test --lib algorithms::binary_search::test 
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_binary_search() {
        let items = vec![1, 2, 3, 4, 5];
        assert_eq!(Some(0), binary_search(1, &items));
        assert_eq!(Some(1), binary_search(2, &items));
        assert_eq!(Some(2), binary_search(3, &items));
        assert_eq!(Some(3), binary_search(4, &items));
        assert_eq!(Some(4), binary_search(5, &items));
        assert_eq!(None, binary_search(0, &items));
        assert_eq!(None, binary_search(90, &items));
        assert_eq!(None, binary_search(9000000, &items));

        let items = vec![2, 4, 6, 80, 90, 120, 180, 900, 2000, 4000, 5000, 60000];
        assert_eq!(None, binary_search(1, &items));

        assert_eq!(None, binary_search(1, &[]));
    }
    
    #[test]
    fn test_binary_search2() {
        let items = vec![1, 2, 3, 4, 5];
        assert_eq!(Ok(0), binary_search2(&1, &items));
        assert_eq!(Ok(1), binary_search2(&2, &items));
        assert_eq!(Ok(2), binary_search2(&3, &items));
        assert_eq!(Ok(3), binary_search2(&4, &items));
        assert_eq!(Ok(4), binary_search2(&5, &items));
        assert_eq!(Err(0), binary_search2(&0, &items));
        assert_eq!(Err(5), binary_search2(&90, &items));
        assert_eq!(Err(5), binary_search2(&9000000, &items));

        let items = vec![2, 4, 6, 80, 90, 120, 180, 900, 2000, 4000, 5000, 60000];
        assert_eq!(Err(0), binary_search2(&1, &items));

        assert_eq!(Err(0), binary_search2(&1, &[]));
    }
}