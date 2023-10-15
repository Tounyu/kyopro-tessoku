use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        N:usize,
        C:char,
        A:Chars,
    }

    let mut sum = 0;
    for a in A {
        if a == 'W' {
            sum += 0;
        }
        if a == 'B' {
            sum += 1;
        }
        if a == 'R' {
            sum += 2;
        }
    }
    if sum % 3 == 0 && C == 'W' {
        println!("Yes");
        return;
    }
    if sum % 3 == 1 && C == 'B' {
        println!("Yes");
        return;
    }
    if sum % 3 == 2 && C == 'R' {
        println!("Yes");
        return;
    }
    println!("No");
}
