use ac_library::Dsu;
use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
       mut abc:[(usize,usize,usize); m],
    }

    let mut dsu = Dsu::new(n + 1);
    let mut sum = 0usize;
    abc.sort_by_key(|&(a, b, c)| c);
    for &(a, b, c) in abc.iter().rev() {
        if !dsu.same(a, b) {
            dsu.merge(a, b);
            sum += c;
        }
    }

    println!("{sum}");
}
