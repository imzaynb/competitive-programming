// Copied from https://codeforces.com/contest/1168/submission/54903799

#[allow(unused_imports)]
use std::io::{BufWriter, stdin, stdout, Write};
use std::cmp::Ordering;
 
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
    let s = scan.next::<String>();
    let t = scan.next::<String>();

    println!("{}", match 
        s
        .to_ascii_lowercase()
        .as_str()
        .cmp(
            t
            .to_ascii_lowercase()
            .as_str()) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    });
}