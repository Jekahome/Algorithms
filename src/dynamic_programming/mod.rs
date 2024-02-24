#![allow(dead_code)]

/// Последовательность Фибоначчи: Классическим примером проблемы,
/// которую можно решить с помощью динамического программирования,
/// является последовательность Фибоначчи.
///
/// Examples:
///
/// println!("{}",fibonacci(10));
///
pub fn fibonacci(n: usize) -> usize {
    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

fn fibonacci_memoization(number: usize) -> usize {
    fn fib_memo(n: usize, memo: &mut [Option<usize>]) -> usize {
        memo[n].unwrap_or_else(|| {
            let result = {
                if n <= 0 {
                    0
                } else if n == 1 {
                    1
                } else {
                    fib_memo(n - 1, memo) + fib_memo(n - 2, memo)
                }
            };
            memo[n] = Some(result);
            result
        })
    }

    fib_memo(number, &mut vec![None; number + 1])
}

// https://codereview.stackexchange.com/questions/204555/recursive-fibonacci-in-rust-with-memoization
fn fib(n: usize) -> usize {
    fn fib_memo(n: usize, memo: &mut [usize; 2]) -> usize {
        let [a, b] = *memo;
        let c = a + b;
        if n == 0 {
            c
        } else {
            *memo = [b, c];
            fib_memo(n - 1, memo)
        }
    }
    if n <= 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib_memo(n - 2, &mut [0, 1])
    }
}

/// $ cargo test dynamic_programming
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(55, fibonacci(10));
        assert_eq!(55, fibonacci_memoization(10));
        assert_eq!(55, fib(10));
    }
}
