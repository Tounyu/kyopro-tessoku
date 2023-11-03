use proconio::input;

fn main() {
    input! {
        w:usize,
    }

    let m = 1000000007usize;

    let s: Vec<usize> = format!("{:b}", w - 1).chars().map(|a| a.to_digit(10).unwrap() as usize).collect();

    let mut ans = 12;

    let mut pow7 = vec![7usize; 60];
    for i in 1..60 {
        pow7[i] = (pow7[i - 1] * pow7[i - 1]) % m;
    }

    // println!("{:?}", pow7);
    // println!("{:?}", s);
    let mut i = 0usize;
    for &keta in s.iter().rev() {
        if keta == 1 {
            ans = (ans * pow7[i]) % m;
        }
        i += 1;
    }

    println!("{}", ans);
}
