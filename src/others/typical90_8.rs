use petgraph::unionfind::UnionFind;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

const MOD:usize = 1_000_000_007;
pub fn main(
    // #[rustfmt::skip] source: proconio::source::line::LineSource<std::io::BufReader<&[u8]>>,
) {
    input! {
      // from source,    // NEED TO BE COMMENT OUT WHEN SUBMIT
      n:usize,
      s:Chars
    }
    let base = "atcoder".chars().collect::<Vec<char>>();
    let mut dict = HashMap::new();
    for i in 0..base.len() {
      dict.insert(base[i], i);
    }
    let mut memo = vec![vec![0;n+1];base.len()];
    for i in (0..n).rev() {
      let c = s[i];
      let ti = *dict.get(&c).unwrap_or(&100);
      for j in 0..base.len() {
        memo[j][i] = memo[j][i+1];
      }
      if ti == 100 { continue }

      if c == 'r' {
        memo[ti][i] += 1;
        memo[ti][i] %= MOD;
      } else {
        memo[ti][i] += memo[ti+1][i];
        memo[ti][i] %= MOD;
      }
    }

    println!("{}", memo[0][0]);
}

