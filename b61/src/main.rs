use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        ab:[(usize,usize);m],
    }

    let mut g = vec![vec![]; n + 1];
    let mut id = ab[0].0;
    let mut max = 0usize;
    for &(a, b) in &ab {
        g[a].push(b);
        g[b].push(a);
        if max < g[a].len() {
            max = g[a].len();
            id = a;
        }
        if max < g[b].len() {
            max = g[b].len();
            id = b;
        }
    }

    println!("{}", id);
}
