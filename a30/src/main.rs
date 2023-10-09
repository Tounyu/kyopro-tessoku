use proconio::input;

fn main() {
    input! {
        n:i64,
        r:i64,
    }

    let m = 10i64.pow(9) + 7;

    let mut a = 1i64;
    let mut b = 1i64;

    for i in 1..=n {
        a = (a * i) % m;
    }
    for i in 1..=r {
        b = (b * i) % m;
    }
    for i in 1..=n - r {
        b = (b * i) % m;
    }

    println!("{}", div(a, b, m));
}

fn div(a: i64, b: i64, m: i64) -> i64 {
    return (a * pow(b, m - 2, m)) % m;
}

fn pow(a: i64, b: i64, m: i64) -> i64 {
    let mut p = a;
    let mut ans = 1;

    let mut q = 1;
    while 2i64.pow(q) < 10i64.pow(18) {
        q += 1;
    }

    for i in 0..q {
        let w = 1 << i;
        if (b / w) % 2 == 1 {
            ans = (ans * p) % m;
        }
        p = (p * p) % m;
    }
    return ans;
}
