/// Поиск с возвратом
/// Проблема N-ферзей. Проблема N-ферзей — это классическая проблема, которую можно решить с 
/// помощью поиска с возвратом. Цель состоит в том, чтобы разместить N-ферзей на шахматной 
/// доске NxN таким образом, чтобы ни один ферзь не мог атаковать другого ферзя.
/// 
/// Example:
///
/// assert_eq!(2,solve_nqueens(4));
/// 
/// Этот алгоритм начинает размещать фигуры в первом ряду и для каждого размещённого ферзя проверяет, 
/// не атакован ли он каким-либо предыдущим ферзём. Если нет, он переходит к следующей строке 
/// и повторяет процесс. Если ферзь находится в позиции, где он подвергается атаке, 
/// алгоритм отступает и пробует другую позицию. 
/// Это продолжается до тех пор, пока все ферзи не будут размещены на доске, не атакуя друг друга. 
fn solve_nqueens(n: isize) -> isize{

    fn could_place(board:&[isize], row: isize, col:isize)-> bool{
        // check if a queen can be placed on board[row][col]
        // check if this row is not under attack from any previous queen in that column
        for i in 0..row{
            if board[i as usize] == col || (board[i as usize] - col).abs() == (i - row).abs(){
                return false;
            }
        }
        return true;
   }
 
   fn backtrack(board:&mut Vec<isize>,n: isize, row:isize, mut count:isize) -> isize{
        for col in 0..n{
            if could_place(board, row, col){
                board[row as usize] = col;
                if row + 1 == n{
                    count += 1;
                }else{
                    count = backtrack(board,n,row + 1, count);
                }
            }
        }
        return count;
    }
 
    let mut board = vec![-1isize;n as usize];
    return backtrack(&mut board, n, 0, 0);
}
 
 
 