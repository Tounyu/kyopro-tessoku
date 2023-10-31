use ac_library::MfGraph;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n:usize,
        m:usize,
        c:[Chars; n],
    }

    let mut f = MfGraph::new(n + 24 + 2);

    let s = n + 24;
    let t = n + 24 + 1;

    for i in 0..n {
        f.add_edge(s, i, 10);
        for j in 0..24 {
            f.add_edge(i, n + j, c[i][j].to_digit(10).unwrap() as usize);
        }
    }
    for j in 0..24 {
        f.add_edge(n + j, t, m);
    }

    let mf = f.flow(s, t);
    if mf >= m * 24 {
        println!("Yes");
    } else {
        println!("No");
    }
}
