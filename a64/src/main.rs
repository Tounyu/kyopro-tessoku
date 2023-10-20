use std::cmp::Reverse;
use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        N:usize,
        M:usize,
        ABC:[(usize, usize, usize); M],
    }

    let mut G = vec![vec![]; N];
    for (a, b, c) in ABC {
        G[a - 1].push((b - 1, c));
        G[b - 1].push((a - 1, c));
    }

    let mut kakutei = vec![false; N];
    let mut cur = vec![2000_000_000; N];

    let mut q = BinaryHeap::new();

    cur[0] = 0usize;
    q.push(Reverse((cur[0], 0usize)));

    while !q.is_empty() {
        let pos = q.pop().unwrap().0.1;

        if kakutei[pos] {
            continue;
        }

        kakutei[pos] = true;
        for &(next, cost) in &G[pos] {
            if cur[next] > cur[pos] + cost {
                cur[next] = cur[pos] + cost;
                q.push(Reverse((cur[next], next)));
            }
        }
    }

    for cur in cur {
        if cur == 2000_000_000 {
            println!("-1");
        } else {
            println!("{}", cur);
        }
    }
}
