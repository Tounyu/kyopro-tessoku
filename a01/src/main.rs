use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).ok();
    let num: i32 = buffer.trim().parse().unwrap();
    println!("{}", num*num);
}
