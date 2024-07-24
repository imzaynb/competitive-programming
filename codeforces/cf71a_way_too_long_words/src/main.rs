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
    let out = &mut BufWriter::new(stdout());

    let n = scan.next::<usize>();

    let words: Vec<String> = (0..n).map(|_| scan.next()).collect();

    for word in words {
        let updated_word = if word.len() <= 10 {
            word 
        } else {
            format!("{}{}{}", 
                    &word[0..1],
                    word.len()-2,
                    &word[word.len()-1..word.len()])
        };
        println!("{updated_word}");
    }
}