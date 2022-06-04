/*
Input
1 2 3

Output
6
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
pub fn gcd(mut n: i64, mut m: i64) -> i64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}
fn solution(n: i64, a: i64, b: i64) -> i64 {
    let lcm = (a * b) / gcd(a, b);
    let mut ten_pow_n: i64 = 1;
    for _i in 0..n {
        ten_pow_n = ten_pow_n * 10;
    }
    let mut ret = ((ten_pow_n - 1) / a) + ((ten_pow_n - 1) / b) - ((ten_pow_n - 1) / lcm);
    ten_pow_n = ten_pow_n / 10;
    ret = ret - (((ten_pow_n - 1) / a) + ((ten_pow_n - 1) / b) - ((ten_pow_n - 1) / lcm));
    return ret;
}

pub fn solve() {
    let mut scan = Scanner::default();
    let n: i64 = scan.next();
    let a: i64 = scan.next();
    let b: i64 = scan.next();
    println!("{}", solution(n, a, b));
}
fn main() {
    solve();
}

/*
  C++ code for this question was wrong
  because int was used there which was creating
  an issue of outof range

    Correct C++ code is below
    #include<bits/stdc++.h>
using namespace std;
long long solve(long long int n,long long int a,long long int b){
     long long int lcm = (a * b) / __gcd(a,b);
     long long tenPowN = 1;
     for(int i = 0;i < n;i++){
          tenPowN = tenPowN * 10;
     }
     cout<<tenPowN<<" "<<lcm<<endl;
     long long ret = ((tenPowN - 1) / a) + ((tenPowN - 1) / b) - ((tenPowN - 1) / lcm);
     tenPowN = tenPowN / 10;
     ret = ret - (((tenPowN - 1) / a) + ((tenPowN - 1) / b) - ((tenPowN - 1) / lcm));
     return ret;
}
int main(){
     long long int n,a,b;
     cin >> n >> a >> b;
     cout << solve(n,a,b) << "\n";
     return 0;
}
*/
