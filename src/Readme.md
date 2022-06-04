### For using rust in Competitive programming

- `proconio` crate can be used
- It's very easy to use it
- but it's not available everywhere
- It is available at Atcoder.

For leaning rust methods

- Just see codes of other peoples in rust on `github` or `atcoder` submissions.

**Boiler Plate**

```rust
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

pub fn solve(){

}

fn main(){
    solve();
}
```
