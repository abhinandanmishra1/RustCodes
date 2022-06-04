/*
  Input test case
     5 4
    -4 2 -1 0 5
    -2 4 1 6
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

/*
 Take Input
    let mut n:i32=scan.next();  --> Single Input
*/

pub fn solve() {
    let mut scan = Scanner::default();
    let mut n: i32 = scan.next();
    let mut m: i32 = scan.next();
    let mut posA = Vec::<i32>::new();
    let mut posB = Vec::<i32>::new();
    let mut negA = Vec::<i32>::new();
    let mut negB = Vec::<i32>::new();

    for _i in 0..n {
        let mut x: i32 = scan.next();
        if x >= 0 {
            posA.push(x);
        } else {
            negA.push(x);
        }
    }
    for _i in 0..m {
        let mut x: i32 = scan.next();
        if x >= 0 {
            posB.push(x);
        } else {
            negB.push(x);
        }
    }
    let mut ans = 0;
    let mut i = 0;
    let mut j = 0;

    /*  ---------------------  */
    posA.sort();
    posB.sort();
    negA.sort();
    negB.sort();
    posA.reverse();
    posB.reverse();
    while i < posA.len() && j < posB.len() {
        ans = ans + posA[i] * posB[j];
        i = i + 1;
        j = j + 1;
    }
    let mut x = 0;
    let mut y = 0;
    while x < negA.len() && y < negB.len() {
        ans = ans + negA[x] * negB[y];
        x = x + 1;
        y = y + 1;
    }
    while y < negB.len() {
        ans = ans + negB[y] * posA[posA.len() - 1];
        y = y + 1;
        posA.pop();
    }
    while j < posB.len() {
        ans = ans + posB[y] * negA[negA.len() - 1];
        j = j + 1;
        negA.pop();
    }

    println!("{}", ans);
}

fn main() {
    solve();
}
