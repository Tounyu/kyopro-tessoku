use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).ok();
    let vec: Vec<&str> = buf.split_whitespace().collect();
    
    let a:i32 = vec[0].parse().unwrap();
    let b:i32 = vec[1].parse().unwrap();
    println!("{}",  a + b);
}
