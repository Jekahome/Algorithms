#![allow(dead_code)]
#![allow(unused_assignments)]

pub use alg_bubble_sort::{
    bubble_sort, bubble_sort2, bubble_sort3, bubble_sort4, bubble_sort5, bubble_sort_optimized,
};
mod alg_bubble_sort {
    // 0 секунд
    //Duration { secs: 0, nanos: 773130511 }
    pub fn bubble_sort(arr: &mut Vec<i32>) {
        let size_ = arr.len() - 1;
        let mut ex: bool;
        let mut index;
        let mut temp: i32;
        loop {
            ex = false;
            index = 0;

            while index < size_ {
                if arr[index] > arr[index + 1] {
                    temp = arr[index];
                    arr[index] = arr[index + 1];
                    arr[index + 1] = temp;
                    ex = true;
                }
                index += 1;
            }
            // жрет 6 секунды
            /*for _ in 0..size_ {
                if arr[index] > arr[index+1] {
                    temp=arr[index];
                    arr[index]=arr[index+1];
                    arr[index+1]=temp;
                    ex=true;
                }
                index+=1;
            }*/
            if ex == false {
                break;
            }
        }
    }

    // 0 секунды
    pub fn bubble_sort2(arr: &mut Vec<i32>) {
        let size_ = arr.len() - 1;
        let mut ex: bool;
        //let mut j ;
        let mut temp: i32;
        let mut lb;
        let mut ub = size_;
        let mut indx;
        loop {
            lb = size_;
            ex = false;
            indx = 0;

            while indx < ub {
                if arr[indx] > arr[indx + 1] {
                    temp = arr[indx];
                    arr[indx] = arr[indx + 1];
                    arr[indx + 1] = temp;
                    ex = true;
                    lb = indx;
                }
                indx += 1;
            }
            // жрет 3 секунды
            /*for _ in 0..ub {
                if arr[indx] > arr[indx+1] {
                    temp=arr[indx];
                    arr[indx]=arr[indx+1];
                    arr[indx+1]=temp;
                    ex=true;
                    lb=indx;
                }
                indx+=1;
            }*/

            if ex == false {
                break;
            }
            ub = lb;
        }
    }

    // 0 секунды
    //Duration { secs: 0, nanos: 543767102 }
    pub fn bubble_sort3(arr: &mut Vec<i32>) {
        let size_ = arr.len() - 1;
        let mut ex: bool;
        let mut temp: i32;
        let mut lb;
        let mut ub = size_;
        let mut r;
        let mut k = size_;

        loop {
            ex = false;
            lb = size_;
            r = ub;
            //  проход снизу вверх начинается от последнего переставленного когда сверху вниз проходили
            while r > 0 {
                if arr[r - 1] > arr[r] {
                    temp = arr[r - 1];
                    arr[r - 1] = arr[r];
                    arr[r] = temp;
                    k = r;
                    ex = true;
                }
                r -= 1;
            }

            lb = k + 1;

            if !ex {
                break;
            } else {
                ex = false;
            }

            // проход сверху вниз
            r = 1;
            while r <= ub {
                if arr[r - 1] > arr[r] {
                    temp = arr[r - 1];
                    arr[r - 1] = arr[r];
                    arr[r] = temp;
                    k = r;
                    ex = true;
                }
                r += 1;
            }
            ub = k - 1;
            //пока не сойдутся индексы отработанных переставлени
            if lb > ub || ex == false {
                break;
            }
        }
    }

    // 0 сеекунд
    //Duration { secs: 0, nanos: 480663368 }
    pub fn bubble_sort4(arr: &mut Vec<i32>) {
        let mut limit = 0;
        let size_ = arr.len() - 1;
        let mut ex: bool;
        let mut temp: i32;
        let mut lb;
        let mut ub = size_;
        let mut r;
        let mut k = size_;

        loop {
            ex = false;
            lb = size_;
            r = ub;
            //  проход снизу вверх начинается от последнего переставленного когда сверху вниз проходили
            while r > limit {
                if arr[r - 1] > arr[r] {
                    temp = arr[r - 1];
                    arr[r - 1] = arr[r];
                    arr[r] = temp;
                    k = r;
                    ex = true;
                }
                r -= 1;
            }

            lb = k + 1;
            limit = k;
            if !ex {
                break;
            } else {
                ex = false;
            }

            // проход сверху вниз
            r = limit;
            while r <= ub {
                if arr[r - 1] > arr[r] {
                    temp = arr[r - 1];
                    arr[r - 1] = arr[r];
                    arr[r] = temp;
                    k = r;
                    ex = true;
                }
                r += 1;
            }
            ub = k - 1;
            //пока не сойдутся индексы отработанных переставлени
            if lb > ub || ex == false {
                break;
            }
        }
    }

    /// Optimized bubble sort
    /// Memorize last swapped index to avoid unnecessary check.
    pub fn bubble_sort_optimized(arr: &mut [i32]) {
        let mut new_len: usize;
        let mut len = arr.len();
        loop {
            new_len = 0;
            for i in 1..len {
                if arr[i - 1] > arr[i] {
                    arr.swap(i - 1, i);
                    new_len = i;
                }
            }
            if new_len == 0 {
                break;
            }
            len = new_len;
        }
    }
    pub fn bubble_sort5(arr: &mut [i32]) {
        let mut swapped = true;
        while swapped {
            // No swap means array is sorted.
            swapped = false;
            for i in 1..arr.len() {
                if arr[i - 1] > arr[i] {
                    arr.swap(i - 1, i);
                    swapped = true
                }
            }
        }
    }
}

/// $ cargo test bubble_sort
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bubble() {
        let mut items = vec![1, 8, 2, 4, 5];
        bubble_sort(&mut items);
        assert_eq!(vec![1, 2, 4, 5, 8], items);
    }
    #[test]
    fn test_bubble2() {
        let mut items = vec![1, 8, 2, 4, 5];
        bubble_sort2(&mut items);
        assert_eq!(vec![1, 2, 4, 5, 8], items);
    }
    #[test]
    fn test_bubble3() {
        let mut items = vec![1, 8, 2, 4, 5];
        bubble_sort3(&mut items);
        assert_eq!(vec![1, 2, 4, 5, 8], items);
    }
    #[test]
    fn test_bubble4() {
        let mut items = vec![1, 8, 2, 4, 5];
        bubble_sort4(&mut items);
        assert_eq!(vec![1, 2, 4, 5, 8], items);
    }
    #[test]
    fn test_bubble_sort_optimized() {
        let mut items = vec![1, 8, 2, 4, 5];
        bubble_sort_optimized(&mut items);
        assert_eq!(vec![1, 2, 4, 5, 8], items);
    }
    #[test]
    fn test_bubble5() {
        let mut items = vec![1, 8, 2, 4, 5];
        bubble_sort5(&mut items);
        assert_eq!(vec![1, 2, 4, 5, 8], items);
    }
}
