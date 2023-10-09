use proconio::input;

fn main() {
    input! {
        n:usize,
        ta:[(char,i64); n],
    }

    let mut ans = 0;

    for (t, a) in ta {
        if t == '+' {
            ans += a;
        } else if t == '-' {
            ans -= a;
        } else if t == '*' {
            ans *= a;
        }

        if ans < 0 {
            ans += 10000;
        }

        ans %= 10000;
        println!("{}", ans);
    }
}
