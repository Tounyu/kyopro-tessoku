use ac_library::Dsu;
use proconio::input;

fn main() {
    input! {
        n:usize,
        q:usize,
    }

    let mut dsu = Dsu::new(n);

    for _i in 0..q {
        input! {
            query:usize,
            u:usize,
            v:usize,
        }
        if query == 1 {
            dsu.merge(u - 1, v - 1);
        }
        if query == 2 {
            if dsu.same(u - 1, v - 1) {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
