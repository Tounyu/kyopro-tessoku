use proconio::input;

fn main() {
    input! {
        n:usize,
        x:usize,
        y:usize,
        a:[usize;n],
    }

    let mut grundy = vec![0; 100009];

    for i in 0..100009 {
        let mut tmp = vec![0, 0, 0];
        if i >= x {
            tmp[grundy[i - x]] = 1;
        }
        if i >= y {
            tmp[grundy[i - y]] = 1;
        }

        if tmp[0] == 0 {
            grundy[i] = 0;
        } else if tmp[1] == 0 {
            grundy[i] = 1;
        } else {
            grundy[i] = 2;
        }
    }

    let mut ans = 0;
    for i in 0..n {
        ans = ans ^ grundy[a[i] % 100000];
    }

    if ans == 0 {
        println!("Second");
    } else {
        println!("First");
    }
}