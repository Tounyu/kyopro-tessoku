use proconio::input;

fn main() {
    input! {
        n:usize,
    }

    println!("{n}");
    for i in 0..n - 1 {
        println!("{} {}", i + 1, i + 2);
    }
    println!("1 {n}");
}
