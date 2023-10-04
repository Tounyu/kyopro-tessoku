use proconio::input;

fn calc_dist(from: (i32, i32), to: (i32, i32)) -> f64 {
    (((from.0 - to.0).pow(2) + (from.1 - to.1).pow(2)) as f64).sqrt()
}

fn main() {
    input! {
        n:usize,
        xy:[(i32, i32); n],
    }

    let mut dp: Vec<Vec<f64>> = vec![vec![1_000_000_000.0; n]; 1 << n];

    for i in 0..n {
        let dist = calc_dist(xy[0], xy[i]);
        dp[1 << i][i] = dist;
    }
    // println!("{:?}", dp[0]);

    for mask in 1..1 << n {
        for i in 0..n {
            if (mask & (1 << i)) != 0 {
                for j in 0..n {
                    let dist = calc_dist(xy[i], xy[j]);
                    if (mask & (1 << j)) == 0 || dist != 0.0 {
                        let mask_j = mask | (1 << j);
                        dp[mask_j][j] = dp[mask_j][j].min(dp[mask][i] + dist);
                    }
                }
            }
        }
        // println!("{:?}", dp[mask]);
    }

    let mask_ans = (1 << n) - 1;
    let mut minimum: f64 = 1_000_000_000.0;
    for i in 1..n {
        let dist = calc_dist(xy[i], xy[0]);
        if dist != 0.0 {
            minimum = minimum.min(dp[mask_ans][i] + dist);
        }
    }

    println!("{:.12}", minimum);
}
