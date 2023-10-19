use ac_library::{Additive, Segtree};
use proconio::input;

fn main() {
    input! {
        N:usize,
        Q:usize,
    }

    let mut segtree = Segtree::<Additive<usize>>::new(N + 1);
    for i in 1..=N {
        segtree.set(i, 0usize);
    }

    for i in 0..Q {
        input! {
            q:i64,
            v1:usize,
            v2:usize,
        }
        if q == 1 {
            segtree.set(v1, v2);
        }
        if q == 2 {
            println!("{}", segtree.prod(v1..v2));
        }
    }
}
