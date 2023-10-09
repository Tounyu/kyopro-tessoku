use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n:i64,
        k:i64,
        s:Chars,
    }

    let sum = s.iter().fold(0, |acc, &c| if c == '1' { acc + 1 } else { acc });

    if sum % 2 == k % 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
