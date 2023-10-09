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
