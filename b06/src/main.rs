use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[i32; n],
        q:usize,
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

        let c = (r - l + 1) as i32;
        let wins = s[r] - s[l - 1];

        if wins * 2 > c {
            println!("win");
        } else if wins * 2 == c {
            println!("draw");
        } else {
            println!("lose");
        }
    }
}
