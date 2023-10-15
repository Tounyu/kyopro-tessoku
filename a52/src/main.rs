use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        Q:usize,
    }

    let mut queue = VecDeque::new();
    // let mut ans = vec![];

    for _i in 0..Q {
        input! {
            q1:i32,
        }
        if q1 == 1 {
            input! {
                s:String,
            }
            queue.push_back(s);
        }
        if q1 == 2 {
            println!("{}", queue[0]);
            // ans.push(format!("{}", ));
        }
        if q1 == 3 {
            queue.pop_front();
        }
    }
    //
    // for ans in ans {
    //     println!("{}", ans);
    // }
}
