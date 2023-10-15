use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        Q:usize,
    }

    let MAX = 10000000i64;
    let mut q = BinaryHeap::new();
    for _i in 0..Q {
        input! {
            q1:i32,
        }
        if q1 == 1 {
            input! {
                v:i64,
            }
            q.push(MAX - v);
        }
        if q1 == 2 {
            if q.len() > 0 {
                let v = MAX - q.pop().unwrap();
                println!("{}", v);
                q.push(MAX - v);
            }
        }
        if q1 == 3 {
            if q.len() > 0 {
                q.pop();
            }
        }
    }
}
