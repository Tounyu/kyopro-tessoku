use proconio::input;

fn main() {
    input! {
        N:usize,
        A:[[usize; N]; N],
        Q:usize,
    }

    let mut I = vec![0usize; N];
    for i in 0..N {
        I[i] = i;
    }

    let mut ans = vec![];
    for i in 0..Q {
        input! {
            q:usize,
            x:usize,
            y:usize,
        }

        if q == 1 {
            let temp = I[x - 1];
            I[x - 1] = I[y - 1];
            I[y - 1] = temp;
        }
        if q == 2 {
            ans.push(A[I[x - 1]][y - 1]);
        }
    }

    for ans in ans {
        println!("{}", ans);
    }
}
