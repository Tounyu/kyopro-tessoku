use proconio::input;

fn main() {

    input! {
        n: usize,
    }
    input! {
        a:[i32; n],
    }

    for i in 0..=n-1 {
        for j in i+1..=n-1 {
            for k in j+1..=n-1 {
                if a[i]+a[j]+a[k] == 1000 {
                    print!("Yes");
                    return;
                }
            }
        }
    }

    print!("No");
}
