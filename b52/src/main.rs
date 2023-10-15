use std::collections::VecDeque;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        N:usize,
        X:usize,
        mut A:Chars,
    }

    let mut q = VecDeque::new();
    q.push_back(X - 1);
    A[X - 1] = '@';

    while q.len() > 0 {
        let pos = q.pop_front().unwrap();
        if pos > 0 && A[pos - 1] == '.' {
            A[pos - 1] = '@';
            q.push_back(pos - 1);
        }
        if pos + 1 < N && A[pos + 1] == '.' {
            A[pos + 1] = '@';
            q.push_back(pos + 1);
        }
    }

    println!("{}", A.iter().fold(String::new(), |mut acc, &x| {
        acc.push(x);
        acc
    }));
}
