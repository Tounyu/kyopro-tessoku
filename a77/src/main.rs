use proconio::input;

fn main() {
    input! {
        n:usize,
        len:usize,
        k:usize,
        a:[usize;n],
    }
    let mut l = 0usize;
    let mut r = 1000000000usize;
    while l < r {
        let m = (r + l + 1) / 2;
        let ans = check(&a, m, k, len);
        if ans {
            l = m;
        } else {
            r = m - 1;
        }
    }

    println!("{}", l);
}

fn check(a: &Vec<usize>, x: usize, k: usize, l: usize) -> bool {
    let mut c = 0usize;
    let mut last_kireme = 0usize;
    for i in 0..a.len() {
        if a[i] - last_kireme >= x && l - a[i] >= x {
            c += 1;
            last_kireme = a[i];
        }
    }
    return c >= k;
}