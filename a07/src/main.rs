use proconio::input;

fn main() {
    input! {
        d:usize,
        n:i32,
    }
    let mut a = vec![0; d + 1];

    for _i in 0..n {
        input! {
            l:usize,
            r:usize,
        }

        a[l - 1] += 1;
        a[r] -= 1;
    }

    let mut sum = 0;
    for i in 0..d {
        sum += a[i];
        println!("{}", sum);
    }
}
