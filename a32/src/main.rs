use proconio::input;

fn main() {
    input! {
        n:usize,
        a:usize,
        b:usize,
    }

    let mut dp: Vec<bool> = vec![];


    for i in 0..=n {
        let mut first = false;
        if i >= a {
            first = first || !dp[i - a];
        }
        if i >= b {
            first = first || !dp[i - b];
        }
        dp.push(first);
        // println!("{:?}", dp);
    }

    println!("{}", if dp[n] { "First" } else { "Second" });
}
