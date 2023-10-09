use proconio::input;

fn main() {
    input! {
        a:i64,
        b:i64,
    }

    let ans = a * b / gcd(a, b);

    println!("{}", ans);
}

fn gcd(x: i64, y: i64) -> i64 {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}
