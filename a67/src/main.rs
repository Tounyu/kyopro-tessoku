use ac_library::Dsu;
use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        mut abc:[(usize,usize,usize); m],
    }

    let mut dsu = Dsu::new(n + 1);
    abc.sort_by_key(|&(_, _, c)| c);
    let mut ans = 0usize;
    for &(a, b, c) in &abc {
        if !dsu.same(a, b) {
            dsu.merge(a, b);
            ans += c;
        }
    }

    println!("{}", ans);
}
