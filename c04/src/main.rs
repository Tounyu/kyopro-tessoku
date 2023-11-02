use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    let mut del = vec![false; 10000000];
    for i in 1..10000000usize {
        if i * i > n {
            break;
        }
        if del[i] {
            continue;
        }
        if n % i == 0 {
            continue;
        }
        let mut j = i;
        while j < 10000000usize {
            del[j] = true;
            j += i;
        }
    }

    let mut ans = vec![];
    for i in 1..10000000usize {
        if i * i > n {
            break;
        }
        if !del[i] {
            ans.push(i);
            ans.push(n / i);
        }
    }
    ans.sort();
    for ans in ans {
        println!("{ans}");
    }
}
