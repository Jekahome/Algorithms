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

// Мемоизация — это нисходящий подход, использующий рекурсию,
// а табуляция — восходящий подход, использующий итерацию.

// Мемоизация — это нисходящий подход, при котором мы кэшируем результаты вызовов
// функций и возвращаем кэшированный результат, если функция вызывается снова с теми же входными данными.
// Он используется, когда мы можем разделить проблему на подзадачи, и у подзадач есть перекрывающиеся подзадачи.
// Мемоизация обычно реализуется с использованием рекурсии и хорошо подходит для задач с
// относительно небольшим набором входных данных.

fn fibonacci_memoization(n: usize) -> usize {
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

    fib_memo(n, &mut vec![None; n + 1])
}

// https://codereview.stackexchange.com/questions/204555/recursive-fibonacci-in-rust-with-memoization
//
// Многочисленные способы повышения эффективности:
// Implement a generic Fibonacci sequence in Rust without using Copy trait (https://codereview.stackexchange.com/q/130042/32521)
// How to swap two variables? (https://stackoverflow.com/q/31798737/155423)
// How to avoid excessive cloning in Rust? (https://stackoverflow.com/q/40965230/155423)
// Is it possible to use a fold with a Vec? (https://stackoverflow.com/q/27760022/155423)
fn fibonacci_memoization_opt(n: usize) -> usize {
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

// Таблизация — это восходящий подход, при котором мы сохраняем результаты подзадач в таблице и
// используем эти результаты для решения более крупных подзадач, пока не решим всю проблему.
// Он используется, когда мы можем определить проблему как последовательность подзадач, и подзадачи не перекрываются.
// Табуляция обычно реализуется с помощью итерации и хорошо подходит для задач с большим набором входных данных.
fn fibonacci_tabulation(n: usize) -> usize {
    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        let mut table = vec![0; n + 1];
        table[0] = 0;
        table[1] = 1;
        for i in 2..=n {
            table[i] = table[i - 1] + table[i - 2];
        }
        return table[n];
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
        assert_eq!(55, fibonacci_memoization_opt(10));
        assert_eq!(55, fibonacci_tabulation(10));
    }
}
