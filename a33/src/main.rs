use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[i64; n],
    }

    let mut ans = a[0];
    for i in 1..n {
        ans = ans ^ a[i];
    }

    if ans != 0 {
        println!("First");
    } else {
        println!("Second");
    }
}
