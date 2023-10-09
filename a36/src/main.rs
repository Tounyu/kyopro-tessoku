use proconio::input;

fn main() {
    input! {
        n:i64,
        k:i64,
    }

    if 2 * n - 2 > k || k % 2 == 1 {
        println!("No");
        return;
    }

    println!("Yes");
}
