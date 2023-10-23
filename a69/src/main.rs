use ac_library::maxflow;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n:usize,
        c:[Chars; n],
    }

    let mut z = maxflow::MfGraph::new(2 * n + 2);
    for i in 0..n {
        for j in 0..n {
            if c[i][j] == '#' {
                z.add_edge(i, n + j, 1);
            }
        }
    }
    for i in 0..n {
        z.add_edge(2 * n, i, 1);
        z.add_edge(n + i, 2 * n + 1, 1);
    }

    println!("{}", z.flow(2 * n, 2 * n + 1));
}
