use std::cmp::Reverse;
use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        abcd:[(usize,usize,usize,usize); m],
    }

    let mut g = vec![vec![]; n];
    for (a, b, c, d) in abcd {
        g[a - 1].push((b - 1, c * 10000 - d));
        g[b - 1].push((a - 1, c * 10000 - d));
    }
    let mut kakutei = vec![false; n];
    let mut cur = vec![0; n];
    for i in 0..n {
        cur[i] = 1 << 60;
    }
    let mut q = BinaryHeap::new();

    cur[0] = 0usize;
    q.push(Reverse((cur[0], 0usize)));

    while !q.is_empty() {
        let pos = q.pop().unwrap().0.1;
        if kakutei[pos] {
            continue;
        }

        kakutei[pos] = true;
        for i in 0..g[pos].len() {
            let next = g[pos][i].0;
            let cost = g[pos][i].1;
            if cur[next] > cur[pos] + cost {
                cur[next] = cur[pos] + cost;
                q.push(Reverse((cur[next], next)));
            }
        }
    }


    let dist = (cur[n - 1] + 9999) / 10000;
    let num = dist * 10000 - cur[n - 1];
    println!("{} {}", dist, num);
}
