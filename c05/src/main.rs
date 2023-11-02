use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    let mut s = format!("{:010b}", n - 1);
    s = s.replace('0', "4");
    s = s.replace('1', "7");
    println!("{s}");
}
