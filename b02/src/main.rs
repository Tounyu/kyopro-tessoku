use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).ok();
    let vec1: Vec<&str> = buf.split_whitespace().collect();
    let a:i32 = vec1[0].parse().unwrap();
    let b:i32 = vec1[1].parse().unwrap();

    let mut ans = false;
    for n in a..=b {
        if (100 % n) == 0 {
            ans = true;
        }
    }

    if ans {
        println!("Yes");
    } else {
        println!("No");
    }

}
