use proconio::input;

fn main() {
    input! {
        N:usize,
        Q:usize,
        A:[usize; N],
        XY:[(usize,usize); Q],
    }

    let mut dp = vec![vec![0usize; N]; 30];
    for j in 0..N {
        dp[0][j] = A[j] - 1;
    }

    for i in 1..30 {
        for j in 0..N {
            dp[i][j] = dp[i - 1][dp[i - 1][j]];
        }
    }

    for (x, y) in XY {
        let mut cx = x - 1;
        for d in (0..30).rev() {
            if (y / (1 << d)) % 2 != 0 {
                cx = dp[d][cx];
            }
        }
        println!("{}", cx + 1);
    }
}
