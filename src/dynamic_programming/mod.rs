#![allow(dead_code)]

/// Последовательность Фибоначчи: Классическим примером проблемы, 
/// которую можно решить с помощью динамического программирования, 
/// является последовательность Фибоначчи.
/// 
/// Examples:
/// 
/// println!("{}",fibonacci(10));
/// 
pub fn fibonacci(n: usize) -> usize{
    if n <= 0{
        return 0;
    }else if n == 1{
        return 1;
        
    }else{
        return fibonacci(n-1) + fibonacci(n-2);
    }
}