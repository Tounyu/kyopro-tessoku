use std::cmp::min;

use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        abc:[(usize,usize,usize); m],
    }

    let mut Z = MaximumFlow { size: 0, used: vec![], g: vec![] };
    Z.init(n);

    for (a, b, c) in abc {
        Z.add_edge(a - 1, b - 1, c);
    }

    println!("{}", Z.max_flow(0, n - 1));
}

struct Edge {
    to: usize,
    cap: usize,
    rev: usize,
}

struct MaximumFlow {
    size: usize,
    used: Vec<bool>,
    g: Vec<Vec<Edge>>,
}

impl MaximumFlow {
    fn init(&mut self, n: usize) {
        self.size = n;
        for _i in 0..self.size {
            self.g.push(vec![]);
            self.used.push(false);
        }
    }

    fn add_edge(&mut self, a: usize, b: usize, c: usize) {
        let &current_ga = &self.g[a].len();
        let &current_gb = &self.g[b].len();
        &self.g[a].push(Edge { to: b, cap: c, rev: current_gb });
        &self.g[b].push(Edge { to: a, cap: 0, rev: current_ga });
    }

    fn dfs(&mut self, pos: usize, goal: usize, F: usize) -> usize {
        if pos == goal {
            return F;
        }
        self.used[pos] = true;
        for i in 0..self.g[pos].len() {
            if self.g[pos][i].cap == 0 {
                continue;
            }
            if self.used[self.g[pos][i].to] {
                continue;
            }

            let flow = self.dfs(self.g[pos][i].to, goal, min(F, self.g[pos][i].cap));
            if flow > 0 {
                self.g[pos][i].cap -= flow;
                let to = self.g[pos][i].to;
                let rev = self.g[pos][i].rev;
                self.g[to][rev].cap += flow;
                return flow;
            }
        }

        0
    }

    fn max_flow(&mut self, s: usize, t: usize) -> usize {
        let mut total_flow = 0usize;
        loop {
            for i in 0..self.size {
                self.used[i] = false;
            }
            let F = self.dfs(s, t, 1000000000);
            if F == 0 {
                break;
            }
            total_flow += F;
        }
        total_flow
    }
}
