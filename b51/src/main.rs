use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        S:Chars,
    }

    let mut l = vec![];
    for i in 0..S.len() {
        let s = S[i];
        if s == '(' {
            l.push(i);
        }
        if s == ')' {
            let l = l.pop().unwrap();
            println!("{} {}", l + 1, i + 1);
        }
    }
}
