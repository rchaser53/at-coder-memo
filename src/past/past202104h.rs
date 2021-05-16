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

    let mut abs = vec![vec![];2];
    for (v,t) in vals {
      abs[t].push(v);
    }

    abs[0].sort();
    let mut memo_a = vec![0;abs[0].len()+1];
    for i in 0..abs[0].len() {
      memo_a[i+1] = memo_a[i] + abs[0][i];
    }

    abs[1].sort();
    let mut memo_b = vec![0;abs[1].len()+1];
    for i in 0..abs[1].len() {
      memo_b[i+1] = memo_b[i] + abs[1][i];
    }

    let mut min = std::usize::MAX;
    for i in 0..=m {
      let j = m - i;
      if memo_a.len() <= i || memo_b.len() <= j { continue }
      let mut num = j / k;
      if j % k != 0 {
        num += 1;
      }
      let v = memo_a[i] + memo_b[j] + num * q;
      min = std::cmp::min(min, v);
    }
    println!("{}", min);
}
