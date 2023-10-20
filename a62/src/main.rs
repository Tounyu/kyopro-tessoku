use proconio::input;

fn main() {
    input! {
        N:usize,
        M:usize,
        AB:[(usize, usize); M],
    }
    let mut G = vec![vec![]; N];
    for (a, b) in AB {
        G[a - 1].push(b - 1);
        G[b - 1].push(a - 1);
    }
    let mut visited = vec![false; N];
    dfs(&G, &mut visited, 0);
    for visited in visited {
        if !visited {
            println!("The graph is not connected.");
            return;
        }
    }
    println!("The graph is connected.");
}

fn dfs(G: &Vec<Vec<usize>>, visited: &mut Vec<bool>, pos: usize) {
    visited[pos] = true;
    for i in 0..G[pos].len() {
        let next = G[pos][i];
        if !visited[next] {
            dfs(G, visited, next);
        }
    }
    return;
}