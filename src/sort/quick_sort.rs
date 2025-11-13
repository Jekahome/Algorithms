#![allow(dead_code)]

pub use ds_quick_sort::quick_sort;
pub use ds_quick_sort_async::quick_sort_async;
pub use ds_quick_sort_par::quick_sort_par;

///
/// Быстрая сортировка — это алгоритм «разделяй и властвуй», который выбирает «основной» элемент
/// из массива и разбивает остальные элементы на два подмассива. Затем подмассивы сортируются рекурсивно.
///
/// Несмотря на то, что его сложность не ниже, чем пирамидальная сортировка или сортировка слиянием,
/// он имеет очень низкий постоянный коэффициент скорости выполнения, что обычно дает ему преимущество
/// в скорости при работе с большим количеством случайных данных.
///
/// Как работает Quick sort?
///
/// 1) Выберите элемент, называемый опорной точкой, из списка или массива. Как правило, точка поворота — это средний элемент массива.
/// 2) Измените порядок списка так, чтобы все элементы со значениями, меньшими, чем опорная точка, располагались перед опорной точкой,
///     а все элементы со значениями, превышающими опорную точку, шли после нее (равные значения могут идти в любом направлении).
///     Это также известно как разделение.
///     После разделения шарнир находится в конечном положении.
/// 3) Рекурсивно примените описанные выше шаги к подсписку элементов с меньшими значениями и отдельно к подсписку элементов с большими значениями.
///     Если массив содержит только один элемент или ноль элементов, то массив сортируется.
///
/// Быстрая сортировка выполняется путем выбора первого (крайнего левого) элемента массива в качестве опорной точки.
/// Затем мы сравниваем его с каждым следующим элементом. Когда мы находим тот, который меньше, мы перемещаем его влево.
/// Перемещение выполняется быстро путем замены этого элемента первым элементом после точки поворота,
/// а затем замены точки поворота элементом после него. Пройдя по всему массиву, мы берем все точки слева от опорной
/// точки и вызываем быструю сортировку для этого подмассива, а затем делаем то же самое со всеми точками справа от опорной точки.
/// Рекурсия выполняется до тех пор, пока мы не достигнем подмассивов длиной 0-1 элементов.
///
///
/// Когда происходит худший случай быстрой сортировки?
///
/// В быстрой сортировке мы выбираем опорный элемент, затем разделяем данный массив вокруг опорного
/// элемента, помещая опорный элемент в правильное положение в отсортированном массиве.
/// Худший случай быстрой сортировки возникает, когда одна часть после раздела содержит все элементы,
/// а другая часть пуста. Например, если входной массив отсортирован и последний или первый элемент
/// выбран в качестве опорного, происходит худшее.

/// Author - Aditya Rana https://github.com/diptangsu/Sorting-Algorithms/blob/master/Rust/QuickSort.rs
/// Generic Quick Sort in rust
mod ds_quick_sort {
    pub fn quick_sort<T: Ord>(arr: &mut [T]) {
        let len = arr.len();
        _quick_sort(arr, 0, (len - 1) as isize);
    }

    fn _quick_sort<T: Ord>(arr: &mut [T], low: isize, high: isize) {
        if low < high {
            let p = partition(arr, low, high);
            _quick_sort(arr, low, p - 1);
            _quick_sort(arr, p + 1, high);
        }
    }

    fn partition<T: Ord>(arr: &mut [T], low: isize, high: isize) -> isize {
        let pivot = high as usize;
        let mut store_index = low - 1;
        let mut last_index = high;

        loop {
            store_index += 1;
            while arr[store_index as usize] < arr[pivot] {
                store_index += 1;
            }
            last_index -= 1;
            while last_index >= 0 && arr[last_index as usize] > arr[pivot] {
                last_index -= 1;
            }
            if store_index >= last_index {
                break;
            } else {
                arr.swap(store_index as usize, last_index as usize);
            }
        }
        arr.swap(store_index as usize, pivot as usize);
        store_index
    }
}

mod ds_quick_sort_async {
    use async_recursion::async_recursion;

    pub fn quick_sort_async<T: Ord + Send>(arr: &mut [T]) {
        let len = arr.len();
        futures::executor::block_on(_quick_sort(arr, 0, (len - 1) as isize));
    }

    #[async_recursion]
    async fn _quick_sort<T: Ord + Send>(arr: &mut [T], low: isize, high: isize) {
        if low < high {
            let p = partition(arr, low, high);

            if p > 0 {
                let (l, r) = arr.split_at_mut(p as usize);
                let (l_len, r_len) = (l.len(), r.len());
                if l_len > 1 && r_len > 1 {
                    let fut_1 = _quick_sort(l, 0, (l_len - 1) as isize);
                    let fut_2 = _quick_sort(r, 0, (r_len - 1) as isize);
                    futures::join!(fut_1, fut_2);
                } else if l_len > 1 {
                    _quick_sort(l, 0, (l_len - 1) as isize).await;
                } else if r_len > 1 {
                    _quick_sort(r, 0, (r_len - 1) as isize).await;
                }
            } else {
                _quick_sort(arr, 1, high).await;
            }
        }
    }

    fn partition<T: Ord + Send>(arr: &mut [T], low: isize, high: isize) -> isize {
        let pivot = high as usize;
        let mut store_index = low - 1;
        let mut last_index = high;

        loop {
            store_index += 1;
            while arr[store_index as usize] < arr[pivot] {
                store_index += 1;
            }
            last_index -= 1;
            while last_index >= 0 && arr[last_index as usize] > arr[pivot] {
                last_index -= 1;
            }
            if store_index >= last_index {
                break;
            } else {
                arr.swap(store_index as usize, last_index as usize);
            }
        }
        arr.swap(store_index as usize, pivot as usize);
        store_index
    }
}

mod ds_quick_sort_par {
    use rayon::prelude::*;
    pub fn quick_sort_par<T: Ord + Send>(arr: &mut [T]) {
        let len = arr.len();
        _quick_sort(arr, 0, (len - 1) as isize);
    }

    fn _quick_sort<T: Ord + Send>(arr: &mut [T], low: isize, high: isize) {
        if low < high {
            let p = partition(arr, low, high);
            if p > 0 {
                // TODO: Затратно использовать par_iter_mut при N < 10000
                if arr.len() > 10000 {
                    let (l, r) = arr.split_at_mut(p as usize);
                    let (l_len, r_len) = (l.len(), r.len());

                    [l, r].par_iter_mut().for_each(|i| {
                        if i.len() == l_len {
                            _quick_sort(i, 0, (l_len - 1) as isize);
                        } else {
                            _quick_sort(i, 0, (r_len - 1) as isize);
                        }
                    });
                } else {
                    _quick_sort(arr, low, p - 1);
                    _quick_sort(arr, p + 1, high);
                }

                /*
                // TODO: N > 100_000 переполняет стек
                std::thread::scope(|s| {
                    s.spawn(|| {
                        _quick_sort(l, 0, (l_len - 1) as isize);
                    });
                    s.spawn(|| {
                        _quick_sort(r, 0, (r_len - 1) as isize);
                    });
                });
                */
            } else {
                _quick_sort(arr, 1, high);
            }
        }
    }

    fn partition<T: Ord + Send>(arr: &mut [T], low: isize, high: isize) -> isize {
        let pivot = high as usize;
        let mut store_index = low - 1;
        let mut last_index = high;

        loop {
            store_index += 1;
            while arr[store_index as usize] < arr[pivot] {
                store_index += 1;
            }
            last_index -= 1;
            while last_index >= 0 && arr[last_index as usize] > arr[pivot] {
                last_index -= 1;
            }
            if store_index >= last_index {
                break;
            } else {
                arr.swap(store_index as usize, last_index as usize);
            }
        }
        arr.swap(store_index as usize, pivot as usize);
        store_index
    }
}

mod ds_quick_sort_easy {
    pub fn quick_sort_easy<T: PartialOrd + std::fmt::Debug>(v: &mut [T]) {
        if v.len() <= 1 {
            return;
        }
        //println!("pre = {:?}", v);
        let p = pivot(v);

        //println!("post = {:?}", v);
        let (a, b) = v.split_at_mut(p);
        quick_sort_easy(a);
        quick_sort_easy(&mut b[1..]);
    }

    fn pivot<T: PartialOrd>(v: &mut [T]) -> usize {
        //let mut p = rand::random::<usize>() % v.len();
        //let mut p = v.len()-1;//b_rand::rand(v.len());
        //v.swap(p, 0);

        let mut p = 0;
        for i in 1..v.len() {
            if v[i] < v[p] {
                v.swap(p + 1, i);
                v.swap(p, p + 1);
                p += 1;
            }
        }
        p
    }
}

/// $ cargo test quick_sort
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut items = vec![1, 8, 2, 1];
        quick_sort(&mut items);
        assert_eq!(vec![1, 1, 2, 8], items);
    }
    #[test]
    fn test_quick_sort_async() {
        let mut items = vec![1, 8, 2, 1];
        quick_sort_async(&mut items);
        assert_eq!(vec![1, 1, 2, 8], items);
    }
    #[test]
    fn test_quick_sort_par() {
        let mut items = vec![1, 8, 2, 1];
        quick_sort_par(&mut items);
        assert_eq!(vec![1, 1, 2, 8], items);
    }
}
