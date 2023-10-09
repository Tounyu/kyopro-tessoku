use proconio::input;

fn main() {
    input! {
        a:i64,
        b:i64,
    }

    println!("{}", pow(a, b, 10i64.pow(9) + 7));
}

fn pow(a: i64, b: i64, m: i64) -> i64 {
    let mut p = a;
    let mut ans = 1;
    for i in 0..30 {
        let w = 1 << i;
        if (b / w) % 2 == 1 {
            ans = (ans * p) % m;
        }
        p = (p * p) % m;
    }
    return ans;
}
