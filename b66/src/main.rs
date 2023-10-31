use ac_library::Dsu;
use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        ab:[(usize,usize); m],
        q:usize,
    }

    let mut dsu = Dsu::new(n + 1);
    let mut add = vec![true; m];
    let mut qq = vec![];

    for _i in 0..q {
        input! {
            query:usize,
        }
        if query == 1 {
            input! {
                x:usize,
            }
            qq.push((query, ab[x - 1].0, ab[x - 1].1));
            add[x - 1] = false;
        }
        if query == 2 {
            input! {
                u:usize,
                v:usize,
            }
            qq.push((query, u, v));
        }
    }

    for i in 0..m {
        if add[i] {
            dsu.merge(ab[i].0, ab[i].1);
        }
    }

    let mut ans = vec![];
    for &(q, u, v) in qq.iter().rev() {
        if q == 1 {
            dsu.merge(u, v);
        }
        if q == 2 {
            if dsu.same(u, v) {
                ans.push("Yes");
            } else {
                ans.push("No");
            }
        }
    }

    for ans in ans.iter().rev() {
        println!("{ans}");
    }
}
