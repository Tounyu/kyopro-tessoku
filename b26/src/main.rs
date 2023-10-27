use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    let mut p = vec![1usize; n + 1];
    for i in 2..=n {
        if p[i] == 0 {
            continue;
        }
        let mut j = 2usize;
        while j * i <= n {
            p[j * i] = 0usize;
            j += 1;
        }
    }

    for i in 2..=n {
        if p[i] > 0 {
            println!("{}", i);
        }
    }
}
