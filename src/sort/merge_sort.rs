pub use ds_merge_sort::{merge_sort, merge_sort_bottom_up};

///
/// Алгоритм Merge sort (сортировки слиянием) — это алгоритм «разделяй и властвуй»,
/// который делит массив на две части, сортирует две половины, а затем снова объединяет их.
///
/// Сортировка слиянием имеет гораздо меньший постоянный коэффициент, чем пирамидальная сортировка,
/// но требует буферного пространства O(n) для хранения промежуточных данных, что очень дорого.
/// Его основным преимуществом является то, что он стабилен, по сравнению с пирамидальной сортировкой,
/// которая таковой не является. Кроме того, его реализация легко распараллеливается.
///
///
/// Как работает Merge sort?
///
/// Сортировка слиянием рекурсивно делит заданный массив вдвое.
/// Как только подмассивы достигают тривиальной длины, начинается слияние.
/// При слиянии берется наименьший элемент между двумя соседними подмассивами и повторяется этот шаг до тех пор,
/// пока не будут взяты все элементы, в результате чего получается отсортированный подмассив.
/// Процесс повторяется для пар соседних подмассивов, пока мы не доберемся до начального массива, но уже отсортированного.
///

/// Сортировка слиянием
/// Принцип работы: разбивать последовательность пополам пока не останутся не делимые последовательности
/// Сортировать с последних последовательностей и сливать их между собой. Сложность O(N log N)
mod ds_merge_sort {
    /// - Top-down
    /// - Recursive
    pub fn merge_sort(arr: &mut [i32]) {
        let mid = arr.len() / 2;
        if mid == 0 {
            return;
        }

        merge_sort(&mut arr[..mid]);
        merge_sort(&mut arr[mid..]);

        // Create an array to store intermediate result.
        let mut ret = arr.to_vec();

        // Merge the two piles.
        merge(&arr[..mid], &arr[mid..], &mut ret[..]);

        // Copy back the result back to original array.
        arr.copy_from_slice(&ret);
    }

    /// Mergesort bottom-up version.
    /// - Buttom-up (for array-based data structure)
    /// - Iterative
    pub fn merge_sort_bottom_up(arr: &mut [i32]) {
        let mut width = 1;
        // Create an array to store intermediate result.
        let mut ret = arr.to_vec();
        let len = arr.len();

        while width < len {
            let mut i = 0;
            while i < len {
                // Check to avoid upper bound and middle index out of bound.
                let upper = ::std::cmp::min(i + 2 * width, len);
                let mid = ::std::cmp::min(i + width, len);

                merge(&arr[i..mid], &arr[mid..upper], &mut ret[i..upper]);

                // Copy the merged result back to original array.
                arr[i..upper].copy_from_slice(&ret[i..upper]);

                // Increase start index to merge next two subsequences.
                i += 2 * width;
            }
            width *= 2;
        }
    }

    /// Merge helper.
    /// * `arr1` - Left pile to sort.
    /// * `arr2` - Right pile to sort.
    /// * `ret` - Result array to return
    fn merge(arr1: &[i32], arr2: &[i32], ret: &mut [i32]) {
        let mut left = 0; // Head of left pile.
        let mut right = 0; // Head of right pile.
        let mut index = 0;

        // Compare element and insert back to result array.
        while left < arr1.len() && right < arr2.len() {
            if arr1[left] <= arr2[right] {
                ret[index] = arr1[left];
                index += 1;
                left += 1;
            } else {
                ret[index] = arr2[right];
                index += 1;
                right += 1;
            }
        }

        // Copy the reset elements to returned array.
        // `memcpy` may be more performant than for-loop assignment.
        if left < arr1.len() {
            ret[index..].copy_from_slice(&arr1[left..]);
        }
        if right < arr2.len() {
            ret[index..].copy_from_slice(&arr2[right..]);
        }
    }
}

/// $ cargo +nightly test --lib algorithms::merge_sort::test
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_merge_sort() {
        let mut items = vec![1, 18, 2, 4, 5];
        merge_sort(&mut items);
        assert_eq!(vec![1, 2, 4, 5, 18], items);
    }

    #[test]
    fn test_mergesort_bottom_up() {
        let mut items = vec![1, 18, 2, 4, 5];
        merge_sort_bottom_up(&mut items);
        assert_eq!(vec![1, 2, 4, 5, 18], items);
    }
}
