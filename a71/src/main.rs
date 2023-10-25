use proconio::input;

fn main() {
    input! {
        n:usize,
        mut a:[usize;n],
        mut b:[usize;n],
    }

    a.sort();
    b.sort();

    let mut ans = 0usize;
    for i in 0..n {
        ans += a[i] * b[n - 1 - i];
    }

    println!("{}", ans);
}
