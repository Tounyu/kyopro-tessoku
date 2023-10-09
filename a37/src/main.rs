use proconio::input;

fn main() {
    input! {
        N:i64,
        M:i64,
        B:i64,
        A:[i64; N],
        C:[i64; M],
    }

    let mut ans = 0i64;

    for a in A {
        ans += a * M;
    }
    for c in C {
        ans += c * N;
    }
    ans += B * N * M;

    println!("{}", ans);
}
