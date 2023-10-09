use proconio::input;

fn main() {
    input! {
        N:i64,
    }

    let mut sum = vec![vec![0; 10]; 18];
    let mut pow = vec![1i64];
    for i in 1..=16 {
        pow.push(pow[i - 1] * 10);
    }

    for i in 0..=15 {
        let m = N / pow[i] % 10;

        for j in 0..10 {
            if j < m {
                sum[i][j as usize] = (N / pow[i + 1] + 1) * pow[i];
            } else if j == m {
                sum[i][j as usize] = (N / pow[i + 1]) * pow[i] + (N % pow[i]) + 1;
            } else {
                sum[i][j as usize] = (N / pow[i + 1]) * pow[i];
            }
        }
    }

    let mut ans = 0;
    for i in 0..=15 {
        for j in 0..10 {
            ans += j as i64 * sum[i][j];
        }
    }

    println!("{}", ans);
}
