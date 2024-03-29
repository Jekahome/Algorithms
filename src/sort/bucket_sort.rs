pub use ds_bucket_sort::bucket_sort;

/// Bucket sort Ковшовая сортировка
/// Также известна как bin sort
mod ds_bucket_sort {
    /// * `arr` - Collection of value to be sorted in place.
    /// * `hasher` - Function hashing to map elements to correspoding buckets.
    /// Ref: https://codereview.stackexchange.com/a/145124
    pub fn bucket_sort<H, F, T>(arr: &mut [T], hasher: F)
    where
        H: Ord,
        F: Fn(&T) -> H,
        T: Ord + Clone,
    {
        // 1. Create buckets.
        let mut buckets: Vec<Bucket<H, T>> = Vec::new();

        // 2. Iterate all elements.
        for value in arr.iter() {
            // 2.1 Create hasher mapping to certain bucket.
            let hash = hasher(&value);

            // 2.2 Search if the bucket with same hash exists.
            let value = value.clone();
            match buckets.binary_search_by(|bucket| bucket.hash.cmp(&hash)) {
                // If exists, push the value to the bucket.
                Ok(index) => buckets[index].values.push(value),
                // If none, create and new bucket and insert value in.
                Err(index) => buckets.insert(index, Bucket::new(hash, value)),
            }
        }

        // 3. Iterate all buckets and flatten their internal collections.
        let ret = buckets
            .into_iter()
            .flat_map(|mut bucket| {
                bucket.values.sort(); // We use built-in sorting here.
                bucket.values
            })
            .collect::<Vec<T>>();

        // 4. Clone back to original array.
        arr.clone_from_slice(&ret);
    }

    /// Bucket to store elements.
    struct Bucket<H, T> {
        hash: H,
        values: Vec<T>,
    }

    impl<H, T> Bucket<H, T> {
        /// Create a new bucket and insert its first value.
        ///
        /// * `hash` - Hash value generated by hasher param of `bucket_sort`.
        /// * `value` - Value to be put in the bucket.
        pub fn new(hash: H, value: T) -> Bucket<H, T> {
            Bucket {
                hash,
                values: vec![value],
            }
        }
    }
}

/// $ cargo test bucket_sort
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bucket_sort() {
        let mut items = vec![1, 8, 2, 4, 5];
        let f = |int: &i32| int / 4;
        bucket_sort(&mut items, f);
        assert_eq!(vec![1, 2, 4, 5, 8], items);

        let mut items = vec![(1, 8), (2, 4), (5, 2), (1, 1)];
        let f = |t: &(i32, i32)| t.0 / 4;
        bucket_sort(&mut items, f);
        assert_eq!(vec![(1, 1), (1, 8), (2, 4), (5, 2)], items);
    }
}
