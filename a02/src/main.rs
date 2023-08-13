use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).ok();
    let vec1: Vec<&str> = buf.split_whitespace().collect();
    let x:i32 = vec1[1].parse().unwrap();

    buf = "".to_string();
    io::stdin().read_line(&mut buf).ok();
    let vec2: Vec<&str> = buf.split_whitespace().collect();

    for val in &vec2 {
        let v:i32 = val.parse().unwrap();
        if v == x {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
