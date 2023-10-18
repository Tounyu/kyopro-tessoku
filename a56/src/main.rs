use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        N:usize,
        Q:usize,
        S:Chars,
        abcd:[(usize,usize,usize,usize); Q],
    }

    let m = 2147483647i64;

    let T: Vec<i64> = S.iter().map(|&x| (x as u8 - 'a' as u8) as i64 + 1).collect();
    // println!("{:?}", T);

    let mut pow100 = vec![1i64];
    for n in 1..=N {
        pow100.push(100 * pow100[n - 1] % m);
    }
    // println!("{:?}", pow100);

    let mut H = vec![0i64];
    for i in 1..=N {
        H.push((100 * H[i - 1] + T[i - 1]) % m);
    }

    for (a, b, c, d) in abcd {
        let hash1 = hash(a, b, &H, &pow100, m);
        let hash2 = hash(c, d, &H, &pow100, m);
        if hash1 == hash2 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

fn hash(l: usize, r: usize, H: &Vec<i64>, pow100: &Vec<i64>, m: i64) -> i64 {
    let mut v = H[r] - (H[l - 1] * pow100[r - l + 1] % m);
    if v < 0 {
        v += m;
    }
    v
}