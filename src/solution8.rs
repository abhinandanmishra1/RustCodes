/*
Input
3 3
1 0 0
1 1 1
0 1 1

*/
use std::io::stdin;

// Scanner Defined
#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}
fn maximum_square(matrix: std::vec::Vec<std::vec::Vec<char>>) -> i32 {
    if matrix.is_empty() {
        return 0;
    } else {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut sz: i32 = 0;
        let mut dp: std::vec::Vec<std::vec::Vec<i32>> = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                if i == 0 || j == 0 || matrix[i][j] == '0' {
                    let v = matrix[i][j].to_digit(10).unwrap();
                    dp[i][j] = v as i32;
                } else {
                    dp[i][j] =
                        std::cmp::min(dp[i - 1][j - 1], std::cmp::min(dp[i - 1][j], dp[i][j - 1]))
                            + 1;
                }
                sz = std::cmp::max(dp[i][j], sz);
            }
        }
        return sz;
    }
}
pub fn solve() {
    let mut scanner = Scanner::default();
    let m: i32 = scanner.next();
    let n: i32 = scanner.next();
    // 2d matrix of characters
    let mut matrix: std::vec::Vec<std::vec::Vec<char>> = vec![vec!['0'; n as usize]; m as usize];
    for i in 0..m {
        for j in 0..n {
            matrix[i as usize][j as usize] = scanner.next();
        }
    }
    let ans = maximum_square(matrix);
    println!("{}", ans);
}

fn main() {
    solve();
}
