use proconio::input;

fn main() {
    input! {
        N:usize,
        M:usize,
        A:[usize; M],
    }

    let mut ans = vec![M; N];
    for a in A {
        ans[a - 1] -= 1;
    }

    for ans in ans {
        println!("{}", ans);
    }
}
