use proconio::input;

fn main() {
    input! {
        n:usize,
        _h:usize,
        _w:usize,
        ab:[(usize, usize); n],
    }

    let mut ans = (ab[0].0 - 1) ^ (ab[0].1 - 1);
    for i in 1..n {
        ans ^= ab[i].0 - 1;
        ans ^= ab[i].1 - 1;
    }

    if ans != 0 {
        println!("First");
    } else {
        println!("Second");
    }
}
