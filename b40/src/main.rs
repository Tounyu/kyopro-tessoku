use proconio::input;

fn main() {
    input! {
        N:usize,
        A:[usize; N],
    }

    let mut sum = vec![0i64; 100];
    for a in A {
        sum[a % 100] += 1;
    }

    let mut ans = 0;
    for i in 1..50 {
        ans += sum[i] * sum[100usize - i];
    }

    if sum[0] > 1 {
        ans += sum[0] * (sum[0] - 1) / 2;
    }
    if sum[50] > 1 {
        ans += sum[50] * (sum[50] - 1) / 2;
    }

    println!("{}", ans);
}
