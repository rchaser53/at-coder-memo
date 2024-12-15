/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    mut s:isize,
    a:[isize;n]
  }

  let mut set = HashSet::new();
  let mut memo = vec![0;2*n+1];
  for i in 0..2*n {
    memo[i+1] = memo[i] + a[i%n];
  }
  for &v in &memo {
    set.insert(v);
  }

  s %= memo[n];
  for i in 0..n {
    if a[i] == s {
      println!("Yes");
      return      
    }
  }

  for i in 0..2*n {
    if set.contains(&s) {
      println!("Yes");
      return
    }
    let v = memo[i+1];
    set.remove(&v);
    s += a[i%n];
  }

  println!("No");
}