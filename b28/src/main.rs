use proconio::input;

fn main() {
    input! {
        n:i64,
    }

    let d = 10i64.pow(9) + 7;

    let mut a1 = 1;
    let mut a2 = 1;
    let mut ans = 0;

    for _i in 3..=n {
        ans = a1 + a2;
        ans %= d;

        a1 = a2;
        a2 = ans;
    }

    println!("{}", ans);
}
