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
    let mut q = vec![];
    let mut dist = vec![-1i64; N];
    q.push(0);
    dist[0] = 0;
    while q.len() > 0 {
        let i = q.pop().unwrap();
        for &to in &G[i] {
            if dist[to] == -1 {
                dist[to] = dist[i] + 1;
                q.insert(0, to);
            }
        }
    }
    for dist in dist {
        println!("{}", dist);
    }
}
