use std::cmp::max;

use proconio::input;

fn main() {
    input! {
        n:usize,
        t:usize,
        ab:[(usize,usize); n - 1],
    }

    let mut g = vec![vec![]; n];
    for (a, b) in ab {
        g[a - 1].push(b - 1);
        g[b - 1].push(a - 1);
    }

    let mut dp = vec![0usize; n];
    let mut visited = vec![false; n];

    dfs(&g, &mut visited, &mut dp, t - 1);

    for dp in dp {
        print!("{} ", dp);
    }
    println!();
}

fn dfs(g: &Vec<Vec<usize>>, visited: &mut Vec<bool>, dp: &mut Vec<usize>, pos: usize) -> usize {
    visited[pos] = true;

    // println!("{}", pos);

    let mut kaikyu = 0usize;
    for &next in &g[pos] {
        if visited[next] {
            continue;
        }
        let temp = dfs(g, visited, dp, next);
        kaikyu = max(kaikyu, temp + 1);
    }
    dp[pos] = kaikyu;
    return kaikyu;
}
