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

    let n = scan.next::<usize>();
    let mut x = 0;

    for _ in 0..n {
        let operation = scan.next::<String>();
        x = if operation.contains("+") { x + 1 } else { x - 1 };
    }
    println!("{x}");
}