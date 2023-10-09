use proconio::input;

fn main() {
    input! {
        q:usize,
        x:[i64; q],
    }

    for vx in x {
        if is_prime(vx) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

fn is_prime(n: i64) -> bool {
    let m = (n as f64).sqrt().floor() as i64;
    for i in 2..=m {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}
