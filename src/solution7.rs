/*
Input
7
2 1 3 9 20 7 8
Ouput
13
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
    let n: usize = scanner.next();
    let mut v1 = Vec::<i64>::new();
    let mut v2 = Vec::<i64>::new();

    for _i in 0..n {
        let x: i64 = scanner.next();
        if x % 2 == 0 {
            v1.push(x);
        } else {
            v2.push(x);
        }
    }
    v1.sort();
    v2.sort();
    let mut x1 = v1.len();
    let mut x2 = v2.len();
    if x1 == 0 {
        x1 = x1 + 1;
        v1.push(0);
    }
    if x2 == 0 {
        x2 = x2 + 1;
        v2.push(0);
    }

    let mut s1 = v1[x1 / 2];
    let mut s2 = v2[x2 / 2];
    if x1 % 2 == 0 {
        s1 += v1[x1 / 2 - 1];
        s1 /= 2;
    }
    if x2 % 2 == 0 {
        s2 += v2[x2 / 2 - 1];
        s2 /= 2;
    }

    println!("{}", s1 + s2);
}

fn main() {
    solve();
}
