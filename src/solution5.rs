/*
Input
9 5
5 2 3 2 4 1 4 2 4

Output
5 2 2
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
    let mut y: i32 = scanner.next();
    let mut h: Vec<i32> = vec![0; n as usize];
    for i in 0..n {
        h[i as usize] = scanner.next();
    }
    let mut burst = 0;
    let mut released = 0;
    let mut unaffected = 0;
    let mut ishit = -1;
    for i in 0..n {
        if h[i as usize] > y {
            released = released + 1;
        } else if h[i as usize] == y {
            burst = burst + 1;
        } else {
            unaffected = unaffected + 1;
        }
        y = y + ishit;
        if y == 0 {
            y = 1;
            ishit = 1;
        }
    }
    println!("{} {} {}", burst, released, unaffected);
}

fn main() {
    solve();
}
