/*
Input
3
1 5
2 4
5 6

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
fn get_max_living_year(a: std::vec::Vec<std::vec::Vec<i32>>, n: i32) -> std::vec::Vec<i32> {
    // WRITE YOUR LOGIC HERE
    let mut events = vec![vec![0; 2]; 2 * n as usize];
    let mut i = 0;
    let mut j = 0;
    while i < n {
        events[j as usize][0] = a[i as usize][0];
        events[j as usize][1] = 1;
        j = j + 1;
        events[j as usize][0] = a[i as usize][1];
        events[j as usize][1] = 0;
        j = j + 1;
        i = i + 1;
    }
    let mut ans = 0;
    let mut year = -1;
    let mut living = 0;
    events.sort();

    for i in 0..events.len() {
        let p = &events[i];
        if p[1] == 1 {
            living = living + 1;

            if living > ans {
                ans = living;
                year = p[0];
            }
        } else {
            living = living - 1;
        }
    }
    let mut arr = vec![0; 2];
    arr[0] = year;
    arr[1] = ans;
    return arr;
}

pub fn solve() {
    // input! {
    //     n:i32,
    //     a:[[i32;2];n]
    // }
    let mut scanner = Scanner::default();
    let n: i32 = scanner.next();
    let mut a = vec![vec![0; 2]; n as usize];
    for i in 0..n {
        let first = scanner.next();
        let second = scanner.next();
        a[i as usize][0] = first;
        a[i as usize][1] = second;
    }
    let res: std::vec::Vec<i32> = get_max_living_year(a, n);
    println!("{} {}", res[0], res[1]);
}

fn main() {
    solve();
}
