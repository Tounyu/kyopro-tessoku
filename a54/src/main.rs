use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        Q:usize,
    }

    let mut arr = HashMap::new();

    for i in 0..Q {
        input! {
            q1: i32,
        }

        if q1 == 1 {
            input! {
                s:String,
                v:i32,
            }
            arr.insert(s, v);
        }
        if q1 == 2 {
            input! {
                s:String,
            }
            println!("{}", arr.get(&s).unwrap());
        }
    }
}
