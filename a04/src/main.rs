use proconio::input;

fn main() {
    input! {
        n:i32,
    }

    let mut ans = String::from("");
    let mut div = 1;
    for _i in 0..=9 {
        let keta:i32 = n / div % 2;
        ans = format!("{}{}",keta, ans);
        div *= 2;
    }

    println!("{}", ans);
}
