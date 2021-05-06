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
      h:usize,
      w:usize,
      rows:[[isize;w];h]
    }

    let mut memo = vec![vec![vec![0;w];h];h+w+1];
    let limit = h+w;
    for i in 0..h {
      for j in 0..w {
        if 0 < i {
          for k in 0..limit {
            memo[k][i][j] = std::cmp::max(memo[k][i][j], memo[k][i-1][j]);
          }
        }
        if 0 < j {
          for k in 0..limit {
            memo[k][i][j] = std::cmp::max(memo[k][i][j], memo[k][i][j-1]);
          }
        }
        for k in (0..limit).rev() {
          memo[k+1][i][j] = std::cmp::max(memo[k+1][i][j], memo[k][i][j]+rows[i][j]);
        }
      }
    }

    for i in 1..h+w {
      println!("{}", memo[i][h-1][w-1]);
    }
}
