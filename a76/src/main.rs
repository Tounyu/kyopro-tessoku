use proconio::input;

fn main() {
    input! {
        n:usize,
        w:i64,
        l:i64,
        r:i64,
       mut x:[i64; n],
    }
    x.insert(0, 0);
    x.push(w);

    let mut dp = vec![0; n + 2];
    let mut sum = vec![0; n + 2];
    let m = 1000000007;

    dp[0] = 1;
    sum[0] = 1;
    for i in 1..=n + 1 {
        let posl = lower_bound(&x, x[i] - r) as usize;
        let mut posr = lower_bound(&x, x[i] - l + 1) as usize;

        if posr > 0 {
            posr -= 1;
            dp[i] = sum[posr];
        } else {
            dp[i] = 0;
        }
        if posl > 0 {
            dp[i] -= sum[posl - 1];
        }
        dp[i] = (dp[i] + m) % m;

        sum[i] = sum[i - 1] + dp[i];
        sum[i] %= m;
    }
    println!("{}", dp[n + 1]);
}

fn lower_bound(v: &Vec<i64>, val: i64) -> i64 {
    let mut l = 0;
    let mut r = v.len();
    while l < r {
        let mid = (l + r) / 2;
        if v[mid] < val {
            l = mid + 1;
        } else {
            r = mid;
        }
    }
    l as i64
}
