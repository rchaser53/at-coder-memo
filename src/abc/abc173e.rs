/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

const MOD:isize = 1_000_000_007;
fn main() {
  input!{
    n:usize,
    k:usize,
    mut vals:[isize;n]
  }

  vals.sort();

  let mut result = 1isize;

  if n == k || (k % 2 == 1 && vals.iter().all(|&av| av <= 0)) {
    for i in n-k..n {
      result *= vals[i];
      result %= MOD;
    }
    result += MOD;
    result %= MOD;
    println!("{}", result);
    return
  }

  let mut min_eq = 0;
  let mut max_eq = n - 1;
  let mut i = 0;
  while i < k - 1 {
    let max_v = vals[max_eq] * vals[max_eq-1];
    let min_v = vals[min_eq] * vals[min_eq+1];
    if min_v < max_v {
      result *= vals[max_eq];
      max_eq -= 1;
      i += 1;
    } else  {
      result *= min_v % MOD;
      min_eq += 2;
      i += 2;
    }
    result %= MOD;
  }
  
  if i == k - 1 {
    result *= vals[max_eq];
    result += MOD;
    result %= MOD;
  }

  println!("{}", result);

}