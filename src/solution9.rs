/*
    Input
2
a b

Output
a
b
aa
ab
ba
bb
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

fn generate(arr: &std::vec::Vec<char>, i: i32, s: String, n: i32) {
    if i == -1 {
        println!("{}", s);
        return;
    }

    for j in 0..n {
        let mut appended: String = s.to_string();
        appended.push(arr[j as usize]);
        generate(arr, i - 1, appended, n);
    }
}
fn crack(arr: &std::vec::Vec<char>, n: i32) {
    for i in 0..n {
        let s = "";
        generate(&arr, i, s.to_string(), n);
    }
}

pub fn solve() {
    let mut scanner = Scanner::default();
    let n: i32 = scanner.next();
    let mut arr = Vec::<char>::new();
    for _i in 0..n {
        let c: char = scanner.next();
        arr.push(c);
    }
    crack(&arr, n);
}

fn main() {
    solve();
}
