use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n:usize,
        st:[(Chars, usize);n],
    }

    let mut ans = vec![];
    for i in 0..10000 {
        let atari: Vec<char> = format!("{:04}", i).chars().collect();
        let ok = st.iter().fold(true, |acc, (s, t)| acc && (check(s, &atari) == *t));
        if ok {
            ans.push(i);
        }
    }

    if ans.len() != 1 {
        println!("Can't Solve");
    } else {
        println!("{:04}", ans[0]);
    }
}

fn check(s: &Vec<char>, atari: &Vec<char>) -> usize {
    if s == atari {
        return 1;
    } else {
        let diff = s.iter().zip(atari.iter()).filter(|&(a, b)| *a != *b).count();
        if diff == 1 {
            return 2;
        }
    }
    return 3;
}
