use proconio::input;

fn main() {
    input! {
        N:usize,
        A:[usize; N - 1],
    }
    let mut G = vec![vec![]; N];
    for i in 0..N - 1 {
        G[A[i] - 1].push(i + 1);
    }
    // println!("{:?}", G);
    let mut dp = vec![0; N];
    for i in (0..N).rev() {
        for j in 0..G[i].len() {
            dp[i] += (dp[G[i][j]] + 1);
        }
    }
    for dp in dp {
        print!("{} ", dp);
    }
    println!();
}
