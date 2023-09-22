use proconio::input;

fn main() {
    input! {
        n:i32,
        k:i32,
    }

    let mut ans = 0;
    for i in 1..=n {
        for j in 1..=n{
            let x = k - i - j;
            if 1 <= x && x <= n  {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
