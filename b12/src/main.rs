use proconio::input;

fn main() {
    input! {
        n:f64,
    }

    let eps: f64 = 0.001;

    let mut l: f64 = 0.0;
    let mut r: f64 = 100000.0;

    loop {
        let mid = (l + r) / 2.0;
        let a = mid.powf(3.0) + mid;

        if (a - n).abs() < eps {
            println!("{}", mid);
            return;
        }

        if a < n {
            l = mid;
        } else {
            r = mid;
        }
    }
}
