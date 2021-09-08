use proconio::input;
use proconio::marker::*;
use std::collections::*;

const MOD:usize = 1_000_000_007;
fn main() {
  input!{
    n:usize,
    s:Chars
  }

  let mut map = HashMap::new();
  for c in s {
    *map.entry(c).or_insert(0) += 1;
  }
  let mut result = 1usize;
  for (_, v) in map {
    result *= v + 1;
    result %= MOD;
  }
  result = (MOD + result - 1) % MOD;
  println!("{}", result);
}