use proconio::input;

fn main() {
    input! {
        n: usize,
        abcd:[(usize,usize,usize,usize); n],
    }

    let size = 2000;
    let mut cumsum = vec![vec![0; size]; size];

    for (a, b, c, d) in abcd {
        cumsum[a + 1][b + 1] += 1;
        cumsum[a + 1][d + 1] -= 1;
        cumsum[c + 1][b + 1] -= 1;
        cumsum[c + 1][d + 1] += 1;
    }

    for x in 1..size {
        for y in 1..size {
            cumsum[x][y] += cumsum[x][y - 1];
        }
    }

    for y in 1..size {
        for x in 1..size {
            cumsum[x][y] += cumsum[x - 1][y];
        }
    }

    let mut ans = 0;
    for y in 1..size {
        for x in 1..size {
            if cumsum[x][y] > 0 {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
