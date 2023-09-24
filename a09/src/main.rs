use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        abcd:[(usize,usize,usize,usize); n],
    }
    let size: usize = 1500;
    let mut cumsum = vec![vec![0; size + 2]; size + 2];

    for (a, b, c, d) in abcd {
        cumsum[a][b] += 1;
        cumsum[c + 1][d + 1] += 1;
        cumsum[c + 1][b] -= 1;
        cumsum[a][d + 1] -= 1;
    }

    for hi in 1..=size + 1 {
        for wi in 1..=size + 1 {
            cumsum[hi][wi] += cumsum[hi][wi - 1];
        }
    }

    for wi in 1..=size + 1 {
        for hi in 1..=size + 1 {
            cumsum[hi][wi] += cumsum[hi - 1][wi];
        }
    }

    for hi in 1..=h {
        for wi in 1..=w {
            if wi == 1 {
                print!("{}", cumsum[hi][wi]);
            } else {
                print!(" {}", cumsum[hi][wi]);
            }
        }
        println!()
    }
}
