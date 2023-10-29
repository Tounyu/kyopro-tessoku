use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        ab:[(usize,usize);m],
    }

    let mut g = vec![vec![]; n + 1];
    for &(a, b) in &ab {
        g[a].push(b);
        g[b].push(a);
    }
    let mut visited = vec![false; n + 1];
    let mut ans = vec![];
    dfs(1, n, &g, &mut visited, &mut ans);

    for n in ans.iter().rev() {
        print!("{} ", n);
    }
    println!();
}

fn dfs(now: usize, goal: usize, g: &Vec<Vec<usize>>, visited: &mut Vec<bool>, ans: &mut Vec<usize>) -> bool {
    visited[now] = true;
    if now == goal {
        ans.push(now);
        return true;
    }

    for &next in &g[now] {
        if !visited[next] && dfs(next, goal, g, visited, ans) {
            ans.push(now);
            return true;
        }
    }

    return false;
}
