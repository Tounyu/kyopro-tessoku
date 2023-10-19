use proconio::input;

fn main() {
    input! {
        N:usize,
        A:[usize; N]
    }

    let mut s = vec![];
    println!("-1");
    for i in 1..N {
        s.push((i + 1, A[i - 1]));
        while s.len() > 0 && s.last().unwrap().1 <= A[i] {
            s.pop();
        }
        if s.len() > 0 {
            println!("{}", s.last().unwrap().0 - 1)
        } else {
            println!("-1");
        }
    }
}
