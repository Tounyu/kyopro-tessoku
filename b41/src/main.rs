use proconio::input;

fn main() {
    input! {
        X:u64,
        Y:u64,
    }

    let mut x = X;
    let mut y = Y;
    let mut memo = vec![];
    while x > 1 || y > 1 {
        memo.push((x, y));
        if x > y {
            x -= y;
        } else {
            y -= x;
        }
    }

    println!("{}", memo.len());
    for &(x, y) in memo.iter().rev() {
        println!("{} {}", x, y);
    }
}
