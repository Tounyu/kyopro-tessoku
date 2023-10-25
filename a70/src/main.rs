use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        a:[usize;n],
        mut xyz:[(usize,usize,usize); m]
    }

    let mut g = vec![vec![]; 1033];

    for i in 0..(1 << n) {
        for j in 0..m {
            let next_state = get_next(i, j, &mut xyz, n);
            g[i].push(next_state);
        }
    }

    let mut goal = (1 << n) - 1;
    let mut start = 0;
    for i in 0..n {
        if a[i] == 1 {
            start += 1 << i;
        }
    }

    let mut dist = vec![-1i64; 1033];
    let mut q = VecDeque::new();
    dist[start] = 0;
    q.push_back(start);
    while !q.is_empty() {
        let pos = q.pop_front().unwrap();
        for i in 0..g[pos].len() {
            let next = g[pos][i];
            if dist[next] == -1 {
                dist[next] = dist[pos] + 1;
                q.push_back(next);
            }
        }
    }
    println!("{}", dist[goal]);
}

fn get_next(pos: usize, index: usize, xyz: &mut Vec<(usize, usize, usize)>, n: usize) -> usize {
    let mut state = vec![0; 19];
    for i in 0..n {
        let wari = (1 << i);
        state[i] = (pos / wari) % 2;
    }
    state[xyz[index].0 - 1] = 1 - state[xyz[index].0 - 1];
    state[xyz[index].1 - 1] = 1 - state[xyz[index].1 - 1];
    state[xyz[index].2 - 1] = 1 - state[xyz[index].2 - 1];

    let mut ret = 0;
    for i in 0..n {
        if state[i] == 1 {
            ret += 1 << i;
        }
    }
    ret
}
