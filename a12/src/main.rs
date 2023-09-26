use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        a:[usize; n],
    }

    let mut l = 0;
    let mut r = 1_000_000_000;

    while l < r {
        let m = (l + r) / 2;

        let mut sum = 0;
        for i in 0..n {
            sum += m / a[i];
        }

        if k <= sum {
            r = m;
        } else {
            l = m + 1;
        }
    }

    println!("{}", l);
}
