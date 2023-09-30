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

    if dp[n][s] == 0 {
        println!("-1");
        return;
    }

    let mut ans = vec![];
    let mut j = s;
    for i in (1..=n).rev() {
        // println!("i={},j={}", i, j);
        if dp[i - 1][j] > 0 {
            // jそのまま
        } else if dp[i - 1][j - a[i]] > 0 {
            ans.push(i);
            j -= a[i];
        }
    }

    let ans_s: Vec<String> = ans.iter().rev().map(|x| (x).to_string()).collect();
    println!("{}", ans_s.len());
    println!("{}", ans_s.join(" "));
}
