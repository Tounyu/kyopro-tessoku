use std::collections::VecDeque;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h:usize,
        w:usize,
        s:(usize,usize),
        g:(usize,usize),
        c:[Chars; h],
    }

    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    let mut dist = vec![1000000000usize; h * w];
    q.push_back((s.0 - 1, s.1 - 1));
    dist[idx(s.0 - 1, s.1 - 1, w)] = 0;
    while !q.is_empty() {
        let (y, x) = q.pop_front().unwrap();
        for (dx, dy) in [(-1i64, 0i64), (1, 0), (0, -1), (0, 1)] {
            let nx = x as i64 + dx;
            let ny = y as i64 + dy;

            if nx < 0 || (w as i64) - 1 < nx {
                continue;
            }
            if ny < 0 || (h as i64) - 1 < ny {
                continue;
            }

            let nxu = nx as usize;
            let nyu = ny as usize;

            if c[nyu][nxu] == '.' {
                let temp = dist[idx(nyu, nxu, w)];
                if dist[idx(y, x, w)] + 1 < temp {
                    dist[idx(nyu, nxu, w)] = dist[idx(y, x, w)] + 1;
                    q.push_back((nyu, nxu));
                }
            }
        }
    }
    println!("{}", dist[idx(g.0 - 1, g.1 - 1, w)]);
}

fn idx(y: usize, x: usize, w: usize) -> usize {
    return y * w + x;
}
