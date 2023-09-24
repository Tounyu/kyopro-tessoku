use proconio::input;

fn main() {
    let size: usize = 1500;
    input! {
        n:usize,
        xy:[(usize,usize); n],
        q:usize,
        abcd:[(usize,usize,usize,usize); q],
    }

    let mut cumsum = vec![vec![0; size + 1]; size + 1];

    for (x, y) in xy {
        cumsum[x][y] += 1;
    }

    for xi in 1..=size {
        for yi in 1..=size {
            cumsum[xi][yi] += cumsum[xi][yi - 1];
        }
    }

    for yi in 1..=size {
        for xi in 1..=size {
            cumsum[xi][yi] += cumsum[xi - 1][yi];
        }
    }

    for (a, b, c, d) in abcd {
        let ans = cumsum[c][d] + cumsum[a - 1][b - 1] - cumsum[a - 1][d] - cumsum[c][b - 1];
        println!("{}", ans);
    }
}
