#![allow(dead_code)]
#![allow(unused_assignments)]

/// Как работает Choice sort?
///
/// Сортировка выбором — это комбинация поиска и сортировки. Во время каждого прохода
/// несортированный элемент с наименьшим (или наибольшим) значением перемещается на
/// соответствующую позицию в массиве. Количество раз, когда сортировка проходит через
/// массив, на единицу меньше количества элементов в массиве.
///
pub use alg_choice_sort::choice_sort;
mod alg_choice_sort {

    pub fn choice_sort(arr: &mut Vec<i32>) {
        let mut i = 0;
        let mut min: i32;
        let mut indx: usize;
        let mut ex: bool = true;
        let size_ = arr.len();
        let mut j;
        while i < size_ {
            ex = true;
            min = arr[i];
            indx = i;

            j = i + 1;
            while j < size_ {
                if min > arr[j] {
                    min = arr[j];
                    indx = j;
                    ex = false;
                }
                j += 1;
            }
            /* for  j  in  i+1..arr.len() {
                if j>=arr.len(){break;}
                if(min > arr[j]){
                    min=arr[j];
                    indx=j;
                    ex=false;
                }
            }*/

            if !ex {
                arr[indx] = arr[i];
                arr[i] = min;
            }
            i += 1;
        }
    }
     
}

/// $ cargo test choice_sort
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_choice_sort() {
        let mut items = vec![1, 8, 2, 4, 5];
        choice_sort(&mut items);
        assert_eq!(vec![1, 2, 4, 5, 8], items);
    }
}
