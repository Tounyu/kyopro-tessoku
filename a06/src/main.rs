use proconio::input;

fn main() {
    input! {
        n:usize,
        q:usize,
        a:[i32; n],
    }

    let mut sum = 0;
    let mut s = vec![];
    s.push(0);
    for av in a {
        sum += av;
        s.push(sum);
    }

    for _i in 1..=q {
        input! {
            l:usize,
            r:usize,
        }
        println!("{}", s[r] - s[l - 1]);
    }
}
