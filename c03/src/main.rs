use proconio::input;

fn main() {
    input! {
        d:usize,
        x:i64,
        a:[i64; d -1],
        q:usize,
        st:[(usize,usize); q],
    }

    let mut dp = vec![0i64; d + 1];
    dp[1] = x;
    for i in 0..d - 1 {
        let idx = i + 2;
        dp[idx] = dp[idx - 1] + a[i];
    }

    for (s, t) in st {
        if dp[s] == dp[t] {
            println!("Same");
        } else {
            if dp[s] > dp[t] {
                println!("{}", s);
            } else {
                println!("{}", t);
            }
        }
    }
}
