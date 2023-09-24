use proconio::input;

fn main() {
    input! {
        h:usize,
        w:usize,
        x:[[usize; w]; h],
        q:usize,
        abcd:[(usize,usize,usize,usize); q],
    }

    let mut cumsum = vec![vec![0; w + 1]; h + 1];

    for hi in 1..=h {
        for wi in 1..=w {
            cumsum[hi][wi] = cumsum[hi][wi - 1] + x[hi - 1][wi - 1];
        }
    }

    for wi in 1..=w {
        for hi in 1..=h {
            cumsum[hi][wi] = cumsum[hi - 1][wi] + cumsum[hi][wi];
        }
    }

    for (a, b, c, d) in abcd {
        let ans = cumsum[c][d] + cumsum[a - 1][b - 1] - cumsum[a - 1][d] - cumsum[c][b - 1];
        println!("{}", ans);
    }
}
