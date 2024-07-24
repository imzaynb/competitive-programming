// Copied from https://codeforces.com/contest/1168/submission/54903799

#[allow(unused_imports)]
use std::io::{BufWriter, stdin, stdout, Write};
 
#[derive(Default)]
struct Scanner {
    buffer: Vec<String>
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

fn main() {
    let mut scan = Scanner::default();

    let n = scan.next::<u64>();
    let m = scan.next::<u64>();
    let a = scan.next::<u64>();

    let n_tiles = 
          (n as f64 / a as f64).ceil() 
        * (m as f64 / a as f64).ceil();

    println!("{n_tiles}");
}
