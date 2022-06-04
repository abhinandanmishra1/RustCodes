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

fn meet_friends(mut a: std::vec::Vec<i32>, mut d: std::vec::Vec<i32>, n: i32) -> i32 {
    let mut count = 0;
    let mut res = -404;
    a.sort();
    d.sort();
    let mut i = 0;
    let mut j = 0;
    while i < n && j < n {
        if a[i as usize] <= d[j as usize] {
            count = count + 1;
            i = i + 1;
        } else {
            count = count - 1;
            j = j + 1;
        }
        res = std::cmp::max(res, count);
    }
    return res;
}

pub fn solve() {
    let mut scan = Scanner::default();
    let n: i32 = scan.next();
    let mut a = Vec::<i32>::new();
    let mut d = Vec::<i32>::new();
    for _i in 0..n {
        let x: i32 = scan.next();
        a.push(x);
    }
    for _i in 0..n {
        let x: i32 = scan.next();
        d.push(x);
    }
    println!("{}", meet_friends(a, d, n));
}

fn main() {
    solve();
}
