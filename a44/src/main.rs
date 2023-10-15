use proconio::input;

fn main() {
    input! {
        N:usize,
        Q:usize,
    }

    let mut memo = vec![0; N];
    let mut is_asc = true;
    let mut ans = vec![];
    for i in 0..N {
        memo[i] = i + 1;
    }

    // println!("{:?}", memo);

    for _i in 0..Q {
        input! {
            q1: i32,
        }

        if q1 == 1 {
            input! {
                x: usize,
                y: usize,
            }
            if is_asc {
                memo[x - 1] = y;
            } else {
                memo[N - x] = y;
            }
        } else if q1 == 2 {
            is_asc = !is_asc
        } else {
            input! {
                x: usize,
            }
            if is_asc {
                ans.push(memo[x - 1]);
            } else {
                ans.push(memo[N - x]);
            }
        }
    }

    for ans in ans {
        println!("{}", ans);
    }
}
