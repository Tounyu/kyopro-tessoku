use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[i32; n],
    }

    let mut l = vec![];
    for av in a.into_iter() {
        let pos = l.partition_point(|&x| x < av);
        if pos < l.len() {
            l[pos] = av;
        } else {
            l.push(av);
        }
    }

    println!("{}", l.len());
}
