use ac_library::MfGraph;
use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        p:[i64; n],
        ab:[(usize, usize); m],
    }

    let mut f = MfGraph::new(n + 2);
    // n == s
    // n+1 == t
    let mut m = 0;  // pを組み合わせたときの最大値
    for i in 0..n {
        if p[i] >= 0 {
            f.add_edge(n, i, p[i]);
            m += p[i];
        } else {
            f.add_edge(i, n + 1, -p[i]);
        }
    }

    for (a, b) in ab {
        f.add_edge(a - 1, b - 1, 1000_000_000);
    }

    println!("{}", m - f.flow(n, n + 1));
}
