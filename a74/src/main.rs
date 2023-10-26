use proconio::input;

fn main() {
    input! {
        n:usize,
        p:[[usize; n]; n],
    }
    let mut h = vec![];
    let mut w = vec![];
    for i in 0..n {
        for j in 0..n {
            if p[i][j] > 0 {
                h.push(p[i][j]);
            }
            if p[j][i] > 0 {
                w.push(p[j][i]);
            }
        }
    }

    let mut ans = 0usize;
    for i in 0..n {
        for j in i + 1..n {
            if h[i] > h[j] {
                ans += 1;
            }
            if w[i] > w[j] {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
