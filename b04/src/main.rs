use proconio::input;

fn main() {
    input! {
        n:i32,
    }

    let mut ans = 0;
    let mut keta = 100000000;
    for _i in (0..=7).rev() {
        let x = 1 << _i;
        let y = n % keta;
        keta /= 10;
        ans += y / keta * x;
    }

    println!("{}", ans);
}
