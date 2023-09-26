use proconio::input;

fn main() {
    input! {
        n:usize,
        x:usize,
        a:[usize; n],
    }

    let mut l = 0;
    let mut r = n - 1;
    let mut m = (l + r) / 2;
    while l <= r {
        m = (l + r) / 2;
        if x < a[m] {
            r = m - 1;
        } else if a[m] < x {
            l = m + 1;
        } else {
            break;
        }
    }

    println!("{}", m + 1);
}
