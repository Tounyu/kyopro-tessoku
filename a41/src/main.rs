use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        N:usize,
        S:Chars,
    }

    for i in 0..S.len() - 2 {
        if S[i] == 'R' && S[i + 1] == 'R' && S[i + 2] == 'R' {
            println!("Yes");
            return;
        }
        if S[i] == 'B' && S[i + 1] == 'B' && S[i + 2] == 'B' {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
