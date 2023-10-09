use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h:usize,
        w:usize,
        c:[Chars; h],
    }

    let mut dp = vec![vec![0i64; w]; h];

    for ih in 0..h {
        for iw in 0..w {
            if ih == 0 && iw == 0 {
                dp[ih][iw] = 1;
                continue;
            }

            if iw != 0 && c[ih][iw - 1] == '.' {
                dp[ih][iw] += dp[ih][iw - 1];
            }
            if ih != 0 && c[ih - 1][iw] == '.' {
                dp[ih][iw] += dp[ih - 1][iw];
            }
        }
    }

    println!("{}", dp[h - 1][w - 1]);
}
