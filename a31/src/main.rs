use proconio::input;

fn main() {
    input! {
        n:i64,
    }

    let a = n / 3;
    let b = n / 5;
    let c = n / 7;

    let d = n / 15;
    let e = n / 21;
    let f = n / 35;

    let g = n / 105;

    println!("{}", a + b + c - d - e - f + g);
}
