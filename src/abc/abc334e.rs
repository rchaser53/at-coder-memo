/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

const MOD:usize = 998244353;
fn mod_pow(a: usize, n: usize) -> usize { 
  if n == 0 {
    1
  } else if n == 1 {
    a % MOD
  } else if n % 2 == 1 {
    let k = mod_pow(a, ((n - 1) / 2));
    let mut ans = (a * k) % MOD;
    (ans * k) % MOD
  } else {
    let k = mod_pow(a, (n / 2));
    (k * k) % MOD
  }
}

// a^{-1} mod を計算する
fn mod_inv(a:usize) -> usize {
  mod_pow(a, MOD-2)
}

fn main() {
  input! {
    h:usize,
    w:usize,
    rows:[Chars;h]
  }

  let mut memo = vec![vec![0;w];h];
  let default = 1_000_000_000;
  let mut red = 0;
  for i in 0..h {
    for j in 0..w {
      if rows[i][j] == '#' {
        memo[i][j] = default;
      } else {
        red += 1;
      }
    }
  }

  let denominator = mod_inv(red);

  let mut count = 1;
  for i in 0..h {
    for j in 0..w {
      if memo[i][j] != default { continue }
      memo[i][j] = count;
      let mut stack = vec![(i,j)];
      while let Some((ci,cj)) = stack.pop() {
        if 0 < ci && memo[ci-1][cj] == default {
          memo[ci-1][cj] = count;
          stack.push((ci-1,cj));
        }
        if ci < h-1 && memo[ci+1][cj] == default {
          memo[ci+1][cj] = count;
          stack.push((ci+1,cj));
        }
        if 0 < cj && memo[ci][cj-1] == default {
          memo[ci][cj-1] = count;
          stack.push((ci,cj-1));
        }
        if cj < w-1 && memo[ci][cj+1] == default {
          memo[ci][cj+1] = count;
          stack.push((ci,cj+1));
        }
      }
      count += 1;
    }
  }

  let base_num = count - 1;
  let mut result = 0;
  for i in 0..h {
    for j in 0..w {
      if memo[i][j] != 0 { continue }
      let mut set = HashSet::new();
      if 0 < i && memo[i-1][j] != 0 {
        set.insert(memo[i-1][j]);
      }
      if i < h-1 && memo[i+1][j] != 0 {
        set.insert(memo[i+1][j]);
      }
      if 0 < j && memo[i][j-1] != 0 {
        set.insert(memo[i][j-1]);
      }
      if j < w-1 && memo[i][j+1] != 0 {
        set.insert(memo[i][j+1]);
      }
      let v = base_num + 1 - set.len();
      result += v * denominator;
      result %= MOD;
    }
  }

  println!("{}", result);
  
}