#![allow(unused_assignments)]

pub use ds_counting_sort::{counting_sort, counting_sort_opt};
/// Сортировка подсчетом
/// Необходимо знать максимальный и минимальный элемент последовательности
/// Выделить новую последовательность с индексами от минимального до максимального включительно элемента
/// Перебирая последовательность увеличивать значение индекса соответствующего значению из последовательности
/// Вывод всех сумм значений и будет результат сортировки
/// TODO: При большой разнице между min и max возможна ошибка памяти
///      `memory allocation of 26164585496 bytes failed`
/// Т.е. недостаток в точ что значения в последовательности это будут индексы новой последовательности
/// и это потребует большого расхода памяти
/// Единственный способ применения, когда последовательность содержит значения от 0 до значений равным ее длине
mod ds_counting_sort {
    // `arr` - Коллекция значений для сортировки на месте.
    // `max` - Верхняя граница целочисленного диапазона.
    // `key` - Функция, извлекающая ключ с целочисленным диапазоном из элементов.
    pub fn counting_sort<F, T>(arr: &mut [T], max: usize, key: F)
    where
        F: Fn(&T) -> usize,
        T: Clone,
    {
        let mut prefix_sums = {
            // 1. Initialize the count array with default value 0.
            let len: usize = if max > arr.len() {
                max + 1
            } else {
                arr.len() + 1
            };
            let mut count_arr = Vec::with_capacity(len);
            count_arr.resize(len, 0);

            // 2. Scan elements to collect counts.
            for value in arr.iter() {
                count_arr[key(value)] += 1;
            }

            // 3. Calculate prefix sum.
            count_arr
                .into_iter()
                .scan(0, |state, x| {
                    *state += x;
                    Some(*state - x)
                })
                .collect::<Vec<usize>>()
        };

        // 4. Use prefix sum as index position of output element.
        for value in arr.to_vec().iter() {
            let index = key(value);
            arr[prefix_sums[index]] = value.clone();
            prefix_sums[index] += 1;
        }
    }

    // Отличие от стандартной реализации
    // Выделение места для индексов происходит по мере их поступления и не более чем нужно
    pub fn counting_sort_opt(arr: &mut Vec<i32>) {
        let mut min = *arr.iter().min().unwrap(); // N
        if min > i32::MIN {
            min = -1;
        }
        let max = arr.iter().max().unwrap() - min; // N

        let mut max_b0: usize = max.to_be_bytes()[0] as usize;
        if max_b0 == 0 {
            max_b0 = 1;
        }
        let mut buff: Vec<Vec<Vec<Vec<u8>>>> = vec![vec![vec![vec![0; 1]; 1]; 1]; max_b0];
        let mut n: i32 = 0;
        for i in arr.iter() {
            // N
            n = i - min;
            let b = n.to_be_bytes();
            alloc0(&mut buff, b[0] as usize); // S
            alloc1(&mut buff[b[0] as usize], b[1] as usize); // S
            alloc2(&mut buff[b[0] as usize][b[1] as usize], b[2] as usize); // S
            alloc3(
                &mut buff[b[0] as usize][b[1] as usize][b[2] as usize],
                b[3] as usize,
            ); // S
            buff[b[0] as usize][b[1] as usize][b[2] as usize][b[3] as usize] += 1;
        }

        let mut index_set = 0;
        for (index0, v0) in buff.iter().enumerate() {
            // M
            for (index1, v1) in v0.iter().enumerate() {
                // K
                for (index2, v2) in v1.iter().enumerate() {
                    // F
                    for (index3, count) in v2.iter().enumerate().filter(|(_, c)| *c > &0) {
                        // G
                        for _ in 0..*count as usize {
                            // H
                            arr[index_set] = i32::from_be_bytes([
                                index0 as u8,
                                index1 as u8,
                                index2 as u8,
                                index3 as u8,
                            ]) + min;
                            index_set += 1;
                        }
                    }
                }
            }
        }
    }

    fn alloc0(arr: &mut Vec<Vec<Vec<Vec<u8>>>>, value: usize) {
        while arr.len() <= value {
            arr.push(vec![vec![vec![0; 1]; 1]; 1]);
        }
    }
    fn alloc1(arr: &mut Vec<Vec<Vec<u8>>>, value: usize) {
        while arr.len() <= value {
            arr.push(vec![vec![0; 1]; 1]);
        }
    }
    fn alloc2(arr: &mut Vec<Vec<u8>>, value: usize) {
        while arr.len() <= value {
            arr.push(vec![0; 1]);
        }
    }
    fn alloc3(arr: &mut Vec<u8>, value: usize) {
        while arr.len() <= value {
            arr.push(0);
        }
    }
}

/// $ cargo +nightly test --lib algorithms::counting_sort::test
#[cfg(test)]
mod test {
    use super::*;

    extern crate rand;
    use rand::Rng;
    const SIZE: usize = 1000; //5;

    fn gen_vec() -> Vec<i32> {
        let mut rng = rand::thread_rng();
        let mut v: Vec<i32> = Vec::with_capacity(SIZE);
        for _i in 0..SIZE {
            v.push(rng.gen::<i32>());
        }
        v
    }

    // cargo +nightly test --lib algorithms::counting_sort::test::test_counting_sort_opt -- --nocapture
    #[test]
    fn test_counting_sort_opt() {
        /*
            let mut items = gen_vec();
            let mut items = vec![999999_u32, 2046506772, 585238740, 3207941434, 3855811927];
            let max = *items.iter().max().unwrap() as usize;
            println!("max={} {:?}",max,items);
            let f =  |int:&u32| *int as usize;
            counting_sort(&mut items,max,f);
        */

        let mut items = gen_vec();
        counting_sort_opt(&mut items);
        assert!(true);
    }

    #[test]
    fn test_counting_sort() {
        let mut items = vec![1, 18, 2, 4, 5];
        let max = *items.iter().max().unwrap() as usize;
        let f = |int: &i32| *int as usize;
        counting_sort(&mut items, max, f);
        assert_eq!(vec![1, 2, 4, 5, 18], items);

        let mut items = vec![(1, 8), (2, 4), (5, 2), (1, 1)];
        let max = 5;
        let f = |t: &(i32, i32)| t.0 as usize;
        counting_sort(&mut items, max, f);
        assert_eq!(vec![(1, 8), (1, 1), (2, 4), (5, 2)], items);
    }
}
