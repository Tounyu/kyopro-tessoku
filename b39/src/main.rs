use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        N:usize,
        D:usize,
        XY:[(usize,i64); N],
    }

    // XY.sort_by(|a, b| a.0.cmp(&b.0));

    let mut sum = 0;
    // for d in 1..D + 1 {
    //     let ymax = XY.iter().fold(0, |acc, &(x, y)| if x <= d { max(acc, y) } else { acc });
    //
    //     if ymax > 0 {
    //         let item = XY.iter().enumerate().find(|&(index, x)| x.1 == ymax).unwrap();
    //         XY.remove(item.0);
    //         sum += ymax;
    //     }
    // }

    let mut q = BinaryHeap::new();
    for d in 1..D + 1 {
        for &(x, y) in &XY {
            if x == d {
                q.push(y);
            }
        }
        if q.len() > 0 {
            sum += q.pop().unwrap();
        }
    }

    println!("{}", sum);
}
