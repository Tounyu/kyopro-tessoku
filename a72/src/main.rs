use std::cmp::min;

use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h:usize,
        w:usize,
        K:usize,
        c:[Chars; h],
    }

    let mut hc = vec![];
    for i in 0..h {
        let mut count = 0usize;
        for j in 0..w {
            if c[i][j] == '.' {
                count += 1;
            }
        }
        hc.push((count, i));
    }
    hc.sort_by_key(|&a| a.0);

    let mut white = w * h;

    // 消す行を増やしながら全探索
    for k in 0..=K {
        for comb in (0..h).combinations(h - k) {
            let remain = K - k;
            // 消さなかった行
            // println!("{}", remain);
            // println!("{:?}", comb);

            let mut wc = vec![];
            for j in 0..w {
                let mut count = 0usize;
                for &i in &comb {
                    if c[i][j] == '.' {
                        count += 1;
                    }
                }
                wc.push((count, j));
            }
            wc.sort_by_key(|&a| a.0);
            let mut temp = 0usize;
            // println!("{:?}", wc);
            for i in 0..wc.len() - remain {
                temp += wc[i].0;
            }
            // println!("{}", temp);
            white = min(white, temp);
        }
    }

    println!("{:?}", w * h - white);
}
