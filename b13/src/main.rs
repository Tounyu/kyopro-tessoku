use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        a:[usize; n],
    }

    let mut cumsum = vec![0; n + 1];
    for i in 1..=n {
        if i == 1 {
            cumsum[i] = a[i - 1];
        } else {
            cumsum[i] = cumsum[i - 1] + a[i - 1];
        }
    }

    let mut r = vec![0; n + 1];
    for i in 1..=n {
        if i == 1 {
            r[i] = 0;
        } else {
            r[i] = r[i - 1];
        }
        while r[i] < n && cumsum[r[i] + 1] - cumsum[i - 1] <= k {
            r[i] += 1;
        }
    }

    let mut ans = 0;
    for i in 1..=n {
        ans += r[i] - i + 1;
    }
    println!("{}", ans);
}
