use proconio::input;

fn main() {
    input! {
        n:usize,
        s:usize,
        mut a:[usize;n]
    }
    a.insert(0, 0);

    let mut dp = vec![vec![0; s + 1]; n + 1];
    dp[0][0] = 1;

    for i in 1..=n {
        for j in 0..=s {
            if j < a[i] {
                if dp[i - 1][j] > 0 {
                    dp[i][j] += 1;
                }
            } else {
                if dp[i - 1][j] > 0 || dp[i - 1][j - a[i]] > 0 {
                    dp[i][j] += 1;
                }
            }
        }
        // println!("{:?}", dp[i]);
    }
    
    if dp[n][s] > 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
