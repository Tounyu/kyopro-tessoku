use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
    }

    let mut dp = vec![vec![0usize; n + 1]; 20];
    for j in 1..=n {
        let sum = j.to_string().chars().fold(0usize, |acc, c| acc + c.to_digit(10).unwrap() as usize);
        dp[0][j] = j - sum;
    }
    for i in 1..20 {
        for j in 0..=n {
            dp[i][j] = dp[i - 1][dp[i - 1][j]];
        }
    }

    for i in 1..=n {
        let mut cx = i;
        for d in (0..20).rev() {
            if (k / (1 << d)) % 2 != 0 {
                cx = dp[d][cx];
            }
            // println!("{}", cx);
        }
        println!("{}", cx);
    }
}
