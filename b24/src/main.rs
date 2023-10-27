use proconio::input;

fn main() {
    input! {
        n:usize,
        mut xy:[(usize,usize); n],
    }

    xy.sort_by(|&a, &b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)));

    let mut y = vec![];
    for i in 0..n {
        let pos = y.partition_point(|&x| x < xy[i].1);
        if pos < y.len() {
            y[pos] = xy[i].1;
        } else {
            y.push(xy[i].1);
        }
    }
    // println!("{:?}", y);
    println!("{}", y.len());
}
