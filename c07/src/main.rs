use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n:usize,
        mut c:[usize;n],
        q:usize,
        x:[usize;q],
    }
    c.sort();
    for i in 1..n {
        c[i] += c[i - 1];
    }

    for x in x {
        let l = c.lower_bound(&(x + 1));
        println!("{}", l);
    }
}
