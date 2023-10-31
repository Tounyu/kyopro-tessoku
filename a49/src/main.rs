use std::cmp::min;

use proconio::input;

fn beam_search(t: usize, pqr: &Vec<(usize, usize, usize)>) -> Vec<char> {
    let width = 10000;
    let n = 20;

    let base_state = State::new(0, vec![0isize; 29], '-', 0);
    let mut beam = vec![vec![base_state; width]; 109];

    let mut num_state = vec![0usize; 109];
    num_state[0] = 1;
    beam[0][0].score = 0;
    for i in 1..=n {
        beam[0][0].x[i] = 0;
    }

    for i in 1..=t {
        let p = pqr[i - 1].0;
        let q = pqr[i - 1].1;
        let r = pqr[i - 1].2;

        let mut candidate = vec![];
        for j in 0..num_state[i - 1] {
            let mut sousa_a = beam[i - 1][j].clone();
            sousa_a.last_move = 'A';
            sousa_a.last_pos = j;
            sousa_a.x[p] += 1;
            sousa_a.x[q] += 1;
            sousa_a.x[r] += 1;
            for k in 1..=n {
                if sousa_a.x[k] == 0 { sousa_a.score += 1; }
            }

            let mut sousa_b = beam[i - 1][j].clone();
            sousa_b.last_move = 'B';
            sousa_b.last_pos = j;
            sousa_b.x[p] -= 1;
            sousa_b.x[q] -= 1;
            sousa_b.x[r] -= 1;
            for k in 1..=n {
                if sousa_b.x[k] == 0 { sousa_b.score += 1; }
            }

            candidate.push(sousa_a);
            candidate.push(sousa_b);
        }

        candidate.sort_by(|a, b| b.score.cmp(&a.score));
        num_state[i] = min(width, candidate.len());
        for j in 0..num_state[i] {
            beam[i][j] = candidate[j].clone();
        }
    }

    let mut ans = vec![];
    let mut current = 0usize;
    for i in (1..=t).rev() {
        ans.push(beam[i][current].last_move.clone());
        current = beam[i][current].last_pos;
    }
    ans
}

fn main() {
    input! {
        t:usize,
        pqr:[(usize, usize,usize); t],
    }

    let ans = beam_search(t, &pqr);

    for ans in ans {
        println!("{ans}")
    }
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct State {
    score: usize,
    x: Vec<isize>,
    last_move: char,
    last_pos: usize,
}

impl State {
    fn new(score: usize, x: Vec<isize>, last_move: char, last_pos: usize) -> Self {
        Self {
            score,
            x,
            last_move,
            last_pos,
        }
    }
}
