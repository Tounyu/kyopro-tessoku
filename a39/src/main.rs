use proconio::input;

fn main() {
    input! {
        N:usize,
        mut LR:[(i64,i64);N],
    }

    LR.sort_by(|&a, &b| a.1.cmp(&b.1));

    let mut time = 0;
    let mut ans = 0;
    for (l, r) in LR {
        if time <= l {
            time = r;
            ans += 1;
        }
    }

    println!("{}", ans);
}
