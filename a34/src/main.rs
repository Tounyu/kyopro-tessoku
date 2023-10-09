use proconio::input;

fn main() {
    input! {
        n:usize,
        x:usize,
        y:usize,
        a:[usize;n],
    }

    let mut grundy = vec![0i64; 100009];

    for i in 0..100009 {
        let mut tmp = vec![];
        if i >= x {
            tmp.push(grundy[i - x]);
        }
        if i >= y {
            tmp.push(grundy[i - y]);
        }
        tmp.sort();
        tmp.dedup();

        if tmp.len() == 0 {
            grundy[i] = 0;
            continue;
        }

        for j in 0..3i64 {
            if tmp.iter().find(|&&t| t == j) == None {
                grundy[i] = j;
                break;
            }
        }
    }

    let mut ans = grundy[a[0]];
    for i in 1..n {
        ans ^= grundy[a[i]];
    }

    if ans != 0 {
        println!("First");
    } else {
        println!("Second");
    }
}
