use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        a:[usize; k],
    }

    let mut dp: Vec<bool> = vec![];

    for i in 0..=n {
        let mut first = false;
        for &va in &a {
            if i >= va {
                first = first || !dp[i - va];
            }
        }
        dp.push(first);
        // println!("{:?}", dp);
    }

    println!("{}", if dp[n] { "First" } else { "Second" });
}
