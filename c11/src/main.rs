use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        a:[usize;n],
    }

    let nn = 1_000_000_000f64;
    let mut r = nn;
    let mut l = 1f64;

    let mut border = 0f64;

    for _i in 0..60 {
        // println!("{l},{r}");
        let mid = (l + r) / 2.0;
        let count = count(&a, mid, n);
        if count < k {
            r = mid;
        } else {
            l = mid;
            if mid > border {
                border = mid;
            }
        }
    }

    for a in a {
        print!("{} ", (a as f64 / border) as usize);
    }

    println!();
}

fn count(a: &Vec<usize>, mid: f64, n: usize) -> usize {
    let mut count = 0usize;
    for i in 0..n {
        count += (a[i] as f64 / mid) as usize;
    }
    count
}
