use proconio::input;

fn main() {
    input! {
        a:i64,
        b:i64,
    }

    println!("{}", gcd(a, b));
}

fn gcd(x: i64, y: i64) -> i64 {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}
