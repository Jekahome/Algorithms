#![allow(unused_assignments)]

pub use algo_interpolation_search::interpolation_search;
mod algo_interpolation_search {
    pub fn interpolation_search(x: usize, arr: &[usize]) -> Option<usize> {
        if arr.len() == 0 {
            return None;
        }
        if arr[0] == x {
            return Some(0);
        }
        let mut low = 0usize;
        let mut high = arr.len() - 1;
        let mut pos = 0usize;

        while low <= high && x >= arr[low] && x <= arr[high] {
            pos = low + (((high - low) / (arr[high] - arr[low])) * (x - arr[low]));

            if arr[pos] == x {
                return Some(pos);
            } else if arr[pos] < x {
                low = pos + 1;
            } else {
                high = low - 1;
            }
        }
        None
    }
}

/// $ cargo test search::interpolation_search
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_interpolation_search() {
        let items = vec![1, 2, 3, 4, 5];
        assert_eq!(Some(0), interpolation_search(1, &items));
        assert_eq!(Some(1), interpolation_search(2, &items));
        assert_eq!(Some(2), interpolation_search(3, &items));
        assert_eq!(Some(3), interpolation_search(4, &items));
        assert_eq!(Some(4), interpolation_search(5, &items));
        assert_eq!(None, interpolation_search(0, &items));
        assert_eq!(None, interpolation_search(90, &items));
        assert_eq!(None, interpolation_search(9000000, &items));

        let items = vec![2, 4, 6, 80, 90, 120, 180, 900, 2000, 4000, 5000, 60000];
        assert_eq!(None, interpolation_search(1, &items));

        assert_eq!(None, interpolation_search(1, &[]));
    }
}
