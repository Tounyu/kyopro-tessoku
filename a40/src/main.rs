use proconio::input;

fn main() {
    input! {
        N:usize,
        A:[usize; N],
    }

    let mut sum = vec![0i64; 101];
    for a in A {
        sum[a] += 1;
    }

    let mut ans = 0;
    // nC3
    for i in 1..sum.len() {
        ans += sum[i] * (sum[i] - 1) * (sum[i] - 2) / 6;
    }

    println!("{}", ans);
}
