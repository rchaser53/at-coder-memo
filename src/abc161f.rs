use proconio::fastout;
use proconio::input;
use std::collections::*;

fn culc(
  n:usize
) -> HashSet<usize> {
  let mut set = HashSet::new();
  let mut i = 1;
  while i * i <= n {
    if n % i == 0 {
      set.insert(i);
      set.insert(n/i);
    }
    i += 1;
  }
  set.remove(&1);
  set
}

#[fastout]
fn main() {
  input! {
    n:usize
  }

  let mut base_set = culc(n-1);
  let set = culc(n);  

  for v in set {
    let mut a = n;
    while a % v == 0 {
      a /= v;
    }
    if a % v == 1 {
      base_set.insert(v);
    }
  }
  println!("{}", base_set.len());
}