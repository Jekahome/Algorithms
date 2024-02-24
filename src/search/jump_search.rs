pub use algo_jump_search::jump_search;
mod algo_jump_search {

    pub fn jump_search<T>(x: &T, arr: &[T]) -> Option<usize>
    where
        T: PartialOrd,
    {
        let n = arr.len();
        if n == 0 {
            return None;
        }
        if &arr[0] == x {
            return Some(0);
        }
        let mut step = (n as f64).sqrt().floor() as usize;
        if n <= 10 {
            step = 1;
        }
        let mut prev = 0;
        let mut i = 0;

        while i < n && &arr[i] < x {
            prev = i;
            i += step;
        }

        if i >= n {
            return None;
        }

        loop {
            prev += 1;
            if &arr[prev] < x && prev < i {
                prev += 1;
            } else {
                break;
            }
        }
        if &arr[prev] == x {
            return Some(prev);
        }
        None
    }
}

/// $ cargo test search::jump_search
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jump_search() {
        let items = vec![1, 2, 3, 4, 5];
        assert_eq!(Some(0), jump_search(&1, &items));
        assert_eq!(Some(1), jump_search(&2, &items));
        assert_eq!(Some(2), jump_search(&3, &items));
        assert_eq!(Some(3), jump_search(&4, &items));
        assert_eq!(Some(4), jump_search(&5, &items));
        assert_eq!(None, jump_search(&0, &items));
        assert_eq!(None, jump_search(&90, &items));
        assert_eq!(None, jump_search(&9000000, &items));

        let items = vec![2, 4, 6, 80, 90, 120, 180, 900, 2000, 4000, 5000, 60000];
        assert_eq!(None, jump_search(&1, &items));

        assert_eq!(None, jump_search(&1, &[]));
    }
}
