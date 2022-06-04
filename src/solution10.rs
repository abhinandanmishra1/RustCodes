/*
Input
10
6
Output
4
2
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

pub fn solve() {
    let mut scanner = Scanner::default();
    let n: i32 = scanner.next();
    let m: i32 = scanner.next();
    println!("{}\n{}", n - m, 2 * m - n);
}

fn main() {
    solve();
}
