/*
Input
5
10201

Output
8

*/

//This solution is written by Abhinandan Mishra
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

fn get_index(c: char) -> usize {
    if c == '0' {
        return 0;
    } else if c == '1' {
        return 1;
    } else {
        return 2;
    }
}
pub fn solve() {
    let mut scanner = Scanner::default();
    let mut n: usize = scanner.next();
    let s: String = scanner.next();

    let mut dp = vec![vec![0; 3]; n];
    let mut j: i32 = n as i32 - 1;
    let char_vec: Vec<char> = s.chars().collect();
    while j >= 0 {
        if j < (n - 1) as i32 {
            dp[j as usize][0] = dp[j as usize + 1][0];
            dp[j as usize][1] = dp[j as usize + 1][1];
            dp[j as usize][2] = dp[j as usize + 1][2];
        }
        dp[j as usize][get_index(char_vec[j as usize])] =
            dp[j as usize][get_index(char_vec[j as usize])] + 1;
        j = j - 1;
    }
    let mut ans = 0;
    n = n - 1;
    for i in 0..n {
        if char_vec[i] == '0' {
            ans = ans + dp[i + 1][1];
            ans = ans + dp[i + 1][2];
        } else if char_vec[i] == '1' {
            ans = ans + dp[i + 1][0];
            ans = ans + dp[i + 1][2];
        } else {
            ans = ans + dp[i + 1][0];
            ans = ans + dp[i + 1][1];
        }
    }
    println!("{}", ans);
}

fn main() {
    solve();
}
