use petgraph::unionfind::UnionFind;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

pub fn main(
    // #[rustfmt::skip] source: proconio::source::line::LineSource<std::io::BufReader<&[u8]>>,
) {
    input! {
      // from source,    // NEED TO BE COMMENT OUT WHEN SUBMIT
      n:usize,
      m:usize,
      k:usize,
      q:usize,
      vals:[(usize,usize);n]
    }

    let mut free = vec![];
    let mut needs = vec![];

    for (v, t) in vals {
      if t == 1 {
        needs.push(v);
      } else {
        free.push(v);
      }
    }
    free.sort();
    needs.sort();

    let mut free_memo = vec![0;free.len()+1];
    for i in 0..free.len() {
      free_memo[i+1] += free_memo[i] + free[i];
    }
    let mut needs_memo = vec![0;needs.len()+1];
    for i in 0..needs.len() {
      needs_memo[i+1] += needs_memo[i] + needs[i];
    }
    let mut min = std::usize::MAX;
    for i in 0..=m {
      let j = m - i;
      if free_memo.len() <= i || needs_memo.len() <= j { continue }
      let v = free_memo[i] + needs_memo[j] + (j+k-1) / k * q;
      min = std::cmp::min(min, v);
    }
    
    println!("{}", min);
}
