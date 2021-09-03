use proconio::input;
use proconio::marker::*;
use std::collections::*;

const MOD:usize = 998244353;

fn main() {
  input!{
    n:usize,
    a:[usize;n],
    b:[usize;n]
  }

  let mut ai = a.into_iter().enumerate().collect::<Vec<(usize,usize)>>();
  ai.sort_by(|a,b|a.1.cmp(&b.1));

  let mut ass = vec![0;n];
  let mut bss = vec![0;n];
  for i in 0..n {
    let (ti, v) = ai[i];
    ass[i] = v;
    bss[i] = b[ti];
  }

  let limit = 5001;
  let mut result = 0usize;
  let mut memo = vec![0;limit];
  for i in 0..n {
    let av = ass[i];
    let bv = bss[i];
    let mut new_memo = memo.clone();
    let mut for_culc = vec![0;limit];
    for i in 0..limit {
      let nv = bv+i;
      if limit <= nv { break }
      new_memo[nv] += memo[i];
      new_memo[nv] %= MOD;
      for_culc[nv] += memo[i];
      for_culc[nv] %= MOD;
    }
    new_memo[bv] += 1;
    new_memo[bv] %= MOD;
    for_culc[bv] += 1;
    for_culc[bv] %= MOD;

    for i in 0..=av {
      result += for_culc[i];
      result %= MOD;
    }
    memo = new_memo;
  }
  println!("{}", result);
}