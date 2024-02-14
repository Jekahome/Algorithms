#![allow(dead_code)]

/// 
/// Пирамидальная сортировка — это алгоритм сортировки на основе сравнения, который строит пирамиду 
/// из входных элементов, а затем многократно извлекает её максимальный элемент и 
/// помещает его в конец отсортированного выходного массива.
/// 


pub use ds_heap_sort::{heap_sort,heap_sort_g};
/// пирамидальная сортировка O(N log N)
/// Принцип работы: постоить сортирующее дерево где родитель больше своих листьев, 
/// второй этап, корень кидать в конец и снова балансировать сортирующее дерево до отсортированной части
mod ds_heap_sort{
    pub fn heap_sort(arr: &mut [i32]) {
        // каждый потомок не должен быть больше родителя
        // начать с нижнего правого родительского узла
        // Просейка нужна для того, чтобы из обычного дерева сделать сортирующее дерево и в дальнейшем поддерживать дерево в таком (сортирующем) состоянии.

        // Этап 1. Формируем из всего массива сортирующее дерево. 
        // Для этого проходим по родителям справа-налево и елемент больше родителя меняем местами с родителем.
        let end = arr.len();
        for start in (0..end / 2).rev() {
            // Skip leaf nodes (end / 2).
            sift_down2(arr, start, end - 1);// или sift_down
        }
   
        // Этап 2. Максимумы ставим в конец неотсортированной части массива т.е. до `end`.
        for end in (1..arr.len()).rev() {
            arr.swap(end, 0);
            sift_down2(arr, 0, end - 1); // или sift_down
        }
    }
    
    /// Internal function for heap to fix itself to conform to heap definition.
    /// Precondiition: all elements below `start` are in heap order
    /// expect `start` itself.
    fn sift_down(arr: &mut [i32], start: usize, end: usize) {
        let mut root = start;
        loop {
            let mut child = root * 2 + 1; // Get the left child
            if child > end {
                break;
            }
            if child < end && arr[child] < arr[child + 1] {
                // Right child exists and is greater.
                child += 1;
            }
    
            if arr[root] < arr[child] {
                // If child is greater than root, swap'em!
                arr.swap(root, child);
                root = child;
            } else {
                break;
            }
        }
    }
 
    fn sift_down2(arr:&mut[i32],i:usize,parent:usize){
        let mut index=i*2+1;
        if index+1 < parent && arr[index] < arr[index+1]{
            index+=1;
        }
        if index <= parent{
            if  arr[i] < arr[index] {
                arr.swap(i, index);
                if index < parent {
                    sift_down2(arr,index,parent);
                }
            }            
        }
    }

    fn print_tree(arr:&[i32]){
        for start in 0..arr.len()/2{
        print!("root {:?} ",arr[start]);
        if (start*2+1) < arr.len() {
            print!("left {}",arr[start*2+1]);
        }
        if (start*2+2) < arr.len() {
            print!(" right {}",arr[start*2+2]);
        }
        print!("\n");
        }
    } 

    /// Generic variant
    /// https://github.com/diptangsu/Sorting-Algorithms/blob/master/Rust/HeapSort.rs
    pub fn heap_sort_g<T: Ord>(arr: &mut [T]) {
        if arr.len() <= 1 {
            return;
        }
    
        let last_parent = (arr.len() - 2) / 2;
        for i in (0..=last_parent).rev() {
            move_down_g(arr, i);
        }
    
        for end in (1..arr.len()).rev() {
            arr.swap(0, end);
            move_down_g(&mut arr[..end], 0);
        }
    }
    
    fn move_down_g<T: Ord>(arr: &mut [T], mut root: usize) {
        let last = arr.len() - 1;
        loop {
            let left = 2 * root + 1;
            if left > last {
                break;
            }
            let right = left + 1;
            let child = if right <= last && arr[right] > arr[left] {
                right
            } else {
                left
            };
    
            if arr[child] > arr[root] {
                arr.swap(root, child);
            }
            root = child;
        }
    }
}

/// $ cargo +nightly test --lib algorithms::heap_sort::test 
#[cfg(test)]
mod test {
    use super::*;
   
    /// $ cargo +nightly test --lib algorithms::heap_sort::test::test_heap_sort 
    #[test]
    fn test_heap_sort(){
        let mut items:Vec<i32> = vec![14,61,85,24,74,26,17,50,40,45,21,32,59,58,13];
        heap_sort(&mut items);
        assert_eq!(items,vec![13, 14, 17, 21, 24, 26, 32, 40, 45, 50, 58, 59, 61, 74, 85]); 
    }
    /// $ cargo +nightly test --lib algorithms::heap_sort::test::test_heap_sort_g 
    #[test]
    fn test_heap_sort_g(){
        let mut items:Vec<i32> = vec![14,61,85,24,74,26,17,50,40,45,21,32,59,58,13];
        heap_sort_g(&mut items);
        assert_eq!(items,vec![13, 14, 17, 21, 24, 26, 32, 40, 45, 50, 58, 59, 61, 74, 85]); 
    }
}