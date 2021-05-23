/* OUTPUT FILE */
#![allow(unused_imports)]
use petgraph::algo::dijkstra;
use petgraph::graph::{DiGraph, NodeIndex, UnGraph};
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

type Target = usize;
type UseValue = usize;
fn upper_bound(arr: &Vec<Target>, x: &UseValue) -> usize {
    let mut low = 0;
    let mut high = arr.len();
    while low != high {
        let mid = (low + high) / 2;
        // NEEDS TO EDIT
        match arr[mid].cmp(x) {
            std::cmp::Ordering::Less | std::cmp::Ordering::Equal => {
                low = mid + 1;
            }
            std::cmp::Ordering::Greater => {
                high = mid;
            }
        }
    }
    low
}

fn lower_bound(arr: &Vec<Target>, x: &UseValue) -> usize {
    let mut low = 0;
    let mut high = arr.len();
    while low != high {
        let mid = (low + high) / 2;
        // NEEDS TO EDIT
        match arr[mid].cmp(x) {
            std::cmp::Ordering::Less => {
                low = mid + 1;
            }
            std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => {
                high = mid;
            }
        }
    }
    low
}

struct Helper {
    g: Vec<Vec<usize>>,
    tours: Vec<usize>,
    depths: Vec<usize>,
    count: usize,
}

impl Helper {
    fn dfs(&mut self, ci: usize, last: usize, depth: usize) {
        self.tours[self.count] = ci;
        self.count += 1;
        self.depths[ci] = depth;

        for i in 0..self.g[ci].len() {
            let ni = self.g[ci][i];
            if last == ni {
                continue;
            }
            self.dfs(ni, ci, depth + 1);
        }
        self.tours[self.count] = ci;
        self.count += 1;
    }
}

pub fn main(
) {
    input! {
      n:usize,
      ps:[Usize1;n-1],
      q:usize,
      queries:[(Usize1,usize);q]
    }

    let mut g = vec![vec![]; n];
    for i in 0..n - 1 {
        g[ps[i]].push(i + 1);
        g[i + 1].push(ps[i]);
    }

    let tours = vec![0; 2 * n];
    let depths = vec![0; n];

    let mut helper = Helper {
        g,
        tours,
        depths,
        count: 0,
    };
    helper.dfs(0, 1_000_000_000, 0);

    let mut memo = vec![vec![]; n];
    for (i, &id) in helper.tours.iter().enumerate() {
        memo[id].push(i);
    }
    let mut targets = vec![vec![]; n + 1];
    for i in 0..n {
        let dv = helper.depths[i];
        targets[dv].push(memo[i][0]);
    }
    for i in 0..=n {
        targets[i].sort();
    }

    for (ti, dv) in queries {
        let start = memo[ti][0];
        let end = memo[ti][1];

        let left = lower_bound(&targets[dv], &start);
        let right = upper_bound(&targets[dv], &end);
        println!("{}", right - left);
    }
}
