pub use ds_radix_sort::radix_sort;
/// https://rosettacode.org/wiki/Sorting_algorithms/Radix_sort
mod ds_radix_sort {
    fn merge(in1: &[i32], in2: &[i32], out: &mut [i32]) {
        let (left, right) = out.split_at_mut(in1.len());
        left.clone_from_slice(in1);
        right.clone_from_slice(in2);
    }

    // least significant digit radix sort
    pub fn radix_sort(data: &mut [i32]) {
        for bit in 0..31 {
            // types of small and big is Vec<i32>.
            // It will be infered from the next call of merge function.
            let (small, big): (Vec<_>, Vec<_>) = data.iter().partition(|&&x| (x >> bit) & 1 == 0);
            merge(&small, &big, data);
        }
        // last bit is sign
        let (negative, positive): (Vec<_>, Vec<_>) = data.iter().partition(|&&x| x < 0);
        merge(&negative, &positive, data);
    }
}

/// $ cargo test radix_sort
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_radix_sort() {
        let mut items: Vec<i32> = vec![14, 61, 85, 24, 74, 26, 17, 50, 40, 45, 21, 32, 59, 58, 13];
        radix_sort(&mut items);
        assert_eq!(
            items,
            vec![13, 14, 17, 21, 24, 26, 32, 40, 45, 50, 58, 59, 61, 74, 85]
        );
    }
}
