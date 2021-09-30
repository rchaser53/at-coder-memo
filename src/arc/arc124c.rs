use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn gcd(a: usize, b: usize) -> usize {
  if b == 0 {
    return a
  }
  gcd(b, a % b)
}

fn lcm(a: usize, b: usize) -> usize {
  a * b / gcd(a, b)
}

fn main() {
  input!{
    n:usize,
    vals:[(usize,usize);n]
  }
  
  let mut set = HashSet::new();
  set.insert(vals[0]);
  for (a,b) in vals {
    let mut new_set = HashSet::new();
    for (x, y) in set {
      new_set.insert((gcd(x,a), gcd(y, b)));
      new_set.insert((gcd(x,b), gcd(y, a)));
    }
    set = new_set;
  }
  println!("{}", set.iter().map(|&(a,b)| lcm(a,b)).max().unwrap());
}