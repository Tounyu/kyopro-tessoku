use proconio::input;

fn main() {
    input! {
        N:usize,
        XY:[(i64,i64); N],
    }

    let mut ans: Vec<usize> = vec![];
    let mut visit = vec![0; N];
    ans.push(0);
    visit[0] = 1;

    for i in 0..N - 1 {
        let mut mindist = 10000000.0;
        let mut next = 0;

        for j in 0..N {
            if visit[j] > 0 {
                continue;
            }
            let dist = dist(XY[ans[i]], XY[j]);
            if dist < mindist {
                mindist = dist;
                next = j;
            }
        }

        ans.push(next);
        visit[next] = 1;

        // println!("{:?}", visit);
    }
    ans.push(0);

    for ans in ans {
        println!("{}", ans + 1);
    }
}

fn dist(a: (i64, i64), b: (i64, i64)) -> f64 {
    return (((a.0 - b.0).pow(2) + (a.1 - b.1).pow(2)) as f64).sqrt();
}
