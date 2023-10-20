use proconio::input;

fn main() {
    input! {
        N:usize,
        M:usize,
        AB:[(usize, usize); M],
    }
    let mut g = vec![vec![]; N];
    for (a, b) in AB {
        g[a - 1].push(b - 1);
        g[b - 1].push(a - 1);
    }

    for i in 0..N {
        g[i].sort();
        print!("{}: {{", i + 1);
        print!("{}", g[i].iter().map(|&x| (x + 1).to_string()).collect::<Vec<String>>().join(", "));
        println!("}}")
    }
}
