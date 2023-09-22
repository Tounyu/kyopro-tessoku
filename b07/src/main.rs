use proconio::input;

fn main() {
    input! {
        t:usize,
        n:usize,
    }

    let mut a = vec![0; t + 1];

    for _i in 0..n {
        input! {
            l:usize,
            r:usize,
        }
        a[l] += 1;
        a[r] -= 1;
    }

    let mut sum = 0;
    for i in 0..t {
        sum += a[i];
        println!("{}", sum);
    }
}
