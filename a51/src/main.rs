use proconio::input;

fn main() {
    input! {
        Q:usize,
    }

    let mut books = vec![];
    let mut ans = vec![];
    for _i in 0..Q {
        input! {
            q1: i32,
        }
        if q1 == 1 {
            input! {
                s:String,
            }
            books.push(s);
        }
        if q1 == 2 {
            let s = format!("{}", books[books.len() - 1]);
            ans.push(s);
        }
        if q1 == 3 {
            books.pop();
        }
    }
    for ans in ans {
        println!("{}", ans);
    }
}
